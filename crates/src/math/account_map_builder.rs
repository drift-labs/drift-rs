use ahash::HashMap;
use solana_sdk::{account::Account, pubkey::Pubkey};

use crate::{
    accounts::State,
    constants::{self, oracle_source_to_owner, state_account},
    ffi::{AccountWithKey, AccountsList},
    types::accounts::{PerpMarket, SpotMarket, User},
    utils::zero_account_to_bytes,
    DriftClient, MarketId, SdkError, SdkResult,
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
    /// Constructs an account map from `user` positions
    ///
    /// It relies on the `client` being subscribed to all the necessary markets and oracles
    pub fn try_build(&mut self, client: &DriftClient, user: &User) -> SdkResult<AccountsList> {
        let mut oracle_markets =
            HashMap::<Pubkey, MarketId>::with_capacity_and_hasher(16, Default::default());
        let mut spot_markets = Vec::<SpotMarket>::with_capacity(user.spot_positions.len());
        let mut perp_markets = Vec::<PerpMarket>::with_capacity(user.perp_positions.len());
        let drift_state_account = client.try_get_account::<State>(state_account())?;

        for p in user.spot_positions.iter().filter(|p| !p.is_available()) {
            let market = client.try_get_spot_market_account(p.market_index)?;
            oracle_markets.insert(market.oracle, MarketId::spot(market.market_index));
            spot_markets.push(market);
        }

        let quote_market = client.try_get_spot_market_account(MarketId::QUOTE_SPOT.index())?;
        if oracle_markets
            .insert(quote_market.oracle, MarketId::QUOTE_SPOT)
            .is_none()
        {
            spot_markets.push(quote_market);
        }

        for p in user.perp_positions.iter().filter(|p| !p.is_available()) {
            let market = client.try_get_perp_market_account(p.market_index)?;
            oracle_markets.insert(market.amm.oracle, MarketId::perp(market.market_index));
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

        let mut latest_oracle_slot = 0;
        for (oracle_key, market) in oracle_markets.iter() {
            let oracle = client
                .try_get_oracle_price_data_and_slot(*market)
                .ok_or(SdkError::NoData)?;

            latest_oracle_slot = oracle.slot.max(oracle.slot);
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

        Ok(AccountsList {
            perp_markets: self.perp_accounts.as_mut_slice(),
            spot_markets: self.spot_accounts.as_mut_slice(),
            oracles: self.oracle_accounts.as_mut_slice(),
            oracle_guard_rails: Some(drift_state_account.oracle_guard_rails),
            latest_slot: latest_oracle_slot,
        })
    }
}
