use std::collections::BTreeSet;
use std::mem;

use anchor_lang::prelude::AccountInfo;
use drift::{
    ids::{pyth_program, switchboard_program},
    instructions::optional_accounts::AccountMaps,
    state::{
        oracle::OracleSource, oracle_map::OracleMap, perp_market::PerpMarket,
        perp_market_map::PerpMarketMap, spot_market::SpotMarket, spot_market_map::SpotMarketMap,
        user::User,
    },
};
use fnv::FnvHashSet;
use solana_sdk::{account::Account, pubkey, pubkey::Pubkey};

use crate::{
    constants, utils::zero_account_to_bytes, AccountProvider, DriftClient, SdkError, SdkResult,
};

pub(crate) type MarketSet = BTreeSet<u16>;

pub const PYTH_PULL_ID: Pubkey = pubkey!("rec5EKMGg6MxZYaMdyBfgwp4d5rB9T1VQH5pJv5LtFJ");

/// Builds an AccountMap of relevant spot, perp, and oracle accounts from rpc
#[derive(Default)]
pub(crate) struct AccountMapBuilder {
    /// placeholder account values populated with real market & oracle account data
    accounts: Vec<(Pubkey, Account)>,
}

impl AccountMapBuilder {
    /// Constructs the account map + drift state account
    pub fn build<T: AccountProvider>(
        &mut self,
        client: &DriftClient<T>,
        user: &User,
    ) -> SdkResult<AccountMaps> {
        let mut oracles = FnvHashSet::<Pubkey>::default();
        let mut spot_markets = Vec::<SpotMarket>::with_capacity(user.spot_positions.len());
        let mut perp_markets = Vec::<PerpMarket>::with_capacity(user.perp_positions.len());

        for p in user.spot_positions.iter().filter(|p| !p.is_available()) {
            let market = client
                .get_spot_market_account(p.market_index)
                .expect("spot market");
            oracles.insert(market.oracle);
            self.accounts.push((market.pubkey, Account::default()));
            spot_markets.push(market);
        }

        let quote_market = client.get_spot_market_account(0).expect("spot market");
        oracles.insert(quote_market.oracle);

        for p in user.perp_positions.iter().filter(|p| !p.is_available()) {
            let market = client
                .get_perp_market_account(p.market_index)
                .expect("perp market");
            oracles.insert(market.amm.oracle);
            self.accounts.push((market.pubkey, Account::default()));
            perp_markets.push(market);
        }

        self.accounts
            .extend(oracles.iter().map(|x| (*x, Default::default())));
        let mut accounts_iter = self.accounts.iter_mut();

        let mut spot_accounts = Vec::<AccountInfo>::with_capacity(spot_markets.len());
        for market in spot_markets.iter() {
            let (pubkey, account) = accounts_iter.next().unwrap();
            account.data = zero_account_to_bytes(*market);
            spot_accounts.push(AccountInfo::new(
                pubkey,
                false,
                false,
                &mut account.lamports,
                &mut account.data[..],
                &constants::PROGRAM_ID,
                false,
                0,
            ));
        }

        let mut perp_accounts = Vec::<AccountInfo>::with_capacity(perp_markets.len());
        for market in perp_markets.iter() {
            let (pubkey, account) = accounts_iter.next().unwrap();
            account.data = zero_account_to_bytes(*market);
            perp_accounts.push(AccountInfo::new(
                pubkey,
                false,
                false,
                &mut account.lamports,
                &mut account.data[..],
                &constants::PROGRAM_ID,
                false,
                0,
            ));
        }

        let mut oracle_accounts = Vec::<AccountInfo>::with_capacity(oracles.len());
        for oracle_key in oracles.iter() {
            let oracle = client
                .backend
                .oracle_map
                .get(oracle_key)
                .expect("oracle exists");
            let owner = match oracle.source {
                OracleSource::Pyth
                | OracleSource::Pyth1K
                | OracleSource::Pyth1M
                | OracleSource::PythStableCoin => &pyth_program::ID,
                OracleSource::PythPull
                | OracleSource::Pyth1KPull
                | OracleSource::Pyth1MPull
                | OracleSource::PythStableCoinPull => &PYTH_PULL_ID,
                OracleSource::Switchboard => &switchboard_program::ID,
                OracleSource::QuoteAsset => &constants::DEFAULT_PUBKEY,
                OracleSource::Prelaunch => &drift::ID,
            };
            let (pubkey, account) = accounts_iter.next().unwrap();
            account.data.clone_from(&oracle.raw);
            oracle_accounts.push(AccountInfo::new(
                pubkey,
                false,
                false,
                &mut account.lamports,
                &mut account.data[..],
                owner,
                false,
                0,
            ));
        }

        let perp_slot = client.backend.perp_market_map.get_latest_slot();
        let spot_slot = client.backend.spot_market_map.get_latest_slot();
        let oracle_slot = client.backend.oracle_map.get_latest_slot();
        let slot = std::cmp::max(oracle_slot, std::cmp::max(perp_slot, spot_slot));

        let perp_market_map = unsafe {
            PerpMarketMap::load(
                &MarketSet::default(),
                mem::transmute(&mut perp_accounts.iter().peekable()),
            )
        }
        .map_err(|err| SdkError::Anchor(Box::new(err.into())))?;

        let spot_market_map = unsafe {
            SpotMarketMap::load(
                &MarketSet::default(),
                mem::transmute(&mut spot_accounts.iter().peekable()),
            )
        }
        .map_err(|err| SdkError::Anchor(Box::new(err.into())))?;
        let state_account = client.backend.state_account.read().unwrap();
        let oracle_map = OracleMap::load(
            &mut oracle_accounts.iter().peekable(),
            slot,
            Some(state_account.oracle_guard_rails),
        )
        .map_err(|err| SdkError::Anchor(Box::new(err.into())))?;

        Ok(AccountMaps {
            spot_market_map,
            perp_market_map,
            oracle_map,
        })
    }
}
