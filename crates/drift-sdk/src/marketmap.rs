use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc, Mutex,
};

use anchor_lang::AnchorDeserialize;
use dashmap::DashMap;
use serde_json::json;
use solana_account_decoder::UiAccountEncoding;
use solana_client::{
    nonblocking::rpc_client::RpcClient,
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    rpc_request::RpcRequest,
    rpc_response::{OptionalContext, RpcKeyedAccount},
};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use tokio::sync::RwLock;

use crate::{
    constants,
    drift_abi::types::{MarketType, OracleSource},
    event_emitter::EventEmitter,
    memcmp::get_market_filter,
    utils::{decode, get_ws_url},
    websocket_program_account_subscriber::{
        WebsocketProgramAccountOptions, WebsocketProgramAccountSubscriber,
    },
    DataAndSlot, PerpMarket, SdkResult, SpotMarket,
};

pub trait Market {
    const MARKET_TYPE: MarketType;
    fn market_index(&self) -> u16;
    fn oracle_info(&self) -> (u16, Pubkey, OracleSource);
}

impl Market for PerpMarket {
    const MARKET_TYPE: MarketType = MarketType::Perp;

    fn market_index(&self) -> u16 {
        self.market_index
    }

    fn oracle_info(&self) -> (u16, Pubkey, OracleSource) {
        (self.market_index(), self.amm.oracle, self.amm.oracle_source)
    }
}

impl Market for SpotMarket {
    const MARKET_TYPE: MarketType = MarketType::Spot;

    fn market_index(&self) -> u16 {
        self.market_index
    }

    fn oracle_info(&self) -> (u16, Pubkey, OracleSource) {
        (self.market_index(), self.oracle, self.oracle_source)
    }
}

pub struct MarketMap<T: AnchorDeserialize + Send> {
    subscribed: AtomicBool,
    subscription: RwLock<WebsocketProgramAccountSubscriber<T>>,
    marketmap: Arc<DashMap<u16, DataAndSlot<T>>>,
    sync_lock: Option<Mutex<()>>,
    latest_slot: Arc<AtomicU64>,
    commitment: CommitmentConfig,
    rpc: RpcClient,
    synced: bool,
}

impl<T> MarketMap<T>
where
    T: AnchorDeserialize + Clone + Send + Sync + Market + 'static,
{
    pub const SUBSCRIPTION_ID: &'static str = "marketmap";

    pub fn new(commitment: CommitmentConfig, endpoint: String, sync: bool) -> Self {
        let filters = vec![get_market_filter(T::MARKET_TYPE)];
        let options = WebsocketProgramAccountOptions {
            filters,
            commitment,
            encoding: UiAccountEncoding::Base64,
        };
        let event_emitter = EventEmitter::new();

        let url = get_ws_url(&endpoint.clone()).unwrap();

        let subscription = WebsocketProgramAccountSubscriber::new(
            MarketMap::<T>::SUBSCRIPTION_ID,
            url,
            options,
            event_emitter,
        );

        let marketmap = Arc::new(DashMap::new());

        let rpc = RpcClient::new_with_commitment(endpoint.clone(), commitment);

        let sync_lock = if sync { Some(Mutex::new(())) } else { None };

        Self {
            subscribed: AtomicBool::new(false),
            subscription: RwLock::new(subscription),
            marketmap,
            sync_lock,
            latest_slot: Arc::new(AtomicU64::new(0)),
            commitment,
            rpc,
            synced: false,
        }
    }

    pub async fn subscribe(&self) -> SdkResult<()> {
        if self.sync_lock.is_some() {
            self.sync().await?;
        }

        if !self.subscribed.load(Ordering::Relaxed) {
            let mut subscription_writer = self.subscription.write().await;

            subscription_writer.subscribe().await?;
            self.subscribed.store(true, Ordering::Relaxed);

            let marketmap = Arc::clone(&self.marketmap);
            let latest_slot = self.latest_slot.clone();

            subscription_writer.event_emitter.subscribe(move |update| {
                if update.data_and_slot.slot > latest_slot.load(Ordering::Relaxed) {
                    latest_slot.store(update.data_and_slot.slot, Ordering::Relaxed);
                }
                marketmap.insert(
                    update.data_and_slot.data.market_index(),
                    update.data_and_slot.clone(),
                );
            });

            drop(subscription_writer)
        }
        Ok(())
    }

    pub async fn unsubscribe(&self) -> SdkResult<()> {
        if self.subscribed.load(Ordering::Relaxed) {
            self.subscription.write().await.unsubscribe().await?;
            self.subscribed.store(false, Ordering::Relaxed);
            self.marketmap.clear();
            self.latest_slot.store(0, Ordering::Relaxed);
        }
        Ok(())
    }

    pub fn values(&self) -> Vec<T> {
        self.marketmap.iter().map(|x| x.data.clone()).collect()
    }

    pub fn oracles(&self) -> Vec<(u16, Pubkey, OracleSource)> {
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

    #[allow(clippy::await_holding_lock)]
    pub(crate) async fn sync(&self) -> SdkResult<()> {
        if self.synced {
            return Ok(());
        }

        let sync_lock = self.sync_lock.as_ref().expect("expected sync lock");

        let lock = match sync_lock.try_lock() {
            Ok(lock) => lock,
            Err(_) => return Ok(()),
        };

        let subscription_reader = self.subscription.read().await;
        let options = subscription_reader.options.clone();
        drop(subscription_reader);

        let account_config = RpcAccountInfoConfig {
            commitment: Some(self.commitment),
            encoding: Some(options.encoding),
            ..RpcAccountInfoConfig::default()
        };

        let gpa_config = RpcProgramAccountsConfig {
            filters: Some(options.filters),
            account_config,
            with_context: Some(true),
            sort_results: None,
        };

        let response = self
            .rpc
            .send::<OptionalContext<Vec<RpcKeyedAccount>>>(
                RpcRequest::GetProgramAccounts,
                json!([constants::PROGRAM_ID.to_string(), gpa_config]),
            )
            .await?;

        if let OptionalContext::Context(accounts) = response {
            for account in accounts.value {
                let slot = accounts.context.slot;
                let market_data = account.account.data;
                let data = decode::<T>(&market_data)?;
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
    use solana_sdk::commitment_config::{CommitmentConfig, CommitmentLevel};

    use super::*;
    use crate::{
        drift_abi::accounts::{PerpMarket, SpotMarket},
        marketmap::MarketMap,
    };

    #[tokio::test]
    #[cfg(feature = "rpc_tests")]
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
        assert_eq!(marketmap.subscribed.load(Ordering::Relaxed), false);
    }

    #[tokio::test]
    #[cfg(feature = "rpc_tests")]
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
