use fnv::FnvHashSet;
use solana_sdk::{account::Account, pubkey::Pubkey};

use crate::{
    constants::{self, oracle_source_to_owner},
    ffi::{AccountWithKey, AccountsList},
    types::accounts::{PerpMarket, SpotMarket, User},
    utils::zero_account_to_bytes,
    DriftClient, MarketId, SdkResult,
};

/// Builds an AccountList of relevant spot, perp, and oracle accounts from rpc
#[derive(Default)]
pub(crate) struct AccountsListBuilder {
    /// placeholder account values populated with real market & oracle account data
    perp_accounts: Vec<AccountWithKey>,
    spot_accounts: Vec<AccountWithKey>,
    oracle_accounts: Vec<AccountWithKey>,
}

impl AccountsListBuilder {
    /// Constructs the account map + drift state account
    pub fn build(&mut self, client: &DriftClient, user: &User) -> SdkResult<AccountsList> {
        let mut oracles = FnvHashSet::<Pubkey>::default();
        let mut spot_markets = Vec::<SpotMarket>::with_capacity(user.spot_positions.len());
        let mut perp_markets = Vec::<PerpMarket>::with_capacity(user.perp_positions.len());
        let drift_state = client.state_config()?;

        for p in user.spot_positions.iter().filter(|p| !p.is_available()) {
            let market = client
                .get_spot_market_account(p.market_index)
                .expect("spot market");
            if oracles.insert(market.oracle) {
                spot_markets.push(market);
            }
        }

        let quote_market = client
            .get_spot_market_account(MarketId::QUOTE_SPOT.index())
            .expect("spot market");
        if oracles.insert(quote_market.oracle) {
            spot_markets.push(quote_market);
        }

        for p in user.perp_positions.iter().filter(|p| !p.is_available()) {
            let market = client
                .get_perp_market_account(p.market_index)
                .expect("perp market");
            oracles.insert(market.amm.oracle);
            perp_markets.push(market);
        }

        for market in spot_markets.iter() {
            self.spot_accounts.push(
                (
                    market.pubkey,
                    Account {
                        data: zero_account_to_bytes(*market),
                        owner: constants::PROGRAM_ID,
                        ..Default::default()
                    },
                )
                    .into(),
            );
        }

        for market in perp_markets.iter() {
            self.perp_accounts.push(
                (
                    market.pubkey,
                    Account {
                        data: zero_account_to_bytes(*market),
                        owner: constants::PROGRAM_ID,
                        ..Default::default()
                    },
                )
                    .into(),
            );
        }

        for oracle_key in oracles.iter() {
            let oracle = client
                .get_oracle_price_data_and_slot(oracle_key)
                .expect("oracle exists");

            let oracle_owner = oracle_source_to_owner(client.context, oracle.source);
            self.oracle_accounts.push(
                (
                    *oracle_key,
                    Account {
                        data: oracle.raw,
                        owner: oracle_owner,
                        ..Default::default()
                    },
                )
                    .into(),
            );
        }

        let oracle_slot = client.backend.oracle_map.get_latest_slot();

        Ok(AccountsList {
            perp_markets: self.perp_accounts.as_mut_slice(),
            spot_markets: self.spot_accounts.as_mut_slice(),
            oracles: self.oracle_accounts.as_mut_slice(),
            oracle_guard_rails: Some(drift_state.oracle_guard_rails),
            latest_slot: oracle_slot,
        })
    }
}
