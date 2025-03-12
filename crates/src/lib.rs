//! Drift SDK

use std::{borrow::Cow, collections::BTreeSet, sync::Arc, time::Duration};

use anchor_lang::{AccountDeserialize, InstructionData};
use drift_pubsub_client::PubsubClient;
use futures_util::TryFutureExt;
use log::debug;
pub use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_rpc_client_api::response::Response;
use solana_sdk::{
    account::Account,
    clock::Slot,
    compute_budget::ComputeBudgetInstruction,
    hash::Hash,
    instruction::{AccountMeta, Instruction},
    message::{v0, Message, VersionedMessage},
    signature::{keypair_from_seed, Keypair, Signature},
    signer::Signer,
    transaction::VersionedTransaction,
};
pub use solana_sdk::{address_lookup_table::AddressLookupTableAccount, pubkey::Pubkey};
use utils::get_ws_url;

pub use crate::types::Context;
use crate::{
    account_map::AccountMap,
    blockhash_subscriber::BlockhashSubscriber,
    constants::{
        derive_perp_market_account, derive_spot_market_account, state_account, MarketExt,
        ProgramData, DEFAULT_PUBKEY, PROGRAM_ID, SYSVAR_INSTRUCTIONS_PUBKEY,
    },
    drift_idl::traits::ToAccountMetas,
    marketmap::MarketMap,
    oraclemap::{Oracle, OracleMap},
    swift_order_subscriber::{SignedOrderInfo, SwiftOrderStream},
    types::{
        accounts::{PerpMarket, SpotMarket, State, User, UserStats},
        DataAndSlot, MarketType, *,
    },
    utils::get_http_url,
};

// utils
pub mod async_utils;
pub mod ffi;
pub mod math;
pub mod memcmp;
pub mod utils;

// constants & types
pub mod constants;
pub mod drift_idl;
pub mod types;

// internal infra
pub mod polled_account_subscriber;
pub mod websocket_account_subscriber;
pub mod websocket_program_account_subscriber;

// subscribers
pub mod auction_subscriber;
pub mod blockhash_subscriber;
pub mod event_subscriber;
pub mod priority_fee_subscriber;
pub mod swift_order_subscriber;

pub mod jit_client;

pub mod marketmap;
pub mod oraclemap;
pub mod slot_subscriber;
pub mod usermap;

// wrappers
pub mod account_map;

#[cfg(feature = "dlob")]
pub mod dlob;

/// DriftClient
///
/// It is cheaply clone-able and consumers are encouraged to do so.
/// It is not recommended to create multiple instances with `::new()` as this will not re-use underlying resources such
/// as network connections or memory allocations
///
/// The client can be used as is to fetch data ad-hoc over RPC or subscribed to receive live updates
/// ```example(no_run)
/// let client = DriftClient::new(
///     Context::MainNet,
///     RpcClient::new("https://rpc.example.com"),
///     key_pair.into()
/// ).await.expect("initializes");
///
/// // queries over RPC
/// let sol_perp_price = client.oracle_price(MarketId::perp(0)).await;
///
/// // Subscribe to live program changes e.g oracle prices, spot/perp market changes, user accounts
/// let markets = [MarketId::perp(0), MarketId::spot(2)];
/// client.subscribe_markets(&markets).await.expect("subscribes");
/// client.subscribe_oracles(&markets).await.expect("subscribes");
///
/// // after subscribing, uses Ws-backed local storage
/// let sol_perp_price = client.oracle_price(MarketId::perp(0)).await;
///
/// client.unsubscribe();
/// ```
#[derive(Clone)]
#[must_use]
pub struct DriftClient {
    pub context: Context,
    backend: &'static DriftClientBackend,
    wallet: Wallet,
}

impl DriftClient {
    /// Create a new `DriftClient` instance
    ///
    /// * `context` - devnet or mainnet
    /// * `rpc_client` - an RpcClient instance
    /// * `wallet` - wallet to use for tx signing convenience
    pub async fn new(context: Context, rpc_client: RpcClient, wallet: Wallet) -> SdkResult<Self> {
        // check URL format here to fail early, otherwise happens at request time.
        let _ = get_http_url(&rpc_client.url())?;
        Ok(Self {
            backend: Box::leak(Box::new(
                DriftClientBackend::new(context, Arc::new(rpc_client)).await?,
            )),
            context,
            wallet,
        })
    }

    /// Starts background subscriptions for live blockhashes
    ///
    /// This is a no-op if already subscribed
    pub async fn subscribe_blockhashes(&self) -> SdkResult<()> {
        self.backend.subscribe_blockhashes().await
    }

    /// Starts background subscriptions for live market account updates
    ///
    /// This is a no-op if already subscribed
    pub async fn subscribe_markets(&self, markets: &[MarketId]) -> SdkResult<()> {
        self.backend.subscribe_markets(markets).await
    }

    /// Subscribe to all spot and perp markets
    ///
    /// This is a no-op if already subscribed
    pub async fn subscribe_all_markets(&self) -> SdkResult<()> {
        let markets = self.get_all_market_ids();
        self.backend.subscribe_markets(&markets).await
    }

    /// Subscribe to all spot markets
    ///
    /// This is a no-op if already subscribed
    pub async fn subscribe_all_spot_markets(&self) -> SdkResult<()> {
        let markets = self.get_all_spot_market_ids();
        self.backend.subscribe_markets(&markets).await
    }

    /// Subscribe to all perp markets
    ///
    /// This is a no-op if already subscribed
    pub async fn subscribe_all_perp_markets(&self) -> SdkResult<()> {
        let markets = self.get_all_perp_market_ids();
        self.backend.subscribe_markets(&markets).await
    }

    /// Starts background subscriptions for live oracle account updates by market
    ///
    /// This is a no-op if already subscribed
    pub async fn subscribe_oracles(&self, markets: &[MarketId]) -> SdkResult<()> {
        self.backend.subscribe_oracles(markets).await
    }

    /// Subscribe to all oracles
    ///
    /// This is a no-op if already subscribed
    pub async fn subscribe_all_oracles(&self) -> SdkResult<()> {
        let markets = self.get_all_market_ids();
        self.backend.subscribe_oracles(&markets).await
    }

    /// Subscribe to all spot market oracles
    ///
    /// This is a no-op if already subscribed
    pub async fn subscribe_all_spot_oracles(&self) -> SdkResult<()> {
        let markets = self.get_all_spot_market_ids();
        self.backend.subscribe_oracles(&markets).await
    }

    /// Subscribe to all perp market oracles
    ///
    /// This is a no-op if already subscribed
    pub async fn subscribe_all_perp_oracles(&self) -> SdkResult<()> {
        let markets = self.get_all_perp_market_ids();
        self.backend.subscribe_oracles(&markets).await
    }

    /// Subscribe to swift order feed(s) for given `markets`
    ///
    /// Returns a stream of swift orders
    pub async fn subscribe_swift_orders(
        &self,
        markets: &[MarketId],
    ) -> SdkResult<SwiftOrderStream> {
        swift_order_subscriber::subscribe_swift_orders(self, markets).await
    }

    /// Returns the MarketIds for all active spot markets (ignores de-listed and settled markets)
    ///
    /// Useful for iterating over all spot markets
    pub fn get_all_spot_market_ids(&self) -> Vec<MarketId> {
        self.program_data()
            .spot_market_configs()
            .iter()
            .filter_map(|m| match m.status {
                MarketStatus::Settlement | MarketStatus::Delisted => {
                    log::debug!("ignoring settled/delisted spot market: {}", m.market_index);
                    None
                }
                _ => Some(MarketId::spot(m.market_index)),
            })
            .collect()
    }

    /// Returns the MarketIds for all active perp markets (ignores de-listed and settled markets)
    ///
    /// Useful for iterating over all perp markets
    pub fn get_all_perp_market_ids(&self) -> Vec<MarketId> {
        self.program_data()
            .perp_market_configs()
            .iter()
            .filter_map(|m| match m.status {
                MarketStatus::Settlement | MarketStatus::Delisted => {
                    log::debug!("ignoring settled/delisted perp market: {}", m.market_index);
                    None
                }
                _ => Some(MarketId::perp(m.market_index)),
            })
            .collect()
    }

    /// Returns the `MarketId`s for all active markets (ignores de-listed and settled markets)
    ///
    /// Useful for iterating over all markets
    pub fn get_all_market_ids(&self) -> Vec<MarketId> {
        let spot_markets = self.get_all_spot_market_ids();
        let perp_markets = self.get_all_perp_market_ids();
        spot_markets.into_iter().chain(perp_markets).collect()
    }

    /// Unsubscribe from network resources
    /// Subsequent queries will pull from the network ad-hoc
    ///
    /// This is a no-op if not subscribed
    pub async fn unsubscribe(&self) -> SdkResult<()> {
        self.backend.unsubscribe().await
    }

    /// Return a handle to the inner RPC client
    #[deprecated]
    pub fn inner(&self) -> &RpcClient {
        &self.backend.rpc_client
    }

    /// Return a handle to the inner RPC client
    pub fn rpc(&self) -> Arc<RpcClient> {
        self.backend.client()
    }

    /// Return a handle to the inner Ws client
    pub fn ws(&self) -> Arc<PubsubClient> {
        self.backend.ws()
    }

    /// Return on-chain program metadata
    ///
    /// Useful for inspecting market ids and config
    pub fn program_data(&self) -> &ProgramData {
        &self.backend.program_data
    }

    /// Get an account's open order by id
    ///
    /// * `account` - the drift user PDA
    pub async fn get_order_by_id(
        &self,
        account: &Pubkey,
        order_id: u32,
    ) -> SdkResult<Option<Order>> {
        let user = self.backend.get_user_account(account).await?;

        Ok(user.orders.iter().find(|o| o.order_id == order_id).copied())
    }

    /// Get an account's open order by user assigned id
    ///
    /// * `account` - the drift user PDA
    pub async fn get_order_by_user_id(
        &self,
        account: &Pubkey,
        user_order_id: u8,
    ) -> SdkResult<Option<Order>> {
        let user = self.backend.get_user_account(account).await?;

        Ok(user
            .orders
            .iter()
            .find(|o| o.user_order_id == user_order_id)
            .copied())
    }

    /// Get all the account's open orders
    ///
    /// * `account` - the drift user PDA
    pub async fn all_orders(&self, account: &Pubkey) -> SdkResult<Vec<Order>> {
        let user = self.backend.get_user_account(account).await?;

        Ok(user
            .orders
            .iter()
            .filter(|o| o.status == OrderStatus::Open)
            .copied()
            .collect())
    }

    /// Get all the account's active positions
    ///
    /// * `account` - the drift user PDA
    pub async fn all_positions(
        &self,
        account: &Pubkey,
    ) -> SdkResult<(Vec<SpotPosition>, Vec<PerpPosition>)> {
        let user = self.backend.get_user_account(account).await?;

        Ok((
            user.spot_positions
                .iter()
                .filter(|s| !s.is_available())
                .copied()
                .collect(),
            user.perp_positions
                .iter()
                .filter(|p| p.is_open_position())
                .copied()
                .collect(),
        ))
    }

    /// Get a perp position by market
    ///
    /// * `account` - the drift user PDA
    ///
    /// Returns the position if it exists
    pub async fn perp_position(
        &self,
        account: &Pubkey,
        market_index: u16,
    ) -> SdkResult<Option<PerpPosition>> {
        let user = self.backend.get_user_account(account).await?;

        Ok(user
            .perp_positions
            .iter()
            .find(|p| p.market_index == market_index && !p.is_available())
            .copied())
    }

    /// Get a spot position by market
    ///
    /// * `account` - the drift user PDA
    ///
    /// Returns the position if it exists
    pub async fn spot_position(
        &self,
        account: &Pubkey,
        market_index: u16,
    ) -> SdkResult<Option<SpotPosition>> {
        let user = self.backend.get_user_account(account).await?;

        Ok(user
            .spot_positions
            .iter()
            .find(|p| p.market_index == market_index && !p.is_available())
            .copied())
    }

    /// Return the `DriftClient`'s wallet
    pub fn wallet(&self) -> &Wallet {
        &self.wallet
    }

    /// Get the user account data
    /// Uses cached value if subscribed, falls back to network query
    ///
    /// * `account` - the drift user PDA (subaccount)
    ///
    /// Returns the deserialized account data (`User`)
    pub async fn get_user_account(&self, account: &Pubkey) -> SdkResult<User> {
        self.backend.get_user_account(account).await
    }

    /// Get a stats account
    ///
    /// Returns the deserialized account data (`UserStats`)
    pub async fn get_user_stats(&self, authority: &Pubkey) -> SdkResult<UserStats> {
        let user_stats_pubkey = Wallet::derive_stats_account(authority);
        self.backend.get_account(&user_stats_pubkey).await
    }

    /// Get the latest recent_block_hash
    /// uses latest cached if subscribed, otherwise falls back to network query
    pub async fn get_latest_blockhash(&self) -> SdkResult<Hash> {
        self.backend.get_latest_blockhash().await
    }

    /// Get some account value deserialized as T
    /// Uses cached value if subscribed, falls back to network query
    ///
    /// * `account` - any onchain account
    ///
    /// Returns the deserialized account data (`User`)
    pub async fn get_account_value<T: AccountDeserialize>(&self, account: &Pubkey) -> SdkResult<T> {
        self.backend.get_account(account).await
    }

    /// Try to get `account` as `T` using latest local value
    ///
    /// requires account was previously subscribed too.
    /// like `get_account_value` without async/network fallback
    pub fn try_get_account<T: AccountDeserialize>(&self, account: &Pubkey) -> SdkResult<T> {
        self.backend.try_get_account(account)
    }

    /// Try get the Drift `State` config account
    /// It contains various exchange level config parameters
    pub fn state_account(&self) -> SdkResult<State> {
        self.backend.try_get_account(state_account())
    }

    /// Sign and send a tx to the network
    ///
    /// Returns the signature on success
    pub async fn sign_and_send(&self, tx: VersionedMessage) -> SdkResult<Signature> {
        let recent_block_hash = self.backend.get_latest_blockhash().await?;
        self.backend
            .sign_and_send(self.wallet(), tx, recent_block_hash)
            .await
            .map_err(|err| err.to_out_of_sol_error().unwrap_or(err))
    }

    /// Sign and send a tx to the network
    ///
    ///  * `recent_block_hash` - some block hash to use for tx signing, if not provided it will be automatically set
    ///  * `config` - custom RPC config to use when submitting the tx
    ///
    /// Returns the signature on success
    pub async fn sign_and_send_with_config(
        &self,
        tx: VersionedMessage,
        recent_block_hash: Option<Hash>,
        config: RpcSendTransactionConfig,
    ) -> SdkResult<Signature> {
        let recent_block_hash = match recent_block_hash {
            Some(h) => h,
            None => self.backend.get_latest_blockhash().await?,
        };
        self.backend
            .sign_and_send_with_config(self.wallet(), tx, recent_block_hash, config)
            .await
            .map_err(|err| err.to_out_of_sol_error().unwrap_or(err))
    }

    /// Get spot market account
    ///
    /// uses latest cached value if subscribed, otherwise falls back to network query
    pub async fn get_spot_market_account(&self, market_index: u16) -> SdkResult<SpotMarket> {
        match self
            .backend
            .try_get_spot_market_account_and_slot(market_index)
        {
            Some(market) => Ok(market.data),
            None => {
                debug!(target: "rpc", "fetch market: spot/{market_index}");
                let market = derive_spot_market_account(market_index);
                self.backend.get_account(&market).await
            }
        }
    }

    /// Get perp market account
    ///
    /// uses latest cached value if subscribed, otherwise falls back to network query
    pub async fn get_perp_market_account(&self, market_index: u16) -> SdkResult<PerpMarket> {
        match self
            .backend
            .try_get_perp_market_account_and_slot(market_index)
        {
            Some(market) => Ok(market.data),
            None => {
                debug!(target: "rpc", "fetch market: perp/{market_index}");
                let market = derive_perp_market_account(market_index);
                self.backend.get_account(&market).await
            }
        }
    }

    /// Try to spot market account from cache
    ///
    /// Returns error if not subscribed
    pub fn try_get_spot_market_account(&self, market_index: u16) -> SdkResult<SpotMarket> {
        if let Some(market) = self
            .backend
            .try_get_spot_market_account_and_slot(market_index)
        {
            Ok(market.data)
        } else {
            Err(SdkError::NoMarketData(MarketId::spot(market_index)))
        }
    }

    /// Try to get perp market account from cache
    ///
    /// Returns error if not subscribed
    pub fn try_get_perp_market_account(&self, market_index: u16) -> SdkResult<PerpMarket> {
        if let Some(market) = self
            .backend
            .try_get_perp_market_account_and_slot(market_index)
        {
            Ok(market.data)
        } else {
            Err(SdkError::NoMarketData(MarketId::perp(market_index)))
        }
    }

    /// Lookup a market by symbol
    ///
    /// This operation is not free so lookups should be reused/cached by the caller
    ///
    /// Returns None if symbol does not map to any known market
    pub fn market_lookup(&self, symbol: &str) -> Option<MarketId> {
        if symbol.to_ascii_lowercase().ends_with("-perp") {
            let markets = self.program_data().perp_market_configs();
            if let Some(market) = markets
                .iter()
                .find(|m| m.symbol().eq_ignore_ascii_case(symbol))
            {
                return Some(MarketId::perp(market.market_index));
            }
        } else {
            let markets = self.program_data().spot_market_configs();
            if let Some(market) = markets
                .iter()
                .find(|m| m.symbol().eq_ignore_ascii_case(symbol))
            {
                return Some(MarketId::spot(market.market_index));
            }
        }

        None
    }

    /// Get live oracle price for `market`
    /// uses latest cached if subscribed, otherwise falls back to network query
    pub async fn oracle_price(&self, market: MarketId) -> SdkResult<i64> {
        self.backend.oracle_price(market).await
    }

    /// Initialize a transaction given a (sub)account address
    ///
    /// ```ignore
    /// let tx = client
    ///     .init_tx(&wallet.sub_account(3), false)
    ///     .cancel_all_orders()
    ///     .place_orders(...)
    ///     .build();
    /// ```
    /// Returns a `TransactionBuilder` for composing the tx
    pub async fn init_tx(
        &self,
        account: &Pubkey,
        delegated: bool,
    ) -> SdkResult<TransactionBuilder> {
        let account_data = self.get_user_account(account).await?;
        Ok(TransactionBuilder::new(
            self.program_data(),
            *account,
            Cow::Owned(account_data),
            delegated,
        ))
    }

    pub async fn get_recent_priority_fees(
        &self,
        writable_markets: &[MarketId],
        window: Option<usize>,
    ) -> SdkResult<Vec<u64>> {
        self.backend
            .get_recent_priority_fees(writable_markets, window)
            .await
    }

    /// Try get the latest oracle data for `market`
    ///
    /// If only the price is required use `oracle_price` instead
    pub fn try_get_oracle_price_data_and_slot(&self, market: MarketId) -> Option<Oracle> {
        self.backend.try_get_oracle_price_data_and_slot(market)
    }

    /// Get the latest oracle data for `market`
    ///
    /// If only the price is required use `oracle_price` instead
    pub async fn get_oracle_price_data_and_slot(&self, market: MarketId) -> SdkResult<Oracle> {
        self.backend.get_oracle(market).await
    }

    /// Subscribe to live updates for some `account`
    /// The latest value may be retrieved with `get_account(..)`
    /// ```example(no_run)
    /// client.subscribe_account(Wallet::derive_subaccount(..)).await;
    /// let subaccount = client.get_account::<User>();
    /// ```
    pub async fn subscribe_account(&self, account: &Pubkey) -> SdkResult<()> {
        self.backend.account_map.subscribe_account(account).await
    }

    /// Unsubscribe from updates for `account`
    pub fn unsubscribe_account(&self, account: &Pubkey) -> SdkResult<()> {
        self.backend.account_map.unsubscribe_account(account);
        Ok(())
    }

    /// Check IDL and libdrift_ffi_sys version
    ///
    /// panics if there's a mismatch
    pub fn check_libs() -> SdkResult<()> {
        let libdrift_version = ffi::check_ffi_version();
        let idl_version = drift_idl::IDL_VERSION;

        if libdrift_version != idl_version {
            log::warn!(
                "libdrift_ffi_sys: {} does not match IDL: {}",
                libdrift_version,
                drift_idl::IDL_VERSION
            );
            return Err(SdkError::LibDriftVersion);
        }

        Ok(())
    }
}

/// Provides the heavy-lifting and network facing features of the SDK
/// It is intended to be a singleton
pub struct DriftClientBackend {
    rpc_client: Arc<RpcClient>,
    pubsub_client: Arc<PubsubClient>,
    program_data: ProgramData,
    blockhash_subscriber: BlockhashSubscriber,
    account_map: AccountMap,
    perp_market_map: MarketMap<PerpMarket>,
    spot_market_map: MarketMap<SpotMarket>,
    oracle_map: OracleMap,
}

impl DriftClientBackend {
    /// Initialize a new `DriftClientBackend`
    async fn new(context: Context, rpc_client: Arc<RpcClient>) -> SdkResult<Self> {
        let pubsub_client =
            Arc::new(PubsubClient::new(&get_ws_url(rpc_client.url().as_str())?).await?);

        let perp_market_map =
            MarketMap::<PerpMarket>::new(Arc::clone(&pubsub_client), rpc_client.commitment());
        let spot_market_map =
            MarketMap::<SpotMarket>::new(Arc::clone(&pubsub_client), rpc_client.commitment());

        let lookup_table_address = context.lut();

        let (_, _, lut) = tokio::try_join!(
            perp_market_map.sync(&rpc_client),
            spot_market_map.sync(&rpc_client),
            rpc_client
                .get_account(&lookup_table_address)
                .map_err(Into::into),
        )?;
        let lookup_table = utils::deserialize_alt(lookup_table_address, &lut)?;

        let mut all_oracles = Vec::<(MarketId, Pubkey, OracleSource)>::with_capacity(
            perp_market_map.len() + spot_market_map.len(),
        );
        for market_oracle_info in perp_market_map
            .oracles()
            .iter()
            .chain(spot_market_map.oracles().iter())
        {
            all_oracles.push(*market_oracle_info);
        }

        let oracle_map = OracleMap::new(
            Arc::clone(&pubsub_client),
            all_oracles.as_slice(),
            rpc_client.commitment(),
        );
        let account_map = AccountMap::new(Arc::clone(&pubsub_client), rpc_client.commitment());
        account_map.subscribe_account(state_account()).await?;

        Ok(Self {
            rpc_client: Arc::clone(&rpc_client),
            pubsub_client,
            blockhash_subscriber: BlockhashSubscriber::new(Duration::from_secs(2), rpc_client),
            program_data: ProgramData::new(
                spot_market_map.values(),
                perp_market_map.values(),
                lookup_table,
            ),
            account_map,
            perp_market_map,
            spot_market_map,
            oracle_map,
        })
    }

    /// Start subscription for latest block hashes
    async fn subscribe_blockhashes(&self) -> SdkResult<()> {
        self.blockhash_subscriber.subscribe();
        Ok(())
    }

    /// Start subscriptions for market accounts
    async fn subscribe_markets(&self, markets: &[MarketId]) -> SdkResult<()> {
        let (perps, spot) = markets
            .iter()
            .partition::<Vec<MarketId>, _>(|x| x.is_perp());
        let _ = tokio::try_join!(
            self.perp_market_map.subscribe(&perps),
            self.spot_market_map.subscribe(&spot),
        )?;

        Ok(())
    }

    /// Start subscriptions for market oracle accounts
    async fn subscribe_oracles(&self, markets: &[MarketId]) -> SdkResult<()> {
        self.oracle_map.subscribe(markets).await
    }

    /// End subscriptions to live program data
    async fn unsubscribe(&self) -> SdkResult<()> {
        self.blockhash_subscriber.unsubscribe();
        self.perp_market_map.unsubscribe_all()?;
        self.spot_market_map.unsubscribe_all()?;
        self.account_map.unsubscribe_account(state_account());
        self.oracle_map.unsubscribe_all()
    }

    fn try_get_perp_market_account_and_slot(
        &self,
        market_index: u16,
    ) -> Option<DataAndSlot<PerpMarket>> {
        if self.perp_market_map.is_subscribed(market_index) {
            self.perp_market_map.get(&market_index)
        } else {
            None
        }
    }

    fn try_get_spot_market_account_and_slot(
        &self,
        market_index: u16,
    ) -> Option<DataAndSlot<SpotMarket>> {
        if self.spot_market_map.is_subscribed(market_index) {
            self.spot_market_map.get(&market_index)
        } else {
            None
        }
    }

    fn try_get_oracle_price_data_and_slot(&self, market: MarketId) -> Option<Oracle> {
        self.oracle_map.get_by_market(&market)
    }

    /// Same as `try_get_oracle_price_data_and_slot` but checks the oracle pubkey has not changed
    /// this can be useful if the oracle address changes in the program
    fn try_get_oracle_price_data_and_slot_checked(&self, market: MarketId) -> Option<Oracle> {
        let current_oracle = self
            .oracle_map
            .get_by_market(&market)
            .expect("oracle")
            .pubkey;

        let program_configured_oracle = if market.is_perp() {
            let market = self.try_get_perp_market_account_and_slot(market.index())?;
            market.data.amm.oracle
        } else {
            let market = self.try_get_spot_market_account_and_slot(market.index())?;
            market.data.oracle
        };

        if program_configured_oracle != current_oracle {
            panic!("invalid oracle: {}", market.index());
        }

        self.try_get_oracle_price_data_and_slot(market)
    }

    /// Return a handle to the inner RPC client
    fn client(&self) -> Arc<RpcClient> {
        Arc::clone(&self.rpc_client)
    }

    /// Return a handle to the inner RPC client
    fn ws(&self) -> Arc<PubsubClient> {
        Arc::clone(&self.pubsub_client)
    }

    /// Get recent tx priority fees
    ///
    /// * `writable_markets` - markets to consider for write locks
    /// * `window` - # of slots to include in the fee calculation
    async fn get_recent_priority_fees(
        &self,
        writable_markets: &[MarketId],
        window: Option<usize>,
    ) -> SdkResult<Vec<u64>> {
        let addresses: Vec<Pubkey> = writable_markets
            .iter()
            .filter_map(|x| match x.kind() {
                MarketType::Spot => self
                    .program_data
                    .spot_market_config_by_index(x.index())
                    .map(|x| x.pubkey),
                MarketType::Perp => self
                    .program_data
                    .perp_market_config_by_index(x.index())
                    .map(|x| x.pubkey),
            })
            .collect();

        let response = self
            .rpc_client
            .get_recent_prioritization_fees(addresses.as_slice())
            .await?;
        let window = window.unwrap_or(5).max(1);
        let fees = response
            .iter()
            .take(window)
            .map(|x| x.prioritization_fee)
            .collect();

        Ok(fees)
    }

    /// Fetch `account` as an Anchor account type `T`
    async fn get_account<T: AccountDeserialize>(&self, account: &Pubkey) -> SdkResult<T> {
        if let Some(value) = self.account_map.account_data(account) {
            Ok(value)
        } else {
            let account_data = self.rpc_client.get_account_data(account).await?;
            T::try_deserialize(&mut account_data.as_slice())
                .map_err(|err| SdkError::Anchor(Box::new(err)))
        }
    }

    /// Fetch `account` as an Anchor account type `T` along with the slot
    async fn get_account_with_slot<T: AccountDeserialize>(
        &self,
        account: &Pubkey,
    ) -> SdkResult<DataAndSlot<T>> {
        if let Some(value) = self.account_map.account_data_and_slot(account) {
            Ok(value)
        } else {
            let (account, slot) = self.get_account_with_slot_raw(account).await?;
            Ok(DataAndSlot {
                slot,
                data: T::try_deserialize(&mut account.data.as_slice())
                    .map_err(|err| SdkError::Anchor(Box::new(err)))?,
            })
        }
    }

    /// Fetch `account` as a drift User account
    ///
    /// uses latest cached if subscribed, otherwise falls back to network query
    async fn get_user_account(&self, account: &Pubkey) -> SdkResult<User> {
        self.get_account(account).await
    }

    /// Try to fetch `account` as `T` using latest local value
    /// requires account was previously subscribed too.
    fn try_get_account<T: AccountDeserialize>(&self, account: &Pubkey) -> SdkResult<T> {
        self.account_map
            .account_data(account)
            .ok_or_else(|| SdkError::NoAccountData(*account))
    }

    /// Returns latest blockhash
    ///
    /// uses latest cached if subscribed, otherwise falls back to network query
    pub async fn get_latest_blockhash(&self) -> SdkResult<Hash> {
        match self.blockhash_subscriber.get_latest_blockhash() {
            Some(hash) => Ok(hash),
            None => self
                .rpc_client
                .get_latest_blockhash()
                .await
                .map_err(SdkError::Rpc),
        }
    }

    /// Sign and send a tx to the network
    ///
    /// Returns the signature on success
    pub async fn sign_and_send(
        &self,
        wallet: &Wallet,
        tx: VersionedMessage,
        recent_block_hash: Hash,
    ) -> SdkResult<Signature> {
        let tx = wallet.sign_tx(tx, recent_block_hash)?;
        self.rpc_client
            .send_transaction(&tx)
            .await
            .map_err(Into::into)
    }

    /// Sign and send a tx to the network with custom send config
    /// allows setting commitment level, retries, etc.
    ///
    /// Returns the signature on success
    pub async fn sign_and_send_with_config(
        &self,
        wallet: &Wallet,
        tx: VersionedMessage,
        recent_block_hash: Hash,
        config: RpcSendTransactionConfig,
    ) -> SdkResult<Signature> {
        let tx = wallet.sign_tx(tx, recent_block_hash)?;
        self.rpc_client
            .send_transaction_with_config(&tx, config)
            .await
            .map_err(Into::into)
    }

    /// Fetch the live oracle price for `market`
    ///
    /// Uses latest local value from an `OracleMap` if subscribed, falls back to network query
    pub async fn oracle_price(&self, market: MarketId) -> SdkResult<i64> {
        self.get_oracle(market).await.map(|o| o.data.price)
    }

    /// Fetch live oracle data for `market`
    ///
    /// Uses latest local value from an `OracleMap` if subscribed, falls back to network query
    pub async fn get_oracle(&self, market: MarketId) -> SdkResult<Oracle> {
        if self.oracle_map.is_subscribed(&market) {
            Ok(self
                .try_get_oracle_price_data_and_slot(market)
                .expect("oracle exists"))
        } else {
            debug!(target: "rpc", "fetch oracle account: {market:?}");
            let (oracle, oracle_source) = match market.kind() {
                MarketType::Perp => {
                    let market = self
                        .program_data
                        .perp_market_config_by_index(market.index())
                        .ok_or(SdkError::InvalidOracle)?;
                    (market.amm.oracle, market.amm.oracle_source)
                }
                MarketType::Spot => {
                    let market = self
                        .program_data
                        .spot_market_config_by_index(market.index())
                        .ok_or(SdkError::InvalidOracle)?;
                    (market.oracle, market.oracle_source)
                }
            };
            let (account_data, slot) = self.get_account_with_slot_raw(&oracle).await?;
            let oracle_price_data =
                ffi::get_oracle_price(oracle_source, &mut (oracle, account_data.clone()), slot)?;

            Ok(Oracle {
                market,
                pubkey: oracle,
                source: oracle_source,
                slot,
                data: oracle_price_data,
                raw: account_data.data,
            })
        }
    }

    /// Get account via rpc along with retrieved slot number
    async fn get_account_with_slot_raw(&self, pubkey: &Pubkey) -> SdkResult<(Account, Slot)> {
        match self
            .rpc_client
            .get_account_with_commitment(pubkey, self.rpc_client.commitment())
            .await
        {
            Ok(Response {
                context,
                value: Some(account),
            }) => Ok((account, context.slot)),
            Ok(Response {
                context: _,
                value: None,
            }) => Err(SdkError::InvalidAccount),
            Err(err) => Err(err.into()),
        }
    }
}

/// Configure markets as forced for inclusion by `TransactionBuilder`
///
/// In contrast, without this Transactions are built using the latest known state of
/// users's open positions and orders, which can result in race conditions when executed onchain.
#[derive(Default)]
struct ForceMarkets {
    /// markets must include as readable
    readable: Vec<MarketId>,
    /// markets must include as writeable
    writeable: Vec<MarketId>,
}

impl ForceMarkets {
    /// Set given `markets` as readable, enforcing there inclusion in a final Tx
    pub fn with_readable(&mut self, markets: &[MarketId]) -> &mut Self {
        self.readable = markets.to_vec();
        self
    }
    /// Set given `markets` as writeable, enforcing there inclusion in a final Tx
    pub fn with_writeable(&mut self, markets: &[MarketId]) -> &mut Self {
        self.writeable = markets.to_vec();
        self
    }
}

/// Composable Tx builder for Drift program
///
/// Prefer `DriftClient::init_tx`
///
/// ```ignore
/// use drift_rs::{types::Context, TransactionBuilder, Wallet};
///
/// let wallet = Wallet::from_seed_bs58(Context::Dev, "seed");
/// let client = DriftClient::new("api.example.com").await.unwrap();
/// let account_data = client.get_account(wallet.default_sub_account()).await.unwrap();
///
/// let tx = TransactionBuilder::new(client.program_data, wallet.default_sub_account(), account_data.into())
///     .cancel_all_orders()
///     .place_orders(&[
///         NewOrder::default().build(),
///         NewOrder::default().build(),
///     ])
///     .legacy()
///     .build();
///
/// let signature = client.sign_and_send(tx, &wallet).await?;
/// ```
///
pub struct TransactionBuilder<'a> {
    /// contextual on-chain program data
    program_data: &'a ProgramData,
    /// sub-account data
    account_data: Cow<'a, User>,
    /// the drift sub-account address
    sub_account: Pubkey,
    /// either account authority or account delegate
    authority: Pubkey,
    /// ordered list of instructions
    ixs: Vec<Instruction>,
    /// use legacy transaction mode
    legacy: bool,
    /// add additional lookup tables (v0 only)
    lookup_tables: Vec<AddressLookupTableAccount>,
    /// some markets forced to include in the tx accounts list
    force_markets: ForceMarkets,
}

impl<'a> TransactionBuilder<'a> {
    /// Initialize a new `TransactionBuilder` for default signer
    ///
    /// * `program_data` - program data from chain
    /// * `sub_account` - drift sub-account address
    /// * `user` - drift sub-account data
    /// * `delegated` - set true to build tx for delegated signing
    pub fn new<'b>(
        program_data: &'b ProgramData,
        sub_account: Pubkey,
        user: Cow<'b, User>,
        delegated: bool,
    ) -> Self
    where
        'b: 'a,
    {
        Self {
            authority: if delegated {
                user.delegate
            } else {
                user.authority
            },
            program_data,
            account_data: user,
            sub_account,
            ixs: Default::default(),
            lookup_tables: vec![program_data.lookup_table.clone()],
            legacy: false,
            force_markets: Default::default(),
        }
    }
    /// force given `markets` to be included in the final tx accounts list (ensure to call before building ixs)
    pub fn force_include_markets(&mut self, readable: &[MarketId], writeable: &[MarketId]) {
        self.force_markets.with_readable(readable);
        self.force_markets.with_writeable(writeable);
    }
    /// Use legacy tx mode
    pub fn legacy(mut self) -> Self {
        self.legacy = true;
        self
    }
    /// Set the tx lookup tables
    pub fn lookup_tables(mut self, lookup_tables: &[AddressLookupTableAccount]) -> Self {
        self.lookup_tables = lookup_tables.to_vec();
        self.lookup_tables
            .push(self.program_data.lookup_table.clone());

        self
    }
    /// Set the priority fee of the tx
    ///
    /// * `microlamports_per_cu` - the price per unit of compute in µ-lamports
    pub fn with_priority_fee(mut self, microlamports_per_cu: u64, cu_limit: Option<u32>) -> Self {
        let cu_limit_ix = ComputeBudgetInstruction::set_compute_unit_price(microlamports_per_cu);
        self.ixs.insert(0, cu_limit_ix);
        if let Some(cu_limit) = cu_limit {
            let cu_price_ix = ComputeBudgetInstruction::set_compute_unit_limit(cu_limit);
            self.ixs.insert(1, cu_price_ix);
        }

        self
    }
    /// Append an ix to the Tx
    pub fn add_ix(mut self, ix: Instruction) -> Self {
        self.ixs.push(ix);
        self
    }

    /// Deposit collateral into account
    pub fn deposit(
        mut self,
        amount: u64,
        spot_market_index: u16,
        user_token_account: Pubkey,
        reduce_only: Option<bool>,
    ) -> Self {
        let accounts = build_accounts(
            self.program_data,
            types::accounts::Deposit {
                state: *state_account(),
                user: self.sub_account,
                user_stats: Wallet::derive_stats_account(&self.authority),
                authority: self.authority,
                spot_market_vault: constants::derive_spot_market_vault(spot_market_index),
                user_token_account,
                token_program: constants::TOKEN_PROGRAM_ID,
            },
            &[self.account_data.as_ref()],
            self.force_markets.readable.iter(),
            [MarketId::spot(spot_market_index)].iter(),
        );

        let ix = Instruction {
            program_id: constants::PROGRAM_ID,
            accounts,
            data: InstructionData::data(&drift_idl::instructions::Deposit {
                market_index: spot_market_index,
                amount,
                reduce_only: reduce_only.unwrap_or(false),
            }),
        };

        self.ixs.push(ix);

        self
    }

    pub fn withdraw(
        mut self,
        amount: u64,
        spot_market_index: u16,
        user_token_account: Pubkey,
        reduce_only: Option<bool>,
    ) -> Self {
        let accounts = build_accounts(
            self.program_data,
            types::accounts::Withdraw {
                state: *state_account(),
                user: self.sub_account,
                user_stats: Wallet::derive_stats_account(&self.authority),
                authority: self.authority,
                spot_market_vault: constants::derive_spot_market_vault(spot_market_index),
                user_token_account,
                drift_signer: constants::derive_drift_signer(),
                token_program: constants::TOKEN_PROGRAM_ID,
            },
            &[self.account_data.as_ref()],
            self.force_markets.readable.iter(),
            [MarketId::spot(spot_market_index)]
                .iter()
                .chain(self.force_markets.writeable.iter()),
        );

        let ix = Instruction {
            program_id: constants::PROGRAM_ID,
            accounts,
            data: InstructionData::data(&drift_idl::instructions::Withdraw {
                market_index: spot_market_index,
                amount,
                reduce_only: reduce_only.unwrap_or(false),
            }),
        };

        self.ixs.push(ix);

        self
    }

    /// Place new orders for account
    pub fn place_orders(mut self, orders: Vec<OrderParams>) -> Self {
        let mut readable_accounts: Vec<MarketId> = orders
            .iter()
            .map(|o| (o.market_index, o.market_type).into())
            .collect();
        readable_accounts.extend(&self.force_markets.readable);

        let accounts = build_accounts(
            self.program_data,
            types::accounts::PlaceOrders {
                state: *state_account(),
                authority: self.authority,
                user: self.sub_account,
            },
            &[self.account_data.as_ref()],
            readable_accounts.iter(),
            self.force_markets.writeable.iter(),
        );

        let ix = Instruction {
            program_id: constants::PROGRAM_ID,
            accounts,
            data: InstructionData::data(&drift_idl::instructions::PlaceOrders { params: orders }),
        };

        self.ixs.push(ix);

        self
    }

    /// Cancel all orders for account
    pub fn cancel_all_orders(mut self) -> Self {
        let accounts = build_accounts(
            self.program_data,
            types::accounts::CancelOrder {
                state: *state_account(),
                authority: self.authority,
                user: self.sub_account,
            },
            &[self.account_data.as_ref()],
            self.force_markets.readable.iter(),
            self.force_markets.writeable.iter(),
        );

        let ix = Instruction {
            program_id: constants::PROGRAM_ID,
            accounts,
            data: InstructionData::data(&drift_idl::instructions::CancelOrders {
                market_index: None,
                market_type: None,
                direction: None,
            }),
        };
        self.ixs.push(ix);

        self
    }

    /// Cancel account's orders matching some criteria
    ///
    /// * `market` - tuple of market ID and type (spot or perp)
    /// * `direction` - long or short
    pub fn cancel_orders(
        mut self,
        market: (u16, MarketType),
        direction: Option<PositionDirection>,
    ) -> Self {
        let (idx, r#type) = market;
        let accounts = build_accounts(
            self.program_data,
            types::accounts::CancelOrder {
                state: *state_account(),
                authority: self.authority,
                user: self.sub_account,
            },
            &[self.account_data.as_ref()],
            [(idx, r#type).into()]
                .iter()
                .chain(self.force_markets.readable.iter()),
            self.force_markets.writeable.iter(),
        );

        let ix = Instruction {
            program_id: constants::PROGRAM_ID,
            accounts,
            data: InstructionData::data(&drift_idl::instructions::CancelOrders {
                market_index: Some(idx),
                market_type: Some(r#type),
                direction,
            }),
        };
        self.ixs.push(ix);

        self
    }

    /// Cancel orders given ids
    pub fn cancel_orders_by_id(mut self, order_ids: Vec<u32>) -> Self {
        let accounts = build_accounts(
            self.program_data,
            types::accounts::CancelOrder {
                state: *state_account(),
                authority: self.authority,
                user: self.sub_account,
            },
            &[self.account_data.as_ref()],
            self.force_markets.readable.iter(),
            self.force_markets.writeable.iter(),
        );

        let ix = Instruction {
            program_id: constants::PROGRAM_ID,
            accounts,
            data: InstructionData::data(&drift_idl::instructions::CancelOrdersByIds { order_ids }),
        };
        self.ixs.push(ix);

        self
    }

    /// Cancel orders by given _user_ ids
    pub fn cancel_orders_by_user_id(mut self, user_order_ids: Vec<u8>) -> Self {
        let accounts = build_accounts(
            self.program_data,
            types::accounts::CancelOrder {
                state: *state_account(),
                authority: self.authority,
                user: self.sub_account,
            },
            &[self.account_data.as_ref()],
            self.force_markets.readable.iter(),
            self.force_markets.writeable.iter(),
        );

        for user_order_id in user_order_ids {
            let ix = Instruction {
                program_id: constants::PROGRAM_ID,
                accounts: accounts.clone(),
                data: InstructionData::data(&drift_idl::instructions::CancelOrderByUserId {
                    user_order_id,
                }),
            };
            self.ixs.push(ix);
        }

        self
    }

    /// Modify existing order(s) by order id
    pub fn modify_orders(mut self, orders: &[(u32, ModifyOrderParams)]) -> Self {
        let accounts = build_accounts(
            self.program_data,
            types::accounts::ModifyOrder {
                state: *state_account(),
                authority: self.authority,
                user: self.sub_account,
            },
            &[self.account_data.as_ref()],
            self.force_markets.readable.iter(),
            self.force_markets.writeable.iter(),
        );

        for (order_id, params) in orders {
            let ix = Instruction {
                program_id: constants::PROGRAM_ID,
                accounts: accounts.clone(),
                data: InstructionData::data(&drift_idl::instructions::ModifyOrder {
                    order_id: Some(*order_id),
                    modify_order_params: *params,
                }),
            };
            self.ixs.push(ix);
        }

        self
    }

    /// Modify existing order(s) by user order id
    pub fn modify_orders_by_user_id(mut self, orders: &[(u8, ModifyOrderParams)]) -> Self {
        let accounts = build_accounts(
            self.program_data,
            types::accounts::PlaceOrders {
                state: *state_account(),
                authority: self.authority,
                user: self.sub_account,
            },
            &[self.account_data.as_ref()],
            self.force_markets.readable.iter(),
            self.force_markets.writeable.iter(),
        );

        for (user_order_id, params) in orders {
            let ix = Instruction {
                program_id: constants::PROGRAM_ID,
                accounts: accounts.clone(),
                data: InstructionData::data(&drift_idl::instructions::ModifyOrderByUserId {
                    user_order_id: *user_order_id,
                    modify_order_params: *params,
                }),
            };
            self.ixs.push(ix);
        }

        self
    }

    /// Add a place and make instruction
    ///
    /// * `order` - the order to place
    /// * `taker_info` - taker account address and data
    /// * `taker_order_id` - the id of the taker's order to match with
    /// * `referrer` - pubkey of the taker's referrer account, if any
    /// * `fulfillment_type` - type of fill for spot orders, ignored for perp orders
    pub fn place_and_make(
        mut self,
        order: OrderParams,
        taker_info: &(Pubkey, User),
        taker_order_id: u32,
        referrer: Option<Pubkey>,
        fulfillment_type: Option<SpotFulfillmentType>,
    ) -> Self {
        let (taker, taker_account) = taker_info;
        let is_perp = order.market_type == MarketType::Perp;
        let perp_writable = [MarketId::perp(order.market_index)];
        let spot_writable = [MarketId::spot(order.market_index), MarketId::QUOTE_SPOT];
        let mut accounts = build_accounts(
            self.program_data,
            types::accounts::PlaceAndMakePerpOrder {
                state: *state_account(),
                authority: self.authority,
                user: self.sub_account,
                user_stats: Wallet::derive_stats_account(&self.authority),
                taker: *taker,
                taker_stats: Wallet::derive_stats_account(taker),
            },
            &[self.account_data.as_ref(), taker_account],
            self.force_markets.readable.iter(),
            if is_perp {
                perp_writable.iter()
            } else {
                spot_writable.iter()
            }
            .chain(self.force_markets.writeable.iter()),
        );

        if let Some(referrer) = referrer {
            accounts.push(AccountMeta::new(
                Wallet::derive_stats_account(&referrer),
                false,
            ));
            accounts.push(AccountMeta::new(referrer, false));
        }

        let ix = if order.market_type == MarketType::Perp {
            Instruction {
                program_id: constants::PROGRAM_ID,
                accounts,
                data: InstructionData::data(&drift_idl::instructions::PlaceAndMakePerpOrder {
                    params: order,
                    taker_order_id,
                }),
            }
        } else {
            Instruction {
                program_id: constants::PROGRAM_ID,
                accounts,
                data: InstructionData::data(&drift_idl::instructions::PlaceAndMakeSpotOrder {
                    params: order,
                    taker_order_id,
                    fulfillment_type,
                }),
            }
        };

        self.ixs.push(ix);
        self
    }

    /// Add a place and take instruction
    ///
    /// * `order` - the order to place
    /// * `maker_info` - pubkey of the maker/counter-party to take against and account data
    /// * `referrer` - pubkey of the maker's referrer account, if any
    /// * `fulfillment_type` - type of fill for spot orders, ignored for perp orders
    pub fn place_and_take(
        mut self,
        order: OrderParams,
        maker_info: Option<(Pubkey, User)>,
        referrer: Option<Pubkey>,
        fulfillment_type: Option<SpotFulfillmentType>,
        success_condition: Option<u32>,
    ) -> Self {
        let mut user_accounts = vec![self.account_data.as_ref()];
        if let Some((ref _maker_pubkey, ref maker)) = maker_info {
            user_accounts.push(maker);
        }

        let is_perp = order.market_type == MarketType::Perp;
        let perp_writable = [MarketId::perp(order.market_index)];
        let spot_writable = [MarketId::spot(order.market_index), MarketId::QUOTE_SPOT];

        let mut accounts = build_accounts(
            self.program_data,
            types::accounts::PlaceAndTakePerpOrder {
                state: *state_account(),
                authority: self.authority,
                user: self.sub_account,
                user_stats: Wallet::derive_stats_account(&self.authority),
            },
            user_accounts.as_slice(),
            self.force_markets.readable.iter(),
            if is_perp {
                perp_writable.iter()
            } else {
                spot_writable.iter()
            }
            .chain(self.force_markets.writeable.iter()),
        );

        if referrer.is_some_and(|r| !maker_info.is_some_and(|(m, _)| m == r)) {
            let referrer = referrer.unwrap();
            accounts.push(AccountMeta::new(
                Wallet::derive_stats_account(&referrer),
                false,
            ));
            accounts.push(AccountMeta::new(referrer, false));
        }

        let ix = if is_perp {
            Instruction {
                program_id: constants::PROGRAM_ID,
                accounts,
                data: InstructionData::data(&drift_idl::instructions::PlaceAndTakePerpOrder {
                    params: order,
                    success_condition,
                }),
            }
        } else {
            Instruction {
                program_id: constants::PROGRAM_ID,
                accounts,
                data: InstructionData::data(&drift_idl::instructions::PlaceAndTakeSpotOrder {
                    params: order,
                    maker_order_id: None,
                    fulfillment_type,
                }),
            }
        };

        self.ixs.push(ix);
        self
    }

    /// Place and try to fill (make) against the swift order (Perps only)
    ///
    /// * `maker_order` order params defined by the maker, e.g. partial or full fill
    /// * `signed_order_info` - the signed swift order info (i.e from taker)
    /// * `taker_account` - taker account data
    /// * `taker_account_referrer` - taker account referrer key
    ///
    pub fn place_and_make_swift_order(
        mut self,
        maker_order: OrderParams,
        signed_order_info: &SignedOrderInfo,
        taker_account: &User,
        taker_account_referrer: &Pubkey,
    ) -> Self {
        let order_params = signed_order_info.order_params();
        assert!(
            order_params.market_type == MarketType::Perp,
            "only swift perps are supported"
        );
        self = self.place_swift_order(signed_order_info, taker_account);

        let perp_writable = [MarketId::perp(order_params.market_index)];
        let mut accounts = build_accounts(
            self.program_data,
            types::accounts::PlaceAndMakeSignedMsgPerpOrder {
                state: *state_account(),
                authority: self.authority,
                user: self.sub_account,
                user_stats: Wallet::derive_stats_account(&self.authority),
                taker: Wallet::derive_user_account(
                    &taker_account.authority,
                    signed_order_info.taker_subaccount_id(),
                ),
                taker_stats: Wallet::derive_stats_account(&taker_account.authority),
                taker_signed_msg_user_orders: Wallet::derive_swift_order_account(
                    &taker_account.authority,
                ),
            },
            &[self.account_data.as_ref(), taker_account],
            self.force_markets.readable.iter(),
            perp_writable
                .iter()
                .chain(self.force_markets.writeable.iter()),
        );

        if taker_account_referrer != &DEFAULT_PUBKEY {
            accounts.push(AccountMeta::new(
                Wallet::derive_stats_account(taker_account_referrer),
                true,
            ));
            accounts.push(AccountMeta::new(*taker_account_referrer, true));
        }

        self.ixs.push(Instruction {
            program_id: constants::PROGRAM_ID,
            accounts,
            data: InstructionData::data(&drift_idl::instructions::PlaceAndMakeSignedMsgPerpOrder {
                params: maker_order,
                signed_msg_order_uuid: signed_order_info.order_uuid(),
            }),
        });

        self
    }

    /// Place a swift order (Perps only)
    ///
    /// ☢️ this Ix will not fill by itself. The caller should add a subsequent Ix
    /// e.g. with JIT proxy, to atomically place and fill the order
    /// or see `place_and_make_swift_order`
    ///
    /// * `signed_order_info` - the signed swift order info
    /// * `taker_account` - taker account data (authority of the swift order)
    ///
    pub fn place_swift_order(
        mut self,
        signed_order_info: &SignedOrderInfo,
        taker_account: &User,
    ) -> Self {
        let order_params = signed_order_info.order_params();
        assert!(
            order_params.market_type == MarketType::Perp,
            "only swift perps are supported"
        );

        let perp_writable = [MarketId::perp(order_params.market_index)];
        let accounts = build_accounts(
            self.program_data,
            types::accounts::PlaceSignedMsgTakerOrder {
                state: *state_account(),
                authority: self.authority,
                user: Wallet::derive_user_account(
                    &taker_account.authority,
                    signed_order_info.taker_subaccount_id(),
                ),
                user_stats: Wallet::derive_stats_account(&taker_account.authority),
                signed_msg_user_orders: Wallet::derive_swift_order_account(
                    &taker_account.authority,
                ),
                ix_sysvar: SYSVAR_INSTRUCTIONS_PUBKEY,
            },
            &[taker_account],
            self.force_markets.readable.iter(),
            perp_writable
                .iter()
                .chain(self.force_markets.writeable.iter()),
        );

        let swift_taker_ix_data = signed_order_info.to_ix_data();
        let ed25519_verify_ix = crate::utils::new_ed25519_ix_ptr(
            swift_taker_ix_data.as_slice(),
            self.ixs.len() as u16 + 1,
        );

        let place_swift_ix = Instruction {
            program_id: constants::PROGRAM_ID,
            accounts,
            data: InstructionData::data(&drift_idl::instructions::PlaceSignedMsgTakerOrder {
                signed_msg_order_params_message_bytes: swift_taker_ix_data,
                is_delegate_signer: signed_order_info.using_delegate_signing(),
            }),
        };

        self.ixs
            .extend_from_slice(&[ed25519_verify_ix, place_swift_ix]);
        self
    }

    /// Build the transaction message ready for signing and sending
    pub fn build(self) -> VersionedMessage {
        if self.legacy {
            let message = Message::new(self.ixs.as_ref(), Some(&self.authority));
            VersionedMessage::Legacy(message)
        } else {
            let message = v0::Message::try_compile(
                &self.authority,
                self.ixs.as_slice(),
                self.lookup_tables.as_slice(),
                Default::default(),
            )
            .expect("ok");
            VersionedMessage::V0(message)
        }
    }

    pub fn program_data(&self) -> &ProgramData {
        self.program_data
    }

    pub fn account_data(&self) -> &Cow<'_, User> {
        &self.account_data
    }
}

/// Builds a set of required accounts from a user's open positions and additional given accounts
///
/// * `base_accounts` - base anchor accounts
/// * `users` - Drift user account data
/// * `markets_readable` - IDs of markets to include as readable
/// * `markets_writable` - IDs of markets to include as writable (takes priority over readable)
///
/// # Panics
///  if the user has positions in an unknown market (i.e unsupported by the SDK)
pub fn build_accounts<'a>(
    program_data: &ProgramData,
    base_accounts: impl ToAccountMetas,
    users: &[&User],
    markets_readable: impl Iterator<Item = &'a MarketId>,
    markets_writable: impl Iterator<Item = &'a MarketId>,
) -> Vec<AccountMeta> {
    // the order of accounts returned must be instruction, oracles, spot, perps see (https://github.com/drift-labs/protocol-v2/blob/master/programs/drift/src/instructions/optional_accounts.rs#L28)
    let mut accounts = BTreeSet::<RemainingAccount>::new();

    // add accounts to the ordered list
    let mut include_market =
        |market_index: u16, market_type: MarketType, writable: bool| match market_type {
            MarketType::Spot => {
                let SpotMarket { pubkey, oracle, .. } = program_data
                    .spot_market_config_by_index(market_index)
                    .expect("exists");
                accounts.extend(
                    [
                        RemainingAccount::Spot {
                            pubkey: *pubkey,
                            writable,
                        },
                        RemainingAccount::Oracle { pubkey: *oracle },
                    ]
                    .iter(),
                )
            }
            MarketType::Perp => {
                let PerpMarket { pubkey, amm, .. } = program_data
                    .perp_market_config_by_index(market_index)
                    .expect("exists");
                accounts.extend(
                    [
                        RemainingAccount::Perp {
                            pubkey: *pubkey,
                            writable,
                        },
                        RemainingAccount::Oracle { pubkey: amm.oracle },
                    ]
                    .iter(),
                )
            }
        };

    for market in markets_writable {
        include_market(market.index(), market.kind(), true);
    }

    for market in markets_readable {
        include_market(market.index(), market.kind(), false);
    }

    for user in users {
        // Drift program performs margin checks which requires reading user positions
        for p in user.spot_positions.iter().filter(|p| !p.is_available()) {
            include_market(p.market_index, MarketType::Spot, false);
        }
        for p in user.perp_positions.iter().filter(|p| !p.is_available()) {
            include_market(p.market_index, MarketType::Perp, false);
        }
    }
    // always manually try to include the quote (USDC) market
    // TODO: this is not exactly the same semantics as the TS sdk
    include_market(MarketId::QUOTE_SPOT.index(), MarketType::Spot, false);

    let mut account_metas = base_accounts.to_account_metas();
    account_metas.extend(accounts.into_iter().map(Into::into));
    account_metas
}

/// Drift wallet
#[derive(Clone, Debug)]
pub struct Wallet {
    /// The signing keypair, it could be authority or delegate
    signer: Arc<Keypair>,
    /// The drift 'authority' account
    /// user (sub)accounts are derived from this
    authority: Pubkey,
    /// The drift 'stats' account
    stats: Pubkey,
}

impl Wallet {
    /// Returns true if the wallet is configured for delegated signing
    pub fn is_delegated(&self) -> bool {
        self.authority != self.signer.pubkey() && self.signer.pubkey().is_on_curve()
    }
    /// Init wallet from a string that could be either a file path or the encoded key, uses default sub-account
    pub fn try_from_str(path_or_key: &str) -> SdkResult<Self> {
        let authority = utils::load_keypair_multi_format(path_or_key)?;
        Ok(Self::new(authority))
    }
    /// Construct a read-only wallet
    pub fn read_only(authority: Pubkey) -> Self {
        Self {
            signer: Arc::new(Keypair::new()),
            authority,
            stats: Wallet::derive_stats_account(&authority),
        }
    }
    /// Init wallet from base58 encoded seed, uses default sub-account
    ///
    /// # panics
    /// if the key is invalid
    pub fn from_seed_bs58(seed: &str) -> Self {
        let authority = Keypair::from_base58_string(seed);
        Self::new(authority)
    }
    /// Init wallet from seed bytes, uses default sub-account
    pub fn from_seed(seed: &[u8]) -> SdkResult<Self> {
        let authority = keypair_from_seed(seed).map_err(|_| SdkError::InvalidSeed)?;
        Ok(Self::new(authority))
    }
    /// Init wallet with keypair
    ///
    /// * `authority` - keypair for tx signing
    pub fn new(authority: Keypair) -> Self {
        Self {
            stats: Wallet::derive_stats_account(&authority.pubkey()),
            authority: authority.pubkey(),
            signer: Arc::new(authority),
        }
    }
    /// Convert the wallet into a delegated one by providing the `authority` public key
    pub fn to_delegated(&mut self, authority: Pubkey) {
        self.stats = Wallet::derive_stats_account(&authority);
        self.authority = authority;
    }
    /// Calculate the address of a drift user account/sub-account
    pub fn derive_user_account(authority: &Pubkey, sub_account_id: u16) -> Pubkey {
        let (account_drift_pda, _seed) = Pubkey::find_program_address(
            &[
                &b"user"[..],
                authority.as_ref(),
                &sub_account_id.to_le_bytes(),
            ],
            &PROGRAM_ID,
        );
        account_drift_pda
    }

    /// Calculate the address of a drift stats account
    pub fn derive_stats_account(account: &Pubkey) -> Pubkey {
        let (account_drift_pda, _seed) = Pubkey::find_program_address(
            &[&b"user_stats"[..], account.as_ref()],
            &constants::PROGRAM_ID,
        );
        account_drift_pda
    }

    /// Calculate the address of `authority`s swift (taker) order account
    pub fn derive_swift_order_account(authority: &Pubkey) -> Pubkey {
        let (account_drift_pda, _seed) = Pubkey::find_program_address(
            &[&b"SIGNED_MSG"[..], authority.as_ref()],
            &constants::PROGRAM_ID,
        );
        account_drift_pda
    }

    /// Signs the given tx `message` returning the tx on success
    pub fn sign_tx(
        &self,
        mut message: VersionedMessage,
        recent_block_hash: Hash,
    ) -> SdkResult<VersionedTransaction> {
        message.set_recent_blockhash(recent_block_hash);
        let signer: &dyn Signer = self.signer.as_ref();
        VersionedTransaction::try_new(message, &[signer]).map_err(Into::into)
    }

    /// Sign message with the wallet's signer
    pub fn sign_message(&self, message: &[u8]) -> SdkResult<Signature> {
        let signer: &dyn Signer = self.signer.as_ref();
        Ok(signer.sign_message(message))
    }
    /// Return the wallet authority address
    pub fn authority(&self) -> &Pubkey {
        &self.authority
    }
    /// Return the wallet signing address
    pub fn signer(&self) -> Pubkey {
        self.signer.pubkey()
    }
    /// Return the drift user stats address
    pub fn stats(&self) -> &Pubkey {
        &self.stats
    }
    /// Return the address of the default sub-account (0)
    pub fn default_sub_account(&self) -> Pubkey {
        self.sub_account(0)
    }
    /// Calculate the drift user address given a `sub_account_id`
    pub fn sub_account(&self, sub_account_id: u16) -> Pubkey {
        Self::derive_user_account(self.authority(), sub_account_id)
    }
}

impl From<Keypair> for Wallet {
    fn from(value: Keypair) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use serde_json::json;
    use solana_account_decoder_client_types::{UiAccount, UiAccountData, UiAccountEncoding};
    use solana_rpc_client::rpc_client::Mocks;
    use solana_rpc_client_api::{
        request::RpcRequest,
        response::{Response, RpcResponseContext},
    };
    use types::accounts::PerpMarket;

    use super::*;

    // static account data for test/mock
    const ACCOUNT_DATA: &str = include_str!("../../res/9Jtc.hex");
    const DEVNET_ENDPOINT: &str = "https://api.devnet.solana.com";

    /// Init a new `DriftClient` with provided mocked RPC responses
    async fn setup(rpc_mocks: Mocks, keypair: Keypair) -> DriftClient {
        let rpc_client = Arc::new(RpcClient::new_mock_with_mocks(
            DEVNET_ENDPOINT.to_string(),
            rpc_mocks,
        ));

        let pubsub_client = Arc::new(
            PubsubClient::new(&get_ws_url(DEVNET_ENDPOINT).unwrap())
                .await
                .expect("ws connects"),
        );

        let perp_market_map =
            MarketMap::<PerpMarket>::new(Arc::clone(&pubsub_client), rpc_client.commitment());
        let spot_market_map =
            MarketMap::<SpotMarket>::new(Arc::clone(&pubsub_client), rpc_client.commitment());

        let backend = DriftClientBackend {
            rpc_client: Arc::clone(&rpc_client),
            pubsub_client: Arc::clone(&pubsub_client),
            program_data: ProgramData::uninitialized(),
            perp_market_map,
            spot_market_map,
            oracle_map: OracleMap::new(Arc::clone(&pubsub_client), &[], rpc_client.commitment()),
            blockhash_subscriber: BlockhashSubscriber::new(
                Duration::from_secs(2),
                Arc::clone(&rpc_client),
            ),
            account_map: AccountMap::new(Arc::clone(&pubsub_client), CommitmentConfig::processed()),
        };

        DriftClient {
            context: Context::DevNet,
            backend: Box::leak(Box::new(backend)),
            wallet: Wallet::new(keypair),
        }
    }

    #[tokio::test]
    async fn test_backend_send_sync() {
        let account_mocks = Mocks::default();
        let client = setup(account_mocks, Keypair::new()).await;

        tokio::task::spawn(async move {
            let _ = client.clone();
        });
    }

    #[tokio::test]
    #[cfg(feature = "rpc_tests")]
    async fn test_marketmap_subscribe() {
        use utils::test_envs::mainnet_endpoint;

        let client = DriftClient::new(
            Context::MainNet,
            RpcAccountProvider::new(&mainnet_endpoint()),
            Keypair::new().into(),
        )
        .await
        .unwrap();

        let _ = client.subscribe().await;

        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

        for _ in 0..20 {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            let perp_market = client.get_perp_market_account_and_slot(0);
            let slot = perp_market.unwrap().slot;
            dbg!(slot);
        }

        for _ in 0..20 {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            let spot_market = client.get_spot_market_account_and_slot(0);
            let slot = spot_market.unwrap().slot;
            dbg!(slot);
        }
    }

    #[tokio::test]
    async fn get_orders() {
        let user = Pubkey::from_str("9JtczxrJjPM4J1xooxr2rFXmRivarb4BwjNiBgXDwe2p").unwrap();
        let account_data = hex::decode(ACCOUNT_DATA).expect("valid hex");

        let mut account_mocks = Mocks::default();
        let account_response = json!(Response {
            context: RpcResponseContext::new(12_345),
            value: Some(UiAccount {
                data: UiAccountData::Binary(
                    solana_sdk::bs58::encode(account_data).into_string(),
                    UiAccountEncoding::Base58
                ),
                owner: user.to_string(),
                executable: false,
                lamports: 0,
                rent_epoch: 0,
                space: None,
            })
        });
        account_mocks.insert(RpcRequest::GetAccountInfo, account_response.clone());

        let client = setup(account_mocks, Keypair::new()).await;

        let orders = client.all_orders(&user).await.unwrap();
        assert_eq!(orders.len(), 3);
    }

    #[tokio::test]
    async fn get_positions() {
        let user = Pubkey::from_str("9JtczxrJjPM4J1xooxr2rFXmRivarb4BwjNiBgXDwe2p").unwrap();
        let account_data = hex::decode(ACCOUNT_DATA).expect("valid hex");

        let mut account_mocks = Mocks::default();
        let account_response = json!(Response {
            context: RpcResponseContext::new(12_345),
            value: Some(UiAccount {
                data: UiAccountData::Binary(
                    solana_sdk::bs58::encode(account_data).into_string(),
                    UiAccountEncoding::Base58
                ),
                owner: user.to_string(),
                executable: false,
                lamports: 0,
                rent_epoch: 0,
                space: None,
            })
        });
        account_mocks.insert(RpcRequest::GetAccountInfo, account_response.clone());
        let client = setup(account_mocks, Keypair::new()).await;

        let (spot, perp) = client.all_positions(&user).await.unwrap();
        assert_eq!(spot.len(), 1);
        assert_eq!(perp.len(), 1);
    }

    #[test]
    fn wallet_read_only() {
        let keypair = Keypair::new();
        let ro = Wallet::read_only(keypair.pubkey());

        let rw = Wallet::new(keypair);
        assert_eq!(rw.authority, ro.authority);
        assert_eq!(rw.stats, ro.stats);
        assert_eq!(rw.default_sub_account(), ro.default_sub_account());
    }
}
