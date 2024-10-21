use std::sync::{Arc, Mutex, RwLock};

use anchor_lang::AccountDeserialize;
use fnv::FnvHashMap;
use solana_sdk::{clock::Slot, commitment_config::CommitmentConfig, pubkey::Pubkey};

use crate::{
    utils::get_ws_url, websocket_account_subscriber::WebsocketAccountSubscriber, SdkResult,
    UnsubHandle,
};

#[derive(Clone, Default)]
pub struct AccountSlot {
    raw: Vec<u8>,
    slot: Slot,
}

pub struct DataAndSlot<T> {
    pub data: T,
    pub slot: Slot,
}

/// Set of subscriptions to a dynamic subset of network accounts
pub struct AccountMap {
    endpoint: String,
    commitment: CommitmentConfig,
    inner: RwLock<FnvHashMap<Pubkey, AccountSub<Subscribed>>>,
}

impl AccountMap {
    pub fn new(endpoint: String, commitment: CommitmentConfig) -> Self {
        Self {
            endpoint,
            commitment,
            inner: Default::default(),
        }
    }
    /// Subscribe user account
    pub async fn subscribe_account(&self, account: &Pubkey) -> SdkResult<()> {
        {
            let map = self.inner.read().expect("acquired");
            if map.contains_key(account) {
                return Ok(());
            }
        }

        let user = AccountSub::new(&self.endpoint, self.commitment, *account);
        let user = user.subscribe().await?;

        let mut map = self.inner.write().expect("acquired");
        map.insert(*account, user);

        Ok(())
    }
    /// Unsubscribe user account
    pub fn unsubscribe_account(&self, account: &Pubkey) {
        let mut map = self.inner.write().expect("acquired");
        if let Some(u) = map.remove(account) {
            let _ = u.unsubscribe();
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
        let accounts = self.inner.read().expect("read");
        accounts.get(account).map(|u| u.get_account_data_and_slot())
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
    /// underlying Ws subscription
    subscription: WebsocketAccountSubscriber,
    /// subscription state
    state: S,
}

impl AccountSub<Unsubscribed> {
    pub const SUBSCRIPTION_ID: &'static str = "account";

    pub fn new(endpoint: &str, commitment: CommitmentConfig, pubkey: Pubkey) -> Self {
        let subscription = WebsocketAccountSubscriber::new(
            get_ws_url(endpoint).expect("valid url"),
            pubkey,
            commitment,
        );

        Self {
            pubkey,
            subscription,
            state: Unsubscribed {},
        }
    }

    /// Start the subscriber task
    pub async fn subscribe(self) -> SdkResult<AccountSub<Subscribed>> {
        let data_and_slot = Arc::new(RwLock::new(AccountSlot::default()));
        let unsub = self
            .subscription
            .subscribe(Self::SUBSCRIPTION_ID, {
                let data_and_slot = Arc::clone(&data_and_slot);
                move |update| {
                    let mut guard = data_and_slot.write().expect("acquired");
                    guard.raw.clone_from(&update.data);
                    guard.slot = update.slot;
                }
            })
            .await?;

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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use solana_sdk::pubkey;

    use super::*;
    use crate::{accounts::User, constants::DEFAULT_PUBKEY, utils::test_envs::mainnet_endpoint, Wallet};

    #[tokio::test]
    async fn test_user_subscribe() {
        let _ = env_logger::try_init();
        let account_map = AccountMap::new(
            mainnet_endpoint().into(),
            CommitmentConfig::confirmed(),
        );
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
