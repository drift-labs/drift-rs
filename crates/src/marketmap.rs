use std::sync::{
    atomic::{AtomicU64, Ordering},
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

use crate::{
    constants,
    drift_idl::types::{MarketType, OracleSource},
    memcmp::get_market_filter,
    utils::get_ws_url,
    websocket_program_account_subscriber::{
        ProgramAccountUpdate, WebsocketProgramAccountOptions, WebsocketProgramAccountSubscriber,
    },
    DataAndSlot, PerpMarket, SdkResult, SpotMarket, UnsubHandle,
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
    subscription: WebsocketProgramAccountSubscriber,
    marketmap: Arc<DashMap<u16, DataAndSlot<T>>>,
    sync_lock: Option<Mutex<()>>,
    latest_slot: Arc<AtomicU64>,
    commitment: CommitmentConfig,
    rpc: RpcClient,
    unsub: Mutex<Option<UnsubHandle>>,
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
            encoding: UiAccountEncoding::Base64Zstd,
        };

        let url = get_ws_url(&endpoint.clone()).unwrap();
        let subscription = WebsocketProgramAccountSubscriber::new(url, options);
        let marketmap = Arc::new(DashMap::new());
        let rpc = RpcClient::new_with_commitment(endpoint.clone(), commitment);
        let sync_lock = if sync { Some(Mutex::new(())) } else { None };

        Self {
            subscription,
            marketmap,
            sync_lock,
            latest_slot: Arc::new(AtomicU64::new(0)),
            commitment,
            rpc,
            unsub: Mutex::default(),
        }
    }

    pub async fn subscribe(&self) -> SdkResult<()> {
        if self.sync_lock.is_some() {
            self.sync().await?;
        }

        let unsub = self.subscription.subscribe(Self::SUBSCRIPTION_ID, {
            let marketmap = Arc::clone(&self.marketmap);
            let latest_slot = self.latest_slot.clone();
            move |update: &ProgramAccountUpdate<T>| {
                if update.data_and_slot.slot > latest_slot.load(Ordering::Relaxed) {
                    latest_slot.store(update.data_and_slot.slot, Ordering::Relaxed);
                }
                marketmap.insert(
                    update.data_and_slot.data.market_index(),
                    update.data_and_slot.clone(),
                );
            }
        });
        let mut guard = self.unsub.lock().unwrap();
        *guard = Some(unsub);

        Ok(())
    }

    pub fn unsubscribe(&self) -> SdkResult<()> {
        let mut guard = self.unsub.lock().expect("uncontested");
        if let Some(unsub) = guard.take() {
            if unsub.send(()).is_err() {
                log::error!("couldn't unsubscribe");
            }
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
        if self.unsub.lock().unwrap().is_some() {
            return Ok(());
        }

        let sync_lock = self.sync_lock.as_ref().expect("expected sync lock");

        let lock = match sync_lock.try_lock() {
            Ok(lock) => lock,
            Err(_) => return Ok(()),
        };

        let options = self.subscription.options.clone();
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

        let response: OptionalContext<Vec<RpcKeyedAccount>> = self
            .rpc
            .send(
                RpcRequest::GetProgramAccounts,
                json!([constants::PROGRAM_ID.to_string(), gpa_config]),
            )
            .await?;

        if let OptionalContext::Context(accounts) = response {
            for account in accounts.value {
                let slot = accounts.context.slot;
                let market_data = account.account.data.decode().expect("Market data");
                let data: T = AnchorDeserialize::deserialize(&mut &market_data[8..])
                    .expect("deserializes Market");
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

#[cfg(feature = "rpc_tests")]
mod tests {
    use solana_sdk::commitment_config::CommitmentLevel;

    use super::*;
    use crate::utils::envs::mainnet_endpoint;

    #[tokio::test]
    async fn test_marketmap_perp() {
        let commitment = CommitmentConfig {
            commitment: CommitmentLevel::Processed,
        };

        let marketmap = MarketMap::<PerpMarket>::new(commitment, mainnet_endpoint(), true);
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
    async fn test_marketmap_spot() {
        let commitment = CommitmentConfig {
            commitment: CommitmentLevel::Processed,
        };

        let marketmap = MarketMap::<SpotMarket>::new(commitment, RPC, true);
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
