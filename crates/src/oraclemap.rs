use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};

use dashmap::DashMap;
use futures_util::{stream::FuturesUnordered, StreamExt};
use log::warn;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    account::Account, clock::Slot, commitment_config::CommitmentConfig, pubkey::Pubkey,
};

use crate::{
    drift_idl::types::OracleSource,
    ffi::{get_oracle_price, OraclePriceData},
    utils::get_ws_url,
    websocket_account_subscriber::{AccountUpdate, WebsocketAccountSubscriber},
    MarketId, SdkError, SdkResult, UnsubHandle,
};

const LOG_TARGET: &str = "oraclemap";

#[derive(Clone, Default, Debug)]
pub struct Oracle {
    pub market: MarketId,
    pub pubkey: Pubkey,
    pub data: OraclePriceData,
    pub source: OracleSource,
    pub slot: u64,
    pub raw: Vec<u8>,
}

/// Dynamic map of Drift market oracle data
///
/// Caller can subscribe to some subset of markets for Ws backed updates
/// Alternatively, the caller may drive the map by calling `sync` periodically
pub struct OracleMap {
    /// Oracle info keyed by market
    oraclemap: Arc<DashMap<MarketId, Oracle, ahash::RandomState>>,
    /// Oracle subscription handles keyed by market
    oracle_subscriptions: DashMap<MarketId, UnsubHandle, ahash::RandomState>,
    latest_slot: Arc<AtomicU64>,
    rpc: RpcClient,
}

impl OracleMap {
    pub const SUBSCRIPTION_ID: &str = "oraclemap";

    /// Create a new `OracleMap`
    ///
    /// * `all_oracles` - Exhaustive list of all Drift oracle pubkeys and source by market
    pub fn new(
        commitment: CommitmentConfig,
        endpoint: String,
        all_oracles: &[(MarketId, Pubkey, OracleSource)],
    ) -> Self {
        let rpc = RpcClient::new_with_commitment(endpoint.clone(), commitment);
        let oraclemap = all_oracles
            .iter()
            .copied()
            .map(|(market, pubkey, source)| {
                (
                    market,
                    Oracle {
                        pubkey,
                        source,
                        ..Default::default()
                    },
                )
            })
            .collect();

        Self {
            oraclemap: Arc::new(oraclemap),
            oracle_subscriptions: Default::default(),
            latest_slot: Arc::new(AtomicU64::new(0)),
            rpc,
        }
    }

    /// Subscribe to oracle updates for given `markets`
    /// Can be called multiple times to subscribe to additional markets
    ///
    /// Panics
    ///
    /// If the `market` oracle pubkey is not loaded
    pub async fn subscribe(&self, markets: &[MarketId]) -> SdkResult<()> {
        log::debug!(target: LOG_TARGET, "subscribe market oracles: {markets:?}");
        self.sync(markets).await?;

        let url = get_ws_url(&self.rpc.url()).expect("valid url");

        let mut pending_subscriptions =
            Vec::<(WebsocketAccountSubscriber, Oracle)>::with_capacity(markets.len());

        for market in markets {
            let oracle_info = self.oraclemap.get(market).expect("oracle exists"); // caller did not supply in `OracleMap::new()`
            let oracle_subscriber = WebsocketAccountSubscriber::new(
                url.clone(),
                oracle_info.pubkey,
                self.rpc.commitment(),
            );

            pending_subscriptions.push((oracle_subscriber, oracle_info.clone()));
        }

        let futs_iter = pending_subscriptions.into_iter().map(|(sub_fut, info)| {
            let oraclemap = Arc::clone(&self.oraclemap);
            async move {
                let unsub = sub_fut
                    .subscribe(Self::SUBSCRIPTION_ID, false, {
                        move |update| update_handler(update, info.market, info.source, &oraclemap)
                    })
                    .await;
                (info.market, unsub)
            }
        });

        let mut subscription_futs = FuturesUnordered::from_iter(futs_iter);

        while let Some((market, unsub)) = subscription_futs.next().await {
            self.oracle_subscriptions.insert(market, unsub?);
        }

        log::debug!(target: LOG_TARGET, "subscribed");
        Ok(())
    }

    /// Unsubscribe from oracle updates for the given `markets`
    pub fn unsubscribe(&self, markets: &[MarketId]) -> SdkResult<()> {
        for market in markets {
            if let Some((market, unsub)) = self.oracle_subscriptions.remove(market) {
                let _ = unsub.send(());
                self.oraclemap.remove(&market);
            }
        }
        log::debug!(target: LOG_TARGET, "unsubscribed markets: {markets:?}");

        Ok(())
    }

    /// Unsubscribe from all oracle updates
    pub fn unsubscribe_all(&self) -> SdkResult<()> {
        let all_markets: Vec<MarketId> =
            self.oracle_subscriptions.iter().map(|x| *x.key()).collect();
        self.unsubscribe(&all_markets)
    }

    /// Fetches account data for each market oracle set by `markets`
    ///
    /// This may be invoked manually to resync oracle data for some set of markets
    pub async fn sync(&self, markets: &[MarketId]) -> SdkResult<()> {
        log::debug!(target: LOG_TARGET, "sync oracles for: {markets:?}");

        let mut market_by_oracle_key = HashMap::<Pubkey, MarketId>::with_capacity(markets.len());
        for market in markets {
            if let Some(oracle) = self.oraclemap.get(market) {
                market_by_oracle_key.insert(oracle.value().pubkey, *market);
            }
        }

        let oracle_pubkeys: Vec<Pubkey> = market_by_oracle_key.keys().copied().collect();

        let (synced_oracles, latest_slot) =
            match get_multi_account_data_with_fallback(&self.rpc, &oracle_pubkeys).await {
                Ok(result) => result,
                Err(err) => {
                    warn!(target: LOG_TARGET, "failed to sync oracle accounts");
                    return Err(err);
                }
            };

        if synced_oracles.len() != oracle_pubkeys.len() {
            warn!(target: LOG_TARGET, "failed to sync all oracle accounts");
            return Err(SdkError::InvalidOracle);
        }

        for (oracle_pubkey, oracle_account) in synced_oracles.iter() {
            let market = market_by_oracle_key
                .get(oracle_pubkey)
                .expect("market oracle syncd");
            self.oraclemap.entry(*market).and_modify(|o| {
                let price_data = get_oracle_price(
                    o.source,
                    &mut (*oracle_pubkey, oracle_account.clone()),
                    latest_slot,
                )
                .expect("valid oracle data");

                o.raw.clone_from(&oracle_account.data);
                o.data = price_data;
                o.slot = latest_slot;
            });
        }

        self.latest_slot.store(latest_slot, Ordering::Relaxed);
        log::debug!(target: LOG_TARGET, "synced");

        Ok(())
    }

    /// Number of oracles known to the `OracleMap`
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.oraclemap.len()
    }

    /// Returns true if the oraclemap has a subscription for `market`
    pub fn is_subscribed(&self, market: &MarketId) -> bool {
        self.oracle_subscriptions.contains_key(market)
    }

    /// Get the address of a perp market oracle
    pub fn current_perp_oracle(&self, market_index: u16) -> Option<Pubkey> {
        self.oraclemap
            .get(&MarketId::perp(market_index))
            .map(|x| x.pubkey)
    }

    /// Get the address of a spot market oracle
    pub fn current_spot_oracle(&self, market_index: u16) -> Option<Pubkey> {
        self.oraclemap
            .get(&MarketId::spot(market_index))
            .map(|x| x.pubkey)
    }

    /// Return Oracle data by pubkey, if known
    /// deprecated, see `get_by_key` instead
    #[deprecated]
    pub fn get(&self, key: &Pubkey) -> Option<Oracle> {
        self.oraclemap
            .iter()
            .find(|o| &o.pubkey == key)
            .map(|o| o.value().clone())
    }

    /// Return Oracle data by pubkey, if known
    pub fn get_by_key(&self, key: &Pubkey) -> Option<Oracle> {
        self.oraclemap
            .iter()
            .find(|o| &o.pubkey == key)
            .map(|o| o.value().clone())
    }

    /// Return Oracle data by market, if known
    pub fn get_by_market(&self, market: MarketId) -> Option<Oracle> {
        self.oraclemap.get(&market).map(|o| o.clone())
    }

    #[allow(dead_code)]
    pub fn values(&self) -> Vec<Oracle> {
        self.oraclemap.iter().map(|x| x.clone()).collect()
    }

    pub fn get_latest_slot(&self) -> u64 {
        self.latest_slot.load(Ordering::Relaxed)
    }
}

/// Handler fn for new oracle account data
fn update_handler(
    update: &AccountUpdate,
    oracle_market: MarketId,
    oracle_source: OracleSource,
    oracle_map: &DashMap<MarketId, Oracle, ahash::RandomState>,
) {
    let oracle_pubkey = update.pubkey;
    let lamports = update.lamports;
    match get_oracle_price(
        oracle_source,
        &mut (
            oracle_pubkey,
            Account {
                owner: update.owner,
                data: update.data.clone(),
                lamports,
                ..Default::default()
            },
        ),
        update.slot,
    ) {
        Ok(price_data) => {
            oracle_map
                .entry(oracle_market)
                .and_modify(|o| {
                    o.data = price_data;
                    o.slot = update.slot;
                    o.raw.clone_from(&update.data);
                })
                .or_insert(Oracle {
                    market: oracle_market,
                    pubkey: oracle_pubkey,
                    data: price_data,
                    source: oracle_source,
                    slot: update.slot,
                    raw: update.data.clone(),
                });
        }
        Err(err) => {
            log::error!("Failed to get oracle price: {err:?}")
        }
    }
}

/// Fetch all accounts with multiple fallbacks
///
/// Tries progressively less intensive RPC methods for wider compatibility with RPC providers:
///    getMultipleAccounts, lastly multiple getAccountInfo
///
/// Returns deserialized accounts and retrieved slot
async fn get_multi_account_data_with_fallback(
    rpc: &RpcClient,
    pubkeys: &[Pubkey],
) -> SdkResult<(Vec<(Pubkey, Account)>, Slot)> {
    let mut account_data = Vec::default();

    // try 'getMultipleAccounts'
    let accounts_response = rpc
        .get_multiple_accounts_with_commitment(pubkeys, rpc.commitment())
        .await;
    if let Ok(response) = accounts_response {
        for (pubkey, account) in pubkeys.iter().zip(response.value) {
            let account = account.expect("market account exists");
            account_data.push((*pubkey, account));
        }
        return Ok((account_data, response.context.slot));
    }
    log::debug!(target: LOG_TARGET, "syncing with getMultipleAccounts failed");

    // try multiple 'getAccount's
    let mut account_requests = FuturesUnordered::from_iter(pubkeys.iter().map(|p| async move {
        (
            p,
            rpc.get_account_with_commitment(p, rpc.commitment()).await,
        )
    }));

    let mut latest_slot = 0;
    while let Some((pubkey, response)) = account_requests.next().await {
        match response {
            Ok(response) => {
                let account = response.value.ok_or({
                    log::warn!("failed to fetch oracle account");
                    SdkError::InvalidOracle
                })?;
                latest_slot = latest_slot.max(response.context.slot);
                account_data.push((*pubkey, account));
            }
            Err(err) => {
                log::warn!("failed to fetch oracle account: {err:?}");
                return Err(err)?;
            }
        }
    }

    Ok((account_data, latest_slot))
}

#[cfg(feature = "rpc_tests")]
mod tests {
    use super::*;
    use crate::{
        drift_idl::accounts::{PerpMarket, SpotMarket},
        marketmap::MarketMap,
        utils::test_envs::mainnet_endpoint,
    };

    #[tokio::test]
    async fn test_oracle_map() {
        let commitment = CommitmentConfig::processed();

        let spot_market_map =
            MarketMap::<SpotMarket>::new(commitment.clone(), mainnet_endpoint(), true);
        let perp_market_map =
            MarketMap::<PerpMarket>::new(commitment.clone(), mainnet_endpoint(), true);

        let _ = spot_market_map.sync().await;
        let _ = perp_market_map.sync().await;

        let perp_oracles = perp_market_map.oracles();
        let spot_oracles = spot_market_map.oracles();

        let mut oracles = vec![];
        oracles.extend(perp_oracles.clone());
        oracles.extend(spot_oracles.clone());

        let mut oracle_infos = vec![];
        for oracle_info in oracles {
            if !oracle_infos.contains(&oracle_info) {
                oracle_infos.push(oracle_info)
            }
        }

        let oracle_infos_len = oracle_infos.len();
        dbg!(oracle_infos_len);

        let oracle_map = OracleMap::new(
            commitment,
            &mainnet_endpoint(),
            true,
            perp_oracles,
            spot_oracles,
        );

        let _ = oracle_map.subscribe().await;

        dbg!(oracle_map.size());

        dbg!("sleeping");
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        dbg!("done sleeping");

        let rlb_perp_market_oracle_pubkey = perp_market_map
            .get(&17)
            .expect("rlb perp market")
            .data
            .amm
            .oracle;
        let rlb_oracle = oracle_map
            .get(&rlb_perp_market_oracle_pubkey)
            .expect("rlb oracle");
        dbg!("rlb oracle info:");
        dbg!(rlb_oracle.data.price);
        dbg!(rlb_oracle.slot);

        dbg!("perp market oracles");
        let mut last_sol_price = 0;
        let mut last_sol_slot = 0;
        let mut last_btc_price = 0;
        let mut last_btc_slot = 0;
        for _ in 0..10 {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            dbg!();
            let sol_perp_market_oracle_pubkey = perp_market_map
                .get(&0)
                .expect("sol perp market")
                .data
                .amm
                .oracle;
            let sol_oracle = oracle_map
                .get(&sol_perp_market_oracle_pubkey)
                .expect("sol oracle");
            dbg!("sol oracle info:");
            dbg!(sol_oracle.data.price);
            dbg!(sol_oracle.slot);
            dbg!(
                "sol price change: {}",
                sol_oracle.data.price - last_sol_price
            );
            dbg!("sol slot change: {}", sol_oracle.slot - last_sol_slot);
            last_sol_price = sol_oracle.data.price;
            last_sol_slot = sol_oracle.slot;

            dbg!();

            let btc_perp_market_oracle_pubkey = perp_market_map
                .get(&1)
                .expect("btc perp market")
                .data
                .amm
                .oracle;
            let btc_oracle = oracle_map
                .get(&btc_perp_market_oracle_pubkey)
                .expect("btc oracle");
            dbg!("btc oracle info:");
            dbg!(btc_oracle.data.price);
            dbg!(btc_oracle.slot);
            dbg!(
                "btc price change: {}",
                btc_oracle.data.price - last_btc_price
            );
            dbg!("btc slot change: {}", btc_oracle.slot - last_btc_slot);
            last_btc_price = btc_oracle.data.price;
            last_btc_slot = btc_oracle.slot;
        }

        dbg!();

        dbg!("spot market oracles");
        let mut last_rndr_price = 0;
        let mut last_rndr_slot = 0;
        let mut last_weth_price = 0;
        let mut last_weth_slot = 0;
        for _ in 0..10 {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            dbg!();
            let rndr_spot_market_oracle_pubkey = spot_market_map
                .get(&11)
                .expect("sol perp market")
                .data
                .oracle;
            let rndr_oracle = oracle_map
                .get(&rndr_spot_market_oracle_pubkey)
                .expect("sol oracle");
            dbg!("rndr oracle info:");
            dbg!(rndr_oracle.data.price);
            dbg!(rndr_oracle.slot);
            dbg!(
                "rndr price change: {}",
                rndr_oracle.data.price - last_rndr_price
            );
            dbg!("rndr slot change: {}", rndr_oracle.slot - last_rndr_slot);
            last_rndr_price = rndr_oracle.data.price;
            last_rndr_slot = rndr_oracle.slot;

            dbg!();

            let weth_spot_market_oracle_pubkey = spot_market_map
                .get(&4)
                .expect("sol perp market")
                .data
                .oracle;
            let weth_oracle = oracle_map
                .get(&weth_spot_market_oracle_pubkey)
                .expect("sol oracle");
            dbg!("weth oracle info:");
            dbg!(weth_oracle.data.price);
            dbg!(weth_oracle.slot);
            dbg!(
                "weth price change: {}",
                weth_oracle.data.price - last_weth_price
            );
            dbg!("weth slot change: {}", weth_oracle.slot - last_weth_slot);
            last_weth_price = weth_oracle.data.price;
            last_weth_slot = weth_oracle.slot;
        }

        let _ = oracle_map.unsubscribe().await;
    }
}
