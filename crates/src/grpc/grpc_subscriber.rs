use std::{collections::HashMap, time::Duration};

use crate::constants::PROGRAM_ID as DRIFT_PROGRAM_ID;

use ahash::HashSet;
use futures_util::{sink::SinkExt, stream::StreamExt};
use log::{error, info, warn};
use solana_rpc_client_api::filter::Memcmp;
use solana_sdk::{
    clock::{Epoch, Slot},
    commitment_config::CommitmentLevel,
    pubkey::Pubkey,
};
use yellowstone_grpc_client::{
    ClientTlsConfig, GeyserGrpcBuilderError, GeyserGrpcClient, GeyserGrpcClientError, Interceptor,
};
use yellowstone_grpc_proto::{
    geyser::{CommitmentLevel as GeyserCommitmentLevel, SubscribeUpdateAccountInfo},
    prelude::{
        subscribe_request_filter_accounts_filter::Filter as AccountsFilterOneof,
        subscribe_request_filter_accounts_filter_memcmp::Data as AccountsFilterMemcmpOneof,
        subscribe_update::UpdateOneof, SubscribeRequest, SubscribeRequestFilterAccounts,
        SubscribeRequestFilterAccountsFilter, SubscribeRequestFilterAccountsFilterMemcmp,
        SubscribeRequestFilterSlots, SubscribeRequestPing,
    },
    tonic::transport::Certificate,
};

type SlotsFilterMap = HashMap<String, SubscribeRequestFilterSlots>;
type AccountFilterMap = HashMap<String, SubscribeRequestFilterAccounts>;
type HookFn = dyn Fn(&AccountUpdate) + Send + Sync + 'static;
type Hooks = Vec<(AccountFilter, Box<HookFn>)>;

/// Account update from gRPC
#[derive(PartialEq, Eq, Clone)]
pub struct AccountUpdate<'a> {
    /// the account's pubkey
    pub pubkey: Pubkey,
    /// lamports in the account
    pub lamports: u64,
    /// data held in the account
    pub data: &'a [u8],
    /// the program that owns the account. If executable, the program that loads the account.
    pub owner: Pubkey,
    /// the account's data contains a loaded program (and is now read-only)
    pub executable: bool,
    /// the epoch at which the account will next owe rent
    pub rent_epoch: Epoch,
    /// Slot the update was retrieved
    pub slot: Slot,
}

/// Provides filter criteria for accounts over gRPC
///
/// There are two filter modes:
///
/// * `full` - requires all filters to trigger a match
/// * `partial` - any one filter will trigger a match
///
/// ```example(no_run)
/// // match on discriminator AND memcmp
///  let full = AccountFilter::full()
///     .with_discriminator(User::DISCRIMINATOR)
///     .with_memcmp(..);
///
/// // match on discriminator OR accounts
/// let partial = AccountFilter::partial()
///     .with_discriminator(User::DISCRIMINATOR)
///     .with_accounts([acc1,acc2])
/// ```
#[derive(Clone, Default, Debug)]
pub struct AccountFilter {
    /// optionally filter updates by discriminator
    discriminator: Option<&'static [u8]>,
    /// optionally filter updates by Solana Memcmp matches
    memcmp: Option<Memcmp>,
    /// optionally filter updates by pubkey
    accounts: Option<HashSet<Pubkey>>,
    /// true = full match mode, false = partial
    is_full: bool,
}

impl AccountFilter {
    /// Create a filter that matches ALL accounts!
    pub fn firehose() -> Self {
        AccountFilter::full()
    }
    /// Create a filter that matches iff all parameters are satisfied
    pub fn full() -> Self {
        AccountFilter {
            is_full: true,
            ..Default::default()
        }
    }
    /// Create a filter that matches when any criteria is satisfied
    pub fn partial() -> Self {
        AccountFilter {
            is_full: false,
            ..Default::default()
        }
    }
    /// add filter for given `pubkeys`
    pub fn with_accounts(mut self, pubkeys: impl Iterator<Item = Pubkey>) -> Self {
        self.accounts = Some(ahash::HashSet::from_iter(pubkeys));
        self
    }
    /// add filter for given anchor account `discriminator`
    pub fn with_discriminator(mut self, discriminator: &'static [u8]) -> Self {
        self.discriminator = Some(discriminator);
        self
    }
    /// add filter for given memcmp filter
    pub fn with_memcmp(mut self, memcmp: Memcmp) -> Self {
        self.memcmp = Some(memcmp);
        self
    }
    /// Returns true if pubkey/account matches the filter
    pub fn matches(&self, pubkey: &Pubkey, account: &SubscribeUpdateAccountInfo) -> bool {
        if !self.is_full {
            // partial matches
            self.discriminator.is_some_and(|x| x == &account.data[..8])
                || self.accounts.as_ref().is_some_and(|x| x.contains(pubkey))
                || self
                    .memcmp
                    .as_ref()
                    .is_some_and(|x| x.bytes_match(&account.data))
        } else {
            // full matches
            (match self.discriminator {
                Some(x) => x == &account.data[..8],
                None => true,
            }) && (match self.accounts.as_ref() {
                Some(x) => x.contains(pubkey),
                None => true,
            }) && (match self.memcmp.as_ref() {
                Some(x) => x.bytes_match(&account.data),
                None => true,
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct GrpcOpts {
    /// Apply a timeout to connecting to the uri.
    connect_timeout_ms: Option<u64>,
    /// Sets the tower service default internal buffer size, default is 1024
    buffer_size: Option<usize>,
    /// Sets whether to use an adaptive flow control. Uses hyper’s default otherwise.
    http2_adaptive_window: Option<bool>,
    /// Set http2 KEEP_ALIVE_TIMEOUT. Uses hyper’s default otherwise.
    http2_keep_alive_interval_ms: Option<u64>,
    /// Sets the max connection-level flow control for HTTP2, default is 65,535
    initial_connection_window_size: Option<u32>,
    ///Sets the SETTINGS_INITIAL_WINDOW_SIZE option for HTTP2 stream-level flow control, default is 65,535
    initial_stream_window_size: Option<u32>,
    ///Set http2 KEEP_ALIVE_TIMEOUT. Uses hyper’s default otherwise.
    keep_alive_timeout_ms: Option<u64>,
    /// Set http2 KEEP_ALIVE_WHILE_IDLE. Uses hyper’s default otherwise.
    keep_alive_while_idle: Option<bool>,
    /// Set whether TCP keepalive messages are enabled on accepted connections.
    tcp_keepalive_ms: Option<u64>,
    /// Set the value of TCP_NODELAY option for accepted connections. Enabled by default.
    tcp_nodelay: Option<bool>,
    /// Apply a timeout to each request.
    timeout_ms: Option<u64>,
    /// Max message size before decoding, full blocks can be super large, default is 1GiB
    max_decoding_message_size: usize,
}

impl Default for GrpcOpts {
    fn default() -> Self {
        Self {
            connect_timeout_ms: None,
            buffer_size: None,
            http2_adaptive_window: None,
            http2_keep_alive_interval_ms: None,
            initial_connection_window_size: None,
            initial_stream_window_size: None,
            keep_alive_timeout_ms: None,
            keep_alive_while_idle: None,
            tcp_keepalive_ms: None,
            tcp_nodelay: None,
            timeout_ms: None,
            max_decoding_message_size: 1024 * 1024 * 1024,
        }
    }
}

/// Options for gRPC subscription
#[derive(Debug, Default, Clone)]
pub struct SubscribeOpts {
    /// Filter by presence of field txn_signature
    accounts_nonempty_txn_signature: Option<bool>,
    /// Filter by Account Pubkey
    accounts_account: Vec<String>,
    /// Filter by Offset and Data, format: `offset,data in base58`
    accounts_memcmp: Vec<Memcmp>,
    /// Filter by Data size
    accounts_datasize: Option<u64>,
    /// Re-send message from slot
    from_slot: Option<u64>,
    /// Send ping in subscribe request
    ping: Option<i32>,
}

#[derive(Debug, thiserror::Error)]
/// drift gRPC error
pub enum GrpcError {
    #[error("grpc connect err: {0}")]
    Geyser(GeyserGrpcBuilderError),
    #[error("grpc request err: {0}")]
    Client(GeyserGrpcClientError),
}

/// specialized Drift gRPC client
pub struct DriftGrpcClient {
    endpoint: String,
    x_token: String,
    grpc_opts: Option<GrpcOpts>,
    on_account_hooks: Hooks,
    on_slot: Box<dyn Fn(Slot) + Send>,
}

impl DriftGrpcClient {
    /// Create a new `DriftGrpcClient`
    ///
    /// It can be started by calling `subscribe`
    pub fn new(endpoint: String, x_token: String) -> Self {
        Self {
            endpoint,
            x_token,
            on_account_hooks: Default::default(),
            grpc_opts: None,
            on_slot: Box::new(move |_slot| {}),
        }
    }

    /// Set gRPC network options
    pub fn grpc_opts(&mut self, grpc_opts: GrpcOpts) {
        let _ = self.grpc_opts.insert(grpc_opts);
    }

    /// Add a callback on slot updates
    ///
    /// `on_slot` must prioritize fast handling or risk blocking the gRPC thread
    pub fn on_slot<F: Fn(Slot) + Send + Sync + 'static>(&mut self, on_slot: F) {
        self.on_slot = Box::new(on_slot);
    }

    /// Add a callback on account updates matching `filter`
    ///
    /// This may be called many times to define multiple callbacks
    ///
    /// * `filter` - filter accounts by criteria
    /// * `on_account` - fn to receive callback on filter match
    ///
    /// DEV: `on_account` must prioritize fast handling or risk blocking the gRPC thread
    pub fn on_account<T: Fn(&AccountUpdate) + Send + Sync + 'static>(
        &mut self,
        filter: AccountFilter,
        on_account: T,
    ) {
        self.on_account_hooks.push((filter, Box::new(on_account)));
    }

    /// Start subscription for geyser updates
    pub async fn subscribe(
        self,
        commitment: CommitmentLevel,
        subscribe_opts: SubscribeOpts,
    ) -> Result<(), GrpcError> {
        let mut grpc_client = grpc_connect(
            self.endpoint.as_str(),
            self.x_token.as_str(),
            self.grpc_opts.clone().unwrap_or_default(),
        )
        .await
        .map_err(|err| {
            error!(target: "grpc", "connect failed: {err:?}");
            GrpcError::Geyser(err)
        })?;

        grpc_client.ping(1).await.map_err(GrpcError::Client)?;
        info!("gRPC connected 🔌");
        let request = subscribe_opts.to_subscribe_request(commitment);
        info!(target: "grpc", "gRPC subscribing: {request:?}");

        tokio::spawn({
            async move {
                Self::geyser_subscribe(grpc_client, request, self.on_account_hooks, self.on_slot)
                    .await
            }
        });
        info!("gRPC subscribed ⚡️");

        Ok(())
    }

    /// Run the gRPC subscription task
    ///
    /// It receives all configured updates and routes them to registered callbacks
    async fn geyser_subscribe(
        mut client: GeyserGrpcClient<impl Interceptor>,
        request: SubscribeRequest,
        on_account: Hooks,
        on_slot: impl Fn(Slot),
    ) {
        let max_retries = 3;
        let mut retry_count = 0;
        let mut latest_slot = 0;
        loop {
            if retry_count >= max_retries {
                log::warn!(target: "grpc", "max retry attempts reached. disconnecting...");
                break;
            }
            let (mut subscribe_tx, mut stream) =
                match client.subscribe_with_request(Some(request.clone())).await {
                    Ok(res) => {
                        retry_count = 0;
                        res
                    }
                    Err(err) => {
                        log::warn!(target: "grpc", "failed subscription: {err:?}");
                        retry_count += 1;
                        continue;
                    }
                };

            while let Some(message) = stream.next().await {
                match message {
                    Ok(msg) => {
                        match msg.update_oneof {
                            Some(UpdateOneof::Account(account_update)) => {
                                let account = match account_update.account {
                                    Some(ref account) => account,
                                    None => {
                                        warn!(target: "grpc", "empty account update: {account_update:?}");
                                        continue;
                                    }
                                };
                                let pubkey = Pubkey::new_from_array(
                                    account.pubkey.as_slice().try_into().unwrap(),
                                );
                                log::trace!(target: "grpc", "account update: {pubkey}");
                                let update = AccountUpdate {
                                    owner: DRIFT_PROGRAM_ID, // assuming not subscribed to any other accounts..
                                    pubkey,
                                    slot: latest_slot,
                                    lamports: account.lamports,
                                    executable: account.executable,
                                    rent_epoch: account.rent_epoch,
                                    data: &account.data,
                                };

                                for (filter, hook) in &on_account {
                                    if filter.matches(&pubkey, account) {
                                        hook(&update);
                                    }
                                }
                            }
                            Some(UpdateOneof::Slot(msg)) => {
                                log::debug!(target: "grpc", "slot: {}", msg.slot);
                                if msg.slot > latest_slot {
                                    latest_slot = msg.slot;
                                    on_slot(latest_slot);
                                }
                            }
                            Some(UpdateOneof::Ping(_)) => {
                                // This is necessary to keep load balancers that expect client pings alive. If your load balancer doesn't
                                // require periodic client pings then this is unnecessary
                                log::debug!(target: "grpc", "ping");
                                // TODO: set timeout
                                if let Err(err) = subscribe_tx
                                    .send(SubscribeRequest {
                                        ping: Some(SubscribeRequestPing { id: 1 }),
                                        ..Default::default()
                                    })
                                    .await
                                {
                                    log::warn!(target: "grpc", "ping failed: {err:?}");
                                }
                            }
                            Some(UpdateOneof::Pong(_)) => {
                                log::debug!(target: "grpc", "pong");
                            }
                            Some(other_update) => {
                                warn!(target: "grpc", "unhandled update: {other_update:?}");
                            }
                            None => {
                                error!(target: "grpc", "update not found in the message");
                                break;
                            }
                        }
                    }
                    Err(error) => {
                        error!(target: "grpc", "stream error: {error:?}");
                        break;
                    }
                }
            }
        }

        warn!(target: "grpc", "gRPC stream closed");
    }
}

impl SubscribeOpts {
    fn to_subscribe_request(&self, commitment: CommitmentLevel) -> SubscribeRequest {
        let mut accounts = AccountFilterMap::default();
        let mut filters = vec![];
        for filter in self.accounts_memcmp.iter() {
            filters.push(SubscribeRequestFilterAccountsFilter {
                filter: Some(AccountsFilterOneof::Memcmp(
                    SubscribeRequestFilterAccountsFilterMemcmp {
                        offset: filter.offset() as u64,
                        data: filter
                            .bytes()
                            .map(|b| AccountsFilterMemcmpOneof::Bytes(b.to_vec())),
                    },
                )),
            });
        }
        if let Some(datasize) = self.accounts_datasize {
            filters.push(SubscribeRequestFilterAccountsFilter {
                filter: Some(AccountsFilterOneof::Datasize(datasize)),
            });
        }

        accounts.insert(
            "client".to_owned(),
            SubscribeRequestFilterAccounts {
                nonempty_txn_signature: self.accounts_nonempty_txn_signature,
                account: self.accounts_account.clone(),
                owner: vec![DRIFT_PROGRAM_ID.to_string()],
                filters,
            },
        );

        let mut slots = SlotsFilterMap::default();
        slots.insert(
            "client".to_owned(),
            SubscribeRequestFilterSlots {
                filter_by_commitment: Some(true),
                interslot_updates: Some(false),
            },
        );

        let ping = self.ping.map(|id| SubscribeRequestPing { id });

        SubscribeRequest {
            slots,
            accounts,
            commitment: Some(match commitment {
                CommitmentLevel::Confirmed => GeyserCommitmentLevel::Confirmed,
                CommitmentLevel::Processed => GeyserCommitmentLevel::Processed,
                CommitmentLevel::Finalized => GeyserCommitmentLevel::Finalized,
            } as i32),
            ping,
            from_slot: self.from_slot,
            ..Default::default()
        }
    }
}

/// Connect to gRPC endpoint
///
/// Returns a new `GeyserGrpcClient`
async fn grpc_connect(
    endpoint: &str,
    x_token: &str,
    opts: GrpcOpts,
) -> Result<GeyserGrpcClient<impl Interceptor>, GeyserGrpcBuilderError> {
    info!(target: "grpc", "gRPC connecting: {endpoint}...");
    let mut tls_config = ClientTlsConfig::new().with_native_roots();
    if let Ok(path) = &std::env::var("GRPC_CA_CERT") {
        let bytes = tokio::fs::read(path)
            .await
            .expect("GRPC_CA_CERT path exists");
        tls_config = tls_config.ca_certificate(Certificate::from_pem(bytes));
    }
    let mut builder = GeyserGrpcClient::build_from_shared(endpoint.to_string())?
        .x_token(Some(x_token))?
        .tls_config(tls_config)?
        .max_decoding_message_size(opts.max_decoding_message_size);

    if let Some(duration) = opts.connect_timeout_ms {
        builder = builder.connect_timeout(Duration::from_millis(duration));
    }
    if let Some(sz) = opts.buffer_size {
        builder = builder.buffer_size(sz);
    }
    if let Some(enabled) = opts.http2_adaptive_window {
        builder = builder.http2_adaptive_window(enabled);
    }
    if let Some(duration) = opts.http2_keep_alive_interval_ms {
        builder = builder.http2_keep_alive_interval(Duration::from_millis(duration));
    }
    if let Some(sz) = opts.initial_connection_window_size {
        builder = builder.initial_connection_window_size(sz);
    }
    if let Some(sz) = opts.initial_stream_window_size {
        builder = builder.initial_stream_window_size(sz);
    }
    if let Some(duration) = opts.keep_alive_timeout_ms {
        builder = builder.keep_alive_timeout(Duration::from_millis(duration));
    }
    if let Some(enabled) = opts.keep_alive_while_idle {
        builder = builder.keep_alive_while_idle(enabled);
    }
    if let Some(duration) = opts.tcp_keepalive_ms {
        builder = builder.tcp_keepalive(Some(Duration::from_millis(duration)));
    }
    if let Some(enabled) = opts.tcp_nodelay {
        builder = builder.tcp_nodelay(enabled);
    }
    if let Some(duration) = opts.timeout_ms {
        builder = builder.timeout(Duration::from_millis(duration));
    }

    builder.connect().await
}
