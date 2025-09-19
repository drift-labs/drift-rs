//! Drift SDK

use std::{
    borrow::Cow,
    collections::BTreeSet,
    sync::{Arc, RwLock},
    time::Duration,
};

use crate::{
    account_map::AccountMap,
    blockhash_subscriber::BlockhashSubscriber,
    constants::{
        derive_perp_market_account, derive_spot_market_account,
        ids::{drift_oracle_receiver_program, wormhole_program},
        state_account, MarketExt, ProgramData, DEFAULT_PUBKEY, PYTH_LAZER_STORAGE_ACCOUNT_KEY,
        SYSVAR_INSTRUCTIONS_PUBKEY, SYSVAR_RENT_PUBKEY,
    },
    drift_idl::traits::ToAccountMetas,
    ffi::OraclePriceData,
    grpc::grpc_subscriber::{AccountFilter, DriftGrpcClient, GeyserSubscribeOpts},
    jupiter::JupiterSwapInfo,
    marketmap::MarketMap,
    oraclemap::{Oracle, OracleMap},
    swift_order_subscriber::{SignedOrderInfo, SwiftOrderStream},
    types::{
        accounts::{PerpMarket, SpotMarket, State, User, UserStats},
        AccountUpdate, DataAndSlot, MarketType, *,
    },
    utils::{get_http_url, get_ws_url},
};
pub use crate::{grpc::GrpcSubscribeOpts, types::Context, wallet::Wallet};
use anchor_lang::{AccountDeserialize, AnchorSerialize, Discriminator, InstructionData};
use base64::Engine;
use bytemuck::Pod;
use constants::{
    high_leverage_mode_account, ASSOCIATED_TOKEN_PROGRAM_ID, PROGRAM_ID, SYSTEM_PROGRAM_ID,
    TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID,
};
pub use drift_pubsub_client::PubsubClient;
use futures_util::TryFutureExt;
use log::debug;
use pythnet_sdk::wire::v1::{AccumulatorUpdateData, Proof};
pub use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_rpc_client_api::{
    config::RpcSimulateTransactionConfig,
    filter::RpcFilterType,
    response::{Response, RpcSimulateTransactionResult},
};
use solana_sdk::{
    account::Account,
    clock::Slot,
    commitment_config::CommitmentLevel,
    compute_budget::ComputeBudgetInstruction,
    hash::Hash,
    instruction::{AccountMeta, Instruction},
    message::{v0, Message, VersionedMessage},
    signature::Signature,
};
pub use solana_sdk::{address_lookup_table::AddressLookupTableAccount, pubkey::Pubkey};

// utils
pub mod async_utils;
pub mod ffi;
pub mod jupiter;
pub mod math;
pub mod memcmp;
pub mod utils;
pub mod wallet;

// constants & types
pub mod constants;
pub mod drift_idl;
pub mod types;

// internal infra
pub mod grpc;
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

pub mod account_map;
pub mod marketmap;
pub mod oraclemap;

pub mod slot_subscriber;
pub mod usermap;

pub mod dlob;

/// DriftClient
///
/// It is cheaply clone-able and consumers are encouraged to do so.
/// It is not recommended to create multiple instances with `::new()` as this will not re-use underlying resources such
/// as network connections or memory allocations
///
/// The client can be used as is to fetch data ad-hoc over RPC or subscribed to receive live updates (transparently)
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
    pub wallet: Wallet,
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

    pub async fn sync_user_accounts(&self, filters: Vec<RpcFilterType>) -> SdkResult<()> {
        self.backend.account_map.sync_user_accounts(filters).await
    }

    pub async fn sync_user_stats_accounts(&self) -> SdkResult<()> {
        self.backend.account_map.sync_stats_accounts().await
    }

    /// Starts background subscriptions for live blockhashes
    ///
    /// This is a no-op if already subscribed
    pub async fn subscribe_blockhashes(&self) -> SdkResult<()> {
        self.backend.subscribe_blockhashes().await
    }

    /// Starts background subscriptions for live market account updates
    ///
    /// * `markets` - list of markets to subscribe
    ///
    /// This is a no-op if already subscribed
    pub async fn subscribe_markets(&self, markets: &[MarketId]) -> SdkResult<()> {
        self.backend.subscribe_markets(markets).await
    }

    pub async fn subscribe_markets_with_callback<F>(
        &self,
        markets: &[MarketId],
        on_account: F,
    ) -> SdkResult<()>
    where
        F: Fn(&crate::AccountUpdate) + Send + Sync + 'static + Clone,
    {
        self.backend
            .subscribe_markets_with_callback(markets, on_account)
            .await
    }

    /// Subscribe to all spot and perp markets
    ///
    /// This is a no-op if already subscribed
    pub async fn subscribe_all_markets(&self) -> SdkResult<()> {
        let markets = self.get_all_market_ids();
        self.backend.subscribe_markets(&markets).await
    }

    pub async fn subscribe_all_markets_with_callback<F>(&self, on_account: F) -> SdkResult<()>
    where
        F: Fn(&crate::AccountUpdate) + Send + Sync + 'static + Clone,
    {
        let markets = self.get_all_market_ids();
        self.backend
            .subscribe_markets_with_callback(&markets, on_account)
            .await
    }

    /// Subscribe to all spot markets
    ///
    /// This is a no-op if already subscribed
    pub async fn subscribe_all_spot_markets(&self) -> SdkResult<()> {
        let markets = self.get_all_spot_market_ids();
        self.backend.subscribe_markets(&markets).await
    }

    pub async fn subscribe_all_spot_markets_with_callback<F>(&self, on_account: F) -> SdkResult<()>
    where
        F: Fn(&crate::AccountUpdate) + Send + Sync + 'static + Clone,
    {
        let markets = self.get_all_spot_market_ids();
        self.backend
            .subscribe_markets_with_callback(&markets, on_account)
            .await
    }

    /// Subscribe to all perp markets
    ///
    /// This is a no-op if already subscribed
    pub async fn subscribe_all_perp_markets(&self) -> SdkResult<()> {
        let markets = self.get_all_perp_market_ids();
        self.backend.subscribe_markets(&markets).await
    }

    pub async fn subscribe_all_perp_markets_with_callback<F>(&self, on_account: F) -> SdkResult<()>
    where
        F: Fn(&crate::AccountUpdate) + Send + Sync + 'static + Clone,
    {
        let markets = self.get_all_perp_market_ids();
        self.backend
            .subscribe_markets_with_callback(&markets, on_account)
            .await
    }

    /// Starts background subscriptions for live oracle account updates by market
    ///
    /// * `markets` - list of markets to subscribe for oracle updates
    ///
    /// This is a no-op if already subscribed
    pub async fn subscribe_oracles(&self, markets: &[MarketId]) -> SdkResult<()> {
        self.backend.subscribe_oracles(markets).await
    }

    pub async fn subscribe_oracles_with_callback<F>(
        &self,
        markets: &[MarketId],
        on_account: F,
    ) -> SdkResult<()>
    where
        F: Fn(&crate::AccountUpdate) + Send + Sync + 'static + Clone,
    {
        self.backend
            .subscribe_oracles_with_callback(markets, on_account)
            .await
    }

    /// Subscribe to all oracles
    ///
    /// This is a no-op if already subscribed
    pub async fn subscribe_all_oracles(&self) -> SdkResult<()> {
        let markets = self.get_all_market_ids();
        self.backend.subscribe_oracles(&markets).await
    }

    /// Subscribe to all oracle account updates with callback
    pub async fn subscribe_all_oracles_with_callback<F>(&self, on_account: F) -> SdkResult<()>
    where
        F: Fn(&crate::AccountUpdate) + Send + Sync + 'static + Clone,
    {
        let markets = self.get_all_market_ids();
        self.backend
            .subscribe_oracles_with_callback(&markets, on_account)
            .await
    }

    /// Subscribe to all spot market oracles
    ///
    /// This is a no-op if already subscribed
    pub async fn subscribe_all_spot_oracles(&self) -> SdkResult<()> {
        let markets = self.get_all_spot_market_ids();
        self.backend.subscribe_oracles(&markets).await
    }

    /// Subscribe to all spot oracle account updates with callback
    pub async fn subscribe_all_spot_oracles_with_callback<F>(&self, on_account: F) -> SdkResult<()>
    where
        F: Fn(&crate::AccountUpdate) + Send + Sync + 'static + Clone,
    {
        let markets = self.get_all_spot_market_ids();
        self.backend
            .subscribe_oracles_with_callback(&markets, on_account)
            .await
    }

    /// Subscribe to all perp market oracles
    ///
    /// This is a no-op if already subscribed
    pub async fn subscribe_all_perp_oracles(&self) -> SdkResult<()> {
        let markets = self.get_all_perp_market_ids();
        self.backend.subscribe_oracles(&markets).await
    }

    /// Subscribe to all perp oracle account updates with callback
    pub async fn subscribe_all_perp_oracles_with_callback<F>(&self, on_account: F) -> SdkResult<()>
    where
        F: Fn(&crate::AccountUpdate) + Send + Sync + 'static + Clone,
    {
        let markets = self.get_all_perp_market_ids();
        self.backend
            .subscribe_oracles_with_callback(&markets, on_account)
            .await
    }

    /// Subscribe to swift order feed(s) for given `markets`
    ///
    /// * `markets` - list of markets to watch for swift orders
    /// * `accept_sanitized` - set to `Some(true)` to also view *sanitized order flow
    /// * `swift_ws_url` - optional custom swift Ws endpoint
    ///
    /// *a sanitized order may have its auction params modified by the program when
    /// placed onchain. Makers should understand the time/price implications to accept these.
    ///
    /// Returns a stream of swift orders
    pub async fn subscribe_swift_orders(
        &self,
        markets: &[MarketId],
        accept_sanitized: Option<bool>,
        swift_ws_url: Option<String>,
    ) -> SdkResult<SwiftOrderStream> {
        swift_order_subscriber::subscribe_swift_orders(
            self,
            markets,
            accept_sanitized.is_some_and(|x| x),
            swift_ws_url,
        )
        .await
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
    /// * `order_id` - order id to query
    ///
    /// Returns the `Order` if it exists
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
    /// * `user_order_id` - user defined order id to query
    ///
    /// Returns the `Order` if it exists
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

    /// Get the account's open orders
    ///
    /// * `account` - the drift user PDA
    ///
    /// Returns the list of open orders
    pub async fn all_orders(&self, account: &Pubkey) -> SdkResult<Vec<Order>> {
        let user = self.backend.get_user_account(account).await?;

        Ok(user
            .orders
            .iter()
            .filter(|o| o.status == OrderStatus::Open)
            .copied()
            .collect())
    }

    /// Get the account's unsettled positions
    ///
    /// * `account` - the drift user PDA
    ///
    /// Returns the list of unsettled positions
    pub async fn unsettled_positions(&self, account: &Pubkey) -> SdkResult<Vec<PerpPosition>> {
        let user = self.backend.get_user_account(account).await?;

        Ok(user
            .perp_positions
            .iter()
            .filter(|p| p.base_asset_amount == 0 && p.quote_asset_amount != 0)
            .copied()
            .collect())
    }

    /// Get all the account's open positions
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

    /// Get the user account data and slot it was fetched at
    /// Uses cached value if subscribed, falls back to network query
    ///
    /// * `account` - the drift user PDA (subaccount)
    ///
    /// Returns the deserialized account data (`User`)
    pub async fn get_user_account_with_slot(
        &self,
        account: &Pubkey,
    ) -> SdkResult<DataAndSlot<User>> {
        self.backend.get_user_account_with_slot(account).await
    }

    /// Get a user stats account
    ///
    /// Returns the deserialized account data (`UserStats`)
    pub async fn get_user_stats(&self, authority: &Pubkey) -> SdkResult<UserStats> {
        let user_stats_pubkey = Wallet::derive_stats_account(authority);
        self.backend.get_account(&user_stats_pubkey).await
    }

    /// Get a user stats account and slot it was fetched at
    ///
    /// Returns the deserialized account data (`UserStats`)
    pub async fn get_user_stats_with_slot(
        &self,
        authority: &Pubkey,
    ) -> SdkResult<DataAndSlot<UserStats>> {
        let user_stats_pubkey = Wallet::derive_stats_account(authority);
        self.backend.get_account_with_slot(&user_stats_pubkey).await
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
    pub async fn get_account_value<T: AccountDeserialize + Pod>(
        &self,
        account: &Pubkey,
    ) -> SdkResult<T> {
        self.backend.get_account(account).await
    }

    /// Try to get `account` as `T` using latest local value
    ///
    /// requires account was previously subscribed too.
    /// like `get_account_value` without async/network fallback
    pub fn try_get_account<T: AccountDeserialize + Pod>(&self, account: &Pubkey) -> SdkResult<T> {
        self.backend.try_get_account(account)
    }

    /// Try get the Drift `State` config account
    /// It contains various exchange level config parameters
    pub fn state_account(&self) -> SdkResult<State> {
        self.backend.try_get_account(state_account())
    }

    /// Simulate the tx on remote RPC node
    pub async fn simulate_tx(
        &self,
        tx: VersionedMessage,
    ) -> SdkResult<RpcSimulateTransactionResult> {
        let response = self
            .rpc()
            .simulate_transaction_with_config(
                &VersionedTransaction {
                    message: tx,
                    // must provide a signature for the RPC call to work
                    signatures: vec![Signature::new_unique()],
                },
                RpcSimulateTransactionConfig {
                    sig_verify: false,
                    replace_recent_blockhash: true,
                    ..Default::default()
                },
            )
            .await;
        response.map(|r| r.value).map_err(Into::into)
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
    /// * `market_index` - spot market index
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
    /// * `market_index` - perp market index
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
    /// * `market_index` - spot market index
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
    /// * `market_index` - spot market index
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

    /// Get spot market account and slot it was fetched at
    ///
    /// * `market_index` - spot market index
    ///
    /// uses latest cached value if subscribed, otherwise falls back to network query
    pub async fn get_spot_market_account_and_slot(
        &self,
        market_index: u16,
    ) -> SdkResult<DataAndSlot<SpotMarket>> {
        match self
            .backend
            .try_get_spot_market_account_and_slot(market_index)
        {
            Some(market) => Ok(market),
            None => {
                debug!(target: "rpc", "fetch market: spot/{market_index}");
                let market = derive_spot_market_account(market_index);
                self.backend.get_account_with_slot(&market).await
            }
        }
    }

    /// Get perp market account and slot it was fetched at
    ///
    /// * `market_index` - perp market index
    ///
    /// uses latest cached value if subscribed, otherwise falls back to network query
    pub async fn get_perp_market_account_and_slot(
        &self,
        market_index: u16,
    ) -> SdkResult<DataAndSlot<PerpMarket>> {
        match self
            .backend
            .try_get_perp_market_account_and_slot(market_index)
        {
            Some(market) => Ok(market),
            None => {
                debug!(target: "rpc", "fetch market: perp/{market_index}");
                let market = derive_perp_market_account(market_index);
                self.backend.get_account_with_slot(&market).await
            }
        }
    }

    /// Try to spot market account from cache and slot it was fetched at
    ///
    /// * `market_index` - spot market index
    ///
    /// Returns error if not subscribed
    pub fn try_get_spot_market_account_and_slot(
        &self,
        market_index: u16,
    ) -> SdkResult<DataAndSlot<SpotMarket>> {
        if let Some(market) = self
            .backend
            .try_get_spot_market_account_and_slot(market_index)
        {
            Ok(market)
        } else {
            Err(SdkError::NoMarketData(MarketId::spot(market_index)))
        }
    }

    /// Try to get perp market account from cache and slot it was fetched at
    ///
    /// * `market_index` - spot market index
    ///
    /// Returns error if not subscribed
    pub fn try_get_perp_market_account_and_slot(
        &self,
        market_index: u16,
    ) -> SdkResult<DataAndSlot<PerpMarket>> {
        if let Some(market) = self
            .backend
            .try_get_perp_market_account_and_slot(market_index)
        {
            Ok(market)
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
            markets
                .iter()
                .find(|m| m.symbol().eq_ignore_ascii_case(symbol))
                .map(|m| MarketId::perp(m.market_index))
        } else {
            let markets = self.program_data().spot_market_configs();
            markets
                .iter()
                .find(|m| m.symbol().eq_ignore_ascii_case(symbol))
                .map(|m| MarketId::spot(m.market_index))
        }
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

    /// Get the AMM `OraclePriceData` if valid, otherwise return the conventional `OraclePriceData`
    ///
    /// ## Params
    /// * `market_index` - perp market index
    /// * `current_slot` - current solana slot
    ///
    pub fn try_get_mmoracle_for_perp_market(
        &self,
        market_index: u16,
        current_slot: Slot,
    ) -> SdkResult<OraclePriceData> {
        let oracle_data = self
            .try_get_oracle_price_data_and_slot(MarketId::perp(market_index))
            .ok_or(SdkError::InvalidOracle)?;
        let perp_market = self.try_get_perp_market_account(market_index)?;
        let oracle_validity_guard_rails = self.state_account().unwrap().oracle_guard_rails.validity;

        perp_market
            .get_mm_oracle_price_data(oracle_data.data, current_slot, &oracle_validity_guard_rails)
            .map(|x| x.safe_oracle_price_data)
    }

    /// Get the latest oracle data for `market`
    ///
    /// If only the price is required use `oracle_price` instead
    pub async fn get_oracle_price_data_and_slot(&self, market: MarketId) -> SdkResult<Oracle> {
        self.backend.get_oracle(market).await
    }

    /// Subscribe to live WebSocket updates for some `account`
    ///
    /// The latest value may be retrieved with `client.get_account(..)`
    /// ```example(no_run)
    /// let subaccount = Wallet::derive_user_account(authority, 1);
    /// client.subscribe_account(&subaccount).await;
    /// let subaccount_data = client.get_account::<User>(&subaccount);
    /// ```
    pub async fn subscribe_account(&self, account: &Pubkey) -> SdkResult<()> {
        self.backend.account_map.subscribe_account(account).await
    }

    /// Same as `subscribe_account` but uses RPC polling
    pub async fn subscribe_account_polled(
        &self,
        account: &Pubkey,
        interval: Duration,
    ) -> SdkResult<()> {
        self.backend
            .account_map
            .subscribe_account_polled(account, Some(interval))
            .await
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

    /// Return a reference to the internal spot market map
    #[cfg(feature = "unsafe_pub")]
    pub fn spot_market_map(&self) -> Arc<MapOf<u16, DataAndSlot<SpotMarket>>> {
        self.backend.spot_market_map.map()
    }

    /// Return a reference to the internal perp market map
    #[cfg(feature = "unsafe_pub")]
    pub fn perp_market_map(&self) -> Arc<MapOf<u16, DataAndSlot<PerpMarket>>> {
        self.backend.perp_market_map.map()
    }

    /// Return a reference to the internal oracle map
    #[cfg(feature = "unsafe_pub")]
    pub fn oracle_map(&self) -> Arc<MapOf<(Pubkey, u8), Oracle>> {
        self.backend.oracle_map.map()
    }

    /// Subscribe to all: markets, oracles, users, and slot updates over gRPC
    ///
    /// Updates are transparently handled by the `DriftClient` and calls to get User accounts, markets, oracles, etc.
    /// will utilize the latest cached updates from the gRPC subscription.
    ///
    /// use `opts` to control what is _cached_ by the client. The gRPC connection will always subscribe
    /// to all drift accounts regardless.
    ///
    /// * `endpoint` - the gRPC endpoint
    /// * `x_token` - gRPC authentication X token
    /// * `opts` - configure callbacks and caching
    /// * `sync` - sync all oracle,market,and User accounts on startup
    ///
    pub async fn grpc_subscribe(
        &self,
        endpoint: String,
        x_token: String,
        opts: GrpcSubscribeOpts,
        sync: bool,
    ) -> SdkResult<()> {
        self.backend
            .grpc_subscribe(endpoint, x_token, opts, sync)
            .await
    }

    /// Unsubscribe the gRPC connection
    pub fn grpc_unsubscribe(&self) {
        self.backend.grpc_unsubscribe();
    }

    /// Return a reference to the internal backend
    #[cfg(feature = "unsafe_pub")]
    pub fn backend(&self) -> &'static DriftClientBackend {
        self.backend
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
    grpc_unsub: RwLock<Option<(UnsubHandle, UnsubHandle)>>,
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

        let lut_pubkeys = context.luts();

        let account_map = AccountMap::new(
            Arc::clone(&pubsub_client),
            Arc::clone(&rpc_client),
            rpc_client.commitment(),
        );

        tokio::try_join!(
            account_map.subscribe_account_polled(state_account(), Some(Duration::from_secs(180))),
            account_map.subscribe_account_polled(
                high_leverage_mode_account(),
                Some(Duration::from_secs(180))
            )
        )?;

        let (_, _, lut_accounts, state_account_data) = tokio::try_join!(
            perp_market_map.sync(&rpc_client),
            spot_market_map.sync(&rpc_client),
            rpc_client
                .get_multiple_accounts(lut_pubkeys)
                .map_err(Into::into),
            rpc_client
                .get_account_data(state_account())
                .map_err(Into::into),
        )?;

        let lookup_tables = lut_pubkeys
            .iter()
            .zip(lut_accounts.iter())
            .map(|(pubkey, account_data)| {
                utils::deserialize_alt(*pubkey, account_data.as_ref().unwrap())
                    .expect("LUT decodes")
            })
            .collect();

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

        Ok(Self {
            rpc_client: Arc::clone(&rpc_client),
            pubsub_client,
            blockhash_subscriber: BlockhashSubscriber::new(Duration::from_secs(2), rpc_client),
            program_data: ProgramData::new(
                spot_market_map.values(),
                perp_market_map.values(),
                lookup_tables,
                State::try_deserialize(&mut state_account_data.as_slice()).unwrap(),
            ),
            account_map,
            perp_market_map,
            spot_market_map,
            oracle_map,
            grpc_unsub: RwLock::default(),
        })
    }

    /// Returns true if `DriftClientBackend` is subscribed via gRPC
    pub fn is_grpc_subscribed(&self) -> bool {
        let unsub = self.grpc_unsub.read().unwrap();
        unsub.is_some()
    }

    /// Start subscription for latest block hashes
    async fn subscribe_blockhashes(&self) -> SdkResult<()> {
        self.blockhash_subscriber.subscribe();
        Ok(())
    }

    /// Start subscriptions for market account updates
    async fn subscribe_markets(&self, markets: &[MarketId]) -> SdkResult<()> {
        self.subscribe_markets_inner(markets, EMPTY_ACCOUNT_CALLBACK)
            .await
    }

    async fn subscribe_markets_with_callback<F>(
        &self,
        markets: &[MarketId],
        on_account: F,
    ) -> SdkResult<()>
    where
        F: Fn(&crate::AccountUpdate) + Send + Sync + 'static + Clone,
    {
        self.subscribe_markets_inner(markets, on_account).await
    }

    async fn subscribe_markets_inner<F>(&self, markets: &[MarketId], on_account: F) -> SdkResult<()>
    where
        F: Fn(&crate::AccountUpdate) + Send + Sync + 'static + Clone,
    {
        if self.is_grpc_subscribed() {
            log::info!("already subscribed markets via gRPC");
            return Err(SdkError::AlreadySubscribed);
        }

        let (perps, spot) = markets
            .iter()
            .partition::<Vec<MarketId>, _>(|x| x.is_perp());
        let _ = tokio::try_join!(
            self.perp_market_map
                .subscribe_with_callback(&perps, on_account.clone()),
            self.spot_market_map
                .subscribe_with_callback(&spot, on_account),
        )?;

        Ok(())
    }

    /// Start subscriptions for market oracle accounts
    async fn subscribe_oracles(&self, markets: &[MarketId]) -> SdkResult<()> {
        self.subscribe_oracles_inner(markets, EMPTY_ACCOUNT_CALLBACK)
            .await
    }

    async fn subscribe_oracles_with_callback<F>(
        &self,
        markets: &[MarketId],
        on_account: F,
    ) -> SdkResult<()>
    where
        F: Fn(&crate::AccountUpdate) + Send + Sync + 'static + Clone,
    {
        self.subscribe_oracles_inner(markets, on_account).await
    }

    async fn subscribe_oracles_inner<F>(&self, markets: &[MarketId], on_account: F) -> SdkResult<()>
    where
        F: Fn(&crate::AccountUpdate) + Send + Sync + 'static + Clone,
    {
        if self.is_grpc_subscribed() {
            log::info!("already subscribed oracles via gRPC");
            return Err(SdkError::AlreadySubscribed);
        }

        self.oracle_map
            .subscribe_with_callback(markets, on_account)
            .await
    }

    /// Subscribe to all: markets, oracles, and slot updates over gRPC
    async fn grpc_subscribe(
        &self,
        endpoint: String,
        x_token: String,
        opts: GrpcSubscribeOpts,
        sync: bool,
    ) -> SdkResult<()> {
        log::debug!(target: "grpc", "subscribing to grpc with config: commitment: {:?}, interslot updates: {:?}", opts.commitment, opts.interslot_updates);
        let mut grpc = DriftGrpcClient::new(endpoint.clone(), x_token.clone())
            .grpc_connection_opts(opts.connection_opts.clone());

        if sync {
            // the DriftClientBackend syncs marketmaps by default
            if self.perp_market_map.len() == 0 {
                self.perp_market_map.sync(&self.rpc_client).await?;
            }
            if self.spot_market_map.len() == 0 {
                self.spot_market_map.sync(&self.rpc_client).await?;
            }
            let spot_markets = self
                .spot_market_map
                .marketmap
                .iter()
                .map(|i| MarketId::spot(*i.key()));
            let perp_markets = self
                .perp_market_map
                .marketmap
                .iter()
                .map(|i| MarketId::perp(*i.key()));
            let all_markets: Vec<MarketId> = spot_markets.chain(perp_markets).collect();

            self.oracle_map
                .sync(all_markets.as_ref(), &self.rpc_client)
                .await?;
        }

        grpc.on_account(
            AccountFilter::partial().with_discriminator(SpotMarket::DISCRIMINATOR),
            self.spot_market_map.on_account_fn(),
        );
        grpc.on_account(
            AccountFilter::partial().with_discriminator(PerpMarket::DISCRIMINATOR),
            self.perp_market_map.on_account_fn(),
        );

        if opts.user_stats_map {
            grpc.on_account(
                AccountFilter::partial().with_discriminator(UserStats::DISCRIMINATOR),
                self.account_map.on_account_fn(),
            );
        }

        let transactions_accounts_include = opts
            .transaction_include_accounts
            .iter()
            .map(|a| a.to_string())
            .collect();
        if let Some(f) = opts.on_transaction {
            grpc.on_transaction(f);
        }

        // set custom callbacks
        if let Some(callbacks) = opts.on_account {
            for (filter, on_account) in callbacks {
                grpc.on_account(filter, on_account)
            }
        }

        if let Some(f) = opts.on_slot {
            grpc.on_slot(f);
        }

        if let Some(f) = opts.on_block_meta {
            grpc.on_block_meta(f);
        }

        if opts.usermap {
            grpc.on_account(
                AccountFilter::partial().with_discriminator(User::DISCRIMINATOR),
                self.account_map.on_account_fn(),
            );
        } else {
            // when usermap is on, the custom accounts are already included
            // usermap off: subscribe to custom `User` accounts
            grpc.on_account(
                AccountFilter::full()
                    .with_discriminator(User::DISCRIMINATOR)
                    .with_accounts(opts.user_accounts.into_iter()),
                self.account_map.on_account_fn(),
            );
        }

        if opts.user_stats_map {
            grpc.on_account(
                AccountFilter::partial().with_discriminator(UserStats::DISCRIMINATOR),
                self.account_map.on_account_fn(),
            );
        }

        // start subscription
        let commitment = opts.commitment.unwrap_or(CommitmentLevel::Confirmed);
        let grpc_unsub = grpc
            .subscribe(
                commitment,
                GeyserSubscribeOpts {
                    accounts_owners: vec![PROGRAM_ID.to_string()],
                    interslot_updates: Some(opts.interslot_updates),
                    transactions_accounts_include,
                    blocks_meta: opts.subscribe_block_meta_updates,
                    slot_updates: opts.subscribe_slot_updates,
                    ..Default::default()
                },
            )
            .await
            .map_err(|err| SdkError::Grpc(Box::new(err)))?;

        // oracle pubkeys are subscribed individually
        // due to ownership differences
        let mut oracles_grpc =
            DriftGrpcClient::new(endpoint, x_token).grpc_connection_opts(opts.connection_opts);

        let oracle_pubkeys: Vec<String> = self
            .oracle_map
            .oracle_by_market
            .iter()
            .map(|(_, (pubkey, _))| pubkey.to_string())
            .collect();

        if let Some(on_oracle) = opts.on_oracle_update {
            oracles_grpc.on_account(AccountFilter::firehose(), on_oracle);
        }

        if opts.oraclemap {
            oracles_grpc.on_account(AccountFilter::firehose(), self.oracle_map.on_account_fn());
        }

        let oracles_grpc_unsub = oracles_grpc
            .subscribe(
                commitment,
                GeyserSubscribeOpts {
                    accounts_pubkeys: oracle_pubkeys,
                    interslot_updates: Some(opts.interslot_updates),
                    ..Default::default()
                },
            )
            .await
            .map_err(|err| SdkError::Grpc(Box::new(err)))?;

        let mut unsub = self.grpc_unsub.write().unwrap();
        let _ = unsub.insert((grpc_unsub, oracles_grpc_unsub));

        Ok(())
    }

    /// Unsubscribe the gRPC connections
    fn grpc_unsubscribe(&self) {
        let mut guard = self.grpc_unsub.write().unwrap();
        if let Some((a, b)) = guard.take() {
            let _ = a.send(());
            let _ = b.send(());
        }
    }

    /// End subscriptions to live program data
    async fn unsubscribe(&self) -> SdkResult<()> {
        self.blockhash_subscriber.unsubscribe();
        self.perp_market_map.unsubscribe_all()?;
        self.spot_market_map.unsubscribe_all()?;
        self.account_map.unsubscribe_account(state_account());
        self.oracle_map.unsubscribe_all()
    }

    pub fn try_get_perp_market_account_and_slot(
        &self,
        market_index: u16,
    ) -> Option<DataAndSlot<PerpMarket>> {
        self.perp_market_map.get(&market_index)
    }

    pub fn try_get_spot_market_account_and_slot(
        &self,
        market_index: u16,
    ) -> Option<DataAndSlot<SpotMarket>> {
        self.spot_market_map.get(&market_index)
    }

    pub fn try_get_oracle_price_data_and_slot(&self, market: MarketId) -> Option<Oracle> {
        self.oracle_map.get_by_market(&market)
    }

    /// Same as `try_get_oracle_price_data_and_slot` but checks the oracle pubkey has not changed
    /// this can be useful if the oracle address changes in the program
    pub fn try_get_oracle_price_data_and_slot_checked(&self, market: MarketId) -> Option<Oracle> {
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
    pub async fn get_account<T: AccountDeserialize + Pod>(&self, account: &Pubkey) -> SdkResult<T> {
        if let Some(value) = self.account_map.account_data(account) {
            Ok(value)
        } else {
            let account_data = self.rpc_client.get_account_data(account).await?;
            if account_data.is_empty() {
                return Err(SdkError::NoAccountData(*account));
            }
            T::try_deserialize(&mut account_data.as_slice())
                .map_err(|err| SdkError::Anchor(Box::new(err)))
        }
    }

    /// Fetch `account` as an Anchor account type `T` along with the retrieved slot
    pub async fn get_account_with_slot<T: AccountDeserialize + Pod>(
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
    pub async fn get_user_account(&self, account: &Pubkey) -> SdkResult<User> {
        self.get_account(account).await
    }

    /// Fetch `account` as a drift User account and slot it was fetched at
    ///
    /// uses latest cached if subscribed, otherwise falls back to network query
    pub async fn get_user_account_with_slot(
        &self,
        account: &Pubkey,
    ) -> SdkResult<DataAndSlot<User>> {
        self.get_account_with_slot(account).await
    }

    /// Try to fetch `account` as `T` using latest local value
    /// requires account was previously subscribed too.
    pub fn try_get_account<T: AccountDeserialize + Pod>(&self, account: &Pubkey) -> SdkResult<T> {
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
                .map_err(|err| SdkError::Rpc(Box::new(err))),
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
        if let Some(oracle) = self.try_get_oracle_price_data_and_slot(market) {
            Ok(oracle)
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

    #[cfg(feature = "unsafe_pub")]
    pub fn account_map(&self) -> &AccountMap {
        &self.account_map
    }

    #[cfg(feature = "unsafe_pub")]
    pub fn perp_market_map(&self) -> &MarketMap<PerpMarket> {
        &self.perp_market_map
    }

    #[cfg(feature = "unsafe_pub")]
    pub fn spot_market_map(&self) -> &MarketMap<SpotMarket> {
        &self.spot_market_map
    }

    #[cfg(feature = "unsafe_pub")]
    pub fn oracle_map(&self) -> &OracleMap {
        &self.oracle_map
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
/// Alternatively, use `DriftClient::init_tx` for simpler instantiation.
///
/// ```example(no_run)
/// use drift_rs::{types::Context, TransactionBuilder, Wallet};
///
/// let wallet = Wallet::from_seed_bs58("seed");
/// let client = DriftClient::new(Context::DevNet, "api.example.com", wallet).await.unwrap();
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
    /// sub-account data
    account_data: Cow<'a, User>,
    /// contextual on-chain program data
    program_data: &'a ProgramData,
    /// ordered list of instructions
    ixs: Vec<Instruction>,
    /// Tx lookup tables (v0 only)
    lookup_tables: Vec<AddressLookupTableAccount>,
    /// some markets forced to include in the tx accounts list
    force_markets: ForceMarkets,
    /// the drift sub-account address
    sub_account: Pubkey,
    /// either account authority or account delegate
    authority: Pubkey,
    /// use legacy transaction mode
    legacy: bool,
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
            lookup_tables: program_data.lookup_tables.to_vec(),
            legacy: false,
            force_markets: Default::default(),
        }
    }
    /// Pubkey of sub-account owner
    fn owner(&self) -> Pubkey {
        self.account_data.authority
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
    /// Extend the tx lookup tables (always includes the defacto drift LUTs)
    pub fn lookup_tables(mut self, lookup_tables: &[AddressLookupTableAccount]) -> Self {
        self.lookup_tables.extend_from_slice(lookup_tables);

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

    /// Set ix at index
    pub fn set_ix(mut self, idx: usize, ix: Instruction) -> Self {
        self.ixs[idx] = ix;
        self
    }

    /// Return the ixs currently included in the Transaction
    pub fn ixs(&self) -> &[Instruction] {
        &self.ixs
    }

    /// Deposit collateral into the user's account for a given spot market.
    ///
    /// Automatically derives the user's associated token account. Optionally supports reduce-only deposits.
    ///
    /// # Parameters
    /// - `amount`: The amount of collateral to deposit (in native units).
    /// - `market_index`: The spot market index to deposit into.
    /// - `reduce_only`: If `Some(true)`, only reduces an existing borrow; otherwise, acts as a normal deposit.
    /// - `transfer_hook`: transfer hook program address, if required by the spot token
    pub fn deposit(
        mut self,
        amount: u64,
        market_index: u16,
        reduce_only: Option<bool>,
        transfer_hook: Option<Pubkey>,
    ) -> Self {
        let spot_market = self
            .program_data
            .spot_market_config_by_index(market_index)
            .expect("spot markets syncd");
        let mut accounts = build_accounts(
            self.program_data,
            types::accounts::Deposit {
                state: *state_account(),
                user: self.sub_account,
                user_stats: Wallet::derive_stats_account(&self.owner()),
                authority: self.authority,
                spot_market_vault: spot_market.vault,
                user_token_account: Wallet::derive_associated_token_address(
                    &self.authority,
                    spot_market,
                ),
                token_program: spot_market.token_program(),
            },
            [self.account_data.as_ref()].into_iter(),
            self.force_markets.readable.iter(),
            [MarketId::spot(market_index)].iter(),
        );

        if spot_market.has_transfer_hook() {
            accounts.push(AccountMeta::new_readonly(
                transfer_hook.expect("requires transfer hook"),
                false,
            ));
        }

        let ix = Instruction {
            program_id: constants::PROGRAM_ID,
            accounts,
            data: InstructionData::data(&drift_idl::instructions::Deposit {
                market_index,
                amount,
                reduce_only: reduce_only.unwrap_or(false),
            }),
        };

        self.ixs.push(ix);

        self
    }

    /// Withdraw collateral from the user's account for a given spot market.
    ///
    /// Automatically derives the user's associated token account. Optionally supports reduce-only withdrawals.
    ///
    /// # Parameters
    /// - `amount`: The amount of collateral to withdraw (in native units).
    /// - `market_index`: The spot market index to withdraw from.
    /// - `reduce_only`: If `Some(true)`, only reduces an existing deposit; otherwise, acts as a normal withdrawal.
    /// - `transfer_hook`: transfer hook program address, if required by the spot token
    pub fn withdraw(
        mut self,
        amount: u64,
        market_index: u16,
        reduce_only: Option<bool>,
        transfer_hook: Option<Pubkey>,
    ) -> Self {
        let spot_market = self
            .program_data
            .spot_market_config_by_index(market_index)
            .expect("spot markets syncd");
        let mut accounts = build_accounts(
            self.program_data,
            types::accounts::Withdraw {
                state: *state_account(),
                user: self.sub_account,
                user_stats: Wallet::derive_stats_account(&self.owner()),
                authority: self.authority,
                spot_market_vault: spot_market.vault,
                user_token_account: Wallet::derive_associated_token_address(
                    &self.authority,
                    spot_market,
                ),
                drift_signer: constants::derive_drift_signer(),
                token_program: spot_market.token_program(),
            },
            [self.account_data.as_ref()].into_iter(),
            self.force_markets.readable.iter(),
            [MarketId::spot(market_index)]
                .iter()
                .chain(self.force_markets.writeable.iter()),
        );

        if spot_market.has_transfer_hook() {
            accounts.push(AccountMeta::new_readonly(
                transfer_hook.expect("requires transfer hook"),
                false,
            ));
        }

        let ix = Instruction {
            program_id: constants::PROGRAM_ID,
            accounts,
            data: InstructionData::data(&drift_idl::instructions::Withdraw {
                market_index,
                amount,
                reduce_only: reduce_only.unwrap_or(false),
            }),
        };

        self.ixs.push(ix);

        self
    }

    /// Place new orders for account
    ///
    /// * `orders` list of orders to place
    pub fn place_orders(mut self, orders: Vec<OrderParams>) -> Self {
        let mut readable_accounts: Vec<MarketId> = orders
            .iter()
            .map(|o| (o.market_index, o.market_type).into())
            .collect();
        readable_accounts.extend(&self.force_markets.readable);

        let mut accounts = build_accounts(
            self.program_data,
            types::accounts::PlaceOrders {
                state: *state_account(),
                authority: self.authority,
                user: self.sub_account,
            },
            [self.account_data.as_ref()].into_iter(),
            readable_accounts.iter(),
            self.force_markets.writeable.iter(),
        );

        if self
            .account_data
            .margin_mode
            .is_high_leverage_mode(MarginRequirementType::Maintenance)
            || orders.iter().any(|x| x.high_leverage_mode())
        {
            accounts.push(AccountMeta::new(*high_leverage_mode_account(), false));
        }

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
            [self.account_data.as_ref()].into_iter(),
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
    /// * `market` - tuple of market index and type (spot or perp)
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
            [self.account_data.as_ref()].into_iter(),
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
            [self.account_data.as_ref()].into_iter(),
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
            [self.account_data.as_ref()].into_iter(),
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
            [self.account_data.as_ref()].into_iter(),
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
            [self.account_data.as_ref()].into_iter(),
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
                user_stats: Wallet::derive_stats_account(&self.owner()),
                taker: *taker,
                taker_stats: Wallet::derive_stats_account(&taker_account.authority),
            },
            [self.account_data.as_ref(), taker_account].into_iter(),
            self.force_markets.readable.iter(),
            if is_perp {
                perp_writable.iter()
            } else {
                spot_writable.iter()
            }
            .chain(self.force_markets.writeable.iter()),
        );

        if order.high_leverage_mode()
            || taker_info
                .1
                .margin_mode
                .is_high_leverage_mode(MarginRequirementType::Maintenance)
        {
            accounts.push(AccountMeta::new(*high_leverage_mode_account(), false));
        }

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
    /// * `maker_info` - pubkey of the maker/counter-party(s) to take against and account data
    /// * `referrer` - pubkey of the maker's referrer account, if any
    /// * `fulfillment_type` - type of fill for spot orders, ignored for perp orders
    pub fn place_and_take(
        mut self,
        order: OrderParams,
        maker_info: &[(Pubkey, User)],
        referrer: Option<Pubkey>,
        fulfillment_type: Option<SpotFulfillmentType>,
        success_condition: Option<u32>,
    ) -> Self {
        let mut user_accounts = vec![self.account_data.as_ref()];

        for (_maker, maker_account) in maker_info {
            user_accounts.push(maker_account);
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
                user_stats: Wallet::derive_stats_account(&self.owner()),
            },
            user_accounts.into_iter(),
            self.force_markets.readable.iter(),
            if is_perp {
                perp_writable.iter()
            } else {
                spot_writable.iter()
            }
            .chain(self.force_markets.writeable.iter()),
        );

        if is_perp && order.high_leverage_mode() {
            accounts.push(AccountMeta::new(*high_leverage_mode_account(), false));
        }

        // if referrer is maker don't add account again
        if referrer.is_some_and(|r| !maker_info.iter().any(|(m, _)| *m == r)) {
            let referrer = referrer.unwrap();
            accounts.push(AccountMeta::new(
                Wallet::derive_stats_account(&referrer),
                false,
            ));
            accounts.push(AccountMeta::new(referrer, false));
        }

        for (maker, maker_account) in maker_info {
            accounts.push(AccountMeta::new(*maker, false));
            accounts.push(AccountMeta::new(
                Wallet::derive_stats_account(&maker_account.authority),
                false,
            ));
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
    /// * `maker_order` - order params defined by the maker, e.g. partial or full fill
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
                user_stats: Wallet::derive_stats_account(&self.owner()),
                taker: signed_order_info.taker_subaccount(),
                taker_stats: Wallet::derive_stats_account(&taker_account.authority),
                taker_signed_msg_user_orders: Wallet::derive_swift_order_account(
                    &taker_account.authority,
                ),
            },
            [self.account_data.as_ref(), taker_account].into_iter(),
            self.force_markets.readable.iter(),
            perp_writable
                .iter()
                .chain(self.force_markets.writeable.iter()),
        );

        if taker_account_referrer != &DEFAULT_PUBKEY {
            accounts.push(AccountMeta::new(*taker_account_referrer, false));
            accounts.push(AccountMeta::new(
                Wallet::derive_stats_account(taker_account_referrer),
                false,
            ));
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
    /// * `taker_account` - taker subaccount data
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

        let perp_readable = [MarketId::perp(order_params.market_index)];
        let mut accounts = build_accounts(
            self.program_data,
            types::accounts::PlaceSignedMsgTakerOrder {
                state: *state_account(),
                authority: self.authority,
                user: signed_order_info.taker_subaccount(),
                user_stats: Wallet::derive_stats_account(&taker_account.authority),
                signed_msg_user_orders: Wallet::derive_swift_order_account(
                    &taker_account.authority,
                ),
                ix_sysvar: SYSVAR_INSTRUCTIONS_PUBKEY,
            },
            [taker_account].into_iter(),
            perp_readable
                .iter()
                .chain(self.force_markets.readable.iter()),
            self.force_markets.writeable.iter(),
        );

        if signed_order_info.order_params().high_leverage_mode()
            || taker_account
                .margin_mode
                .is_high_leverage_mode(MarginRequirementType::Maintenance)
        {
            accounts.push(AccountMeta::new(*high_leverage_mode_account(), false));
        }

        let swift_taker_ix_data = signed_order_info.to_ix_data();
        let ed25519_verify_ix = crate::utils::new_ed25519_ix_ptr(
            swift_taker_ix_data.as_slice(),
            self.ixs.len() as u16 + 1,
            None,
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

    /// Set the subaccount's _max_ initial margin ratio.
    ///
    /// * `sub_account_id` - index of the subaccount
    /// * `margin_ratio` - new margin ratio in MARGIN_PRECISION
    ///
    /// MARGIN_PRECISION => 1x leverage
    /// MARGIN_PRECISION * 10 => .1x leverage
    /// MARGIN_PRECISION / 10 =>  10x leverage
    ///
    pub fn set_max_initial_margin_ratio(mut self, margin_ratio: u32, sub_account_id: u16) -> Self {
        let accounts = build_accounts(
            self.program_data,
            types::accounts::UpdateUserCustomMarginRatio {
                authority: self.authority,
                user: self.sub_account,
            },
            [self.account_data.as_ref()].into_iter(),
            std::iter::empty(),
            std::iter::empty(),
        );
        let ix = Instruction {
            program_id: constants::PROGRAM_ID,
            accounts,
            data: InstructionData::data(&drift_idl::instructions::UpdateUserCustomMarginRatio {
                sub_account_id,
                margin_ratio,
            }),
        };
        self.ixs.push(ix);

        self
    }

    /// Add a spot `begin_swap` ix
    ///
    /// This should be followed by a subsequent `end_swap` ix
    pub fn begin_swap(
        mut self,
        amount_in: u64,
        in_market: &SpotMarket,
        out_market: &SpotMarket,
        payer_token_account: &Pubkey,
        payee_token_account: &Pubkey,
    ) -> Self {
        let in_token_program = in_market.token_program();
        let out_token_program = out_market.token_program();

        let mut accounts = build_accounts(
            self.program_data,
            types::accounts::BeginSwap {
                state: *state_account(),
                user: self.sub_account,
                user_stats: Wallet::derive_stats_account(&self.owner()),
                authority: self.authority,
                out_spot_market_vault: out_market.vault,
                in_spot_market_vault: in_market.vault,
                in_token_account: *payer_token_account,
                out_token_account: *payee_token_account,
                token_program: in_token_program,
                drift_signer: self.program_data.state().signer,
                instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
            },
            [self.account_data.as_ref()].into_iter(),
            [MarketId::QUOTE_SPOT].iter(),
            [
                MarketId::spot(in_market.market_index),
                MarketId::spot(out_market.market_index),
            ]
            .iter(),
        );

        if out_token_program != in_token_program {
            accounts.push(AccountMeta::new_readonly(out_token_program, false));
        }

        if out_market.is_token_2022_program() || in_market.is_token_2022_program() {
            accounts.push(AccountMeta::new_readonly(in_market.mint, false));
            accounts.push(AccountMeta::new_readonly(out_market.mint, false));
        }

        let ix = Instruction {
            program_id: constants::PROGRAM_ID,
            accounts,
            data: InstructionData::data(&drift_idl::instructions::BeginSwap {
                in_market_index: in_market.market_index,
                out_market_index: out_market.market_index,
                amount_in,
            }),
        };
        self.ixs.push(ix);

        self
    }

    /// Add a spot `end_swap` ix
    ///
    /// This should follow a preceding `begin_swap` ix
    pub fn end_swap(
        mut self,
        in_market: &SpotMarket,
        out_market: &SpotMarket,
        payer_token_account: &Pubkey,
        payee_token_account: &Pubkey,
        limit_price: Option<u64>,
        reduce_only: Option<SwapReduceOnly>,
    ) -> Self {
        let out_token_program = out_market.token_program();
        let in_token_program = in_market.token_program();

        let mut accounts = build_accounts(
            self.program_data,
            types::accounts::EndSwap {
                state: *state_account(),
                user: self.sub_account,
                user_stats: Wallet::derive_stats_account(&self.owner()),
                authority: self.authority,
                out_spot_market_vault: out_market.vault,
                in_spot_market_vault: in_market.vault,
                in_token_account: *payer_token_account,
                out_token_account: *payee_token_account,
                token_program: in_token_program,
                drift_signer: self.program_data.state().signer,
                instructions: SYSVAR_INSTRUCTIONS_PUBKEY,
            },
            [self.account_data.as_ref()].into_iter(),
            [MarketId::QUOTE_SPOT].iter(),
            [
                MarketId::spot(in_market.market_index),
                MarketId::spot(out_market.market_index),
            ]
            .iter(),
        );

        if out_token_program != in_token_program {
            accounts.push(AccountMeta::new_readonly(out_token_program, false));
        }

        if out_market.is_token_2022_program() || in_market.is_token_2022_program() {
            accounts.push(AccountMeta::new_readonly(in_market.mint, false));
            accounts.push(AccountMeta::new_readonly(out_market.mint, false));
        }

        let ix = Instruction {
            program_id: constants::PROGRAM_ID,
            accounts,
            data: InstructionData::data(&drift_idl::instructions::EndSwap {
                in_market_index: in_market.market_index,
                out_market_index: out_market.market_index,
                limit_price,
                reduce_only,
            }),
        };
        self.ixs.push(ix);

        self
    }

    /// Add a Jupiter token swap to the tx
    ///
    /// # Arguments
    /// * `jupiter_swap_info` - Jupiter swap route and instructions
    /// * `in_market` - Spot market of the input token
    /// * `out_market` - Spot market of the output token
    /// * `in_token_account` - Input token account pubkey
    /// * `out_token_account` - Output token account pubkey
    /// * `limit_price` - Set a limit price
    /// * `reduce_only` - Set a reduce only order
    pub fn jupiter_swap(
        mut self,
        jupiter_swap_info: JupiterSwapInfo,
        in_market: &SpotMarket,
        out_market: &SpotMarket,
        in_token_account: &Pubkey,
        out_token_account: &Pubkey,
        limit_price: Option<u64>,
        reduce_only: Option<SwapReduceOnly>,
    ) -> Self {
        let jupiter_swap_ixs = jupiter_swap_info.ixs;

        // initialize token accounts
        if !jupiter_swap_ixs.setup_instructions.is_empty() {
            // jupiter swap ixs imply account creation is required
            // provide our own creation ixs
            // new_self.ixs.extend(jupiter_swap_ixs.setup_instructions);
            let create_in_account_ix = Instruction {
                program_id: ASSOCIATED_TOKEN_PROGRAM_ID,
                accounts: vec![
                    AccountMeta::new(self.authority, true), // payer
                    AccountMeta::new(*in_token_account, false),
                    AccountMeta::new_readonly(self.authority, false), // wallet
                    AccountMeta::new_readonly(in_market.mint, false),
                    AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
                    AccountMeta::new_readonly(in_market.token_program(), false),
                ],
                data: vec![1], // idempotent mode
            };
            let create_out_account_ix = Instruction {
                program_id: ASSOCIATED_TOKEN_PROGRAM_ID,
                accounts: vec![
                    AccountMeta::new(self.authority, true), // payer
                    AccountMeta::new(*out_token_account, false),
                    AccountMeta::new_readonly(self.authority, false), // wallet
                    AccountMeta::new_readonly(out_market.mint, false),
                    AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
                    AccountMeta::new_readonly(out_market.token_program(), false),
                ],
                data: vec![1], // idempotent mode
            };
            self.ixs
                .extend_from_slice(&[create_in_account_ix, create_out_account_ix]);
        }

        let mut new_self = self.begin_swap(
            jupiter_swap_info.quote.in_amount,
            in_market,
            out_market,
            in_token_account,
            out_token_account,
        );

        // TODO: support jito bundle
        if !jupiter_swap_ixs.other_instructions.is_empty() {
            panic!("jupiter swap unsupported ix: Jito tip");
        }

        new_self.ixs.push(jupiter_swap_ixs.swap_instruction);

        // support SOL unwrap ixs, ignore account delete/reclaim ixs
        if let Some(unwrap_ix) = jupiter_swap_ixs.cleanup_instruction {
            if unwrap_ix.program_id != TOKEN_PROGRAM_ID
                && unwrap_ix.program_id != TOKEN_2022_PROGRAM_ID
            {
                new_self.ixs.push(unwrap_ix);
            }
        }

        new_self = new_self.end_swap(
            in_market,
            out_market,
            in_token_account,
            out_token_account,
            limit_price,
            reduce_only,
        );

        // Add the jup tx LUTs
        new_self.lookup_tables(&jupiter_swap_info.luts)
    }

    /// Settle perp PnL for some user account and market
    ///
    /// * `market_index` market to settle position for
    /// * `target_pubkey` target subaccount address, leave None to settle PnL for the signer
    /// * `target_account` target subaccount data, leave None to settle PnL for the signer
    ///
    pub fn settle_pnl(
        mut self,
        market_index: u16,
        target_pubkey: Option<&Pubkey>,
        target_account: Option<&User>,
    ) -> Self {
        let accounts = build_accounts(
            self.program_data,
            types::accounts::SettlePnl {
                state: *state_account(),
                user: target_pubkey.copied().unwrap_or(self.sub_account),
                authority: self.authority,
                spot_market_vault: self
                    .program_data
                    .spot_market_config_by_index(MarketId::QUOTE_SPOT.index())
                    .unwrap()
                    .vault,
            },
            [target_account.unwrap_or(&self.account_data)].into_iter(),
            std::iter::empty(),
            [MarketId::QUOTE_SPOT, MarketId::perp(market_index)].iter(),
        );

        let ix = Instruction {
            program_id: constants::PROGRAM_ID,
            accounts,
            data: InstructionData::data(&drift_idl::instructions::SettlePnl { market_index }),
        };

        self.ixs.push(ix);

        self
    }

    /// Settle perp multiple PnLs for user account and markets
    ///
    /// * `markets` market indexes to settle positions for
    /// * `mode` Choose Must or try settle PnL
    /// * `target_pubkey` target subaccount address, leave None to settle PnL for the signer
    /// * `target_account` target subaccount data, leave None to settle PnL for the signer
    ///
    pub fn settle_pnl_multi(
        mut self,
        markets: &[u16],
        mode: SettlePnlMode,
        target_pubkey: Option<&Pubkey>,
        target_account: Option<&User>,
    ) -> Self {
        let perp_iter: Vec<MarketId> = markets.iter().map(|i| MarketId::perp(*i)).collect();
        let accounts = build_accounts(
            self.program_data,
            types::accounts::SettlePnl {
                state: *state_account(),
                user: target_pubkey.copied().unwrap_or(self.sub_account),
                authority: self.authority,
                spot_market_vault: self
                    .program_data
                    .spot_market_config_by_index(MarketId::QUOTE_SPOT.index())
                    .unwrap()
                    .vault,
            },
            [target_account.unwrap_or(&self.account_data)].into_iter(),
            std::iter::empty(),
            perp_iter
                .iter()
                .chain(std::iter::once(&MarketId::QUOTE_SPOT)),
        );

        let ix = Instruction {
            program_id: constants::PROGRAM_ID,
            accounts,
            data: InstructionData::data(&drift_idl::instructions::SettleMultiplePnls {
                market_indexes: markets.to_vec(),
                mode,
            }),
        };

        self.ixs.push(ix);

        self
    }

    /// Fill a perpetual order by matching it against maker orders
    ///
    /// This instruction allows a filler to execute a taker's order by matching it against
    /// existing maker orders in the order book. The filler receives a fee for providing
    /// liquidity and executing the trade.
    ///
    /// * `market_index` - the perpetual market index to fill orders on
    /// * `taker` - the taker's subaccount pubkey
    /// * `taker_account` - the taker's user account data
    /// * `taker_stats` - the taker's user stats account data
    /// * `taker_order_id` - optional order ID to fill, if None fills the best available order
    /// * `makers` - list of maker user accounts that will provide liquidity
    pub fn fill_perp_order(
        mut self,
        market_index: u16,
        taker: Pubkey,
        taker_account: &User,
        taker_stats: &UserStats,
        taker_order_id: Option<u32>,
        makers: &[User],
    ) -> Self {
        let mut accounts = build_accounts(
            self.program_data,
            types::accounts::FillPerpOrder {
                state: *state_account(),
                authority: self.authority,
                user: taker,
                user_stats: Wallet::derive_stats_account(&taker_account.authority),
                filler: self.sub_account,
                filler_stats: Wallet::derive_stats_account(&self.owner()),
            },
            makers.iter().chain(std::iter::once(taker_account)),
            std::iter::empty(),
            std::iter::once(&MarketId::perp(market_index)),
        );

        for maker in makers {
            accounts.extend([
                AccountMeta::new(
                    Wallet::derive_user_account(&maker.authority, maker.sub_account_id),
                    false,
                ),
                AccountMeta::new(Wallet::derive_stats_account(&maker.authority), false),
            ]);
        }

        if taker_stats.is_referred() {
            accounts.extend([
                AccountMeta::new(Wallet::derive_user_account(&taker_stats.referrer, 0), false),
                AccountMeta::new(Wallet::derive_stats_account(&taker_stats.referrer), false),
            ]);
        }

        let ix = Instruction {
            program_id: constants::PROGRAM_ID,
            accounts,
            data: InstructionData::data(&drift_idl::instructions::FillPerpOrder {
                order_id: taker_order_id,
                maker_order_id: None,
            }),
        };

        self.ixs.push(ix);
        self
    }

    /// Trigger a conditional order (stop loss, take profit, etc.)
    ///
    /// This instruction allows a filler to trigger a conditional order when the specified
    /// market conditions are met. Conditional orders include stop losses, take profits,
    /// and other trigger-based order types.
    ///
    /// * `user` - the user's subaccount pubkey that owns the conditional order
    /// * `user_account` - the user's account data containing the conditional order
    /// * `order_id` - the ID of the conditional order to trigger
    /// * `market` - tuple of (market_index, market_type) for the market the order is on
    pub fn trigger_order(
        mut self,
        user: Pubkey,
        user_account: &User,
        order_id: u32,
        market: (u16, MarketType),
    ) -> Self {
        let accounts = build_accounts(
            self.program_data,
            types::accounts::TriggerOrder {
                state: *state_account(),
                authority: self.authority,
                user,
                filler: self.sub_account,
            },
            std::iter::once(user_account),
            std::iter::empty(),
            std::iter::once(&MarketId::from(market)),
        );

        let ix = Instruction {
            program_id: constants::PROGRAM_ID,
            accounts,
            data: InstructionData::data(&drift_idl::instructions::TriggerOrder { order_id }),
        };

        self.ixs.push(ix);

        self
    }

    /// Initialize a Swift (signed message) order account for the authority/wallet.
    ///
    /// Prepares the account for off-chain signed order flow.
    pub fn initialize_swift_account(mut self) -> Self {
        let accounts = build_accounts(
            self.program_data,
            types::accounts::InitializeSignedMsgUserOrders {
                signed_msg_user_orders: Wallet::derive_swift_order_account(&self.authority),
                authority: self.authority,
                payer: self.authority,
                rent: SYSVAR_RENT_PUBKEY,
                system_program: SYSTEM_PROGRAM_ID,
            },
            std::iter::empty(),
            std::iter::empty(),
            std::iter::empty(),
        );

        let ix = Instruction {
            program_id: constants::PROGRAM_ID,
            accounts,
            data: InstructionData::data(&drift_idl::instructions::InitializeSignedMsgUserOrders {
                num_orders: 16,
            }),
        };
        self.ixs.push(ix);

        self
    }

    /// Initialize a new user account (subaccount) for the authority/wallet.
    ///
    /// Optionally set a custom name and referrer.
    /// For `sub_account_id = 0`, also initializes the user stats account.
    ///
    /// # Parameters
    /// - `sub_account_id`: The subaccount index to initialize (0 for main account).
    /// - `name`: Optional custom name for the account. If `None`, a default name is used.
    /// - `referrer`: Optional referrer pubkey for the account.
    ///
    /// # Example
    /// ```
    /// use drift_rs::{TransactionBuilder, Wallet};
    /// use solana_sdk::pubkey::Pubkey;
    ///
    /// let wallet = Wallet::new_random();
    /// let program_data = /* obtain ProgramData */;
    /// let sub_account_id = 0;
    /// let mut builder = TransactionBuilder::new(&program_data, wallet.default_sub_account(), /* user data */, false);
    ///
    /// // Initialize the user account and the swift account, then deposit 100_000 USDC (spot market 0)
    /// builder = builder
    ///     .initialize_user_account(sub_account_id, None, None)
    ///     .initialize_swift_account()
    ///     .deposit(100_000, 0, None);
    /// ```
    pub fn initialize_user_account(
        mut self,
        sub_account_id: u16,
        name: Option<String>,
        referrer: Option<Pubkey>,
    ) -> Self {
        let mut accounts = build_accounts(
            self.program_data,
            types::accounts::InitializeUser {
                state: *state_account(),
                authority: self.authority,
                user: Wallet::derive_user_account(&self.authority, sub_account_id),
                user_stats: Wallet::derive_stats_account(&self.owner()),
                payer: self.authority,
                rent: SYSVAR_RENT_PUBKEY,
                system_program: SYSTEM_PROGRAM_ID,
            },
            std::iter::empty(),
            std::iter::empty(),
            std::iter::empty(),
        );

        if let Some(referrer) = referrer {
            accounts.extend_from_slice(&[
                AccountMeta::new(Wallet::derive_user_account(&referrer, 0), false),
                AccountMeta::new(Wallet::derive_stats_account(&referrer), false),
            ]);
        }

        if sub_account_id == 0 {
            let ix = Instruction {
                program_id: constants::PROGRAM_ID,
                accounts: accounts.clone(),
                data: InstructionData::data(&drift_idl::instructions::InitializeUserStats {}),
            };
            self.ixs.push(ix);
        }

        let name = name.unwrap_or_else(|| {
            if sub_account_id == 0 {
                "Main Account".into()
            } else {
                format!("Subaccount {}", sub_account_id + 1)
            }
        });

        let ix = Instruction {
            program_id: constants::PROGRAM_ID,
            accounts,
            data: InstructionData::data(&drift_idl::instructions::InitializeUser {
                sub_account_id,
                name: name.as_bytes()[..32].try_into().unwrap(),
            }),
        };

        self.ixs.push(ix);

        self
    }

    /// Liquidate a perp position for a given user.
    ///
    /// This method constructs a liquidation instruction for a perpetual market position.
    /// The liquidator will be the subaccount associated with this `TransactionBuilder` (i.e., the builder's default subaccount).
    ///
    /// # Parameters
    /// - `market_index`: The index of the perp market to liquidate on.
    /// - `user_account`: The user account (liquidatee) whose position will be liquidated.
    /// - `liquidator_max_base_asset_amount`: The maximum base asset amount the liquidator is willing to liquidate.
    /// - `limit_price`: Optional limit price for the liquidation (if `None`, no limit is set).
    ///
    /// # Returns
    /// Returns an updated `TransactionBuilder` with the liquidation instruction appended.
    pub fn liquidate_perp(
        mut self,
        market_index: u16,
        user_account: &User,
        liquidator_max_base_asset_amount: u64,
        limit_price: Option<u64>,
    ) -> Self {
        let accounts = build_accounts(
            self.program_data,
            types::accounts::LiquidatePerp {
                state: *state_account(),
                authority: self.authority,
                user: Wallet::derive_user_account(
                    &user_account.authority,
                    user_account.sub_account_id,
                ),
                user_stats: Wallet::derive_stats_account(&user_account.authority),
                liquidator: self.sub_account,
                liquidator_stats: Wallet::derive_stats_account(&self.owner()),
            },
            [&self.account_data, user_account].into_iter(),
            std::iter::empty(),
            std::iter::once(&MarketId::perp(market_index)),
        );

        let liquidate_ix = Instruction {
            program_id: PROGRAM_ID,
            accounts,
            data: InstructionData::data(&drift_idl::instructions::LiquidatePerp {
                market_index,
                liquidator_max_base_asset_amount,
                limit_price,
            }),
        };

        self.ixs.push(liquidate_ix);
        self
    }

    /// Post a Pyth Lazer oracle update
    ///
    /// Appends an Ed25519 signature verify ix and Pyth Lazer oracle update ix to the transaction.
    ///
    /// # Parameters
    ///
    /// - `feed_ids`: Pyth Lazer feed IDs for which the oracle update should be posted.
    /// - `pyth_message`: the Pyth message update
    ///
    /// # Returns
    ///
    /// Returns the updated `TransactionBuilder` with the new instructions appended.
    pub fn post_pyth_lazer_oracle_update(mut self, feed_ids: &[u32], pyth_message: &[u8]) -> Self {
        let ed25519_verify_ix =
            crate::utils::new_ed25519_ix_ptr(pyth_message, self.ixs.len() as u16 + 1, Some(4));

        let mut accounts = build_accounts(
            self.program_data,
            types::accounts::PostPythLazerOracleUpdate {
                keeper: self.authority,
                pyth_lazer_storage: PYTH_LAZER_STORAGE_ACCOUNT_KEY,
                ix_sysvar: SYSVAR_INSTRUCTIONS_PUBKEY,
            },
            std::iter::empty(),
            std::iter::empty(),
            std::iter::empty(),
        );
        accounts.extend(feed_ids.iter().map(|f| {
            AccountMeta::new(crate::utils::derive_pyth_lazer_oracle_public_key(*f), false)
        }));

        let pyth_update_ix = Instruction {
            program_id: PROGRAM_ID,
            accounts,
            data: InstructionData::data(&drift_idl::instructions::PostPythLazerOracleUpdate {
                pyth_message: pyth_message.to_vec(),
            }),
        };

        self.ixs
            .extend_from_slice(&[ed25519_verify_ix, pyth_update_ix]);

        self
    }

    pub fn disable_user_hlm(
        mut self,
        user: Pubkey,
        user_account_data: &User,
        disable_maintenance: bool,
    ) -> Self {
        let accounts = build_accounts(
            self.program_data,
            types::accounts::DisableUserHighLeverageMode {
                authority: self.authority,
                state: *state_account(),
                user,
                high_leverage_mode_config: *high_leverage_mode_account(),
            },
            [user_account_data].into_iter(),
            std::iter::empty(),
            std::iter::empty(),
        );
        let ix = Instruction {
            program_id: PROGRAM_ID,
            accounts,
            data: InstructionData::data(&drift_idl::instructions::DisableUserHighLeverageMode {
                disable_maintenance,
            }),
        };

        self.ixs.push(ix);
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

    /// Update Pyth pull oracle with VAA data
    ///
    /// This method adds ix to update a Pyth pull oracle using VAA data.
    /// NOTE: Only the first update in the VAA will be posted
    ///
    /// ## Parameters
    /// * `vaa_proof` - ` Base64 encoded VAA string from Pyth
    /// * `feed_id` - The Pyth feed ID to update (can include 0x prefix)
    /// * `vaa_signature_count` - max. number of VAA signatures to keep in the proof (default: 2)
    ///
    /// ## Panics
    /// if the provided VAA proof is invalid
    ///
    /// ## Example
    /// ```no_run
    /// use drift_rs::{TransactionBuilder, Wallet};
    /// use solana_pubkey::Pubkey;
    ///
    /// let wallet = Wallet::new();
    /// let program_data =
    /// let mut builder = TransactionBuilder::new(&program_data, wallet.default_sub_account(), /* user data */, false);
    ///
    /// // Update Pyth pull oracle
    /// builder = builder.post_pyth_pull_oracle_update_atomic(
    ///     "<BASE64_ENCODED_VAA>",
    ///     &hex_literal::hex!("1234567890abcdef"),
    ///     Some(2),
    /// );
    /// ```
    pub fn post_pyth_pull_oracle_update_atomic(
        mut self,
        vaa_proof: &str,
        feed_id: &[u8; 32],
        vaa_signature_count: Option<usize>,
    ) -> Self {
        // Parse VAA data
        let vaa_bytes = base64::prelude::BASE64_STANDARD.decode(vaa_proof).unwrap();
        let accumulator_update_data = AccumulatorUpdateData::try_from_slice(&vaa_bytes).unwrap();
        let Proof::WormholeMerkle { vaa, updates } = accumulator_update_data.proof;
        let mut vaa: Vec<u8> = vaa.into();

        // Read guardian set index from VAA (big-endian, offset 1)
        let guardian_set_index = u32::from_be_bytes(vaa[1..=4].try_into().unwrap());

        // Get guardian set PDA
        let guardian_set = {
            let guardian_set_bytes = guardian_set_index.to_be_bytes();
            let seeds = &[b"GuardianSet".as_slice(), &guardian_set_bytes];
            let (pubkey, _bump) = Pubkey::find_program_address(seeds, &wormhole_program::ID);
            pubkey
        };

        // Get Pyth pull oracle public key
        let price_feed = {
            let seeds = [b"pyth_pull".as_slice(), feed_id];
            let (pubkey, _bump) = Pubkey::find_program_address(&seeds, &constants::PROGRAM_ID);
            pubkey
        };

        // strip extraneous VAA signatures
        let existing_signature_count = vaa[5] as usize;
        let target_signature_count = vaa_signature_count.unwrap_or(2);
        if existing_signature_count > target_signature_count {
            // truncate extra signatures
            let _: Vec<u8> = vaa
                .drain(6 + (target_signature_count * 66)..6 + existing_signature_count * 66)
                .collect();
            vaa[5] = target_signature_count as u8;
        }

        // only post first update from the VAA
        if let Some(update) = updates.first() {
            let params = PostUpdateAtomicParams {
                vaa,
                merkle_price_update: update.clone(),
                treasury_id: 0,
            };
            let encoded_params = params.try_to_vec().unwrap();

            let accounts = build_accounts(
                self.program_data,
                drift_idl::accounts::PostPythPullOracleUpdateAtomic {
                    keeper: self.authority,
                    pyth_solana_receiver: drift_oracle_receiver_program::ID,
                    guardian_set,
                    price_feed,
                },
                std::iter::empty(),
                std::iter::empty(),
                std::iter::empty(),
            );

            // Add the oracle update instruction
            let oracle_update_ix = Instruction {
                program_id: constants::PROGRAM_ID,
                accounts,
                data: InstructionData::data(
                    &drift_idl::instructions::PostPythPullOracleUpdateAtomic {
                        feed_id: *feed_id,
                        params: encoded_params,
                    },
                ),
            };

            self.ixs.push(oracle_update_ix);
        }

        self
    }
    /// Update user position margin ratio
    ///
    /// ## Params
    /// * `market_index` - perp market index of the position
    /// * `margin_ratio` - new margin ratio for the position
    ///
    pub fn update_user_perp_position_custom_margin_ratio(
        mut self,
        market_index: u16,
        margin_ratio: u16,
    ) -> Self {
        let accounts = build_accounts(
            self.program_data(),
            drift_idl::accounts::UpdateUserPerpPositionCustomMarginRatio {
                user: self.sub_account,
                authority: self.owner(),
            },
            std::iter::empty(),
            std::iter::empty(),
            std::iter::empty(),
        );
        self.ixs.push(Instruction {
            program_id: PROGRAM_ID,
            accounts,
            data: InstructionData::data(
                &drift_idl::instructions::UpdateUserPerpPositionCustomMarginRatio {
                    sub_account_id: self.account_data.sub_account_id,
                    perp_market_index: market_index,
                    margin_ratio,
                },
            ),
        });
        self
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
    users: impl Iterator<Item = &'a User>,
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
        // always manually try to include the quote (USDC) market
        // TODO: this is not exactly the same semantics as the TS sdk
        include_market(MarketId::QUOTE_SPOT.index(), MarketType::Spot, false);
    }

    let mut account_metas = base_accounts.to_account_metas();
    account_metas.extend(accounts.into_iter().map(Into::into));
    account_metas
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
    use solana_sdk::signature::Keypair;
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
            account_map: AccountMap::new(
                Arc::clone(&pubsub_client),
                Arc::clone(&rpc_client),
                CommitmentConfig::processed(),
            ),
            grpc_unsub: Default::default(),
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

    #[tokio::test]
    async fn test_place_orders_high_leverage() {
        // Create a test user with high leverage mode
        let mut user = User::default();
        user.margin_mode = MarginMode::HighLeverage;
        let user = Cow::Owned(user);

        // Create program data
        let program_data = ProgramData::new(
            vec![SpotMarket::default()],
            vec![PerpMarket::default()],
            vec![],
            State::default(),
        );
        let sub_account = Pubkey::new_unique();

        // Create transaction builder
        let builder = TransactionBuilder::new(&program_data, sub_account, user, false);

        // Test case 1: Place orders with high leverage mode account included due to user margin mode
        let orders = vec![OrderParams {
            market_index: 0,
            market_type: MarketType::Perp,
            direction: PositionDirection::Long,
            order_type: OrderType::Limit,
            ..Default::default()
        }];

        let tx = builder.place_orders(orders).build();

        // Check that high leverage mode account is included
        let high_leverage_account = *high_leverage_mode_account();
        assert!(tx.static_account_keys().contains(&high_leverage_account));

        // Test case 2: Place orders with high leverage mode account included due to order params
        let mut user = User::default();
        user.margin_mode = MarginMode::Default; // Not high leverage
        let user = Cow::Owned(user);
        let builder = TransactionBuilder::new(&program_data, sub_account, user, false);

        let orders = vec![OrderParams {
            market_index: 0,
            market_type: MarketType::Perp,
            direction: PositionDirection::Long,
            order_type: OrderType::Limit,
            bit_flags: OrderParams::HIGH_LEVERAGE_MODE_FLAG,
            ..Default::default()
        }];

        let tx = builder.place_orders(orders).build();

        // Check that high leverage mode account is included
        assert!(tx.static_account_keys().contains(&high_leverage_account));
    }
}
