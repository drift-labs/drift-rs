use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use crate::event_emitter::EventEmitter;
use crate::memcmp::{get_market_filter, get_non_idle_user_filter, get_user_filter};
use crate::utils::{decode, get_ws_url};
use crate::websocket_program_account_subscriber::{
    ProgramAccountUpdate, WebsocketProgramAccountOptions, WebsocketProgramAccountSubscriber,
};
use crate::SdkResult;
use anchor_lang::AccountDeserialize;
use dashmap::DashMap;
use drift::state::user::MarketType;
use drift::state::perp_market::PerpMarket;
use drift::state::spot_market::SpotMarket;
use serde_json::json;
use solana_account_decoder::UiAccountEncoding;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use solana_client::rpc_request::RpcRequest;
use solana_client::rpc_response::{OptionalContext, RpcKeyedAccount};
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;

pub trait MarketIndex {
    fn market_index(&self) -> u16;
    fn market_type() -> MarketType;
}

impl MarketIndex for PerpMarket {
    fn market_index(&self) -> u16 {
        self.market_index
    }

    fn market_type() -> MarketType {
        MarketType::Perp
    }
}

impl MarketIndex for SpotMarket {
    fn market_index(&self) -> u16 {
        self.market_index
    }

    fn market_type() -> MarketType {
        MarketType::Spot
    }
}

pub struct MarketMap<T> {
    subscribed: bool,
    subscription: WebsocketProgramAccountSubscriber,
    pub(crate) marketmap: Arc<DashMap<u16, T>>,
    sync_lock: Option<Mutex<()>>,
    latest_slot: Arc<AtomicU64>,
    commitment: CommitmentConfig,
    rpc: RpcClient,
}

impl<T: AccountDeserialize + Clone + Send + Sync + MarketIndex + 'static> MarketMap<T> {
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
            subscribed: false,
            subscription,
            marketmap,
            sync_lock,
            latest_slot: Arc::new(AtomicU64::new(0)),
            commitment,
            rpc,
        }
    }

    pub async fn subscribe(&mut self) -> SdkResult<()> {
        if let Some(_) = self.sync_lock {
            self.sync().await?;
        }

        if !self.subscribed {
            self.subscription.subscribe::<T>().await?;
            self.subscribed = true;
        }

        let marketmap = self.marketmap.clone();
        let latest_slot = self.latest_slot.clone();

        self.subscription
            .event_emitter
            .subscribe("marketmap", move |event| {
                if let Some(update) = event.as_any().downcast_ref::<ProgramAccountUpdate<T>>() {
                    let market_data_and_slot = update.data_and_slot.clone();
                    if update.data_and_slot.slot > latest_slot.load(Ordering::Relaxed) {
                        latest_slot.store(update.data_and_slot.slot, Ordering::Relaxed);
                    }
                    marketmap.insert(update.data_and_slot.clone().data.market_index(), market_data_and_slot.data);
                }
            });

        Ok(())
    }

    pub async fn unsubscribe(&mut self) -> SdkResult<()> {
        if self.subscribed {
            self.subscription.unsubscribe().await?;
            self.subscribed = false;
            self.marketmap.clear();
            self.latest_slot.store(0, Ordering::Relaxed);
        }
        Ok(())
    }

    pub fn size(&self) -> usize {
        self.marketmap.len()
    }

    pub fn contains(&self, market_index: &u16) -> bool {
        self.marketmap.contains_key(market_index)
    }

    pub fn get(&self, market_index: &u16) -> Option<T> {
        self.marketmap.get(market_index).map(|market| market.value().clone())
    }

    async fn sync(&mut self) -> SdkResult<()> {
        let sync_lock = self.sync_lock.as_ref().expect("expected sync lock");

        let lock = match sync_lock.try_lock() {
            Ok(lock) => lock,
            Err(_) => return Ok(()),
        };

        let account_config = RpcAccountInfoConfig {
            commitment: Some(self.commitment),
            encoding: Some(self.subscription.options.encoding),
            ..RpcAccountInfoConfig::default()
        };

        let gpa_config = RpcProgramAccountsConfig {
            filters: Some(self.subscription.options.filters.clone()),
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
                let market_data = account.account.data;
                let data = decode::<T>(market_data)?;
                self.marketmap.insert(data.market_index(), data);
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

        let mut marketmap = MarketMap::<PerpMarket>::new(commitment, endpoint, true);
        marketmap.subscribe().await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

        dbg!(marketmap.size());
        assert!(marketmap.size() == 28);

        dbg!(marketmap.get_latest_slot());

        marketmap.unsubscribe().await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

        assert_eq!(marketmap.size(), 0);
        assert_eq!(marketmap.subscribed, false);
    }

    #[tokio::test]
    #[cfg(rpc_tests)]
    async fn test_marketmap_spot() {
        let endpoint = "rpc".to_string();
        let commitment = CommitmentConfig {
            commitment: CommitmentLevel::Processed,
        };

        let mut marketmap = MarketMap::<SpotMarket>::new(commitment, endpoint, true);
        marketmap.subscribe().await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

        dbg!(marketmap.size());
        assert!(marketmap.size() == 13);

        dbg!(marketmap.get_latest_slot());

        marketmap.unsubscribe().await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

        assert_eq!(marketmap.size(), 0);
        assert_eq!(marketmap.subscribed, false);
    }
}
