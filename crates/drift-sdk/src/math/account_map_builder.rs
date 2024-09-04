use fnv::FnvHashSet;
use solana_program::account_info::AccountInfo;
use solana_sdk::{account::Account, pubkey::Pubkey};

use crate::{
    constants::{self, ids, PROGRAM_ID},
    drift_abi::types::OracleSource,
    ffi::{AccountList, AccountsList, IntoFfi},
    types::User,
    utils::zero_account_to_bytes,
    AccountProvider, DriftClient, PerpMarket, SdkResult, SpotMarket,
};

/// Builds an AccountList of relevant spot, perp, and oracle accounts from rpc
#[derive(Default)]
pub(crate) struct AccountsListBuilder<'a> {
    /// placeholder account values populated with real market & oracle account data
    accounts: Vec<(Pubkey, Account)>,
    perp_accounts: Vec<AccountInfo<'a>>,
    spot_accounts: Vec<AccountInfo<'a>>,
    oracle_accounts: Vec<AccountInfo<'a>>,
}

impl<'a> AccountsListBuilder<'a> {
    /// Constructs the account map + drift state account
    pub fn build<T: AccountProvider>(
        &'a mut self,
        client: &DriftClient<T>,
        user: &User,
    ) -> SdkResult<AccountsList> {
        let mut oracles = FnvHashSet::<Pubkey>::default();
        let mut spot_markets = Vec::<SpotMarket>::with_capacity(user.spot_positions.len());
        let mut perp_markets = Vec::<PerpMarket>::with_capacity(user.perp_positions.len());

        for p in user
            .spot_positions
            .iter()
            .filter(|p| !p.ffi().is_available())
        {
            let market = client
                .get_spot_market_account(p.market_index)
                .expect("spot market");
            oracles.insert(market.oracle);
            self.accounts.push((market.pubkey, Account::default()));
            spot_markets.push(market);
        }

        let quote_market = client.get_spot_market_account(0).expect("spot market");
        if oracles.insert(quote_market.oracle) {
            spot_markets.push(quote_market);
        }

        for p in user
            .perp_positions
            .iter()
            .filter(|p| !p.ffi().is_available())
        {
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

        for market in spot_markets.iter() {
            let (pubkey, account) = accounts_iter.next().unwrap();
            account.data = zero_account_to_bytes(*market);
            self.spot_accounts.push(AccountInfo::new(
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

        for market in perp_markets.iter() {
            let (pubkey, account) = accounts_iter.next().unwrap();
            account.data = zero_account_to_bytes(*market);
            self.perp_accounts.push(AccountInfo::new(
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
                | OracleSource::PythStableCoin => &ids::pyth_program::ID,
                OracleSource::PythPull
                | OracleSource::Pyth1KPull
                | OracleSource::Pyth1MPull
                | OracleSource::PythStableCoinPull => &ids::drift_oracle_receiver_program::ID,
                OracleSource::Switchboard => &ids::switchboard_program::ID,
                OracleSource::SwitchboardOnDemand => &ids::switchboard_on_demand::ID,
                OracleSource::QuoteAsset => &constants::DEFAULT_PUBKEY,
                OracleSource::Prelaunch => &PROGRAM_ID,
            };
            let (pubkey, account) = accounts_iter.next().unwrap();
            account.data.clone_from(&oracle.raw);
            self.oracle_accounts.push(AccountInfo::new(
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

        let oracle_slot = client.backend.oracle_map.get_latest_slot();

        Ok(AccountsList {
            perp_markets: AccountList::new(self.perp_accounts.as_mut_slice()),
            spot_markets: AccountList::new(self.spot_accounts.as_mut_slice()),
            oracles: AccountList::new(self.oracle_accounts.as_mut_slice()),
            latest_slot: oracle_slot,
        })
    }
}
