use ahash::{HashMap, HashMapExt};
use arrayvec::ArrayVec;
use solana_sdk::{account::Account, pubkey::Pubkey};

use crate::{
    accounts::State,
    constants::{self, oracle_source_to_owner, state_account},
    ffi::{AccountWithKey, AccountsList},
    types::accounts::User,
    utils::zero_account_to_bytes,
    DriftClient, MarketId, SdkError, SdkResult,
};

/// Builds a list of users's associated spot, perp, and oracle accounts
///
/// ```example(no_run)
/// let mut builder = AccountsListBuilder::default();
/// let accounts_list = builder.try_build(client, user).expect("build accounts");
/// ```
#[derive(Default)]
pub struct AccountsListBuilder {
    /// placeholder account values populated with real market & oracle account data
    perp_accounts: ArrayVec<AccountWithKey, 16>,
    spot_accounts: ArrayVec<AccountWithKey, 16>,
    oracle_accounts: ArrayVec<AccountWithKey, 32>,
}

impl AccountsListBuilder {
    /// Constructs an accounts list from `user` positions sync
    ///
    /// * `client` - drift client instance
    /// * `user` - the account to build against
    /// * `force_markets` - additional market accounts that should be included in the account list
    ///
    /// It relies on the `client` being subscribed to all the necessary markets and oracles
    pub fn try_build(
        &mut self,
        client: &DriftClient,
        user: &User,
        force_markets: &[MarketId],
    ) -> SdkResult<AccountsList<'_>> {
        let mut oracle_markets = HashMap::<Pubkey, MarketId>::with_capacity(16);
        let drift_state_account = client.try_get_account::<State>(state_account())?;

        let force_spot_iter = force_markets
            .iter()
            .filter(|m| m.is_spot())
            .map(|m| m.index());

        let spot_market_idxs = ahash::HashSet::from_iter(
            user.spot_positions
                .iter()
                .filter(|p| !p.is_available())
                .map(|p| p.market_index)
                .chain(force_spot_iter)
                .chain(std::iter::once(MarketId::QUOTE_SPOT.index())),
        );

        for idx in spot_market_idxs {
            let market = client.try_get_spot_market_account(idx)?;
            oracle_markets.insert(market.oracle, MarketId::spot(market.market_index));
            self.spot_accounts.push(
                (
                    market.pubkey,
                    Account {
                        data: zero_account_to_bytes(market),
                        owner: constants::PROGRAM_ID,
                        ..Default::default()
                    },
                )
                    .into(),
            );
        }

        let force_perp_iter = force_markets
            .iter()
            .filter(|m| m.is_perp())
            .map(|m| m.index());
        let perp_market_idxs = ahash::HashSet::from_iter(
            user.perp_positions
                .iter()
                .filter(|p| !p.is_available())
                .map(|p| p.market_index)
                .chain(force_perp_iter),
        );

        for idx in perp_market_idxs {
            let market = client.try_get_perp_market_account(idx)?;
            oracle_markets.insert(market.amm.oracle, MarketId::perp(market.market_index));
            self.perp_accounts.push(
                (
                    market.pubkey,
                    Account {
                        data: zero_account_to_bytes(market),
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
                .ok_or(SdkError::NoMarketData(*market))?;

            latest_oracle_slot = oracle.slot.max(latest_oracle_slot);
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

    /// Constructs an accounts list from `user` positions
    /// fetching from RPC as necessary
    ///
    /// * `client` - drift client instance
    /// * `user` - the account to build against
    /// * `force_markets` - additional market accounts that should be included in the account list
    ///
    /// like `try_build` but will fall back to network queries to fetch market/oracle accounts as required
    /// if the client is already subscribed to necessary market/oracles then no network requests are made.
    pub async fn build(
        &mut self,
        client: &DriftClient,
        user: &User,
        force_markets: &[MarketId],
    ) -> SdkResult<AccountsList<'_>> {
        let mut oracle_markets = HashMap::<Pubkey, MarketId>::with_capacity(16);
        let drift_state_account = client.try_get_account::<State>(state_account())?;

        // TODO: could batch the requests
        let force_spot_iter = force_markets
            .iter()
            .filter(|m| m.is_spot())
            .map(|m| m.index());
        let spot_market_idxs = ahash::HashSet::from_iter(
            user.spot_positions
                .iter()
                .filter(|p| !p.is_available())
                .map(|p| p.market_index)
                .chain(force_spot_iter)
                .chain(std::iter::once(MarketId::QUOTE_SPOT.index())),
        );

        for market_idx in spot_market_idxs.iter() {
            let market = client.get_spot_market_account(*market_idx).await?;
            oracle_markets.insert(market.oracle, MarketId::spot(market.market_index));

            self.spot_accounts.push(
                (
                    market.pubkey,
                    Account {
                        data: zero_account_to_bytes(market),
                        owner: constants::PROGRAM_ID,
                        ..Default::default()
                    },
                )
                    .into(),
            );
        }

        let force_perp_iter = force_markets
            .iter()
            .filter(|m| m.is_perp())
            .map(|m| m.index());
        let perp_market_idxs = ahash::HashSet::from_iter(
            user.perp_positions
                .iter()
                .filter(|p| !p.is_available())
                .map(|p| p.market_index)
                .chain(force_perp_iter),
        );

        for market_idx in perp_market_idxs.iter() {
            let market = client.get_perp_market_account(*market_idx).await?;
            oracle_markets.insert(market.amm.oracle, MarketId::perp(market.market_index));

            self.perp_accounts.push(
                (
                    market.pubkey,
                    Account {
                        data: zero_account_to_bytes(market),
                        owner: constants::PROGRAM_ID,
                        ..Default::default()
                    },
                )
                    .into(),
            );
        }

        let mut latest_oracle_slot = 0;
        for (oracle_key, market) in oracle_markets.iter() {
            let oracle = client.get_oracle_price_data_and_slot(*market).await?;

            latest_oracle_slot = oracle.slot.max(latest_oracle_slot);
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
