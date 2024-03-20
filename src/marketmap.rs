use std::cell::{Cell, RefCell};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use crate::event_emitter::EventEmitter;
use crate::memcmp::get_market_filter;
use crate::utils::{decode, get_ws_url};
use crate::websocket_program_account_subscriber::{
    ProgramAccountUpdate, WebsocketProgramAccountOptions, WebsocketProgramAccountSubscriber,
};
use crate::{DataAndSlot, SdkResult};
use anchor_lang::AccountDeserialize;
use dashmap::DashMap;
use drift::state::oracle::OracleSource;
use drift::state::perp_market::PerpMarket;
use drift::state::spot_market::SpotMarket;
use drift::state::user::MarketType;
use serde_json::json;
use solana_account_decoder::UiAccountEncoding;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use solana_client::rpc_request::RpcRequest;
use solana_client::rpc_response::{OptionalContext, RpcKeyedAccount};
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;

pub trait Market {
    fn market_index(&self) -> u16;
    fn market_type() -> MarketType;
    fn oracle_info(&self) -> (Pubkey, OracleSource);
}

impl Market for PerpMarket {
    fn market_index(&self) -> u16 {
        self.market_index
    }

    fn market_type() -> MarketType {
        MarketType::Perp
    }

    fn oracle_info(&self) -> (Pubkey, OracleSource) {
        (self.amm.oracle, self.amm.oracle_source)
    }
}

impl Market for SpotMarket {
    fn market_index(&self) -> u16 {
        self.market_index
    }

    fn market_type() -> MarketType {
        MarketType::Spot
    }

    fn oracle_info(&self) -> (Pubkey, OracleSource) {
        (self.oracle, self.oracle_source)
    }
}

pub struct MarketMap<T: AccountDeserialize> {
    subscribed: Cell<bool>,
    subscription: RefCell<WebsocketProgramAccountSubscriber>,
    marketmap: Arc<DashMap<u16, DataAndSlot<T>>>,
    sync_lock: Option<Mutex<()>>,
    latest_slot: Arc<AtomicU64>,
    commitment: CommitmentConfig,
    rpc: RpcClient,
    synced: bool,
}

impl<T: AccountDeserialize + Clone + Send + Sync + Market + 'static> MarketMap<T> {
    pub fn new(commitment: CommitmentConfig, endpoint: String, sync: bool) -> Self {
        let filters = vec![get_market_filter(T::market_type())];
        let options = WebsocketProgramAccountOptions {
            filters,
            commitment,
            encoding: UiAccountEncoding::Base64,
        };
        let event_emitter = EventEmitter::new();

        let url = get_ws_url(&endpoint.clone()).unwrap();

        let subscription =
            WebsocketProgramAccountSubscriber::new("marketmap", url, options, event_emitter);

        let marketmap = Arc::new(DashMap::new());

        let rpc = RpcClient::new_with_commitment(endpoint.clone(), commitment);

        let sync_lock = if sync { Some(Mutex::new(())) } else { None };

        Self {
            subscribed: Cell::new(false),
            subscription: RefCell::new(subscription),
            marketmap,
            sync_lock,
            latest_slot: Arc::new(AtomicU64::new(0)),
            commitment,
            rpc,
            synced: false,
        }
    }

    pub async fn subscribe(&self) -> SdkResult<()> {
        if let Some(_) = self.sync_lock {
            self.sync().await?;
        }

        if !self.subscribed.get() {
            self.subscription.try_borrow_mut()?.subscribe::<T>().await?;
            self.subscribed.set(true);

            let marketmap = self.marketmap.clone();
            let latest_slot = self.latest_slot.clone();

            self.subscription
                .try_borrow()?
                .event_emitter
                .subscribe("marketmap", move |event| {
                    if let Some(update) = event.as_any().downcast_ref::<ProgramAccountUpdate<T>>() {
                        let market_data_and_slot = update.data_and_slot.clone();
                        if update.data_and_slot.slot > latest_slot.load(Ordering::Relaxed) {
                            latest_slot.store(update.data_and_slot.slot, Ordering::Relaxed);
                        }
                        marketmap.insert(
                            update.data_and_slot.clone().data.market_index(),
                            DataAndSlot {
                                data: market_data_and_slot.data,
                                slot: update.data_and_slot.slot,
                            },
                        );
                    }
                });
        }
        Ok(())
    }

    pub async fn unsubscribe(&self) -> SdkResult<()> {
        if self.subscribed.get() {
            self.subscription.try_borrow_mut()?.unsubscribe().await?;
            self.subscribed.set(false);
            self.marketmap.clear();
            self.latest_slot.store(0, Ordering::Relaxed);
        }
        Ok(())
    }

    pub fn values(&self) -> Vec<T> {
        self.marketmap.iter().map(|x| x.data.clone()).collect()
    }

    pub fn oracles(&self) -> Vec<(Pubkey, OracleSource)> {
        self.values().iter().map(|x| x.oracle_info()).collect()
    }

    pub fn size(&self) -> usize {
        self.marketmap.len()
    }

    pub fn contains(&self, market_index: &u16) -> bool {
        self.marketmap.contains_key(market_index)
    }

    pub fn get(&self, market_index: &u16) -> Option<DataAndSlot<T>> {
        self.marketmap
            .get(market_index)
            .map(|market| market.clone())
    }

    pub(crate) async fn sync(&self) -> SdkResult<()> {
        if self.synced {
            return Ok(());
        }

        let sync_lock = self.sync_lock.as_ref().expect("expected sync lock");

        let lock = match sync_lock.try_lock() {
            Ok(lock) => lock,
            Err(_) => return Ok(()),
        };

        let options = self.subscription.try_borrow()?.options.clone();

        let account_config = RpcAccountInfoConfig {
            commitment: Some(self.commitment),
            encoding: Some(options.encoding),
            ..RpcAccountInfoConfig::default()
        };

        let gpa_config = RpcProgramAccountsConfig {
            filters: Some(options.filters),
            account_config,
            with_context: Some(true),
        };

        let response = self
            .rpc
            .send::<OptionalContext<Vec<RpcKeyedAccount>>>(
                RpcRequest::GetProgramAccounts,
                json!([drift::id().to_string(), gpa_config]),
            )
            .await?;

        if let OptionalContext::Context(accounts) = response {
            for account in accounts.value {
                let slot = accounts.context.slot;
                let market_data = account.account.data;
                let data = decode::<T>(market_data)?;
                self.marketmap
                    .insert(data.market_index(), DataAndSlot { data, slot });
            }

            self.latest_slot
                .store(accounts.context.slot, Ordering::Relaxed);
        }

        drop(lock);
        Ok(())
    }

    pub fn get_latest_slot(&self) -> u64 {
        self.latest_slot.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use crate::marketmap::MarketMap;
    use drift::state::perp_market::PerpMarket;
    use drift::state::spot_market::SpotMarket;
    use solana_sdk::commitment_config::CommitmentConfig;
    use solana_sdk::commitment_config::CommitmentLevel;

    #[tokio::test]
    #[cfg(rpc_tests)]
    async fn test_marketmap_perp() {
        let endpoint = "rpc".to_string();
        let commitment = CommitmentConfig {
            commitment: CommitmentLevel::Processed,
        };

        let marketmap = MarketMap::<PerpMarket>::new(commitment, endpoint, true);
        marketmap.subscribe().await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

        dbg!(marketmap.size());
        assert!(marketmap.size() == 28);

        dbg!(marketmap.get_latest_slot());

        marketmap.unsubscribe().await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

        assert_eq!(marketmap.size(), 0);
        assert_eq!(marketmap.subscribed.get(), false);
    }

    #[tokio::test]
    #[cfg(rpc_tests)]
    async fn test_marketmap_spot() {
        let endpoint = "rpc".to_string();
        let commitment = CommitmentConfig {
            commitment: CommitmentLevel::Processed,
        };

        let marketmap = MarketMap::<SpotMarket>::new(commitment, endpoint, true);
        marketmap.subscribe().await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

        dbg!(marketmap.size());
        assert!(marketmap.size() == 13);

        dbg!(marketmap.get_latest_slot());

        marketmap.unsubscribe().await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

        assert_eq!(marketmap.size(), 0);
        assert_eq!(marketmap.subscribed.get(), false);
    }
}
