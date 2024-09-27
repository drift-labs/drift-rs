use std::{
    collections::VecDeque,
    str::FromStr,
    sync::{Arc, OnceLock},
    task::{Context, Poll},
    time::Duration,
};

use anchor_lang::{AnchorDeserialize, Discriminator};
use base64::Engine;
use fnv::FnvHashSet;
use futures_util::{future::BoxFuture, stream::FuturesOrdered, FutureExt, Stream, StreamExt};
use log::{debug, info, warn};
use regex::Regex;
pub use solana_client::nonblocking::{pubsub_client::PubsubClient, rpc_client::RpcClient};
use solana_client::{
    rpc_client::GetConfirmedSignaturesForAddress2Config, rpc_config::RpcTransactionLogsConfig,
    rpc_response::RpcLogsResponse,
};
pub use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::{pubkey::Pubkey, signature::Signature, transaction::VersionedTransaction};
use solana_transaction_status::{
    option_serializer::OptionSerializer, EncodedTransactionWithStatusMeta, UiTransactionEncoding,
};
use tokio::{
    sync::{
        mpsc::{channel, Receiver, Sender},
        RwLock,
    },
    task::JoinHandle,
};

use crate::{
    async_utils::{retry_policy::TaskRetryPolicy, spawn_retry_task},
    constants,
    drift_idl::{
        events::{FundingPaymentRecord, OrderActionRecord, OrderRecord},
        types::{MarketType, Order, OrderAction, OrderActionExplanation, PositionDirection},
    },
    types::SdkResult,
};

const LOG_TARGET: &str = "events";
const EMPTY_SIGNATURE: &str = "1111111111111111111111111111111111111111111111111111111111111111";

impl EventRpcProvider for RpcClient {
    fn get_tx(
        &self,
        signature: Signature,
    ) -> BoxFuture<SdkResult<EncodedTransactionWithStatusMeta>> {
        async move {
            let result = self
                .get_transaction_with_config(
                    &signature,
                    solana_client::rpc_config::RpcTransactionConfig {
                        encoding: Some(UiTransactionEncoding::Base64),
                        max_supported_transaction_version: Some(0),
                        ..Default::default()
                    },
                )
                .await?;

            Ok(result.transaction)
        }
        .boxed()
    }
    fn get_tx_signatures(
        &self,
        account: Pubkey,
        after: Option<Signature>,
        limit: Option<usize>,
    ) -> BoxFuture<SdkResult<Vec<String>>> {
        async move {
            let results = self
                .get_signatures_for_address_with_config(
                    &account,
                    GetConfirmedSignaturesForAddress2Config {
                        until: after,
                        limit,
                        ..Default::default()
                    },
                )
                .await?;

            Ok(results.iter().map(|r| r.signature.clone()).collect())
        }
        .boxed()
    }
}

/// RPC functions required for drift event subscriptions
pub trait EventRpcProvider: Send + Sync + 'static {
    /// Fetch tx signatures of account
    /// `after` only return txs more recent than this signature, if given
    /// `limit` return at most this many signatures, if given
    fn get_tx_signatures(
        &self,
        account: Pubkey,
        after: Option<Signature>,
        limit: Option<usize>,
    ) -> BoxFuture<SdkResult<Vec<String>>>;
    /// Fetch tx with `signature`
    fn get_tx(
        &self,
        signature: Signature,
    ) -> BoxFuture<SdkResult<EncodedTransactionWithStatusMeta>>;
}

/// Provides sub-account event streaming
pub struct EventSubscriber;

impl EventSubscriber {
    /// Subscribe to drift events of `sub_account`, backed by Ws APIs
    ///
    /// The underlying stream will reconnect according to the given `retry_policy`
    pub async fn subscribe(
        endpoint: &str,
        sub_account: Pubkey,
        retry_policy: impl TaskRetryPolicy,
    ) -> SdkResult<DriftEventStream> {
        log_stream(endpoint, sub_account, retry_policy).await
    }
    /// Subscribe to drift events of `sub_account`, backed by RPC polling APIs
    pub fn subscribe_polled(provider: impl EventRpcProvider, account: Pubkey) -> DriftEventStream {
        polled_stream(provider, account)
    }
}

struct LogEventStream {
    cache: Arc<RwLock<TxSignatureCache>>,
    endpoint: Arc<String>,
    sub_account: Pubkey,
    event_tx: Sender<DriftEvent>,
    commitment: CommitmentConfig,
}

impl LogEventStream {
    /// Returns a future for running the configured log event stream
    async fn stream_fn(self) {
        let sub_account = self.sub_account;
        info!(target: LOG_TARGET, "log stream connecting: {sub_account:?}");
        let provider_init = PubsubClient::new(self.endpoint.as_str()).await;

        // the provider's internal websocket connection can close, if so need to reconnect
        if let Err(ref err) = provider_init {
            warn!(target: LOG_TARGET, "log subscription failed {err:?}, retrying: {sub_account:?}");
            return;
        }

        let provider = provider_init.unwrap();
        let subscribe_result = provider
            .logs_subscribe(
                solana_client::rpc_config::RpcTransactionLogsFilter::Mentions(vec![self
                    .sub_account
                    .to_string()]),
                RpcTransactionLogsConfig {
                    commitment: Some(self.commitment),
                },
            )
            .await;

        // the provider's internal websocket connection can close, if so need to reconnect
        if let Err(ref err) = subscribe_result {
            warn!(target: LOG_TARGET, "log subscription failed {err:?}, retrying: {sub_account:?}");
            return;
        }

        let (mut log_stream, unsub_fn) = subscribe_result.unwrap();
        debug!(target: LOG_TARGET, "start log subscription: {sub_account:?}");

        while let Some(response) = log_stream.next().await {
            self.process_log(response.value).await;
        }

        warn!(target: LOG_TARGET, "log stream ended: {sub_account:?}");
        unsub_fn().await;
    }

    /// Process a log response from RPC, emitting any relevant events
    async fn process_log(&self, response: RpcLogsResponse) {
        let signature = response.signature;
        if response.err.is_some() {
            debug!(target: LOG_TARGET, "skipping failed tx: {signature:?}");
            return;
        }
        if signature == EMPTY_SIGNATURE {
            debug!(target: LOG_TARGET, "skipping empty signature");
            return;
        }
        {
            let mut cache = self.cache.write().await;
            if cache.contains(&signature) {
                debug!(target: LOG_TARGET, "skipping cached tx: {signature:?}");
                return;
            }
            cache.insert(signature.clone());
        }

        debug!(target: LOG_TARGET, "log extracting events, tx: {signature:?}");
        for (tx_idx, log) in response.logs.iter().enumerate() {
            // a drift sub-account should not interact with any other program by definition
            if let Some(event) = try_parse_log(log.as_str(), &signature, tx_idx) {
                // unrelated events from same tx should not be emitted e.g. a filler tx which produces other fill events
                if event.pertains_to(self.sub_account) {
                    if self.event_tx.send(event).await.is_err() {
                        warn!("event receiver closed");
                        return;
                    }
                }
            }
        }
    }
}

/// Creates a poll-ed stream using JSON-RPC interfaces
fn polled_stream(provider: impl EventRpcProvider, sub_account: Pubkey) -> DriftEventStream {
    let (event_tx, event_rx) = channel(256);
    let cache = Arc::new(RwLock::new(TxSignatureCache::new(128)));
    let join_handle = tokio::spawn(
        PolledEventStream {
            cache: Arc::clone(&cache),
            provider,
            sub_account,
            event_tx,
        }
        .stream_fn(),
    );

    DriftEventStream {
        rx: event_rx,
        task: join_handle,
    }
}

/// Creates a Ws-backed event stream using `logsSubscribe` interface
async fn log_stream(
    endpoint: &str,
    sub_account: Pubkey,
    retry_policy: impl TaskRetryPolicy,
) -> SdkResult<DriftEventStream> {
    debug!(target: LOG_TARGET, "stream events for {sub_account:?}");
    let (event_tx, event_rx) = channel(256);

    let cache = Arc::new(RwLock::new(TxSignatureCache::new(256)));
    let endpoint = Arc::new(endpoint.to_string());

    // spawn the event subscription task
    let join_handle = spawn_retry_task(
        move || {
            let log_stream = LogEventStream {
                endpoint: Arc::clone(&endpoint),
                cache: Arc::clone(&cache),
                sub_account,
                event_tx: event_tx.clone(),
                commitment: CommitmentConfig::confirmed(),
            };
            log_stream.stream_fn()
        },
        retry_policy,
    );

    Ok(DriftEventStream {
        rx: event_rx,
        task: join_handle,
    })
}

pub struct PolledEventStream<T: EventRpcProvider> {
    cache: Arc<RwLock<TxSignatureCache>>,
    event_tx: Sender<DriftEvent>,
    provider: T,
    sub_account: Pubkey,
}

impl<T: EventRpcProvider> PolledEventStream<T> {
    async fn stream_fn(self) {
        debug!(target: LOG_TARGET, "poll events for {:?}", self.sub_account);
        // poll for events in any tx after this tx
        // initially fetch the most recent tx from account
        debug!(target: LOG_TARGET, "fetch initial txs");
        let res = self
            .provider
            .get_tx_signatures(self.sub_account, None, Some(1))
            .await;
        debug!(target: LOG_TARGET, "fetched initial txs");

        let mut last_seen_tx = res.expect("fetched tx").first().cloned();
        let provider_ref = &self.provider;
        'outer: loop {
            // don't needlessly spam the RPC or hog the executor
            tokio::time::sleep(Duration::from_millis(400)).await;

            debug!(target: LOG_TARGET, "poll txs for events");
            let signatures = provider_ref
                .get_tx_signatures(
                    self.sub_account,
                    last_seen_tx
                        .clone()
                        .map(|s| Signature::from_str(&s).unwrap()),
                    None,
                )
                .await;

            if let Err(err) = signatures {
                warn!(target: LOG_TARGET, "poll tx signatures: {err:?}");
                continue;
            }

            let signatures = signatures.unwrap();
            // txs from RPC are ordered newest to oldest
            // process in reverse order, so subscribers receive events in chronological order
            let mut futs = {
                FuturesOrdered::from_iter(
                    signatures
                        .into_iter()
                        .map(|s| async move {
                            (
                                s.clone(),
                                provider_ref
                                    .get_tx(
                                        Signature::from_str(s.as_str()).expect("valid signature"),
                                    )
                                    .await,
                            )
                        })
                        .rev(),
                )
            };
            if futs.is_empty() {
                continue;
            }

            while let Some((signature, response)) = futs.next().await {
                debug!(target: LOG_TARGET, "poll extracting events, tx: {signature:?}");
                if let Err(err) = response {
                    warn!(target: LOG_TARGET, "poll processing tx: {err:?}");
                    // retry querying the batch
                    continue 'outer;
                }

                last_seen_tx = Some(signature.clone());
                {
                    let mut cache = self.cache.write().await;
                    if cache.contains(&signature) {
                        debug!(target: LOG_TARGET, "poll skipping cached tx: {signature:?}");
                        continue;
                    }
                    cache.insert(signature.clone());
                }

                let EncodedTransactionWithStatusMeta {
                    meta, transaction, ..
                } = response.unwrap();
                if meta.is_none() {
                    continue;
                }
                let meta = meta.unwrap();

                if let Some(VersionedTransaction { message, .. }) = transaction.decode() {
                    // only txs interacting with drift program
                    if !message
                        .static_account_keys()
                        .iter()
                        .any(|k| k == &constants::PROGRAM_ID)
                    {
                        continue;
                    }
                }
                // ignore failed txs
                if meta.err.is_some() {
                    continue;
                }

                if let OptionSerializer::Some(logs) = meta.log_messages {
                    for (tx_idx, log) in logs.iter().enumerate() {
                        if let Some(event) = try_parse_log(log.as_str(), signature.as_str(), tx_idx)
                        {
                            if event.pertains_to(self.sub_account) {
                                self.event_tx.try_send(event).expect("sent");
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Provides a stream API of drift sub-account events
pub struct DriftEventStream {
    /// handle to end the stream task
    task: JoinHandle<()>,
    /// channel of events from stream task
    rx: Receiver<DriftEvent>,
}

impl DriftEventStream {
    /// End the event stream
    pub fn unsubscribe(&self) {
        self.task.abort();
    }
}

impl Drop for DriftEventStream {
    fn drop(&mut self) {
        self.unsubscribe()
    }
}

impl Stream for DriftEventStream {
    type Item = DriftEvent;
    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        self.as_mut().rx.poll_recv(cx)
    }
}

const PROGRAM_LOG: &str = "Program log: ";
const PROGRAM_DATA: &str = "Program data: ";

/// Try deserialize a drift event type from raw log string
/// https://github.com/coral-xyz/anchor/blob/9d947cb26b693e85e1fd26072bb046ff8f95bdcf/client/src/lib.rs#L552
pub fn try_parse_log(raw: &str, signature: &str, tx_idx: usize) -> Option<DriftEvent> {
    // Log emitted from the current program.
    if let Some(log) = raw
        .strip_prefix(PROGRAM_LOG)
        .or_else(|| raw.strip_prefix(PROGRAM_DATA))
    {
        if let Ok(borsh_bytes) = base64::engine::general_purpose::STANDARD.decode(log) {
            let (disc, mut data) = borsh_bytes.split_at(8);
            let disc: [u8; 8] = disc.try_into().unwrap();

            return DriftEvent::from_discriminant(disc, &mut data, signature, tx_idx);
        }

        // experimental
        let order_cancel_missing_re = ORDER_CANCEL_MISSING_RE
            .get_or_init(|| Regex::new(r"could not find( user){0,1} order id (\d+)").unwrap());
        if let Some(captures) = order_cancel_missing_re.captures(log) {
            let order_id = captures
                .get(2)
                .unwrap()
                .as_str()
                .parse::<u32>()
                .expect("<u32");
            let event = if captures.get(1).is_some() {
                // cancel by user order Id
                DriftEvent::OrderCancelMissing {
                    user_order_id: order_id as u8,
                    order_id: 0,
                    signature: signature.to_string(),
                }
            } else {
                // cancel by order id
                DriftEvent::OrderCancelMissing {
                    user_order_id: 0,
                    order_id,
                    signature: signature.to_string(),
                }
            };

            return Some(event);
        }
    }

    None
}

static ORDER_CANCEL_MISSING_RE: OnceLock<Regex> = OnceLock::new();

/// Enum of all drift program events
#[derive(Debug, PartialEq)]
pub enum DriftEvent {
    OrderFill {
        maker: Option<Pubkey>,
        maker_fee: i64,
        maker_order_id: u32,
        maker_side: Option<PositionDirection>,
        taker: Option<Pubkey>,
        taker_fee: u64,
        taker_order_id: u32,
        taker_side: Option<PositionDirection>,
        base_asset_amount_filled: u64,
        quote_asset_amount_filled: u64,
        market_index: u16,
        market_type: MarketType,
        oracle_price: i64,
        signature: String,
        tx_idx: usize,
        ts: u64,
    },
    OrderCancel {
        taker: Option<Pubkey>,
        maker: Option<Pubkey>,
        taker_order_id: u32,
        maker_order_id: u32,
        signature: String,
        tx_idx: usize,
        ts: u64,
    },
    /// An order cancel for a missing order Id / user order id
    OrderCancelMissing {
        user_order_id: u8,
        order_id: u32,
        signature: String,
    },
    OrderCreate {
        order: Order,
        user: Pubkey,
        ts: u64,
        signature: String,
        tx_idx: usize,
    },
    // sub-case of cancel?
    OrderExpire {
        order_id: u32,
        user: Option<Pubkey>,
        fee: u64,
        ts: u64,
        signature: String,
        tx_idx: usize,
    },
    FundingPayment {
        amount: i64,
        market_index: u16,
        user: Pubkey,
        ts: u64,
        signature: String,
        tx_idx: usize,
    },
}

impl DriftEvent {
    /// Return true if the event is connected to sub-account
    fn pertains_to(&self, sub_account: Pubkey) -> bool {
        let subject = &Some(sub_account);
        match self {
            Self::OrderCancel { maker, taker, .. } | Self::OrderFill { maker, taker, .. } => {
                maker == subject || taker == subject
            }
            Self::OrderCreate { user, .. } => *user == sub_account,
            Self::OrderExpire { user, .. } => user == subject,
            Self::OrderCancelMissing { .. } => true,
            Self::FundingPayment { user, .. } => *user == sub_account,
        }
    }
    /// Deserialize drift event by discriminant
    fn from_discriminant(
        disc: [u8; 8],
        data: &mut &[u8],
        signature: &str,
        tx_idx: usize,
    ) -> Option<Self> {
        match disc {
            // deser should only fail on a breaking protocol changes
            OrderActionRecord::DISCRIMINATOR => Self::from_oar(
                OrderActionRecord::deserialize(data).expect("deserializes"),
                signature,
                tx_idx,
            ),
            OrderRecord::DISCRIMINATOR => Self::from_order_record(
                OrderRecord::deserialize(data).expect("deserializes"),
                signature,
                tx_idx,
            ),
            FundingPaymentRecord::DISCRIMINATOR => Some(Self::from_funding_payment_record(
                FundingPaymentRecord::deserialize(data).expect("deserializes"),
                signature,
                tx_idx,
            )),
            _ => {
                debug!(target: LOG_TARGET, "unhandled event: {disc:?}");
                None
            }
        }
    }
    fn from_funding_payment_record(
        value: FundingPaymentRecord,
        signature: &str,
        tx_idx: usize,
    ) -> Self {
        Self::FundingPayment {
            amount: value.funding_payment,
            market_index: value.market_index,
            ts: value.ts.unsigned_abs(),
            user: value.user,
            signature: signature.to_string(),
            tx_idx,
        }
    }
    fn from_order_record(value: OrderRecord, signature: &str, tx_idx: usize) -> Option<Self> {
        Some(DriftEvent::OrderCreate {
            order: value.order,
            user: value.user,
            ts: value.ts.unsigned_abs(),
            signature: signature.to_string(),
            tx_idx,
        })
    }
    fn from_oar(value: OrderActionRecord, signature: &str, tx_idx: usize) -> Option<Self> {
        match value.action {
            OrderAction::Cancel => {
                if let OrderActionExplanation::OrderExpired = value.action_explanation {
                    // TODO: would be nice to report the `user_order_id` too...
                    Some(DriftEvent::OrderExpire {
                        fee: value.filler_reward.unwrap_or_default(),
                        order_id: value
                            .maker_order_id
                            .or(value.taker_order_id)
                            .expect("order id set"),
                        ts: value.ts.unsigned_abs(),
                        signature: signature.to_string(),
                        tx_idx,
                        user: value.maker.or(value.taker),
                    })
                } else {
                    Some(DriftEvent::OrderCancel {
                        maker: value.maker,
                        taker: value.taker,
                        maker_order_id: value.maker_order_id.unwrap_or_default(),
                        taker_order_id: value.taker_order_id.unwrap_or_default(),
                        ts: value.ts.unsigned_abs(),
                        signature: signature.to_string(),
                        tx_idx,
                    })
                }
            }
            OrderAction::Fill => Some(DriftEvent::OrderFill {
                maker: value.maker,
                maker_fee: value.maker_fee.unwrap_or_default(),
                maker_order_id: value.maker_order_id.unwrap_or_default(),
                maker_side: value.maker_order_direction,
                taker: value.taker,
                taker_fee: value.taker_fee.unwrap_or_default(),
                taker_order_id: value.taker_order_id.unwrap_or_default(),
                taker_side: value.taker_order_direction,
                base_asset_amount_filled: value.base_asset_amount_filled.unwrap_or_default(),
                quote_asset_amount_filled: value.quote_asset_amount_filled.unwrap_or_default(),
                oracle_price: value.oracle_price,
                market_index: value.market_index,
                market_type: value.market_type,
                ts: value.ts.unsigned_abs(),
                signature: signature.to_string(),
                tx_idx,
            }),
            // Place - parsed from `OrderRecord` event, ignored here due to lack of useful info
            // Expire - never emitted
            // Trigger - unimplemented
            OrderAction::Place | OrderAction::Expire | OrderAction::Trigger => None,
        }
    }
}

/// fixed capacity cache of tx signatures
struct TxSignatureCache {
    capacity: usize,
    entries: FnvHashSet<String>,
    age: VecDeque<String>,
}

impl TxSignatureCache {
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            entries: FnvHashSet::<String>::with_capacity_and_hasher(capacity, Default::default()),
            age: VecDeque::with_capacity(capacity),
        }
    }
    fn contains(&self, x: &str) -> bool {
        self.entries.contains(x)
    }
    fn insert(&mut self, x: String) {
        self.entries.insert(x.clone());
        self.age.push_back(x);

        if self.age.len() >= self.capacity {
            if let Some(ref oldest) = self.age.pop_front() {
                self.entries.remove(oldest);
            }
        }
    }
    #[cfg(test)]
    fn reset(&mut self) {
        self.entries.clear()
    }
}

#[cfg(test)]
mod test {
    use anchor_lang::prelude::*;
    use base64::Engine;
    use fnv::FnvHashMap;
    use futures_util::future::ready;
    use solana_sdk::{
        hash::Hash,
        instruction::{AccountMeta, Instruction},
        message::{v0, VersionedMessage},
        pubkey::Pubkey,
    };
    use solana_transaction_status::{TransactionStatusMeta, VersionedTransactionWithStatusMeta};
    use tokio::sync::Mutex;

    use super::*;
    use crate::SdkError;

    #[cfg(feature = "rpc_tests")]
    #[tokio::test]
    async fn event_streaming_logs() {
        use crate::async_utils::retry_policy;
        let mut event_stream = EventSubscriber::subscribe(
            "wss://api.devnet.solana.com",
            Pubkey::from_str("9JtczxrJjPM4J1xooxr2rFXmRivarb4BwjNiBgXDwe2p").unwrap(),
            retry_policy::never(),
        )
        .await
        .unwrap()
        .take(5);

        while let Some(event) = event_stream.next().await {
            dbg!(event);
        }
    }

    #[tokio::test]
    async fn log_stream_handles_jit_proxy_events() {
        let cache = TxSignatureCache::new(16);
        let (event_tx, mut event_rx) = channel(16);

        let mut log_stream = LogEventStream {
            cache: Arc::new(cache.into()),
            endpoint: Arc::new("wss://api.devnet.solana.com".into()),
            sub_account: "GgZkrSFgTAXZn1rNtZ533wpZi6nxx8whJC9bxRESB22c"
                .try_into()
                .unwrap(),
            event_tx,
            commitment: CommitmentConfig::confirmed(),
        };

        let logs: Vec<String> = [
            "Program ComputeBudget111111111111111111111111111111 invoke [1]",
            "Program ComputeBudget111111111111111111111111111111 success",
            "Program J1TnP8zvVxbtF5KFp5xRmWuvG9McnhzmBd9XGfCyuxFP invoke [1]",
            "Program log: Instruction: ArbPerp",
            "Program dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH invoke [2]",
            "Program log: Instruction: PlaceAndTakePerpOrder",
            "Program log: Invalid Spot 0 Oracle: Stale (oracle_delay=23)",
            "Program log: 4DRDR8LtbQFOKvplAAAAAAAAGAABAAAAAAAAAAAAAAFGJn8TpIimFlKv8ZWRhmuU81x+ojkf3K4d+++MbslDfAGZcTYAAQEBAM5q/TIAAAABAAAAAAAAAAABAAAAAAAAAAAAAAAAAACTWxEAAAAAAA==",
            "Program log: aBNAOFkVAlpOKvplAAAAAEYmfxOkiKYWUq/xlZGGa5TzXH6iOR/crh3774xuyUN8qZQ2DwAAAABMTREAAAAAAADOav0yAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAJlxNgAYAAEBAQAAAQAAAQAAAAAA",
            "Program log: 4DRDR8LtbQFOKvplAAAAAAIIGAABAUYmfxOkiKYWUq/xlZGGa5TzXH6iOR/crh3774xuyUN8AQAAAAAAAAAAAceaAwAAAAAAAQDOav0yAAAAAQQgzQ4AAAAAAQIjAQAAAAAAAQA+////////AAAAAUYmfxOkiKYWUq/xlZGGa5TzXH6iOR/crh3774xuyUN8AZlxNgABAQEAzmr9MgAAAAEAzmr9MgAAAAEEIM0OAAAAAAHpAf4sI0TDV0Ec0LWHs9mO40bjfKEm3A+yye5HFCQQQQEzPgAAAQABANraQssAAAABANraQssAAAABLJgAOwAAAACTWxEAAAAAAA==",
            "Program dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH consumed 373815 of 1334075 compute units",
            "Program dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH success",
            "Program dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH invoke [2]",
            "Program log: Instruction: PlaceAndTakePerpOrder",
            "Program log: Invalid Spot 0 Oracle: Stale (oracle_delay=23)",
            "Program log: 4DRDR8LtbQFOKvplAAAAAAAAGAABAAAAAAAAAAAAAAFGJn8TpIimFlKv8ZWRhmuU81x+ojkf3K4d+++MbslDfAGacTYAAQABAM5q/TIAAAABAAAAAAAAAAABAAAAAAAAAAAAAAAAAACTWxEAAAAAAA==",
            "Program log: aBNAOFkVAlpOKvplAAAAAEYmfxOkiKYWUq/xlZGGa5TzXH6iOR/crh3774xuyUN8qZQ2DwAAAACAPBEAAAAAAADOav0yAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAJpxNgAYAAEBAQABAAAAAQAAAAAA",
            "Program log: 4DRDR8LtbQFOKvplAAAAAAIQGAABAUYmfxOkiKYWUq/xlZGGa5TzXH6iOR/crh3774xuyUN8AQAAAAAAAAAAAciaAwAAAAAAAQDgBS0LAAAAAQBYOwMAAAAAAYs/AAAAAAAAAAAB+Ejx//////8AAUYmfxOkiKYWUq/xlZGGa5TzXH6iOR/crh3774xuyUN8AZpxNgABAAEAzmr9MgAAAAEA4AUtCwAAAAEAWDsDAAAAAAAAAAAAAJNbEQAAAAAA",
            "Program log: 4DRDR8LtbQFOKvplAAAAAAIIGAABAUYmfxOkiKYWUq/xlZGGa5TzXH6iOR/crh3774xuyUN8AQAAAAAAAAAAAcmaAwAAAAAAAQDuZNAnAAAAAYBpgwsAAAAAAV3iAAAAAAAAARhp////////AAAAAUYmfxOkiKYWUq/xlZGGa5TzXH6iOR/crh3774xuyUN8AZpxNgABAAEAzmr9MgAAAAEAzmr9MgAAAAGAwb4OAAAAAAFmQRGN8PRJqt5D5pVvCspbc3f0ZBdTB1Kcw0YfuzxCOAH2/poHAQEBAIjmn+sAAAABAFrDjp4AAAABgPDZLQAAAACTWxEAAAAAAA==",
            "Program dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH consumed 269624 of 934786 compute units",
            "Program dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH success",
            "Program log: pnl 792986",
            "Program J1TnP8zvVxbtF5KFp5xRmWuvG9McnhzmBd9XGfCyuxFP consumed 738458 of 1399850 compute units",
            "Program J1TnP8zvVxbtF5KFp5xRmWuvG9McnhzmBd9XGfCyuxFP success",
            ].into_iter().map(Into::into).collect();

        log_stream.process_log(RpcLogsResponse {
            signature: "2jLk34wWwgecuws9iD9Ug63JdL8kYBePdtcakzG34zEx9KYVYD6HuokxMZYpFw799cJZBcaCMZ47WAxkGJjM7zNC".into(),
            err: None,
            logs: logs.clone(),
        }).await;

        // case 1: jit taker
        assert_eq!(
            event_rx.try_recv().expect("one event"),
            DriftEvent::OrderFill {
                maker: Some(
                    "GgZkrSFgTAXZn1rNtZ533wpZi6nxx8whJC9bxRESB22c".try_into().unwrap(),
                ),
                maker_fee: -49664,
                maker_order_id: 15923,
                maker_side: Some(
                    PositionDirection::Long,
                ),
                taker: Some(
                    "5iqawn52cdBmsjC4hDegyFnX1iNRTNDV5mRsGzgqbuyD".try_into().unwrap(),
                ),
                taker_fee: 74498,
                taker_order_id: 3568025,
                taker_side: Some(
                    PositionDirection::Short,
                ),
                base_asset_amount_filled: 219000000000,
                quote_asset_amount_filled: 248324100,
                market_index: 24,
                market_type: MarketType::Perp,
                oracle_price: 1137555,
                signature: "2jLk34wWwgecuws9iD9Ug63JdL8kYBePdtcakzG34zEx9KYVYD6HuokxMZYpFw799cJZBcaCMZ47WAxkGJjM7zNC".into(),
                tx_idx: 9,
                ts: 1710893646,
            }
        );
        assert!(event_rx.try_recv().is_err()); // no more events

        // case 2: jit maker
        // reset the cache and account to process the log from maker's side this time
        log_stream.sub_account = "5iqawn52cdBmsjC4hDegyFnX1iNRTNDV5mRsGzgqbuyD"
            .try_into()
            .unwrap();
        log_stream.cache.write().await.reset();

        log_stream.process_log(RpcLogsResponse {
            signature: "2jLk34wWwgecuws9iD9Ug63JdL8kYBePdtcakzG34zEx9KYVYD6HuokxMZYpFw799cJZBcaCMZ47WAxkGJjM7zNC".into(),
            err: None,
            logs: logs.clone(),
        }).await;

        assert!(event_rx.try_recv().is_ok()); // place/create
        assert!(event_rx.try_recv().is_ok()); // fill with match
        assert!(event_rx.try_recv().is_ok()); // place/create
        assert!(event_rx.try_recv().is_ok()); // fill with amm
        assert!(event_rx.try_recv().is_ok()); // fill with match
        assert!(event_rx.try_recv().is_err()); // no more events
    }

    #[test]
    fn test_log() {
        let result = try_parse_log("Program log: 4DRDR8LtbQH+x7JlAAAAAAIIAAABAbpHl8YM/aWjrjfQ48x0R2DclPigyXtYx+5d/vSVjUIZAQoCAAAAAAAAAaJhIgAAAAAAAQDC6wsAAAAAAZjQCQEAAAAAAWsUAAAAAAAAAWTy////////AAAAAaNzGgMga9TnxjVkycO4bmqSGjK6kP92OrKdZMYqFV+aAS4eKQ4BAQEAHkHaNAAAAAEAwusLAAAAAAGY0AkBAAAAAAFneQwBwHPUIY9ykEdbxsTV7Lh6K+vISfq8nLCTm/rWoAHwCQAAAQABAMLrCwAAAAABAMLrCwAAAAABmNAJAQAAAAA9Zy8FAAAAAA==", "sig", 0);
        dbg!(result);
    }

    #[test]
    fn parses_jit_proxy_logs() {
        let cpi_logs = &[
            "Program log: 4DRDR8LtbQFOKvplAAAAAAAAGAABAAAAAAAAAAAAAAFGJn8TpIimFlKv8ZWRhmuU81x+ojkf3K4d+++MbslDfAGZcTYAAQEBAM5q/TIAAAABAAAAAAAAAAABAAAAAAAAAAAAAAAAAACTWxEAAAAAAA==",
            "Program log: aBNAOFkVAlpOKvplAAAAAEYmfxOkiKYWUq/xlZGGa5TzXH6iOR/crh3774xuyUN8qZQ2DwAAAABMTREAAAAAAADOav0yAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAJlxNgAYAAEBAQAAAQAAAQAAAAAA",
            "Program log: 4DRDR8LtbQFOKvplAAAAAAIIGAABAUYmfxOkiKYWUq/xlZGGa5TzXH6iOR/crh3774xuyUN8AQAAAAAAAAAAAceaAwAAAAAAAQDOav0yAAAAAQQgzQ4AAAAAAQIjAQAAAAAAAQA+////////AAAAAUYmfxOkiKYWUq/xlZGGa5TzXH6iOR/crh3774xuyUN8AZlxNgABAQEAzmr9MgAAAAEAzmr9MgAAAAEEIM0OAAAAAAHpAf4sI0TDV0Ec0LWHs9mO40bjfKEm3A+yye5HFCQQQQEzPgAAAQABANraQssAAAABANraQssAAAABLJgAOwAAAACTWxEAAAAAAA==",
            "Program log: 4DRDR8LtbQFOKvplAAAAAAAAGAABAAAAAAAAAAAAAAFGJn8TpIimFlKv8ZWRhmuU81x+ojkf3K4d+++MbslDfAGacTYAAQABAM5q/TIAAAABAAAAAAAAAAABAAAAAAAAAAAAAAAAAACTWxEAAAAAAA==",
            "Program log: aBNAOFkVAlpOKvplAAAAAEYmfxOkiKYWUq/xlZGGa5TzXH6iOR/crh3774xuyUN8qZQ2DwAAAACAPBEAAAAAAADOav0yAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAJpxNgAYAAEBAQABAAAAAQAAAAAA",
            "Program log: 4DRDR8LtbQFOKvplAAAAAAIQGAABAUYmfxOkiKYWUq/xlZGGa5TzXH6iOR/crh3774xuyUN8AQAAAAAAAAAAAciaAwAAAAAAAQDgBS0LAAAAAQBYOwMAAAAAAYs/AAAAAAAAAAAB+Ejx//////8AAUYmfxOkiKYWUq/xlZGGa5TzXH6iOR/crh3774xuyUN8AZpxNgABAAEAzmr9MgAAAAEA4AUtCwAAAAEAWDsDAAAAAAAAAAAAAJNbEQAAAAAA",
            "Program log: 4DRDR8LtbQFOKvplAAAAAAIIGAABAUYmfxOkiKYWUq/xlZGGa5TzXH6iOR/crh3774xuyUN8AQAAAAAAAAAAAcmaAwAAAAAAAQDuZNAnAAAAAYBpgwsAAAAAAV3iAAAAAAAAARhp////////AAAAAUYmfxOkiKYWUq/xlZGGa5TzXH6iOR/crh3774xuyUN8AZpxNgABAAEAzmr9MgAAAAEAzmr9MgAAAAGAwb4OAAAAAAFmQRGN8PRJqt5D5pVvCspbc3f0ZBdTB1Kcw0YfuzxCOAH2/poHAQEBAIjmn+sAAAABAFrDjp4AAAABgPDZLQAAAACTWxEAAAAAAA==",
        ];

        for log in cpi_logs {
            let result = try_parse_log(log, "sig", 0);
            dbg!(log, result);
        }
    }

    #[tokio::test]
    async fn polled_event_stream_caching() {
        let _ = env_logger::try_init();
        struct MockRpcProvider {
            tx_responses: FnvHashMap<String, EncodedTransactionWithStatusMeta>,
            signatures: tokio::sync::Mutex<Vec<String>>,
        }

        impl MockRpcProvider {
            async fn add_signatures(&self, signatures: Vec<String>) {
                let mut all_signatures = self.signatures.lock().await;
                all_signatures.extend(signatures.into_iter());
            }
        }

        impl EventRpcProvider for Arc<MockRpcProvider> {
            fn get_tx(
                &self,
                signature: Signature,
            ) -> BoxFuture<SdkResult<EncodedTransactionWithStatusMeta>> {
                ready(
                    self.tx_responses
                        .get(signature.to_string().as_str())
                        .ok_or(SdkError::Deserializing)
                        .cloned(),
                )
                .boxed()
            }
            fn get_tx_signatures(
                &self,
                _account: Pubkey,
                after: Option<Signature>,
                _limit: Option<usize>,
            ) -> BoxFuture<SdkResult<Vec<String>>> {
                async move {
                    let after = after.map(|s| s.to_string());
                    let mut self_signatures = self.signatures.lock().await;
                    if after.is_none() {
                        return Ok(self_signatures.clone());
                    }

                    if let Some(idx) = self_signatures
                        .iter()
                        .position(|s| Some(s) == after.as_ref())
                    {
                        if idx > 0 {
                            // newest -> oldest
                            *self_signatures = self_signatures[..idx].to_vec();
                        } else {
                            self_signatures.clear();
                        }
                    }

                    Ok(self_signatures.clone())
                }
                .boxed()
            }
        }

        let (event_tx, mut event_rx) = channel(16);
        let sub_account = Pubkey::new_unique();
        let cache = Arc::new(RwLock::new(TxSignatureCache::new(16)));

        let mut order_events: Vec<(OrderActionRecord, OrderRecord)> = (0..5)
            .map(|id| {
                (
                    get_order_action_record(
                        id as i64,
                        OrderAction::Place,
                        OrderActionExplanation::None,
                        0,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        None,
                        Some(sub_account.clone()),
                        Some(Order {
                            order_id: id,
                            ..Default::default()
                        }),
                        0,
                    ),
                    OrderRecord {
                        ts: id as i64,
                        user: sub_account,
                        order: Order {
                            order_id: id,
                            ..Default::default()
                        },
                    },
                )
            })
            .collect();
        let signatures: Vec<String> = (0..order_events.len())
            .map(|_| Signature::new_unique().to_string())
            .collect();
        let mut tx_responses = FnvHashMap::<String, EncodedTransactionWithStatusMeta>::default();
        for s in signatures.iter() {
            let (oar, or) = order_events.pop().unwrap();
            tx_responses.insert(
                s.clone(),
                make_transaction(
                    sub_account,
                    Signature::from_str(s).unwrap(),
                    Some(vec![
                        format!("{PROGRAM_LOG}{}", serialize_event(oar)),
                        format!("{PROGRAM_LOG}{}", serialize_event(or),),
                    ]),
                ),
            );
        }

        let mock_rpc_provider = Arc::new(MockRpcProvider {
            tx_responses,
            signatures: Mutex::new(vec![signatures.first().unwrap().clone()]),
        });

        tokio::spawn(
            PolledEventStream {
                cache: Arc::clone(&cache),
                provider: Arc::clone(&mock_rpc_provider),
                sub_account,
                event_tx,
            }
            .stream_fn(),
        );
        tokio::time::sleep(Duration::from_secs(1)).await;

        // add 4 new tx signtaures
        // 1) cached
        // 2,3) emit events
        // 4) cached
        {
            let mut cache_ = cache.write().await;
            cache_.insert(signatures[1].clone());
            cache_.insert(signatures[4].clone());
        }
        mock_rpc_provider
            .add_signatures(signatures[1..].to_vec())
            .await;
        tokio::time::sleep(Duration::from_secs(1)).await;

        assert!(event_rx.recv().await.is_some_and(|f| {
            if let DriftEvent::OrderCreate { order, .. } = f {
                println!("{}", order.order_id);
                order.order_id == 1
            } else {
                false
            }
        }));
        assert!(event_rx.recv().await.is_some_and(|f| {
            if let DriftEvent::OrderCreate { order, .. } = f {
                println!("{}", order.order_id);
                order.order_id == 2
            } else {
                false
            }
        }));
        tokio::time::sleep(Duration::from_secs(1)).await;
        assert!(event_rx.try_recv().is_err());
    }

    /// Make transaction with dummy instruction for drift program
    fn make_transaction(
        account: Pubkey,
        signature: Signature,
        logs: Option<Vec<String>>,
    ) -> EncodedTransactionWithStatusMeta {
        let mut meta = TransactionStatusMeta::default();
        meta.log_messages = logs;
        VersionedTransactionWithStatusMeta {
            transaction: VersionedTransaction {
                signatures: vec![signature],
                message: VersionedMessage::V0(
                    v0::Message::try_compile(
                        &account,
                        &[Instruction {
                            program_id: constants::PROGRAM_ID,
                            accounts: vec![AccountMeta::new_readonly(constants::PROGRAM_ID, true)],
                            data: Default::default(),
                        }],
                        &[],
                        Hash::new_unique(),
                    )
                    .expect("v0 message"),
                ),
            },
            meta,
        }
        .encode(UiTransactionEncoding::Base64, Some(0), false)
        .unwrap()
    }

    /// serialize event to string like Drift program log
    pub fn serialize_event<T: AnchorSerialize + Discriminator>(event: T) -> String {
        let mut data_buf = T::discriminator().to_vec();
        event.serialize(&mut data_buf).expect("serializes");
        base64::engine::general_purpose::STANDARD.encode(data_buf)
    }

    pub fn get_order_action_record(
        ts: i64,
        action: OrderAction,
        action_explanation: OrderActionExplanation,
        market_index: u16,
        filler: Option<Pubkey>,
        fill_record_id: Option<u64>,
        filler_reward: Option<u64>,
        base_asset_amount_filled: Option<u64>,
        quote_asset_amount_filled: Option<u64>,
        taker_fee: Option<u64>,
        maker_rebate: Option<u64>,
        referrer_reward: Option<u64>,
        quote_asset_amount_surplus: Option<i64>,
        spot_fulfillment_method_fee: Option<u64>,
        taker: Option<Pubkey>,
        taker_order: Option<Order>,
        maker: Option<Pubkey>,
        maker_order: Option<Order>,
        oracle_price: i64,
    ) -> OrderActionRecord {
        OrderActionRecord {
            ts,
            action,
            action_explanation,
            market_index,
            market_type: if let Some(taker_order) = taker_order {
                taker_order.market_type
            } else if let Some(maker_order) = maker_order {
                maker_order.market_type
            } else {
                panic!("inalid order");
            },
            filler,
            filler_reward,
            fill_record_id,
            base_asset_amount_filled,
            quote_asset_amount_filled,
            taker_fee,
            maker_fee: match maker_rebate {
                Some(maker_rebate) => Some(maker_rebate as i64),
                None => None,
            },
            referrer_reward: match referrer_reward {
                Some(referrer_reward) if referrer_reward > 0 => {
                    Some(referrer_reward.try_into().unwrap())
                }
                _ => None,
            },
            quote_asset_amount_surplus,
            spot_fulfillment_method_fee,
            taker,
            taker_order_id: taker_order.map(|order| order.order_id),
            taker_order_direction: taker_order.map(|order| order.direction),
            taker_order_base_asset_amount: taker_order.map(|order| order.base_asset_amount),
            taker_order_cumulative_base_asset_amount_filled: taker_order
                .map(|order| order.base_asset_amount_filled),
            taker_order_cumulative_quote_asset_amount_filled: taker_order
                .as_ref()
                .map(|order| order.quote_asset_amount_filled),
            maker,
            maker_order_id: maker_order.map(|order| order.order_id),
            maker_order_direction: maker_order.map(|order| order.direction),
            maker_order_base_asset_amount: maker_order.map(|order| order.base_asset_amount),
            maker_order_cumulative_base_asset_amount_filled: maker_order
                .map(|order| order.base_asset_amount_filled),
            maker_order_cumulative_quote_asset_amount_filled: maker_order
                .map(|order| order.quote_asset_amount_filled),
            oracle_price,
        }
    }
}
