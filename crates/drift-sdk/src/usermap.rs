use std::{
    str::FromStr,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
};

use anchor_lang::AnchorDeserialize;
use dashmap::DashMap;
use serde_json::json;
use solana_account_decoder::UiAccountEncoding;
use solana_client::{
    nonblocking::rpc_client::RpcClient,
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    rpc_filter::RpcFilterType,
    rpc_request::RpcRequest,
    rpc_response::{OptionalContext, RpcKeyedAccount},
};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};

use crate::{
    constants,
    drift_abi::accounts::User,
    event_emitter::EventEmitter,
    memcmp::{get_non_idle_user_filter, get_user_filter},
    utils::{decode, get_ws_url},
    websocket_program_account_subscriber::{
        WebsocketProgramAccountOptions, WebsocketProgramAccountSubscriber,
    },
    SdkResult,
};

pub struct UserMap {
    subscribed: bool,
    subscription: WebsocketProgramAccountSubscriber<User>,
    pub(crate) usermap: Arc<DashMap<String, User>>,
    sync_lock: Option<Mutex<()>>,
    latest_slot: Arc<AtomicU64>,
    commitment: CommitmentConfig,
    rpc: RpcClient,
}

impl UserMap {
    pub const SUBSCRIPTION_ID: &'static str = "usermap";

    pub fn new(
        commitment: CommitmentConfig,
        endpoint: String,
        sync: bool,
        additional_filters: Option<Vec<RpcFilterType>>,
    ) -> Self {
        let mut filters = vec![get_user_filter(), get_non_idle_user_filter()];
        filters.extend(additional_filters.unwrap_or_default());
        let options = WebsocketProgramAccountOptions {
            filters,
            commitment,
            encoding: UiAccountEncoding::Base64,
        };
        let event_emitter = EventEmitter::new();

        let url = get_ws_url(&endpoint).unwrap();

        let subscription = WebsocketProgramAccountSubscriber::new(
            UserMap::SUBSCRIPTION_ID,
            url,
            options,
            event_emitter,
        );

        let usermap = Arc::new(DashMap::new());

        let rpc = RpcClient::new_with_commitment(endpoint.clone(), commitment);

        let sync_lock = if sync { Some(Mutex::new(())) } else { None };

        Self {
            subscribed: false,
            subscription,
            usermap,
            sync_lock,
            latest_slot: Arc::new(AtomicU64::new(0)),
            commitment,
            rpc,
        }
    }

    pub async fn subscribe(&mut self) -> SdkResult<()> {
        if self.sync_lock.is_some() {
            self.sync().await?;
        }

        if !self.subscribed {
            self.subscription.subscribe().await?;
            self.subscribed = true;

            let usermap = self.usermap.clone();
            let latest_slot = self.latest_slot.clone();

            self.subscription.event_emitter.subscribe(move |update| {
                if update.data_and_slot.slot > latest_slot.load(Ordering::Relaxed) {
                    latest_slot.store(update.data_and_slot.slot, Ordering::Relaxed);
                }
                usermap.insert(update.pubkey.clone(), update.data_and_slot.data);
            });
        }

        Ok(())
    }

    pub async fn unsubscribe(&mut self) -> SdkResult<()> {
        if self.subscribed {
            self.subscription.unsubscribe().await?;
            self.subscribed = false;
            self.usermap.clear();
            self.latest_slot.store(0, Ordering::Relaxed);
        }
        Ok(())
    }

    pub fn size(&self) -> usize {
        self.usermap.len()
    }

    pub fn contains(&self, pubkey: &str) -> bool {
        self.usermap.contains_key(pubkey)
    }

    pub fn get(&self, pubkey: &str) -> Option<User> {
        self.usermap.get(pubkey).map(|user| *user.value())
    }

    pub async fn must_get(&self, pubkey: &str) -> SdkResult<User> {
        if let Some(user) = self.get(pubkey) {
            Ok(user)
        } else {
            let user_data = self
                .rpc
                .get_account_data(&Pubkey::from_str(pubkey).unwrap())
                .await?;
            let user = User::deserialize(&mut user_data.as_slice()).unwrap();
            self.usermap.insert(pubkey.to_string(), user);
            Ok(self.get(pubkey).unwrap())
        }
    }

    #[allow(clippy::await_holding_lock)]
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
                let pubkey = account.pubkey;
                let user_data = account.account.data;
                let data = decode::<User>(&user_data)?;
                self.usermap.insert(pubkey, data);
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

    #[tokio::test]
    #[cfg(rpc_tests)]
    async fn test_usermap() {
        use solana_sdk::commitment_config::{CommitmentConfig, CommitmentLevel};

        use crate::usermap::UserMap;

        let endpoint = "rpc_url".to_string();
        let commitment = CommitmentConfig {
            commitment: CommitmentLevel::Processed,
        };

        let mut usermap = UserMap::new(commitment, endpoint, true);
        usermap.subscribe().await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;

        dbg!(usermap.size());
        assert!(usermap.size() > 50000);

        dbg!(usermap.get_latest_slot());

        usermap.unsubscribe().await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

        assert_eq!(usermap.size(), 0);
        assert_eq!(usermap.subscribed, false);
    }
}
