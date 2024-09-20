use std::{sync::Arc, time::Duration};

use log::error;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use tokio::sync::RwLock;

/// Subscribes to account updates at regular polled intervals
pub struct PolledAccountSubscriber {
    account: Pubkey,
    interval: Duration,
    rpc_client: RpcClient,
}

impl PolledAccountSubscriber {
    /// Create a new polling account subscriber
    ///
    /// `poll_interval` configurable polling interval
    /// `account` the account to poll
    /// `rpc_client` Provides account fetching implementation
    pub fn new(account: Pubkey, poll_interval: Duration, rpc_client: RpcClient) -> Self {
        Self {
            account,
            interval: poll_interval,
            rpc_client,
        }
    }

    /// Start the account subscriber
    ///
    /// Returns a channel for emitted updates
    pub fn subscribe<T>(self, value: Arc<RwLock<T>>)
    where
        T: anchor_lang::AccountDeserialize + Sync + Send + 'static,
    {
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(self.interval);

            loop {
                let _ = interval.tick().await;
                match self.rpc_client.get_account_data(&self.account).await {
                    Ok(new_account_data) => {
                        let mut value = value.write().await;
                        let new_value =
                            T::try_deserialize_unchecked(&mut new_account_data.as_slice())
                                .expect("valid account data");
                        *value = new_value;
                    }
                    Err(err) => error!("{err:?}"),
                }
            }
        });
    }
}
