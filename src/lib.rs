//! Drift SDK

use std::{borrow::Cow, rc::Rc, sync::Arc, time::Duration};

use anchor_lang::{AccountDeserialize, Discriminator, InstructionData, ToAccountMetas};
use async_utils::{retry_policy, spawn_retry_task};
use blockhash_subscriber::BlockhashSubscriber;
use constants::derive_perp_market_account;
use drift::{
    controller::position::PositionDirection,
    instructions::SpotFulfillmentType,
    state::{
        oracle::get_oracle_price,
        order_params::{ModifyOrderParams, OrderParams},
        perp_market::PerpMarket,
        spot_market::SpotMarket,
        state::State,
        user::{MarketType, Order, OrderStatus, PerpPosition, SpotPosition, User, UserStats},
    },
};
use event_emitter::EventEmitter;
use fnv::FnvHashMap;
use futures_util::{future::BoxFuture, FutureExt, StreamExt};
use log::{debug, warn};
use marketmap::MarketMap;
use oraclemap::{Oracle, OracleMap};
use solana_account_decoder::UiAccountEncoding;
use solana_client::{
    nonblocking::{pubsub_client::PubsubClient, rpc_client::RpcClient},
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    rpc_filter::{Memcmp, RpcFilterType},
};
use solana_sdk::{
    account::Account,
    account_info::IntoAccountInfo,
    clock::Slot,
    commitment_config::{CommitmentConfig, CommitmentLevel},
    compute_budget::ComputeBudgetInstruction,
    hash::Hash,
    instruction::{AccountMeta, Instruction},
    message::{v0, Message, VersionedMessage},
    signature::{keypair_from_seed, Keypair, Signature},
    signer::Signer,
    transaction::VersionedTransaction,
};
pub use solana_sdk::{address_lookup_table_account::AddressLookupTableAccount, pubkey::Pubkey};
use tokio::{
    select,
    sync::{
        watch::{self, Receiver},
        RwLock,
    },
};
use user::DriftUser;
use utils::get_ws_url;
use websocket_account_subscriber::{AccountUpdate, WebsocketAccountSubscriber};

use crate::{
    constants::{
        derive_spot_market_account, market_lookup_table, state_account, MarketExt, ProgramData,
    },
    utils::decode,
};

// utils
pub mod async_utils;
pub mod math;
pub mod memcmp;
pub mod utils;

// constants & types
pub mod constants;
pub mod types;

// internal infra
pub mod event_emitter;
pub mod websocket_account_subscriber;
pub mod websocket_program_account_subscriber;

// subscribers
pub mod auction_subscriber;
pub mod blockhash_subscriber;
pub mod dlob_client;
pub mod event_subscriber;

#[cfg(feature = "jit")]
pub mod jit_client;

pub mod marketmap;
pub mod oraclemap;
pub mod slot_subscriber;
pub mod usermap;

// wrappers
pub mod user;

pub mod dlob;

use types::*;

/// Provides solana Account fetching API
pub trait AccountProvider: 'static + Sized + Send + Sync {
    // TODO: async fn when it stabilizes
    /// Return the Account information of `account`
    fn get_account(&self, account: Pubkey) -> BoxFuture<SdkResult<Account>>;
    /// the HTTP endpoint URL
    fn endpoint(&self) -> String;
    /// return configured commitment level of the provider
    fn commitment_config(&self) -> CommitmentConfig;
}

/// Account provider that always fetches from RPC
#[derive(Clone)]
pub struct RpcAccountProvider {
    client: Arc<RpcClient>,
}

impl RpcAccountProvider {
    pub fn new(endpoint: &str) -> Self {
        Self::with_commitment(endpoint, CommitmentConfig::confirmed())
    }
    /// Create a new RPC account provider with provided commitment level
    pub fn with_commitment(endpoint: &str, commitment: CommitmentConfig) -> Self {
        Self {
            client: Arc::new(RpcClient::new_with_commitment(
                endpoint.to_string(),
                commitment,
            )),
        }
    }
    async fn get_account_impl(&self, account: Pubkey) -> SdkResult<Account> {
        let account_data: Account = self.client.get_account(&account).await?;
        Ok(account_data)
    }
}

impl AccountProvider for RpcAccountProvider {
    fn get_account(&self, account: Pubkey) -> BoxFuture<SdkResult<Account>> {
        self.get_account_impl(account).boxed()
    }
    fn endpoint(&self) -> String {
        self.client.url()
    }
    fn commitment_config(&self) -> CommitmentConfig {
        self.client.commitment()
    }
}

/// Account provider using websocket subscriptions to receive and cache account updates
#[derive(Clone)]
pub struct WsAccountProvider {
    url: String,
    rpc_client: Arc<RpcClient>,
    /// map from account pubkey to (account data, last modified ts)
    account_cache: Arc<RwLock<FnvHashMap<Pubkey, Receiver<(Account, Slot)>>>>,
}

struct AccountSubscription {
    account: Pubkey,
    url: String,
    rpc_client: Arc<RpcClient>,
    /// sink for account updates
    tx: Arc<watch::Sender<(Account, Slot)>>,
}

impl AccountSubscription {
    const RPC_CONFIG: RpcAccountInfoConfig = RpcAccountInfoConfig {
        encoding: Some(UiAccountEncoding::Base64Zstd),
        data_slice: None,
        commitment: Some(CommitmentConfig {
            commitment: CommitmentLevel::Confirmed,
        }),
        min_context_slot: None,
    };
    async fn stream_fn(self) {
        let ws_client =
            match PubsubClient::new(self.url.as_str().replace("http", "ws").as_str()).await {
                Ok(ws_client) => ws_client,
                Err(err) => {
                    warn!(target: "account", "connect client {:?} failed: {err:?}", self.account);
                    return;
                }
            };

        let result = ws_client
            .account_subscribe(&self.account, Some(Self::RPC_CONFIG))
            .await;

        if let Err(err) = result {
            warn!(target: "account", "subscribe account {:?} failed: {err:?}", self.account);
            return;
        }
        debug!(target: "account", "start account stream {:?}", self.account);
        let (mut account_stream, unsub_fn) = result.unwrap();

        let mut poll_interval = tokio::time::interval(Duration::from_secs(10));
        let _ = poll_interval.tick().await; // ignore, immediate first tick
        loop {
            select! {
                biased;
                response = account_stream.next() => {
                    if let Some(account_update) = response {
                        let slot = account_update.context.slot;
                        let account_data = account_update
                            .value
                            .decode::<Account>()
                            .expect("account");
                        self.tx.send_if_modified(|current| {
                            if slot > current.1 {
                                debug!(target: "account", "stream update writing to cache");
                                *current = (account_data, slot);
                                true
                            } else {
                                debug!(target: "account", "stream update old");
                               false
                            }
                        });
                    } else {
                        // websocket subscription/stream closed, try reconnect..
                        warn!(target: "account", "account stream closed: {:?}", self.account);
                        break;
                    }
                }
                _ = poll_interval.tick() => {
                    if let Ok(account_data) = self.rpc_client.get_account_with_config(&self.account, Default::default()).await {
                        self.tx.send_if_modified(|current| {
                            let slot = account_data.context.slot;
                            // only update with polled value if its newer
                            if slot > current.1 {
                                debug!(target: "account", "poll update, writing to cache");
                                *current = (account_data.value.unwrap(), slot);
                                true
                            } else {
                                debug!(target: "account", "poll update, too old");
                                false
                            }
                        });
                    } else {
                        // consecutive errors would indicate an issue, there's not much that can be done besides log/panic...
                    }
                }
            }
        }
        unsub_fn().await;
        warn!(target: "account", "stream ended: {:?}", self.account);
    }
}

impl WsAccountProvider {
    /// Create a new WsAccountProvider given an endpoint that serves both http(s) and ws(s)
    pub async fn new(url: &str) -> SdkResult<Self> {
        Self::new_with_commitment(url, CommitmentConfig::confirmed()).await
    }
    /// Create a new WsAccountProvider with provided commitment level
    pub async fn new_with_commitment(url: &str, commitment: CommitmentConfig) -> SdkResult<Self> {
        Ok(Self {
            url: url.to_string(),
            rpc_client: Arc::new(RpcClient::new_with_commitment(url.to_string(), commitment)),
            account_cache: Default::default(),
        })
    }
    /// Subscribe to account updates via web-socket and polling
    fn subscribe_account(&self, account: Pubkey, tx: watch::Sender<(Account, Slot)>) {
        let rpc_client = Arc::clone(&self.rpc_client);
        let tx = Arc::new(tx);
        let url = self.url.clone();
        spawn_retry_task(
            move || {
                let account_sub = AccountSubscription {
                    account,
                    url: url.clone(),
                    rpc_client: Arc::clone(&rpc_client),
                    tx: Arc::clone(&tx),
                };
                account_sub.stream_fn()
            },
            retry_policy::forever(5),
        );
    }
    /// Fetch an account and initiate subscription for future updates
    async fn get_account_impl(&self, account: Pubkey) -> SdkResult<Account> {
        {
            let cache = self.account_cache.read().await;
            if let Some(account_data_rx) = cache.get(&account) {
                let (account_data, _last_modified) = account_data_rx.borrow().clone();
                return Ok(account_data);
            }
        }

        // fetch initial account data, stream only updates on changes
        let account_data: Account = self.rpc_client.get_account(&account).await?;
        let (tx, rx) = watch::channel((account_data.clone(), 0));
        {
            let mut cache = self.account_cache.write().await;
            cache.insert(account, rx);
        }
        self.subscribe_account(account, tx);

        Ok(account_data)
    }
}

impl AccountProvider for WsAccountProvider {
    fn get_account(&self, account: Pubkey) -> BoxFuture<SdkResult<Account>> {
        self.get_account_impl(account).boxed()
    }
    fn endpoint(&self) -> String {
        self.rpc_client.url()
    }
    fn commitment_config(&self) -> CommitmentConfig {
        self.rpc_client.commitment()
    }
}

/// Drift Client API
///
/// It is cheaply clone-able and consumers are encouraged to do so
/// It is not recommended to create multiple instances with `::new()` as this will not re-use underlying resources such
/// as network connections or memory allocations
#[derive(Clone)]
#[must_use]
pub struct DriftClient<T: AccountProvider> {
    backend: &'static DriftClientBackend<T>,
    wallet: Wallet,
    pub active_sub_account_id: u16,
    pub sub_account_ids: Vec<u16>,
    pub users: Vec<DriftUser>,
}

impl<T: AccountProvider> DriftClient<T> {
    pub async fn new(context: Context, account_provider: T, wallet: Wallet) -> SdkResult<Self> {
        Self::new_with_opts(context, account_provider, wallet, ClientOpts::default()).await
    }

    pub async fn new_with_opts(
        context: Context,
        account_provider: T,
        wallet: Wallet,
        opts: ClientOpts,
    ) -> SdkResult<Self> {
        Ok(Self {
            backend: Box::leak(Box::new(
                DriftClientBackend::new(context, account_provider).await?,
            )),
            wallet,
            active_sub_account_id: opts.active_sub_account_id(),
            sub_account_ids: opts.sub_account_ids(),
            users: vec![],
        })
    }

    pub async fn add_user(&mut self, sub_account_id: u16) -> SdkResult<()> {
        let pubkey =
            Wallet::derive_user_account(self.wallet.authority(), sub_account_id, &drift::ID);
        let mut user = DriftUser::new(pubkey, self, sub_account_id).await?;
        user.subscribe().await?;
        self.users.push(user);
        Ok(())
    }

    pub fn get_user(&self, sub_account_id: u16) -> Option<&DriftUser> {
        self.users.iter().find(|u| u.sub_account == sub_account_id)
    }

    /// Subscribe to the Drift Client Backend
    /// This is a no-op if already subscribed
    pub async fn subscribe(&self) -> SdkResult<()> {
        self.backend.subscribe().await
    }

    /// Unsubscribe from the Drift Client Backend
    /// This is a no-op if not subscribed
    pub async fn unsubscribe(&self) -> SdkResult<()> {
        self.backend.unsubscribe().await
    }

    /// Return a handle to the inner RPC client
    pub fn inner(&self) -> &RpcClient {
        self.backend.client()
    }

    /// Return on-chain program metadata
    pub fn program_data(&self) -> &ProgramData {
        &self.backend.program_data
    }

    /// Get the active sub account id
    pub fn get_sub_account_id_for_ix(&self, sub_account_id: Option<u16>) -> u16 {
        sub_account_id.unwrap_or(self.active_sub_account_id)
    }

    /// Get an account's open order by id
    ///
    /// `account` the drift user PDA
    pub async fn get_order_by_id(
        &self,
        account: &Pubkey,
        order_id: u32,
    ) -> SdkResult<Option<Order>> {
        let user = self.backend.get_account::<User>(account).await?;

        Ok(user.orders.iter().find(|o| o.order_id == order_id).copied())
    }

    /// Get an account's open order by user assigned id
    ///
    /// `account` the drift user PDA
    pub async fn get_order_by_user_id(
        &self,
        account: &Pubkey,
        user_order_id: u8,
    ) -> SdkResult<Option<Order>> {
        let user = self.backend.get_account::<User>(account).await?;

        Ok(user
            .orders
            .iter()
            .find(|o| o.user_order_id == user_order_id)
            .copied())
    }

    /// Get all the account's open orders
    ///
    /// `account` the drift user PDA
    pub async fn all_orders(&self, account: &Pubkey) -> SdkResult<Vec<Order>> {
        let user = self.backend.get_account::<User>(account).await?;

        Ok(user
            .orders
            .iter()
            .filter(|o| o.status == OrderStatus::Open)
            .copied()
            .collect())
    }

    /// Get all the account's active positions
    ///
    /// `account` the drift user PDA
    pub async fn all_positions(
        &self,
        account: &Pubkey,
    ) -> SdkResult<(Vec<SpotPosition>, Vec<PerpPosition>)> {
        let user = self.backend.get_account::<User>(account).await?;

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
    /// `account` the drift user PDA
    ///
    /// Returns the position if it exists
    pub async fn perp_position(
        &self,
        account: &Pubkey,
        market_index: u16,
    ) -> SdkResult<Option<PerpPosition>> {
        let user = self.backend.get_account::<User>(account).await?;

        Ok(user
            .perp_positions
            .iter()
            .find(|p| p.market_index == market_index && !p.is_available())
            .copied())
    }

    /// Get a spot position by market
    ///
    /// `account` the drift user PDA
    ///
    /// Returns the position if it exists
    pub async fn spot_position(
        &self,
        account: &Pubkey,
        market_index: u16,
    ) -> SdkResult<Option<SpotPosition>> {
        let user = self.backend.get_account::<User>(account).await?;

        Ok(user
            .spot_positions
            .iter()
            .find(|p| p.market_index == market_index && !p.is_available())
            .copied())
    }

    /// Return the DriftClient's wallet
    pub fn wallet(&self) -> &Wallet {
        &self.wallet
    }

    /// Get the user account data
    ///
    /// `account` the drift user PDA
    ///
    /// Returns the deserialized account data (`User`)
    pub async fn get_user_account(&self, account: &Pubkey) -> SdkResult<User> {
        self.backend.get_account(account).await
    }

    /// Get a stats account
    ///
    /// Returns the deserialized account data (`UserStats`)
    pub async fn get_user_stats(&self, authority: &Pubkey) -> SdkResult<UserStats> {
        let user_stats_pubkey = Wallet::derive_stats_account(authority, &constants::PROGRAM_ID);
        self.backend.get_account(&user_stats_pubkey).await
    }

    /// Get the latest recent_block_hash
    pub async fn get_latest_blockhash(&self) -> SdkResult<Hash> {
        self.backend
            .client()
            .get_latest_blockhash()
            .await
            .map_err(SdkError::Rpc)
    }

    /// Sign and send a tx to the network
    ///
    /// Returns the signature on success
    pub async fn sign_and_send(&self, tx: VersionedMessage) -> SdkResult<Signature> {
        self.backend
            .sign_and_send(self.wallet(), tx)
            .await
            .map_err(|err| err.to_out_of_sol_error().unwrap_or(err))
    }

    /// Sign and send a tx to the network
    ///
    /// Returns the signature on success
    pub async fn sign_and_send_with_config(
        &self,
        tx: VersionedMessage,
        config: RpcSendTransactionConfig,
    ) -> SdkResult<Signature> {
        self.backend
            .sign_and_send_with_config(self.wallet(), tx, config)
            .await
            .map_err(|err| err.to_out_of_sol_error().unwrap_or(err))
    }

    /// Get live info of a spot market
    pub async fn get_spot_market_info(&self, market_index: u16) -> SdkResult<SpotMarket> {
        let market = derive_spot_market_account(market_index);
        self.backend.get_account(&market).await
    }

    /// Get live info of a perp market
    pub async fn get_perp_market_info(&self, market_index: u16) -> SdkResult<PerpMarket> {
        let market = derive_perp_market_account(market_index);
        self.backend.get_account(&market).await
    }

    /// Lookup a market by symbol
    ///
    /// This operation is not free so lookups should be reused/cached by the caller
    ///
    /// Returns None if symbol does not map to any known market
    pub fn market_lookup(&self, symbol: &str) -> Option<MarketId> {
        if symbol.contains('-') {
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
        let account_data = self
            .get_user(self.active_sub_account_id)
            .expect("user")
            .get_user_account();
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

    pub fn get_perp_market_account_and_slot(
        &self,
        market_index: u16,
    ) -> Option<DataAndSlot<PerpMarket>> {
        self.backend.get_perp_market_account_and_slot(market_index)
    }

    pub fn get_spot_market_account_and_slot(
        &self,
        market_index: u16,
    ) -> Option<DataAndSlot<SpotMarket>> {
        self.backend.get_spot_market_account_and_slot(market_index)
    }

    pub fn get_perp_market_account(&self, market_index: u16) -> Option<PerpMarket> {
        self.backend
            .get_perp_market_account_and_slot(market_index)
            .map(|x| x.data)
    }

    pub fn get_spot_market_account(&self, market_index: u16) -> Option<SpotMarket> {
        self.backend
            .get_spot_market_account_and_slot(market_index)
            .map(|x| x.data)
    }

    pub fn num_perp_markets(&self) -> usize {
        self.backend.num_perp_markets()
    }

    pub fn num_spot_markets(&self) -> usize {
        self.backend.num_spot_markets()
    }

    pub fn get_oracle_price_data_and_slot(&self, oracle_pubkey: &Pubkey) -> Option<Oracle> {
        self.backend.get_oracle_price_data_and_slot(oracle_pubkey)
    }

    pub fn get_oracle_price_data_and_slot_for_perp_market(
        &self,
        market_index: u16,
    ) -> Option<Oracle> {
        self.backend
            .get_oracle_price_data_and_slot_for_perp_market(market_index)
    }

    pub fn get_oracle_price_data_and_slot_for_spot_market(
        &self,
        market_index: u16,
    ) -> Option<Oracle> {
        self.backend
            .get_oracle_price_data_and_slot_for_spot_market(market_index)
    }
}

/// Provides the heavy-lifting and network facing features of the SDK
/// It is intended to be a singleton
pub struct DriftClientBackend<T: AccountProvider> {
    rpc_client: RpcClient,
    account_provider: T,
    program_data: ProgramData,
    perp_market_map: MarketMap<PerpMarket>,
    spot_market_map: MarketMap<SpotMarket>,
    oracle_map: Arc<OracleMap>,
    state_account: Arc<std::sync::RwLock<State>>,
    blockhash_subscriber: Arc<RwLock<BlockhashSubscriber>>,
}

impl<T: AccountProvider> DriftClientBackend<T> {
    /// Initialize a new `DriftClientBackend`
    async fn new(context: Context, account_provider: T) -> SdkResult<DriftClientBackend<T>> {
        let rpc_client = RpcClient::new_with_commitment(
            account_provider.endpoint(),
            account_provider.commitment_config(),
        );

        let perp_market_map = MarketMap::<PerpMarket>::new(
            account_provider.commitment_config(),
            account_provider.endpoint(),
            true,
        );
        let spot_market_map = MarketMap::<SpotMarket>::new(
            account_provider.commitment_config(),
            account_provider.endpoint(),
            true,
        );

        tokio::try_join!(perp_market_map.sync(), spot_market_map.sync())?;

        let perp_oracles = perp_market_map.oracles();
        let spot_oracles = spot_market_map.oracles();

        let oracle_map = OracleMap::new(
            account_provider.commitment_config(),
            account_provider.endpoint(),
            true,
            perp_oracles,
            spot_oracles,
        );

        let state = account_provider
            .get_account(*state_account())
            .await
            .expect("state account");

        let blockhash_subscriber = Arc::new(RwLock::new(BlockhashSubscriber::new(
            2,
            account_provider.endpoint(),
        )));

        let mut this = Self {
            rpc_client,
            account_provider,
            program_data: ProgramData::uninitialized(),
            perp_market_map,
            spot_market_map,
            oracle_map: Arc::new(oracle_map),
            state_account: Arc::new(std::sync::RwLock::new(
                State::try_deserialize(&mut state.data.as_ref()).expect("valid state"),
            )),
            blockhash_subscriber,
        };

        let lookup_table_address = market_lookup_table(context);

        let (markets, lookup_table_account): (
            SdkResult<(Vec<SpotMarket>, Vec<PerpMarket>)>,
            SdkResult<Account>,
        ) = tokio::join!(
            get_market_accounts(&this.rpc_client),
            this.get_account_raw(&lookup_table_address),
        );

        let (spot, perp) = markets?;
        let lookup_table = utils::deserialize_alt(lookup_table_address, &lookup_table_account?)?;
        this.program_data = ProgramData::new(spot, perp, lookup_table);

        Ok(this)
    }

    async fn subscribe(&self) -> SdkResult<()> {
        tokio::try_join!(
            self.perp_market_map.subscribe(),
            self.spot_market_map.subscribe(),
            self.oracle_map.subscribe(),
            self.state_subscribe(),
            BlockhashSubscriber::subscribe(self.blockhash_subscriber.clone()),
        )?;
        Ok(())
    }

    async fn unsubscribe(&self) -> SdkResult<()> {
        tokio::try_join!(
            self.perp_market_map.unsubscribe(),
            self.spot_market_map.unsubscribe(),
            self.oracle_map.unsubscribe(),
        )?;
        Ok(())
    }

    async fn state_subscribe(&self) -> SdkResult<()> {
        let pubkey = *state_account();

        let mut subscription = WebsocketAccountSubscriber::new(
            "state",
            get_ws_url(&self.rpc_client.url()).expect("valid url"),
            pubkey,
            self.rpc_client.commitment(),
            EventEmitter::new(),
        );

        let state = self.state_account.clone();

        subscription.event_emitter.subscribe("state", move |event| {
            if let Some(update) = event.as_any().downcast_ref::<AccountUpdate>() {
                let new_data = decode::<State>(update.data.data.clone()).expect("valid state data");
                let mut state_writer = state.write().unwrap();
                *state_writer = new_data;
            }
        });

        subscription.subscribe().await?;

        Ok(())
    }

    fn get_perp_market_account_and_slot(
        &self,
        market_index: u16,
    ) -> Option<DataAndSlot<PerpMarket>> {
        self.perp_market_map.get(&market_index)
    }

    fn get_spot_market_account_and_slot(
        &self,
        market_index: u16,
    ) -> Option<DataAndSlot<SpotMarket>> {
        self.spot_market_map.get(&market_index)
    }

    fn num_perp_markets(&self) -> usize {
        self.perp_market_map.size()
    }

    fn num_spot_markets(&self) -> usize {
        self.spot_market_map.size()
    }

    fn get_oracle_price_data_and_slot(&self, oracle_pubkey: &Pubkey) -> Option<Oracle> {
        self.oracle_map.get(oracle_pubkey)
    }

    fn get_oracle_price_data_and_slot_for_perp_market(&self, market_index: u16) -> Option<Oracle> {
        let market = self.get_perp_market_account_and_slot(market_index)?;

        let oracle = market.data.amm.oracle;
        let current_oracle = self
            .oracle_map
            .current_perp_oracle(market_index)
            .expect("oracle");

        if oracle != current_oracle {
            let source = market.data.amm.oracle_source;
            let clone = self.oracle_map.clone();
            tokio::task::spawn_local(async move {
                let _ = clone.add_oracle(oracle, source).await;
                clone.update_perp_oracle(market_index, oracle)
            });
        }

        self.get_oracle_price_data_and_slot(&current_oracle)
    }

    fn get_oracle_price_data_and_slot_for_spot_market(&self, market_index: u16) -> Option<Oracle> {
        let market = self.get_spot_market_account_and_slot(market_index)?;

        let oracle = market.data.oracle;
        let current_oracle = self
            .oracle_map
            .current_spot_oracle(market_index)
            .expect("oracle");

        if oracle != current_oracle {
            let source = market.data.oracle_source;
            let clone = self.oracle_map.clone();
            tokio::task::spawn_local(async move {
                let _ = clone.add_oracle(oracle, source).await;
                clone.update_spot_oracle(market_index, oracle);
            });
        }

        self.get_oracle_price_data_and_slot(&market.data.oracle)
    }

    /// Return a handle to the inner RPC client
    fn client(&self) -> &RpcClient {
        &self.rpc_client
    }

    /// Get recent tx priority fees
    ///
    /// - `window` # of slots to include in the fee calculation
    async fn get_recent_priority_fees(
        &self,
        writable_markets: &[MarketId],
        window: Option<usize>,
    ) -> SdkResult<Vec<u64>> {
        let addresses: Vec<Pubkey> = writable_markets
            .iter()
            .filter_map(|x| match x.kind {
                MarketType::Spot => self
                    .program_data
                    .spot_market_config_by_index(x.index)
                    .map(|x| x.pubkey),
                MarketType::Perp => self
                    .program_data
                    .perp_market_config_by_index(x.index)
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

    /// Get all drift program accounts by Anchor type
    async fn get_program_accounts<U: AccountDeserialize + Discriminator>(
        &self,
    ) -> SdkResult<Vec<U>> {
        let accounts = self
            .rpc_client
            .get_program_accounts_with_config(
                &constants::PROGRAM_ID,
                RpcProgramAccountsConfig {
                    filters: Some(vec![RpcFilterType::Memcmp(Memcmp::new_raw_bytes(
                        0,
                        U::DISCRIMINATOR.to_vec(),
                    ))]),
                    account_config: RpcAccountInfoConfig {
                        encoding: Some(UiAccountEncoding::Base64Zstd),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            )
            .await?;

        accounts
            .iter()
            .map(|(_, account_data)| {
                U::try_deserialize(&mut account_data.data.as_ref())
                    .map_err(|err| SdkError::Anchor(Box::new(err)))
            })
            .collect()
    }

    /// Fetch an `account` as an Anchor account type
    async fn get_account<U: AccountDeserialize>(&self, account: &Pubkey) -> SdkResult<U> {
        let account_data = self.account_provider.get_account(*account).await?;
        U::try_deserialize(&mut account_data.data.as_ref()).map_err(|_err| SdkError::InvalidAccount)
    }

    /// Fetch an `account`
    async fn get_account_raw(&self, account: &Pubkey) -> SdkResult<Account> {
        self.account_provider
            .get_account(*account)
            .await
            .map_err(Into::into)
    }

    /// Sign and send a tx to the network
    ///
    /// Returns the signature on success
    pub async fn sign_and_send(
        &self,
        wallet: &Wallet,
        tx: VersionedMessage,
    ) -> SdkResult<Signature> {
        let blockhash_reader = self.blockhash_subscriber.read().await;
        let recent_block_hash = blockhash_reader.get_valid_blockhash();
        drop(blockhash_reader);
        let tx = wallet.sign_tx(tx, recent_block_hash)?;
        self.rpc_client
            .send_transaction(&tx)
            .await
            .map_err(|err| err.into())
    }

    /// Sign and send a tx to the network with custom send config
    /// allows setting commitment level, retries, etc.
    ///
    /// Returns the signature on success
    pub async fn sign_and_send_with_config(
        &self,
        wallet: &Wallet,
        tx: VersionedMessage,
        config: RpcSendTransactionConfig,
    ) -> SdkResult<Signature> {
        let blockhash_reader = self.blockhash_subscriber.read().await;
        let recent_block_hash = blockhash_reader.get_valid_blockhash();
        drop(blockhash_reader);
        let tx = wallet.sign_tx(tx, recent_block_hash)?;
        self.rpc_client
            .send_transaction_with_config(&tx, config)
            .await
            .map_err(|err| err.into())
    }

    /// Fetch the live oracle price for `market`
    pub async fn oracle_price(&self, market: MarketId) -> SdkResult<i64> {
        let (oracle, oracle_source) = match market.kind {
            MarketType::Perp => {
                let market = self
                    .program_data
                    .perp_market_config_by_index(market.index)
                    .ok_or(SdkError::InvalidOracle)?;
                (market.amm.oracle, market.amm.oracle_source)
            }
            MarketType::Spot => {
                let market = self
                    .program_data
                    .spot_market_config_by_index(market.index)
                    .ok_or(SdkError::InvalidOracle)?;
                (market.oracle, market.oracle_source)
            }
        };

        let (current_slot, oracle_account) = tokio::join!(
            self.rpc_client.get_slot(),
            self.account_provider.get_account(oracle)
        );
        let price_data = get_oracle_price(
            &oracle_source,
            &(oracle, oracle_account?).into_account_info(),
            current_slot?,
        )
        .unwrap();
        Ok(price_data.price)
    }
}

/// Composable Tx builder for Drift program
///
/// Prefer `DriftClient::init_tx`
///
/// ```ignore
/// use drift_sdk::{types::Context, TransactionBuilder, Wallet};
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
}

impl<'a> TransactionBuilder<'a> {
    /// Initialize a new `TransactionBuilder` for default signer
    ///
    /// `program_data` program data from chain
    /// `sub_account` drift sub-account address
    /// `account_data` drift sub-account data
    /// `delegated` set true to build tx for delegated signing
    pub fn new<'b>(
        program_data: &'b ProgramData,
        sub_account: Pubkey,
        account_data: Cow<'b, User>,
        delegated: bool,
    ) -> Self
    where
        'b: 'a,
    {
        Self {
            authority: if delegated {
                account_data.delegate
            } else {
                account_data.authority
            },
            program_data,
            account_data,
            sub_account,
            ixs: Default::default(),
            lookup_tables: vec![program_data.lookup_table.clone()],
            legacy: false,
        }
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
    /// `microlamports_per_cu` the price per unit of compute in µ-lamports
    pub fn with_priority_fee(mut self, microlamports_per_cu: u64, cu_limit: Option<u32>) -> Self {
        let cu_limit_ix = ComputeBudgetInstruction::set_compute_unit_price(microlamports_per_cu);
        self.ixs.insert(0, cu_limit_ix);
        if let Some(cu_limit) = cu_limit {
            let cu_price_ix = ComputeBudgetInstruction::set_compute_unit_limit(cu_limit);
            self.ixs.insert(1, cu_price_ix);
        }

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
            drift::accounts::Deposit {
                state: *state_account(),
                user: self.sub_account,
                user_stats: Wallet::derive_stats_account(&self.authority, &constants::PROGRAM_ID),
                authority: self.authority,
                spot_market_vault: constants::derive_spot_market_vault(spot_market_index),
                user_token_account,
                token_program: constants::TOKEN_PROGRAM_ID,
            },
            &[self.account_data.as_ref()],
            &[],
            &[MarketId::spot(spot_market_index)],
        );

        let ix = Instruction {
            program_id: constants::PROGRAM_ID,
            accounts,
            data: InstructionData::data(&drift::instruction::Deposit {
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
            drift::accounts::Withdraw {
                state: *state_account(),
                user: self.sub_account,
                user_stats: Wallet::derive_stats_account(&self.authority, &constants::PROGRAM_ID),
                authority: self.authority,
                spot_market_vault: constants::derive_spot_market_vault(spot_market_index),
                user_token_account,
                drift_signer: constants::derive_drift_signer(),
                token_program: constants::TOKEN_PROGRAM_ID,
            },
            &[self.account_data.as_ref()],
            &[],
            &[MarketId::spot(spot_market_index)],
        );

        let ix = Instruction {
            program_id: constants::PROGRAM_ID,
            accounts,
            data: InstructionData::data(&drift::instruction::Withdraw {
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
        let readable_accounts: Vec<MarketId> = orders
            .iter()
            .map(|o| (o.market_index, o.market_type).into())
            .collect();

        let accounts = build_accounts(
            self.program_data,
            drift::accounts::PlaceOrder {
                state: *state_account(),
                authority: self.authority,
                user: self.sub_account,
            },
            &[self.account_data.as_ref()],
            readable_accounts.as_ref(),
            &[],
        );

        let ix = Instruction {
            program_id: constants::PROGRAM_ID,
            accounts,
            data: InstructionData::data(&drift::instruction::PlaceOrders { params: orders }),
        };

        self.ixs.push(ix);

        self
    }

    /// Cancel all orders for account
    pub fn cancel_all_orders(mut self) -> Self {
        let accounts = build_accounts(
            self.program_data,
            drift::accounts::CancelOrder {
                state: *state_account(),
                authority: self.authority,
                user: self.sub_account,
            },
            &[self.account_data.as_ref()],
            &[],
            &[],
        );

        let ix = Instruction {
            program_id: constants::PROGRAM_ID,
            accounts,
            data: InstructionData::data(&drift::instruction::CancelOrders {
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
    /// `market` - tuple of market ID and type (spot or perp)
    ///
    /// `direction` - long or short
    pub fn cancel_orders(
        mut self,
        market: (u16, MarketType),
        direction: Option<PositionDirection>,
    ) -> Self {
        let (idx, kind) = market;
        let accounts = build_accounts(
            self.program_data,
            drift::accounts::CancelOrder {
                state: *state_account(),
                authority: self.authority,
                user: self.sub_account,
            },
            &[self.account_data.as_ref()],
            &[(idx, kind).into()],
            &[],
        );

        let ix = Instruction {
            program_id: constants::PROGRAM_ID,
            accounts,
            data: InstructionData::data(&drift::instruction::CancelOrders {
                market_index: Some(idx),
                market_type: Some(kind),
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
            drift::accounts::CancelOrder {
                state: *state_account(),
                authority: self.authority,
                user: self.sub_account,
            },
            &[self.account_data.as_ref()],
            &[],
            &[],
        );

        let ix = Instruction {
            program_id: constants::PROGRAM_ID,
            accounts,
            data: InstructionData::data(&drift::instruction::CancelOrdersByIds { order_ids }),
        };
        self.ixs.push(ix);

        self
    }

    /// Cancel orders by given _user_ ids
    pub fn cancel_orders_by_user_id(mut self, user_order_ids: Vec<u8>) -> Self {
        let accounts = build_accounts(
            self.program_data,
            drift::accounts::CancelOrder {
                state: *state_account(),
                authority: self.authority,
                user: self.sub_account,
            },
            &[self.account_data.as_ref()],
            &[],
            &[],
        );

        for user_order_id in user_order_ids {
            let ix = Instruction {
                program_id: constants::PROGRAM_ID,
                accounts: accounts.clone(),
                data: InstructionData::data(&drift::instruction::CancelOrderByUserId {
                    user_order_id,
                }),
            };
            self.ixs.push(ix);
        }

        self
    }

    /// Modify existing order(s) by order id
    pub fn modify_orders(mut self, orders: &[(u32, ModifyOrderParams)]) -> Self {
        for (order_id, params) in orders {
            let accounts = build_accounts(
                self.program_data,
                drift::accounts::PlaceOrder {
                    state: *state_account(),
                    authority: self.authority,
                    user: self.sub_account,
                },
                &[self.account_data.as_ref()],
                &[],
                &[],
            );

            let ix = Instruction {
                program_id: constants::PROGRAM_ID,
                accounts,
                data: InstructionData::data(&drift::instruction::ModifyOrder {
                    order_id: Some(*order_id),
                    modify_order_params: params.clone(),
                }),
            };
            self.ixs.push(ix);
        }

        self
    }

    /// Modify existing order(s) by user order id
    pub fn modify_orders_by_user_id(mut self, orders: &[(u8, ModifyOrderParams)]) -> Self {
        for (user_order_id, params) in orders {
            let accounts = build_accounts(
                self.program_data,
                drift::accounts::PlaceOrder {
                    state: *state_account(),
                    authority: self.authority,
                    user: self.sub_account,
                },
                &[self.account_data.as_ref()],
                &[],
                &[],
            );

            let ix = Instruction {
                program_id: constants::PROGRAM_ID,
                accounts,
                data: InstructionData::data(&drift::instruction::ModifyOrderByUserId {
                    user_order_id: *user_order_id,
                    modify_order_params: params.clone(),
                }),
            };
            self.ixs.push(ix);
        }

        self
    }

    /// Add a place and make instruction
    ///
    /// `order` the order to place
    /// `taker_info` taker account address and data
    /// `taker_order_id` the id of the taker's order to match with
    /// `referrer` pukey of the taker's referrer account, if any
    /// `fulfilment_type` type of fill for spot orders, ignored for perp orders
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
            drift::accounts::PlaceAndMake {
                state: *state_account(),
                authority: self.authority,
                user: self.sub_account,
                user_stats: Wallet::derive_stats_account(&self.authority, &constants::PROGRAM_ID),
                taker: *taker,
                taker_stats: Wallet::derive_stats_account(taker, &constants::PROGRAM_ID),
            },
            &[self.account_data.as_ref(), &taker_account],
            &[],
            if is_perp {
                &perp_writable
            } else {
                &spot_writable
            },
        );

        if let Some(referrer) = referrer {
            accounts.push(AccountMeta::new(
                Wallet::derive_stats_account(&referrer, &constants::PROGRAM_ID),
                false,
            ));
            accounts.push(AccountMeta::new(referrer, false));
        }

        let ix = if order.market_type == MarketType::Perp {
            Instruction {
                program_id: constants::PROGRAM_ID,
                accounts,
                data: InstructionData::data(&drift::instruction::PlaceAndMakePerpOrder {
                    params: order,
                    taker_order_id,
                }),
            }
        } else {
            Instruction {
                program_id: constants::PROGRAM_ID,
                accounts,
                data: InstructionData::data(&drift::instruction::PlaceAndMakeSpotOrder {
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
    /// `order` the order to place
    ///
    /// `maker_info` pubkey of the maker/counterparty to take against and account data
    ///
    /// `referrer` pubkey of the maker's referrer account, if any
    ///
    /// `fulfilment_type` type of fill for spot orders, ignored for perp orders
    pub fn place_and_take(
        mut self,
        order: OrderParams,
        maker_info: Option<(Pubkey, User)>,
        referrer: Option<Pubkey>,
        fulfillment_type: Option<SpotFulfillmentType>,
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
            drift::accounts::PlaceAndTake {
                state: *state_account(),
                authority: self.authority,
                user: self.sub_account,
                user_stats: Wallet::derive_stats_account(&self.authority, &constants::PROGRAM_ID),
            },
            user_accounts.as_slice(),
            &[],
            if is_perp {
                &perp_writable
            } else {
                &spot_writable
            },
        );

        if referrer.is_some_and(|r| !maker_info.is_some_and(|(m, _)| m == r)) {
            let referrer = referrer.unwrap();
            accounts.push(AccountMeta::new(
                Wallet::derive_stats_account(&referrer, &constants::PROGRAM_ID),
                false,
            ));
            accounts.push(AccountMeta::new(referrer, false));
        }

        let ix = if is_perp {
            Instruction {
                program_id: constants::PROGRAM_ID,
                accounts,
                data: InstructionData::data(&drift::instruction::PlaceAndTakePerpOrder {
                    params: order,
                    maker_order_id: None,
                }),
            }
        } else {
            Instruction {
                program_id: constants::PROGRAM_ID,
                accounts,
                data: InstructionData::data(&drift::instruction::PlaceAndTakeSpotOrder {
                    params: order,
                    maker_order_id: None,
                    fulfillment_type,
                }),
            }
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
}

/// Builds a set of required accounts from a user's open positions and additional given accounts
///
/// `base_accounts` base anchor accounts
///
/// `user` Drift user account data
///
/// `markets_readable` IDs of markets to include as readable
///
/// `markets_writable` IDs of markets to include as writable (takes priority over readable)
///
/// # Panics
///  if the user has positions in an unknown market (i.e unsupported by the SDK)
pub fn build_accounts(
    program_data: &ProgramData,
    base_accounts: impl ToAccountMetas,
    users: &[&User],
    markets_readable: &[MarketId],
    markets_writable: &[MarketId],
) -> Vec<AccountMeta> {
    // the order of accounts returned must be instruction, oracles, spot, perps see (https://github.com/drift-labs/protocol-v2/blob/master/programs/drift/src/instructions/optional_accounts.rs#L28)
    let mut seen = [0_u64; 2]; // [spot, perp]
    let mut accounts = Vec::<RemainingAccount>::default();

    // add accounts to the ordered list
    let mut include_market = |market_index: u16, market_type: MarketType, writable: bool| {
        let index_bit = 1_u64 << market_index as u8;
        // always safe since market type is 0 or 1
        let seen_by_type = unsafe { seen.get_unchecked_mut(market_type as usize % 2) };
        if *seen_by_type & index_bit > 0 {
            return;
        }
        *seen_by_type |= index_bit;

        let (account, oracle) = match market_type {
            MarketType::Spot => {
                let SpotMarket { pubkey, oracle, .. } = program_data
                    .spot_market_config_by_index(market_index)
                    .expect("exists");
                (
                    RemainingAccount::Spot {
                        pubkey: *pubkey,
                        writable,
                    },
                    oracle,
                )
            }
            MarketType::Perp => {
                let PerpMarket { pubkey, amm, .. } = program_data
                    .perp_market_config_by_index(market_index)
                    .expect("exists");
                (
                    RemainingAccount::Perp {
                        pubkey: *pubkey,
                        writable,
                    },
                    &amm.oracle,
                )
            }
        };
        if let Err(idx) = accounts.binary_search(&account) {
            accounts.insert(idx, account);
        }
        let oracle = RemainingAccount::Oracle { pubkey: *oracle };
        if let Err(idx) = accounts.binary_search(&oracle) {
            accounts.insert(idx, oracle);
        }
    };

    for MarketId { index, kind } in markets_writable {
        include_market(*index, *kind, true);
    }

    for MarketId { index, kind } in markets_readable {
        include_market(*index, *kind, false);
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
    include_market(MarketId::QUOTE_SPOT.index, MarketType::Spot, false);

    let mut account_metas = base_accounts.to_account_metas(None);
    account_metas.extend(accounts.into_iter().map(Into::into));
    account_metas
}

/// Fetch all market accounts from drift program (does not require `getProgramAccounts` RPC which is often unavailable)
pub async fn get_market_accounts(
    client: &RpcClient,
) -> SdkResult<(Vec<SpotMarket>, Vec<PerpMarket>)> {
    let state_data = client
        .get_account_data(state_account())
        .await
        .expect("state account fetch");
    let state = State::try_deserialize(&mut state_data.as_slice()).expect("state deserializes");
    let spot_market_pdas: Vec<Pubkey> = (0..state.number_of_spot_markets)
        .map(derive_spot_market_account)
        .collect();
    let perp_market_pdas: Vec<Pubkey> = (0..state.number_of_markets)
        .map(derive_perp_market_account)
        .collect();

    let (spot_markets, perp_markets) = tokio::join!(
        client.get_multiple_accounts(spot_market_pdas.as_slice()),
        client.get_multiple_accounts(perp_market_pdas.as_slice())
    );

    let spot_markets = spot_markets?
        .into_iter()
        .map(|x| {
            let account = x.unwrap();
            SpotMarket::try_deserialize(&mut account.data.as_slice()).unwrap()
        })
        .collect();

    let perp_markets = perp_markets?
        .into_iter()
        .map(|x| {
            let account = x.unwrap();
            PerpMarket::try_deserialize(&mut account.data.as_slice()).unwrap()
        })
        .collect();

    Ok((spot_markets, perp_markets))
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
            signer: Arc::new(Keypair::from_bytes(&[0_u8; 64]).expect("empty signer")),
            authority,
            stats: Wallet::derive_stats_account(&authority, &constants::PROGRAM_ID),
        }
    }
    /// Init wallet from base58 encoded seed, uses default sub-account
    ///
    /// # panics
    /// if the key is invalid
    pub fn from_seed_bs58(seed: &str) -> Self {
        let authority: Keypair = Keypair::from_base58_string(seed);
        Self::new(authority)
    }
    /// Init wallet from seed bytes, uses default sub-account
    pub fn from_seed(seed: &[u8]) -> SdkResult<Self> {
        let authority: Keypair = keypair_from_seed(seed).map_err(|_| SdkError::InvalidSeed)?;
        Ok(Self::new(authority))
    }
    /// Init wallet with keypair
    ///
    /// `authority` keypair for tx signing
    pub fn new(authority: Keypair) -> Self {
        Self {
            stats: Wallet::derive_stats_account(&authority.pubkey(), &constants::PROGRAM_ID),
            authority: authority.pubkey(),
            signer: Arc::new(authority),
        }
    }
    /// Convert the wallet into a delegated one by providing the `authority` public key
    pub fn to_delegated(&mut self, authority: Pubkey) {
        self.stats = Wallet::derive_stats_account(&authority, &constants::PROGRAM_ID);
        self.authority = authority;
    }
    /// Calculate the address of a drift user account/sub-account
    pub fn derive_user_account(
        authority: &Pubkey,
        sub_account_id: u16,
        program: &Pubkey,
    ) -> Pubkey {
        let (account_drift_pda, _seed) = Pubkey::find_program_address(
            &[
                &b"user"[..],
                authority.as_ref(),
                &sub_account_id.to_le_bytes(),
            ],
            program,
        );
        account_drift_pda
    }

    /// Calculate the address of a drift stats account
    pub fn derive_stats_account(account: &Pubkey, program: &Pubkey) -> Pubkey {
        let (account_drift_pda, _seed) =
            Pubkey::find_program_address(&[&b"user_stats"[..], account.as_ref()], program);
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
        Self::derive_user_account(self.authority(), sub_account_id, &constants::PROGRAM_ID)
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

    use drift::state::perp_market::PerpMarket;
    use serde_json::json;
    use solana_account_decoder::{UiAccount, UiAccountData};
    use solana_client::{
        rpc_client::Mocks,
        rpc_request::RpcRequest,
        rpc_response::{Response, RpcResponseContext},
    };

    use super::*;

    // static account data for test/mock
    const ACCOUNT_DATA: &str = include_str!("../res/9Jtc.hex");
    const DEVNET_ENDPOINT: &str = "https://api.devnet.solana.com";

    /// Init a new `DriftClient` with provided mocked RPC responses
    async fn setup(
        rpc_mocks: Mocks,
        account_provider_mocks: Mocks,
        keypair: Keypair,
    ) -> DriftClient<RpcAccountProvider> {
        let perp_market_map = MarketMap::<PerpMarket>::new(
            CommitmentConfig::processed(),
            DEVNET_ENDPOINT.to_string(),
            false,
        );
        let spot_market_map = MarketMap::<SpotMarket>::new(
            CommitmentConfig::processed(),
            DEVNET_ENDPOINT.to_string(),
            false,
        );

        let backend = DriftClientBackend {
            rpc_client: RpcClient::new_mock_with_mocks(DEVNET_ENDPOINT.to_string(), rpc_mocks),
            account_provider: RpcAccountProvider {
                client: Arc::new(RpcClient::new_mock_with_mocks(
                    DEVNET_ENDPOINT.to_string(),
                    account_provider_mocks,
                )),
            },
            program_data: ProgramData::uninitialized(),
            perp_market_map,
            spot_market_map,
            oracle_map: Arc::new(OracleMap::new(
                CommitmentConfig::processed(),
                DEVNET_ENDPOINT.to_string(),
                true,
                vec![],
                vec![],
            )),
            state_account: Arc::new(std::sync::RwLock::new(State::default())),
            blockhash_subscriber: Arc::new(RwLock::new(BlockhashSubscriber::new(
                2,
                DEVNET_ENDPOINT.to_string(),
            ))),
        };

        DriftClient {
            backend: Box::leak(Box::new(backend)),
            wallet: Wallet::new(keypair),
            active_sub_account_id: 0,
            sub_account_ids: vec![0],
            users: vec![],
        }
    }

    #[tokio::test]
    async fn test_backend_send_sync() {
        let account_mocks = Mocks::default();
        let client = setup(Default::default(), account_mocks, Keypair::new()).await;

        tokio::task::spawn(async move {
            let _ = client.clone();
        });
    }

    #[tokio::test]
    #[cfg(rpc_tests)]
    async fn test_marketmap_subscribe() {
        let endpoint = "rpc";

        let client = DriftClient::new(
            Context::MainNet,
            RpcAccountProvider::new(endpoint),
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
    async fn get_market_accounts() {
        let client = DriftClient::new(
            Context::DevNet,
            RpcAccountProvider::new(DEVNET_ENDPOINT),
            Keypair::new().into(),
        )
        .await
        .unwrap();
        let accounts: Vec<SpotMarket> = client
            .backend
            .get_program_accounts()
            .await
            .expect("found accounts");
        assert!(accounts.len() > 1);

        let accounts: Vec<PerpMarket> = client
            .backend
            .get_program_accounts()
            .await
            .expect("found accounts");
        assert!(accounts.len() > 1);
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
            })
        });
        account_mocks.insert(RpcRequest::GetAccountInfo, account_response.clone());

        let client = setup(Default::default(), account_mocks, Keypair::new()).await;

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
            })
        });
        account_mocks.insert(RpcRequest::GetAccountInfo, account_response.clone());
        let client = setup(Default::default(), account_mocks, Keypair::new()).await;

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
