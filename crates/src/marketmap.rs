use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc, Mutex,
};

use anchor_lang::{AccountDeserialize, AnchorDeserialize};
use dashmap::DashMap;
use futures_util::{stream::FuturesUnordered, StreamExt};
use serde_json::json;
use solana_account_decoder::UiAccountEncoding;
use solana_client::{
    nonblocking::rpc_client::RpcClient,
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    rpc_request::RpcRequest,
    rpc_response::{OptionalContext, RpcKeyedAccount},
};
use solana_sdk::{clock::Slot, commitment_config::CommitmentConfig, pubkey::Pubkey};

use crate::{
    accounts::State,
    constants::{self, derive_perp_market_account, derive_spot_market_account, state_account},
    drift_idl::types::{MarketType, OracleSource},
    memcmp::get_market_filter,
    utils::get_ws_url,
    websocket_program_account_subscriber::{
        ProgramAccountUpdate, WebsocketProgramAccountOptions, WebsocketProgramAccountSubscriber,
    },
    DataAndSlot, PerpMarket, SdkError, SdkResult, SpotMarket, UnsubHandle,
};

const LOG_TARGET: &str = "marketmap";

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
            rpc,
            unsub: Mutex::default(),
        }
    }

    pub async fn subscribe(&self) -> SdkResult<()> {
        log::debug!(target: LOG_TARGET, "subscribing: {:?}", T::MARKET_TYPE);
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
        log::debug!(target: LOG_TARGET, "subscribed: {:?}", T::MARKET_TYPE);

        Ok(())
    }

    pub fn unsubscribe(&self) -> SdkResult<()> {
        log::debug!(target: LOG_TARGET, "unsubscribing: {:?}", T::MARKET_TYPE);
        let mut guard = self.unsub.lock().expect("uncontested");
        if let Some(unsub) = guard.take() {
            if unsub.send(()).is_err() {
                log::error!("couldn't unsubscribe");
            }
            self.marketmap.clear();
            self.latest_slot.store(0, Ordering::Relaxed);
        }
        log::debug!(target: LOG_TARGET, "unsubscribed: {:?}", T::MARKET_TYPE);

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

        log::debug!(target: LOG_TARGET, "syncing marketmap: {:?}", T::MARKET_TYPE);
        let (markets, latest_slot) = get_market_accounts_with_fallback::<T>(&self.rpc).await?;
        for market in markets {
            self.marketmap.insert(
                market.market_index(),
                DataAndSlot {
                    data: market,
                    slot: latest_slot,
                },
            );
        }
        self.latest_slot.store(latest_slot, Ordering::Relaxed);

        drop(lock);
        log::debug!(target: LOG_TARGET, "synced marketmap: {:?}", T::MARKET_TYPE);
        Ok(())
    }

    pub fn get_latest_slot(&self) -> u64 {
        self.latest_slot.load(Ordering::Relaxed)
    }
}

/// Fetch all market (program) accounts with multiple fallbacks
///
/// Tries progressively less intensive RPC methods for wider compatiblity with RPC providers:
///     getProgramAccounts, getMultipleAccounts, latstly multiple getAccountInfo
///
/// Returns deserialized accounts and retreived slot
async fn get_market_accounts_with_fallback<T: Market + AnchorDeserialize>(
    rpc: &RpcClient,
) -> SdkResult<(Vec<T>, Slot)> {
    let mut markets = Vec::<T>::default();

    let account_config = RpcAccountInfoConfig {
        commitment: Some(rpc.commitment()),
        encoding: Some(UiAccountEncoding::Base64Zstd),
        ..RpcAccountInfoConfig::default()
    };

    let gpa_config = RpcProgramAccountsConfig {
        filters: Some(vec![get_market_filter(T::MARKET_TYPE)]),
        account_config: account_config.clone(),
        with_context: Some(true),
        sort_results: None,
    };

    // try 'getProgramAccounts'
    let response: Result<OptionalContext<Vec<RpcKeyedAccount>>, _> = rpc
        .send(
            RpcRequest::GetProgramAccounts,
            json!([constants::PROGRAM_ID.to_string(), gpa_config]),
        )
        .await;

    if let Ok(OptionalContext::Context(accounts)) = response {
        for account in accounts.value {
            let market_data = account.account.data.decode().expect("Market data");
            let data = T::deserialize(&mut &market_data[8..]).expect("deserializes Market");
            markets.push(data);
        }
        return Ok((markets, accounts.context.slot));
    }
    log::debug!(target: LOG_TARGET, "syncing with getProgramAccounts failed: {:?}", T::MARKET_TYPE);

    let state_response = rpc
        .get_account_with_config(state_account(), account_config)
        .await
        .expect("state account fetch");

    let state_data = state_response.value.expect("state has data").data;
    let state =
        State::try_deserialize_unchecked(&mut state_data.as_slice()).expect("state deserializes");

    let market_pdas: Vec<Pubkey> = match T::MARKET_TYPE {
        MarketType::Spot => (0..state.number_of_spot_markets)
            .map(derive_spot_market_account)
            .collect(),
        MarketType::Perp => (0..state.number_of_markets)
            .map(derive_perp_market_account)
            .collect(),
    };

    // try 'getMultipleAccounts'
    let market_respones = rpc
        .get_multiple_accounts_with_commitment(market_pdas.as_slice(), rpc.commitment())
        .await;
    if let Ok(response) = market_respones {
        for account in response.value {
            match account {
                Some(account) => {
                    markets.push(
                        T::deserialize(&mut &account.data.as_slice()[8..])
                            .expect("market deserializes"),
                    );
                }
                None => {
                    log::warn!("failed to fetch market account");
                    return Err(SdkError::InvalidAccount)?;
                }
            }
        }
        return Ok((markets, response.context.slot));
    }
    log::debug!(target: LOG_TARGET, "syncing with getMultipleAccounts failed: {:?}", T::MARKET_TYPE);

    // try multiple 'getAccount's
    let mut market_requests =
        FuturesUnordered::from_iter(market_pdas.iter().map(|acc| rpc.get_account_data(acc)));

    while let Some(market_repsonse) = market_requests.next().await {
        match market_repsonse {
            Ok(data) => {
                markets
                    .push(T::deserialize(&mut &data.as_slice()[8..]).expect("market deserializes"));
            }
            Err(err) => {
                log::warn!("failed to fetch market account: {err:?}");
                return Err(err)?;
            }
        }
    }

    Ok((markets, state_response.context.slot))
}

#[cfg(test)]
mod tests {
    use solana_client::nonblocking::rpc_client::RpcClient;

    use super::get_market_accounts_with_fallback;
    use crate::{accounts::PerpMarket, utils::test_envs::devnet_endpoint};

    #[tokio::test]
    async fn get_market_accounts_with_fallback_works() {
        let result =
            get_market_accounts_with_fallback::<PerpMarket>(&RpcClient::new(devnet_endpoint()))
                .await;

        assert!(result.is_ok_and(|r| r.0.len() > 0 && r.1 > 0));
    }
}

#[cfg(feature = "rpc_tests")]
mod rpc_tests {
    use solana_sdk::commitment_config::CommitmentLevel;

    use super::*;
    use crate::utils::test_envs::mainnet_endpoint;

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
