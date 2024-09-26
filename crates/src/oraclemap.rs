use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc, Mutex,
};

use dashmap::DashMap;
use futures_util::{stream::FuturesUnordered, StreamExt};
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcAccountInfoConfig};
use solana_sdk::{account::Account, commitment_config::CommitmentConfig, pubkey::Pubkey};
use tokio::sync::RwLock;

use crate::{
    drift_idl::types::OracleSource,
    ffi::{get_oracle_price, OraclePriceData},
    utils::get_ws_url,
    websocket_account_subscriber::{AccountUpdate, WebsocketAccountSubscriber},
    SdkResult, UnsubHandle,
};

#[derive(Clone, Debug)]
pub struct Oracle {
    pub pubkey: Pubkey,
    pub data: OraclePriceData,
    pub source: OracleSource,
    pub slot: u64,
    pub raw: Vec<u8>,
}

pub(crate) struct OracleMap {
    pub(crate) oraclemap: Arc<DashMap<Pubkey, Oracle>>,
    oracle_infos: DashMap<Pubkey, OracleSource>,
    sync_lock: Option<Mutex<()>>,
    latest_slot: Arc<AtomicU64>,
    rpc: RpcClient,
    oracle_subscribers: RwLock<Vec<UnsubHandle>>,
    perp_oracles: DashMap<u16, Pubkey>,
    spot_oracles: DashMap<u16, Pubkey>,
}

impl OracleMap {
    pub const SUBSCRIPTION_ID: &'static str = "oraclemap";

    pub fn new(
        commitment: CommitmentConfig,
        endpoint: String,
        sync: bool,
        perp_oracles: Vec<(u16, Pubkey, OracleSource)>,
        spot_oracles: Vec<(u16, Pubkey, OracleSource)>,
    ) -> Self {
        let oraclemap = Arc::new(DashMap::new());
        let rpc = RpcClient::new_with_commitment(endpoint.clone(), commitment);
        let sync_lock = if sync { Some(Mutex::new(())) } else { None };

        let oracle_infos_map: DashMap<_, _> = perp_oracles
            .iter()
            .chain(spot_oracles.iter())
            .map(|(_, pubkey, oracle_source)| (*pubkey, *oracle_source))
            .collect();

        let perp_oracles_map: DashMap<_, _> = perp_oracles
            .iter()
            .map(|(market_index, pubkey, _)| (*market_index, *pubkey))
            .collect();

        let spot_oracles_map: DashMap<_, _> = spot_oracles
            .iter()
            .map(|(market_index, pubkey, _)| (*market_index, *pubkey))
            .collect();

        Self {
            oraclemap,
            oracle_infos: oracle_infos_map,
            sync_lock,
            latest_slot: Arc::new(AtomicU64::new(0)),
            rpc,
            oracle_subscribers: Default::default(),
            perp_oracles: perp_oracles_map,
            spot_oracles: spot_oracles_map,
        }
    }

    pub async fn subscribe(&self) -> SdkResult<()> {
        if self.sync_lock.is_some() {
            self.sync().await?;
        }

        if self.is_subscribed().await {
            return Ok(());
        }

        let url = get_ws_url(&self.rpc.url()).expect("valid url");

        let mut pending_subscriptions =
            Vec::<WebsocketAccountSubscriber>::with_capacity(self.oracle_infos.len());
        for oracle_info in self.oracle_infos.iter() {
            let oracle_pubkey = oracle_info.key();
            let oracle_subscriber =
                WebsocketAccountSubscriber::new(url.clone(), *oracle_pubkey, self.rpc.commitment());
            pending_subscriptions.push(oracle_subscriber);
        }

        let futs_iter = pending_subscriptions.iter().map(|s| {
            let source = *self.oracle_infos.get(&s.pubkey).expect("oracle source");
            s.subscribe(Self::SUBSCRIPTION_ID, {
                let oracle_map = Arc::clone(&self.oraclemap);
                move |update| handler_fn(&oracle_map, source, update)
            })
        });
        let mut subscription_futs = FuturesUnordered::from_iter(futs_iter);

        let mut oracle_subscriptions = self.oracle_subscribers.write().await;
        while let Some(unsub) = subscription_futs.next().await {
            oracle_subscriptions.push(unsub.expect("oracle subscribed"));
        }

        Ok(())
    }

    pub async fn unsubscribe(&self) -> SdkResult<()> {
        {
            let mut oracle_subscribers = self.oracle_subscribers.write().await;
            for unsub in oracle_subscribers.drain(..) {
                let _ = unsub.send(());
            }
        }

        self.oraclemap.clear();
        self.latest_slot.store(0, Ordering::Relaxed);

        Ok(())
    }

    #[allow(clippy::await_holding_lock)]
    async fn sync(&self) -> SdkResult<()> {
        let sync_lock = self.sync_lock.as_ref().expect("expected sync lock");

        let _lock = match sync_lock.try_lock() {
            Ok(lock) => lock,
            Err(_) => return Ok(()),
        };

        let account_config = RpcAccountInfoConfig {
            commitment: Some(self.rpc.commitment()),
            encoding: None,
            ..RpcAccountInfoConfig::default()
        };

        let mut pubkeys = self
            .oracle_infos
            .iter()
            .map(|oracle_info_ref| *oracle_info_ref.key())
            .collect::<Vec<Pubkey>>();
        pubkeys.sort();

        let mut oracle_infos = self
            .oracle_infos
            .iter()
            .map(|oracle_info_ref| (*oracle_info_ref.key(), *oracle_info_ref.value()))
            .collect::<Vec<(Pubkey, OracleSource)>>();
        oracle_infos.sort_by_key(|key| key.0);

        let response = self
            .rpc
            .get_multiple_accounts_with_config(&pubkeys, account_config)
            .await?;

        if response.value.len() != pubkeys.len() {
            return Err(crate::SdkError::Generic(format!(
                "failed to get all oracle accounts, expected: {}, got: {}",
                pubkeys.len(),
                response.value.len()
            )));
        }

        let slot = response.context.slot;

        for (account, oracle_info) in response.value.iter().zip(oracle_infos.iter()) {
            if let Some(oracle_account) = account {
                let oracle_pubkey = oracle_info.0;
                let price_data = get_oracle_price(
                    oracle_info.1,
                    &mut (oracle_pubkey, oracle_account.clone()),
                    slot,
                )?;
                self.oraclemap.insert(
                    oracle_pubkey,
                    Oracle {
                        pubkey: oracle_pubkey,
                        data: price_data,
                        source: oracle_info.1,
                        slot,
                        raw: account.as_ref().expect("account").data.clone(),
                    },
                );
            }
        }

        self.latest_slot.store(slot, Ordering::Relaxed);

        Ok(())
    }

    /// Return whether the `OracleMap`` is subscribed to network changes
    pub async fn is_subscribed(&self) -> bool {
        let subscribers = self.oracle_subscribers.read().await;
        !subscribers.is_empty()
    }

    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.oraclemap.len()
    }

    pub fn contains(&self, key: &Pubkey) -> bool {
        self.oracle_infos.contains_key(key)
    }

    pub fn current_perp_oracle(&self, market_index: u16) -> Option<Pubkey> {
        self.perp_oracles.get(&market_index).map(|x| *x)
    }

    pub fn current_spot_oracle(&self, market_index: u16) -> Option<Pubkey> {
        self.spot_oracles.get(&market_index).map(|x| *x)
    }

    pub fn get(&self, key: &Pubkey) -> Option<Oracle> {
        self.oraclemap.get(key).map(|x| x.clone())
    }

    #[allow(dead_code)]
    pub fn values(&self) -> Vec<Oracle> {
        self.oraclemap.iter().map(|x| x.clone()).collect()
    }

    pub async fn add_oracle(&self, oracle: Pubkey, source: OracleSource) -> SdkResult<()> {
        if self.contains(&oracle) {
            return Ok(()); // don't add a duplicate
        }

        self.oracle_infos.insert(oracle, source);

        let new_oracle_subscriber = WebsocketAccountSubscriber::new(
            get_ws_url(&self.rpc.url()).expect("valid url"),
            oracle,
            self.rpc.commitment(),
        );
        let oracle_source = *self.oracle_infos.get(&oracle).expect("oracle source");

        let unsub = new_oracle_subscriber
            .subscribe(Self::SUBSCRIPTION_ID, {
                let oracle_map = Arc::clone(&self.oraclemap);
                move |update| handler_fn(&oracle_map, oracle_source, update)
            })
            .await?;

        let mut oracle_subscribers = self.oracle_subscribers.write().await;
        oracle_subscribers.push(unsub);

        Ok(())
    }

    pub fn get_latest_slot(&self) -> u64 {
        self.latest_slot.load(Ordering::Relaxed)
    }
}

/// Handler fn for new oracle account data
fn handler_fn(
    oracle_map: &Arc<DashMap<Pubkey, Oracle>>,
    oracle_source: OracleSource,
    update: &AccountUpdate,
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
                .entry(oracle_pubkey)
                .and_modify(|o| {
                    o.data = price_data;
                    o.slot = update.slot;
                    o.raw.clone_from(&update.data);
                })
                .or_insert(Oracle {
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
