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
use std::cell::{Cell, RefCell};
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Copy, Clone, Debug)]
pub struct OraclePriceDataAndSlot {
    pub data: OraclePriceData,
    pub slot: u64,
}

pub(crate) struct OracleMap {
    subscribed: Cell<bool>,
    pub(crate) oraclemap: Arc<DashMap<String, OraclePriceDataAndSlot>>,
    event_emitter: &'static EventEmitter,
    oracle_infos: DashMap<Pubkey, OracleSource>,
    sync_lock: Option<Mutex<()>>,
    latest_slot: Arc<AtomicU64>,
    commitment: CommitmentConfig,
    rpc: RpcClient,
    oracle_subscribers: RefCell<Vec<WebsocketAccountSubscriber>>,
}

impl OracleMap {
    pub fn new(
        commitment: CommitmentConfig,
        endpoint: String,
        sync: bool,
        oracle_infos: Vec<(Pubkey, OracleSource)>,
    ) -> Self {
        let oraclemap = Arc::new(DashMap::new());

        let event_emitter = EventEmitter::new();

        let rpc = RpcClient::new_with_commitment(endpoint.clone(), commitment);

        let sync_lock = if sync { Some(Mutex::new(())) } else { None };

        let oracle_infos_map = DashMap::new();
        for (pubkey, source) in oracle_infos {
            oracle_infos_map.insert(pubkey, source);
        }

        Self {
            subscribed: Cell::new(false),
            oraclemap,
            oracle_infos: oracle_infos_map,
            sync_lock,
            latest_slot: Arc::new(AtomicU64::new(0)),
            commitment,
            event_emitter: Box::leak(Box::new(event_emitter)),
            rpc,
            oracle_subscribers: RefCell::new(vec![]),
        }
    }

    pub async fn subscribe(&self) -> SdkResult<()> {
        if let Some(_) = self.sync_lock {
            self.sync().await?;
        }

        if !self.subscribed.get() {
            let url = get_ws_url(&self.rpc.url()).expect("valid url");
            let subscription_name: &'static str = "oraclemap";

            let mut oracle_subscribers = vec![];
            for oracle_info in self.oracle_infos.iter() {
                let oracle_pubkey = oracle_info.key();
                let oracle_subscriber = WebsocketAccountSubscriber::new(
                    subscription_name,
                    url.clone(),
                    *oracle_pubkey,
                    self.commitment,
                    self.event_emitter.clone(),
                );
                oracle_subscribers.push(oracle_subscriber);
            }

            self.subscribed.set(true);

            let oracle_source_by_oracle_key = self.oracle_infos.clone();
            let oracle_map = self.oraclemap.clone();

            self.event_emitter.subscribe("oraclemap", move |event| {
                if let Some(update) = event.as_any().downcast_ref::<AccountUpdate>() {
                    let oracle_pubkey = Pubkey::from_str(&update.pubkey).expect("valid pubkey");
                    let oracle_source_maybe = oracle_source_by_oracle_key.get(&oracle_pubkey);
                    if let Some(oracle_source) = oracle_source_maybe {
                        let ui_account_data = &update.data.data;
                        let data_maybe =
                            if let UiAccountData::Binary(blob, UiAccountEncoding::Base64) =
                                ui_account_data
                            {
                                base64::decode(blob).ok()
                            } else {
                                None
                            };
                        let mut data = data_maybe.expect("valid data");
                        let owner = Pubkey::from_str(&update.data.owner).expect("valid pubkey");
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
                        let price_data = get_oracle_price(
                            &oracle_source.value(),
                            &oracle_account_info,
                            update.slot,
                        )
                        .map_err(|err| crate::SdkError::Anchor(Box::new(err.into())));
                        if price_data.is_ok() {
                            let price_data = price_data.unwrap();

                            let oracle_price_data_and_slot = OraclePriceDataAndSlot {
                                data: price_data,
                                slot: update.slot,
                            };
                            oracle_map.insert(update.pubkey.clone(), oracle_price_data_and_slot);
                        }
                    }
                }
            });

            for oracle_subscriber in oracle_subscribers.clone().iter_mut() {
                oracle_subscriber.subscribe().await?;
            }

            let mut oracle_subscribers_mut = self.oracle_subscribers.try_borrow_mut()?;
            *oracle_subscribers_mut = oracle_subscribers;
        }

        Ok(())
    }

    pub async fn unsubscribe(&self) -> SdkResult<()> {
        if self.subscribed.get() {
            let mut oracle_subscribers = self.oracle_subscribers.try_borrow_mut()?;
            for oracle_subscriber in oracle_subscribers.iter_mut() {
                oracle_subscriber.unsubscribe().await?;
            }
            self.subscribed.set(false);
            self.oraclemap.clear();
            self.latest_slot.store(0, Ordering::Relaxed);
        }
        Ok(())
    }

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

        for (i, account) in response.value.iter().enumerate() {
            if let Some(oracle_account) = account {
                let oracle_info = oracle_infos[i].clone();
                let oracle_pubkey = oracle_info.0.clone();
                let mut oracle_components = (oracle_pubkey, oracle_account.clone());
                let account_info = oracle_components.into_account_info();
                let price_data = get_oracle_price(&oracle_info.1, &account_info, slot)
                    .map_err(|err| crate::SdkError::Anchor(Box::new(err.into())))?;
                self.oraclemap.insert(
                    oracle_pubkey.to_string(),
                    OraclePriceDataAndSlot {
                        data: price_data,
                        slot,
                    },
                );
            }
        }

        self.latest_slot.store(slot, Ordering::Relaxed);

        drop(lock);

        Ok(())
    }

    pub fn size(&self) -> usize {
        self.oraclemap.len()
    }

    pub fn contains(&self, key: &str) -> bool {
        self.oraclemap.contains_key(key)
    }

    pub fn get(&self, key: &str) -> Option<OraclePriceDataAndSlot> {
        self.oraclemap.get(key).map(|v| v.clone())
    }

    pub fn values(&self) -> Vec<OraclePriceData> {
        self.oraclemap
            .iter()
            .map(|x| x.value().data.clone())
            .collect()
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
        oracles.extend(perp_oracles);
        oracles.extend(spot_oracles);

        let mut oracle_infos = vec![];
        for oracle_info in oracles {
            if !oracle_infos.contains(&oracle_info) {
                oracle_infos.push(oracle_info)
            }
        }

        let oracle_infos_len = oracle_infos.len();
        dbg!(oracle_infos_len);

        let oracle_map = OracleMap::new(commitment, endpoint, true, oracle_infos);

        let _ = oracle_map.subscribe().await;

        dbg!(oracle_map.size());
        assert_eq!(oracle_map.size(), oracle_infos_len);

        dbg!("sleeping");
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        dbg!("done sleeping");

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
                .get(&sol_perp_market_oracle_pubkey.to_string())
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
                .get(&btc_perp_market_oracle_pubkey.to_string())
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
                .get(&rndr_spot_market_oracle_pubkey.to_string())
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
                .get(&weth_spot_market_oracle_pubkey.to_string())
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
