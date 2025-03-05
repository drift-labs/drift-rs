use std::{
    sync::{Arc, Mutex, RwLock},
    time::Duration,
};

use log::error;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{account::Account, pubkey::Pubkey};
use tokio::sync::oneshot;

use crate::{SdkError, SdkResult, UnsubHandle};

/// Subscribed state token
struct Subscribed {
    account: Arc<RwLock<Account>>,
    unsub: Mutex<Option<UnsubHandle>>,
}
/// Unsubscribed state token
struct Unsubscribed;

/// Subscribes to account updates at regular polled intervals
pub struct PolledAccountSubscriber<S> {
    pubkey: Pubkey,
    interval: Duration,
    rpc_client: Arc<RpcClient>,
    state: S,
}

impl PolledAccountSubscriber<Unsubscribed> {
    /// Create a new polling account subscriber
    ///
    /// `poll_interval` configurable polling interval
    /// `pubkey` the account to poll
    /// `rpc_client` Provides account fetching implementation
    pub fn new(
        pubkey: Pubkey,
        poll_interval: Duration,
        rpc_client: Arc<RpcClient>,
    ) -> PolledAccountSubscriber<Unsubscribed> {
        Self {
            pubkey,
            interval: poll_interval,
            rpc_client: Arc::clone(&rpc_client),
            state: Unsubscribed {},
        }
    }

    /// Start the account subscriber
    ///
    /// Returns a channel for emitted updates
    pub fn subscribe(self) -> PolledAccountSubscriber<Subscribed> {
        let (unsub_tx, mut unsub_rx) = oneshot::channel();
        let account = Arc::new(RwLock::new(Account::default()));

        tokio::spawn({
            let mut interval = tokio::time::interval(self.interval);
            let account = Arc::clone(&account);
            let pubkey = self.pubkey;
            let rpc_client = Arc::clone(&self.rpc_client);

            async move {
                loop {
                    let _ = interval.tick().await;
                    match rpc_client.get_account(&pubkey).await {
                        Ok(new_account) => {
                            let mut value = account.write().expect("acquired");
                            *value = new_account;
                        }
                        Err(err) => error!("{err:?}"),
                    }

                    if unsub_rx.try_recv().is_ok() {
                        break;
                    }
                }
            }
        });

        PolledAccountSubscriber {
            pubkey: self.pubkey,
            interval: self.interval,
            rpc_client: self.rpc_client,
            state: Subscribed {
                account,
                unsub: Mutex::new(Some(unsub_tx)),
            },
        }
    }
}

impl PolledAccountSubscriber<Subscribed> {
    /// Stop the subscriber task, if it exists
    pub fn unsubscribe(&self) {
        let mut guard = self.state.unsub.lock().expect("uncontested");
        if let Some(unsub) = guard.take() {
            if unsub.send(()).is_err() {
                log::error!("couldn't unsubscribe");
            }
        }
    }

    /// Get account deserialized as a `T`
    pub fn get_value<T>(&self) -> SdkResult<T>
    where
        T: anchor_lang::AccountDeserialize + Sync + Send + 'static,
    {
        let acc = self.state.account.read().expect("acquired");
        T::try_deserialize_unchecked(&mut acc.data.as_slice()).map_err(|_| SdkError::Deserializing)
    }

    /// Get raw account value
    pub fn get_account_data(&self) -> Account {
        let acc = self.state.account.read().expect("acquired");
        acc.clone()
    }
}

#[cfg(test)]
mod tests {
    use anchor_lang::AccountSerialize;
    use serde_json::json;
    use solana_account_decoder::{UiAccount, UiAccountEncoding};
    use solana_rpc_client::rpc_client::Mocks;
    use solana_rpc_client_api::request::RpcRequest;

    use super::*;
    use crate::{accounts::User, SpotPosition};

    #[tokio::test]
    async fn polled_account_subscriber_updates() {
        // mock account response
        let owner = Pubkey::new_unique();
        let sub_account = Pubkey::new_unique();

        let mut mock_user = User {
            authority: owner,
            ..Default::default()
        };
        mock_user.spot_positions[1] = SpotPosition {
            scaled_balance: 12_345,
            market_index: 1,
            ..Default::default()
        };

        let mut buf = Vec::<u8>::default();
        mock_user.try_serialize(&mut buf).expect("serializes");

        let account = Account {
            data: buf,
            ..Default::default()
        };

        let mut response_mocks = Mocks::default();
        let account_response = json!({
            "context": {
                "slot": 12_345,
            },
            "value": UiAccount::encode(&sub_account, &account, UiAccountEncoding::Base64Zstd, None, None),
        });
        response_mocks.insert(RpcRequest::GetAccountInfo, account_response);

        let mock_rpc = RpcClient::new_mock_with_mocks(
            "https://api.mainnet-beta.solana.com".into(),
            response_mocks,
        );

        // test
        let subscriber =
            PolledAccountSubscriber::new(sub_account, Duration::from_secs(5), Arc::new(mock_rpc));
        let subscriber = subscriber.subscribe();
        let _ = tokio::time::sleep(Duration::from_millis(500)).await;

        let user = subscriber.get_value::<User>().expect("get account");
        assert_eq!(user, mock_user);
    }
}
