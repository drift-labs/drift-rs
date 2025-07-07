//! Hybrid solana account map backed by Ws or RPC polling
use std::{
    ops::Deref,
    sync::{Arc, Mutex},
    time::Duration,
};

use bytemuck::Pod;
use dashmap::DashMap;
use drift_pubsub_client::PubsubClient;
use log::debug;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{clock::Slot, commitment_config::CommitmentConfig, pubkey::Pubkey};

use crate::{
    grpc::AccountUpdate,
    polled_account_subscriber::PolledAccountSubscriber,
    types::{DataAndSlot, EMPTY_ACCOUNT_CALLBACK},
    websocket_account_subscriber::WebsocketAccountSubscriber,
    SdkResult, UnsubHandle,
};

const LOG_TARGET: &str = "accountmap";

#[derive(Clone, Default)]
pub struct AccountSlot {
    raw: Arc<[u8]>,
    slot: Slot,
}

/// Set of subscriptions to network accounts
///
/// Accounts are subscribed by either Ws or polling at fixed intervals
pub struct AccountMap {
    pubsub: Arc<PubsubClient>,
    rpc: Arc<RpcClient>,
    commitment: CommitmentConfig,
    inner: Arc<DashMap<Pubkey, AccountSlot, ahash::RandomState>>,
    subscriptions: Arc<DashMap<Pubkey, AccountSub<Subscribed>, ahash::RandomState>>,
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
            inner: Arc::default(),
            subscriptions: Arc::default(),
        }
    }
    /// Subscribe account with Ws
    ///
    /// * `account` pubkey to subscribe
    ///
    pub async fn subscribe_account(&self, account: &Pubkey) -> SdkResult<()> {
        self.subscribe_account_inner(account, EMPTY_ACCOUNT_CALLBACK)
            .await
    }

    /// Subscribe account with Ws callback
    ///
    /// * `account` pubkey to subscribe
    /// * `on_account` callback function
    ///
    pub async fn subscribe_account_with_callback<F>(
        &self,
        account: &Pubkey,
        on_account: F,
    ) -> SdkResult<()>
    where
        F: Fn(&crate::AccountUpdate) + Send + Sync + 'static + Clone,
    {
        self.subscribe_account_inner(account, on_account).await
    }

    /// Subscribe account with Ws - inner implementation
    ///
    /// * `account` pubkey to subscribe
    /// * `on_account` callback function
    ///
    async fn subscribe_account_inner<F>(&self, account: &Pubkey, on_account: F) -> SdkResult<()>
    where
        F: Fn(&crate::AccountUpdate) + Send + Sync + 'static + Clone,
    {
        if self.inner.contains_key(account) {
            return Ok(());
        }
        debug!(target: LOG_TARGET, "subscribing: {account:?}");

        let user = AccountSub::new(Arc::clone(&self.pubsub), self.commitment, *account);
        let sub = user.subscribe(Arc::clone(&self.inner), on_account).await?;
        self.subscriptions.insert(*account, sub);

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
        self.subscribe_account_polled_inner(account, interval, EMPTY_ACCOUNT_CALLBACK)
            .await
    }

    pub async fn subscribe_account_polled_with_callback<F>(
        &self,
        account: &Pubkey,
        interval: Option<Duration>,
        on_account: F,
    ) -> SdkResult<()>
    where
        F: Fn(&crate::AccountUpdate) + Send + Sync + 'static + Clone,
    {
        self.subscribe_account_polled_inner(account, interval, on_account)
            .await
    }

    /// Subscribe account with RPC polling - inner implementation
    ///
    /// * `account` pubkey to subscribe
    /// * `interval` to poll the account
    /// * `on_account` callback function
    ///
    async fn subscribe_account_polled_inner<F>(
        &self,
        account: &Pubkey,
        interval: Option<Duration>,
        on_account: F,
    ) -> SdkResult<()>
    where
        F: Fn(&crate::AccountUpdate) + Send + Sync + 'static + Clone,
    {
        if self.inner.contains_key(account) {
            return Ok(());
        }
        debug!(
            target: LOG_TARGET,
            "subscribing: {account:?} @ {interval:?}"
        );

        let user = AccountSub::polled(Arc::clone(&self.rpc), *account, interval);
        let sub = user.subscribe(Arc::clone(&self.inner), on_account).await?;
        self.subscriptions.insert(*account, sub);

        Ok(())
    }

    /// On account update callback for gRPC hook
    pub fn on_account_fn(&self) -> impl Fn(&AccountUpdate) {
        let accounts = Arc::clone(&self.inner);
        let subscriptions = Arc::clone(&self.subscriptions);
        move |update| {
            accounts
                .entry(update.pubkey)
                .and_modify(|x| {
                    x.slot = update.slot;
                    x.raw = Arc::from(&update.data[8..]);
                    if update.lamports == 0 {
                        accounts.remove(&update.pubkey);
                    }
                })
                .or_insert({
                    subscriptions.insert(
                        update.pubkey,
                        AccountSub {
                            pubkey: update.pubkey,
                            subscription: SubscriptionImpl::Grpc,
                            state: Subscribed {
                                unsub: Mutex::default(),
                            },
                        },
                    );
                    AccountSlot {
                        slot: update.slot,
                        raw: Arc::from(&update.data[8..]),
                    }
                });
        }
    }
    /// Unsubscribe user account
    pub fn unsubscribe_account(&self, account: &Pubkey) {
        if let Some((acc, sub)) = self.subscriptions.remove(account) {
            debug!(target: LOG_TARGET, "unsubscribing: {acc:?}");
            self.inner.remove(account);
            let _ = sub.unsubscribe();
        }
    }
    /// Return data of the given `account` as T, if it exists
    pub fn account_data<T: Pod>(&self, account: &Pubkey) -> Option<T> {
        self.account_data_and_slot(account).map(|x| x.data)
    }
    /// Return data of the given `account` as T and slot, if it exists
    pub fn account_data_and_slot<T: Pod>(&self, account: &Pubkey) -> Option<DataAndSlot<T>> {
        self.inner.get(account).map(|x| {
            let arc = x.raw.clone();
            DataAndSlot {
                slot: x.slot,
                data: *AccountRef {
                    arc,
                    _marker: std::marker::PhantomData,
                },
            }
        })
    }
}

struct Subscribed {
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
    pub async fn subscribe<F>(
        self,
        accounts: Arc<DashMap<Pubkey, AccountSlot, ahash::RandomState>>,
        on_account: F,
    ) -> SdkResult<AccountSub<Subscribed>>
    where
        F: Fn(&crate::AccountUpdate) + Send + Sync + 'static + Clone,
    {
        let unsub = match self.subscription {
            SubscriptionImpl::Ws(ref ws) => {
                let on_account = on_account.clone();
                let unsub = ws
                    .subscribe(Self::SUBSCRIPTION_ID, true, move |update| {
                        accounts
                            .entry(update.pubkey)
                            .and_modify(|x| {
                                x.slot = update.slot;
                                x.raw = Arc::from(&update.data[8..]);
                                if update.lamports == 0 {
                                    accounts.remove(&update.pubkey);
                                }
                            })
                            .or_insert(AccountSlot {
                                raw: Arc::from(&update.data[8..]),
                                slot: update.slot,
                            });

                        on_account(&update);
                    })
                    .await?;
                Some(unsub)
            }
            SubscriptionImpl::Polled(ref poll) => {
                let on_account = on_account.clone();
                let unsub = poll.subscribe(move |update| {
                    accounts
                        .entry(update.pubkey)
                        .and_modify(|x| {
                            x.slot = update.slot;
                            x.raw = Arc::from(&update.data[8..]);
                            if update.lamports == 0 {
                                accounts.remove(&update.pubkey);
                            }
                        })
                        .or_insert(AccountSlot {
                            raw: Arc::from(&update.data[8..]),
                            slot: update.slot,
                        });

                    on_account(update);
                });
                Some(unsub)
            }
            SubscriptionImpl::Grpc => None,
        };

        Ok(AccountSub {
            pubkey: self.pubkey,
            subscription: self.subscription,
            state: Subscribed {
                unsub: Mutex::new(unsub),
            },
        })
    }
}

impl AccountSub<Subscribed> {
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
    Grpc,
}

/// Wrapper type that keeps the Arc alive and provides access to T
pub struct AccountRef<T> {
    arc: Arc<[u8]>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: Pod> Deref for AccountRef<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        bytemuck::from_bytes(&self.arc)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use solana_sdk::pubkey;

    use super::*;
    use crate::{
        accounts::User,
        constants::{state_account, DEFAULT_PUBKEY},
        types::accounts::State,
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

        let (res1, res2, res3) = tokio::join!(
            account_map.subscribe_account(&user_1),
            account_map.subscribe_account(&user_2),
            account_map.subscribe_account_polled(state_account(), Some(Duration::from_secs(2))),
        );
        assert!(res1.and(res2).and(res3).is_ok());

        let handle = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(8)).await;
            let account_data = account_map.account_data::<User>(&user_1);
            assert!(account_data.is_some_and(|x| x.authority != DEFAULT_PUBKEY));
            account_map.unsubscribe_account(&user_1);

            let account_data = account_map.account_data::<User>(&user_1);
            assert!(account_data.is_none());

            let account_data = account_map.account_data::<User>(&user_2);
            assert!(account_data.is_some_and(|x| x.authority != DEFAULT_PUBKEY));

            let state_account = account_map.account_data::<State>(state_account());
            assert!(state_account.is_some());
        });

        assert!(handle.await.is_ok());
    }
}
