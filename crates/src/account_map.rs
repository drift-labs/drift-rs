//! Hybrid solana account map backed by Ws or RPC polling
use std::{
    sync::{Arc, Mutex, RwLock},
    time::Duration,
};

use anchor_lang::AccountDeserialize;
use dashmap::DashMap;
use drift_pubsub_client::PubsubClient;
use log::debug;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{clock::Slot, commitment_config::CommitmentConfig, pubkey::Pubkey};

use crate::{
    polled_account_subscriber::PolledAccountSubscriber, types::DataAndSlot,
    websocket_account_subscriber::WebsocketAccountSubscriber, SdkResult, UnsubHandle,
};

const LOG_TARGET: &str = "accountmap";

#[derive(Clone, Default)]
pub struct AccountSlot {
    raw: Vec<u8>,
    slot: Slot,
}

/// Set of subscriptions to network accounts
///
/// Accounts are subscribed by either Ws or polling at fixed intervals
pub struct AccountMap {
    pubsub: Arc<PubsubClient>,
    rpc: Arc<RpcClient>,
    commitment: CommitmentConfig,
    inner: DashMap<Pubkey, AccountSub<Subscribed>, ahash::RandomState>,
}

impl AccountMap {
    pub fn new(
        pubsub: Arc<PubsubClient>,
        rpc: Arc<RpcClient>,
        commitment: CommitmentConfig,
    ) -> Self {
        Self {
            pubsub,
            rpc,
            commitment,
            inner: Default::default(),
        }
    }
    /// Subscribe account with Ws
    ///
    /// * `account` pubkey to subscribe
    ///
    pub async fn subscribe_account(&self, account: &Pubkey) -> SdkResult<()> {
        if self.inner.contains_key(account) {
            return Ok(());
        }
        debug!(target: LOG_TARGET, "subscribing: {account:?}");

        let user = AccountSub::new(Arc::clone(&self.pubsub), self.commitment, *account);
        let user = user.subscribe().await?;

        self.inner.insert(*account, user);

        Ok(())
    }
    /// Subscribe account with RPC polling
    ///
    /// * `account` pubkey to subscribe
    /// * `interval` to poll the account
    ///
    pub async fn subscribe_account_polled(
        &self,
        account: &Pubkey,
        interval: Option<Duration>,
    ) -> SdkResult<()> {
        if self.inner.contains_key(account) {
            return Ok(());
        }
        debug!(target: LOG_TARGET, "subscribing: {account:?} @ {interval:?}");

        let user = AccountSub::polled(Arc::clone(&self.rpc), *account, interval);
        let user = user.subscribe().await?;

        self.inner.insert(*account, user);

        Ok(())
    }
    /// Unsubscribe user account
    pub fn unsubscribe_account(&self, account: &Pubkey) {
        if let Some((acc, unsub)) = self.inner.remove(account) {
            debug!(target: LOG_TARGET, "unsubscribing: {acc:?}");
            let _ = unsub.unsubscribe();
        }
    }
    /// Return data of the given `account` as T, if it exists
    pub fn account_data<T: AccountDeserialize>(&self, account: &Pubkey) -> Option<T> {
        self.account_data_and_slot(account).map(|x| x.data)
    }
    /// Return data of the given `account` as T and slot, if it exists
    pub fn account_data_and_slot<T: AccountDeserialize>(
        &self,
        account: &Pubkey,
    ) -> Option<DataAndSlot<T>> {
        self.inner
            .get(account)
            .map(|u| u.get_account_data_and_slot())
    }
}

struct Subscribed {
    data_and_slot: Arc<RwLock<AccountSlot>>,
    unsub: Mutex<Option<UnsubHandle>>,
}
struct Unsubscribed;

/// A subscription to a solana account
pub struct AccountSub<S> {
    /// account pubkey
    pub pubkey: Pubkey,
    /// underlying subscription
    subscription: SubscriptionImpl,
    /// subscription state
    state: S,
}

impl AccountSub<Unsubscribed> {
    pub const SUBSCRIPTION_ID: &'static str = "account";

    /// Create a new Ws account subscriber
    pub fn new(pubsub: Arc<PubsubClient>, commitment: CommitmentConfig, pubkey: Pubkey) -> Self {
        let subscription = WebsocketAccountSubscriber::new(pubsub, pubkey, commitment);

        Self {
            pubkey,
            subscription: SubscriptionImpl::Ws(subscription),
            state: Unsubscribed {},
        }
    }

    /// Create a new polled account subscriber
    pub fn polled(rpc: Arc<RpcClient>, pubkey: Pubkey, interval: Option<Duration>) -> Self {
        let subscription =
            PolledAccountSubscriber::new(pubkey, interval.unwrap_or(Duration::from_secs(5)), rpc);

        Self {
            pubkey,
            subscription: SubscriptionImpl::Polled(subscription),
            state: Unsubscribed {},
        }
    }

    /// Start the subscriber task
    pub async fn subscribe(self) -> SdkResult<AccountSub<Subscribed>> {
        let data_and_slot = Arc::new(RwLock::new(AccountSlot::default()));

        let unsub = match self.subscription {
            SubscriptionImpl::Ws(ref ws) => {
                let data_and_slot = Arc::clone(&data_and_slot);
                ws.subscribe(Self::SUBSCRIPTION_ID, true, move |update| {
                    let mut guard = data_and_slot.write().expect("acquired");
                    guard.raw.clone_from(&update.data);
                    guard.slot = update.slot;
                })
                .await?
            }
            SubscriptionImpl::Polled(ref poll) => {
                let data_and_slot = Arc::clone(&data_and_slot);
                poll.subscribe(move |update| {
                    let mut guard = data_and_slot.write().expect("acquired");
                    guard.raw.clone_from(&update.data);
                    guard.slot = update.slot;
                })
            }
        };

        Ok(AccountSub {
            pubkey: self.pubkey,
            subscription: self.subscription,
            state: Subscribed {
                data_and_slot,
                unsub: Mutex::new(Some(unsub)),
            },
        })
    }
}

impl AccountSub<Subscribed> {
    /// Return the latest value of the account data along with last updated slot
    /// # Panics
    /// Panics if account data cannot be deserialized as `T`
    pub fn get_account_data_and_slot<T: AccountDeserialize>(&self) -> DataAndSlot<T> {
        let guard = self.state.data_and_slot.read().expect("acquired");
        DataAndSlot {
            slot: guard.slot,
            data: T::try_deserialize_unchecked(&mut guard.raw.as_slice()).expect("desrializes"),
        }
    }

    /// Stop the user subscriber task, if it exists
    pub fn unsubscribe(self) -> AccountSub<Unsubscribed> {
        let mut guard = self.state.unsub.lock().expect("acquire");
        if let Some(unsub) = guard.take() {
            if unsub.send(()).is_err() {
                log::error!("couldn't unsubscribe");
            }
        }

        AccountSub {
            pubkey: self.pubkey,
            subscription: self.subscription,
            state: Unsubscribed,
        }
    }
}

enum SubscriptionImpl {
    Ws(WebsocketAccountSubscriber),
    Polled(PolledAccountSubscriber),
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use solana_sdk::pubkey;

    use super::*;
    use crate::{
        accounts::User,
        constants::DEFAULT_PUBKEY,
        utils::{get_ws_url, test_envs::mainnet_endpoint},
        Wallet,
    };

    #[tokio::test]
    async fn test_user_subscribe() {
        let _ = env_logger::try_init();
        let pubsub = Arc::new(
            PubsubClient::new(&get_ws_url(&mainnet_endpoint()).unwrap())
                .await
                .expect("ws connects"),
        );
        let rpc = Arc::new(RpcClient::new(mainnet_endpoint()));
        let account_map = AccountMap::new(pubsub, rpc, CommitmentConfig::confirmed());
        let user_1 = Wallet::derive_user_account(
            &pubkey!("DxoRJ4f5XRMvXU9SGuM4ZziBFUxbhB3ubur5sVZEvue2"),
            0,
        );
        let user_2 = Wallet::derive_user_account(
            &pubkey!("Drift7AMLeq3FoKBMpT9wzqyMM3HVvvZFtsn81iSSkWV"),
            0,
        );

        let (res1, res2) = tokio::join!(
            account_map.subscribe_account(&user_1),
            account_map.subscribe_account(&user_2),
        );
        assert!(res1.and(res2).is_ok());

        let handle = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(5)).await;
            let account_data = account_map.account_data::<User>(&user_1);
            assert!(account_data.is_some_and(|x| x.authority != DEFAULT_PUBKEY));
            account_map.unsubscribe_account(&user_1);

            let account_data = account_map.account_data::<User>(&user_2);
            assert!(account_data.is_some_and(|x| x.authority != DEFAULT_PUBKEY));

            let account_data = account_map.account_data::<User>(&user_1);
            assert!(account_data.is_none());
        });

        assert!(handle.await.is_ok());
    }
}
