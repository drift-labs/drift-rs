use std::sync::{Arc, Mutex, RwLock};

use anchor_lang::AccountDeserialize;
use fnv::FnvHashMap;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};

use crate::{
    drift_idl::accounts::User, utils::get_ws_url,
    websocket_account_subscriber::WebsocketAccountSubscriber,
    websocket_program_account_subscriber::UnsubHandle, DataAndSlot, SdkResult,
};

/// Subscribes to a dynamic subset of User accounts
pub struct UserMap {
    endpoint: String,
    commitment: CommitmentConfig,
    inner: RwLock<FnvHashMap<Pubkey, DriftUser<Subscribed>>>,
}

impl UserMap {
    pub fn new(endpoint: String, commitment: CommitmentConfig) -> Self {
        Self {
            endpoint,
            commitment,
            inner: Default::default(),
        }
    }
    /// Subscribe user account
    pub async fn subscribe_user(&self, account: &Pubkey) -> SdkResult<()> {
        {
            let map = self.inner.read().expect("acquired");
            if map.contains_key(account) {
                return Ok(());
            }
        }

        let user = DriftUser::new(&self.endpoint, self.commitment, *account);
        let user = user.subscribe().await?;

        let mut map = self.inner.write().expect("acquired");
        map.insert(*account, user);

        Ok(())
    }
    /// Unsubscribe user account
    pub fn unsubscribe_user(&self, account: &Pubkey) {
        let mut map = self.inner.write().expect("acquired");
        if let Some(u) = map.remove(account) {
            let _ = u.unsubscribe();
        }
    }
    /// Return `User` data of the given `account`, if it exists
    pub fn user_account_data(&self, account: &Pubkey) -> Option<User> {
        let usermap = self.inner.read().expect("read");
        usermap.get(account).map(|u| u.get_user_account())
    }
    /// Return `User` data of the given `account` and slot, if it exists
    pub fn user_account_data_and_slot(&self, account: &Pubkey) -> Option<DataAndSlot<User>> {
        let usermap = self.inner.read().expect("read");
        usermap.get(account).map(|u| u.get_user_account_and_slot())
    }
}

struct Subscribed {
    data_and_slot: Arc<RwLock<DataAndSlot<User>>>,
    unsub: Mutex<Option<UnsubHandle>>,
}
struct Unsubscribed;

/// A subscription to a drift User account
pub struct DriftUser<S> {
    pub pubkey: Pubkey,
    /// underlying Ws subscription
    subscription: WebsocketAccountSubscriber,
    state: S,
}

impl DriftUser<Unsubscribed> {
    pub const SUBSCRIPTION_ID: &'static str = "user";

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
    pub async fn subscribe(self) -> SdkResult<DriftUser<Subscribed>> {
        let data_and_slot = Arc::new(RwLock::new(DataAndSlot {
            slot: 0,
            data: Default::default(),
        }));
        let unsub = self
            .subscription
            .subscribe(Self::SUBSCRIPTION_ID, {
                let current_data_and_slot = Arc::clone(&data_and_slot);
                move |update| {
                    let mut data_and_slot = current_data_and_slot.write().expect("acquired");
                    *data_and_slot = DataAndSlot {
                        data: User::try_deserialize(&mut update.data.as_slice())
                            .expect("valid user data"),
                        slot: update.slot,
                    };
                }
            })
            .await?;

        Ok(DriftUser {
            pubkey: self.pubkey,
            subscription: self.subscription,
            state: Subscribed {
                data_and_slot,
                unsub: Mutex::new(Some(unsub)),
            },
        })
    }
}

impl DriftUser<Subscribed> {
    /// Return the latest value of the `User` account along with last updated slot
    pub fn get_user_account_and_slot(&self) -> DataAndSlot<User> {
        let reader = self.state.data_and_slot.read().expect("reader");
        reader.clone()
    }

    /// Return the latest value of the `User` account
    pub fn get_user_account(&self) -> User {
        self.get_user_account_and_slot().data
    }

    /// Stop the user subscriber task, if it exists
    pub fn unsubscribe(self) -> DriftUser<Unsubscribed> {
        let mut guard = self.state.unsub.lock().expect("acquire");
        if let Some(unsub) = guard.take() {
            if unsub.send(()).is_err() {
                log::error!("couldn't unsubscribe");
            }
        }

        DriftUser {
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
    use crate::{constants::DEFAULT_PUBKEY, utils::envs::mainnet_endpoint, Wallet};

    #[tokio::test]
    async fn test_user_subscribe() {
        let _ = env_logger::try_init();
        let user_map = UserMap::new(mainnet_endpoint(), CommitmentConfig::confirmed());
        let user_1 = Wallet::derive_user_account(
            &pubkey!("DxoRJ4f5XRMvXU9SGuM4ZziBFUxbhB3ubur5sVZEvue2"),
            0,
        );
        let user_2 = Wallet::derive_user_account(
            &pubkey!("Drift7AMLeq3FoKBMpT9wzqyMM3HVvvZFtsn81iSSkWV"),
            0,
        );

        let (res1, res2) = tokio::join!(
            user_map.subscribe_user(&user_1),
            user_map.subscribe_user(&user_2),
        );
        assert!(res1.and(res2).is_ok());

        let handle = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(2)).await;
            let user_account_data = user_map.user_account_data(&user_1);
            assert!(user_account_data.is_some_and(|x| x.authority != DEFAULT_PUBKEY));
            user_map.unsubscribe_user(&user_1);

            let user_account_data = user_map.user_account_data(&user_2);
            assert!(user_account_data.is_some_and(|x| x.authority != DEFAULT_PUBKEY));

            let user_account_data = user_map.user_account_data(&user_1);
            assert!(user_account_data.is_none());
        });

        assert!(handle.await.is_ok());
    }
}
