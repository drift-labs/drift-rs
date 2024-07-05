use crate::utils::get_ws_url;
use crate::websocket_account_subscriber::{AccountUpdate, WebsocketAccountSubscriber};
use crate::{event_emitter::EventEmitter, SdkResult};
use dashmap::DashMap;
use drift::state::oracle::{get_oracle_price, OraclePriceData, OracleSource};
use solana_account_decoder::{UiAccountData, UiAccountEncoding};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::RpcAccountInfoConfig;
use solana_sdk::account_info::{AccountInfo, IntoAccountInfo};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use tokio::sync::RwLock;

#[derive(Clone, Debug)]
pub struct Oracle {
    pub pubkey: Pubkey,
    pub data: OraclePriceData,
    pub source: OracleSource,
    pub slot: u64,
    pub raw: Vec<u8>,
}

pub(crate) struct OracleMap {
    subscribed: AtomicBool,
    pub(crate) oraclemap: Arc<DashMap<Pubkey, Oracle>>,
    event_emitter: &'static EventEmitter,
    oracle_infos: DashMap<Pubkey, OracleSource>,
    sync_lock: Option<Mutex<()>>,
    latest_slot: Arc<AtomicU64>,
    commitment: CommitmentConfig,
    rpc: RpcClient,
    oracle_subscribers: RwLock<Vec<WebsocketAccountSubscriber>>,
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

        let event_emitter = EventEmitter::new();

        let rpc = RpcClient::new_with_commitment(endpoint.clone(), commitment);

        let sync_lock = if sync { Some(Mutex::new(())) } else { None };

        let mut all_oracles = vec![];
        all_oracles.extend(perp_oracles.clone());
        all_oracles.extend(spot_oracles.clone());

        let oracle_infos_map: DashMap<_, _> = all_oracles
            .iter()
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
            subscribed: AtomicBool::new(false),
            oraclemap,
            oracle_infos: oracle_infos_map,
            sync_lock,
            latest_slot: Arc::new(AtomicU64::new(0)),
            commitment,
            event_emitter: Box::leak(Box::new(event_emitter)),
            rpc,
            oracle_subscribers: RwLock::new(vec![]),
            perp_oracles: perp_oracles_map,
            spot_oracles: spot_oracles_map,
        }
    }

    pub async fn subscribe(&self) -> SdkResult<()> {
        if self.sync_lock.is_some() {
            self.sync().await?;
        }

        if !self.subscribed.load(Ordering::Relaxed) {
            let url = get_ws_url(&self.rpc.url()).expect("valid url");

            let mut oracle_subscribers = vec![];
            for oracle_info in self.oracle_infos.iter() {
                let oracle_pubkey = oracle_info.key();
                let oracle_subscriber = WebsocketAccountSubscriber::new(
                    OracleMap::SUBSCRIPTION_ID,
                    url.clone(),
                    *oracle_pubkey,
                    self.commitment,
                    self.event_emitter.clone(),
                );
                oracle_subscribers.push(oracle_subscriber);
            }

            self.subscribed.store(true, Ordering::Relaxed);

            let oracle_source_by_oracle_key = self.oracle_infos.clone();
            let oracle_map = self.oraclemap.clone();

            self.event_emitter
                .subscribe(OracleMap::SUBSCRIPTION_ID, move |event| {
                    if let Some(update) = event.as_any().downcast_ref::<AccountUpdate>() {
                        let oracle_pubkey = Pubkey::from_str(&update.pubkey).expect("valid pubkey");
                        let oracle_source_maybe = oracle_source_by_oracle_key.get(&oracle_pubkey);
                        if let Some(oracle_source) = oracle_source_maybe {
                            if let UiAccountData::Binary(blob, UiAccountEncoding::Base64) =
                                &update.data.data
                            {
                                let mut data = base64::decode(blob).expect("valid data");
                                let owner =
                                    Pubkey::from_str(&update.data.owner).expect("valid pubkey");
                                let mut lamports = update.data.lamports;
                                let oracle_account_info = AccountInfo::new(
                                    &oracle_pubkey,
                                    false,
                                    false,
                                    &mut lamports,
                                    &mut data,
                                    &owner,
                                    false,
                                    update.data.rent_epoch,
                                );
                                match get_oracle_price(
                                    oracle_source.value(),
                                    &oracle_account_info,
                                    update.slot,
                                ) {
                                    Ok(price_data) => {
                                        oracle_map.insert(
                                            oracle_pubkey,
                                            Oracle {
                                                pubkey: oracle_pubkey,
                                                data: price_data,
                                                source: *oracle_source.value(),
                                                slot: update.slot,
                                                raw: data,
                                            },
                                        );
                                    }
                                    Err(err) => {
                                        log::error!("Failed to get oracle price: {:?}", err)
                                    }
                                }
                            }
                        }
                    }
                });

            let mut subscribers_clone = oracle_subscribers.clone();

            let subscribe_futures = subscribers_clone
                .iter_mut()
                .map(|subscriber| subscriber.subscribe())
                .collect::<Vec<_>>();
            let results = futures_util::future::join_all(subscribe_futures).await;
            results.into_iter().collect::<Result<Vec<_>, _>>()?;

            let mut oracle_subscribers_mut = self.oracle_subscribers.write().await;
            *oracle_subscribers_mut = oracle_subscribers;
        }

        Ok(())
    }

    pub async fn unsubscribe(&self) -> SdkResult<()> {
        if self.subscribed.load(Ordering::Relaxed) {
            let mut oracle_subscribers = self.oracle_subscribers.write().await;
            let unsubscribe_futures = oracle_subscribers
                .iter_mut()
                .map(|subscriber| subscriber.unsubscribe())
                .collect::<Vec<_>>();

            let results = futures_util::future::join_all(unsubscribe_futures).await;
            results.into_iter().collect::<Result<Vec<_>, _>>()?;
            self.subscribed.store(false, Ordering::Relaxed);
            self.oraclemap.clear();
            self.latest_slot.store(0, Ordering::Relaxed);
        }
        Ok(())
    }

    #[allow(clippy::await_holding_lock)]
    async fn sync(&self) -> SdkResult<()> {
        let sync_lock = self.sync_lock.as_ref().expect("expected sync lock");

        let lock = match sync_lock.try_lock() {
            Ok(lock) => lock,
            Err(_) => return Ok(()),
        };

        let account_config = RpcAccountInfoConfig {
            commitment: Some(self.commitment),
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
                let mut oracle_components = (oracle_pubkey, oracle_account.clone());
                let account_info = oracle_components.into_account_info();
                let price_data = get_oracle_price(&oracle_info.1, &account_info, slot)
                    .map_err(|err| crate::SdkError::Anchor(Box::new(err.into())))?;
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

        drop(lock);

        Ok(())
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

        let mut new_oracle_subscriber = WebsocketAccountSubscriber::new(
            OracleMap::SUBSCRIPTION_ID,
            get_ws_url(&self.rpc.url()).expect("valid url"),
            oracle,
            self.commitment,
            self.event_emitter.clone(),
        );

        new_oracle_subscriber.subscribe().await?;
        let mut oracle_subscribers = self.oracle_subscribers.write().await;
        oracle_subscribers.push(new_oracle_subscriber);

        Ok(())
    }

    pub fn update_spot_oracle(&self, market_index: u16, oracle: Pubkey) {
        self.spot_oracles.insert(market_index, oracle);
    }

    pub fn update_perp_oracle(&self, market_index: u16, oracle: Pubkey) {
        self.perp_oracles.insert(market_index, oracle);
    }

    pub fn get_latest_slot(&self) -> u64 {
        self.latest_slot.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::marketmap::MarketMap;
    use drift::state::perp_market::PerpMarket;
    use drift::state::spot_market::SpotMarket;

    #[tokio::test]
    #[cfg(rpc_tests)]
    async fn test_oracle_map() {
        let commitment = CommitmentConfig::processed();
        let endpoint = "rpc".to_string();

        let spot_market_map =
            MarketMap::<SpotMarket>::new(commitment.clone(), endpoint.clone(), true);
        let perp_market_map =
            MarketMap::<PerpMarket>::new(commitment.clone(), endpoint.clone(), true);

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

        let oracle_map = OracleMap::new(commitment, endpoint, true, perp_oracles, spot_oracles);

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
