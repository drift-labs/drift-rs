use std::str::FromStr;

use futures_util::StreamExt;
use log::warn;
use solana_account_decoder::UiAccountEncoding;
use solana_client::{
    nonblocking::{pubsub_client::PubsubClient, rpc_client::RpcClient},
    rpc_config::RpcAccountInfoConfig,
};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use tokio::sync::oneshot;

use crate::{utils::get_http_url, SdkError, SdkResult, UnsubHandle};

const LOG_TARGET: &str = "wsaccsub";

#[derive(Clone, Debug)]
pub struct AccountUpdate {
    /// Address of the account
    pub pubkey: Pubkey,
    /// Owner of the account
    pub owner: Pubkey,
    pub lamports: u64,
    /// Serialized account data (e.g. Anchor/Borsh)
    pub data: Vec<u8>,
    /// Slot retrieved
    pub slot: u64,
}

#[derive(Clone)]
pub struct WebsocketAccountSubscriber {
    url: String,
    pub(crate) pubkey: Pubkey,
    pub(crate) commitment: CommitmentConfig,
}

impl WebsocketAccountSubscriber {
    pub fn new(url: String, pubkey: Pubkey, commitment: CommitmentConfig) -> Self {
        WebsocketAccountSubscriber {
            url,
            pubkey,
            commitment,
        }
    }

    /// Start a Ws account subscription task
    ///
    /// * `subscription_name` - some user defined identifier for the subscription
    /// * `sync` - true if subscription should fetch account data on start
    /// * `handler_fn` - handles updates from the subscription task
    ///
    /// Fetches the account to set the initial value, then uses event based updates
    pub async fn subscribe<F>(
        &self,
        subscription_name: &'static str,
        sync: bool,
        handler_fn: F,
    ) -> SdkResult<UnsubHandle>
    where
        F: 'static + Send + Fn(&AccountUpdate),
    {
        if sync {
            // seed initial account state
            log::debug!(target: LOG_TARGET, "seeding account: {subscription_name}-{:?}", self.pubkey);
            let owner: Pubkey;
            let rpc = RpcClient::new(get_http_url(&self.url)?);
            match rpc
                .get_account_with_commitment(&self.pubkey, self.commitment)
                .await
            {
                Ok(response) => {
                    if let Some(account) = response.value {
                        owner = account.owner;
                        handler_fn(&AccountUpdate {
                            owner,
                            lamports: account.lamports,
                            pubkey: self.pubkey,
                            data: account.data,
                            slot: response.context.slot,
                        });
                    } else {
                        return Err(SdkError::InvalidAccount);
                    }
                }
                Err(err) => {
                    warn!("seeding account failed: {err:?}");
                    return Err(err.into());
                }
            }
            drop(rpc);
        }

        let account_config = RpcAccountInfoConfig {
            commitment: Some(self.commitment),
            encoding: Some(UiAccountEncoding::Base64Zstd),
            ..RpcAccountInfoConfig::default()
        };

        let mut attempt = 0;
        let max_reconnection_attempts = 20;
        let base_delay = tokio::time::Duration::from_secs(2);

        let url = self.url.clone();
        let (unsub_tx, mut unsub_rx) = oneshot::channel::<()>();

        tokio::spawn({
            let mut latest_slot = 0;
            let pubkey = self.pubkey;

            async move {
                log::debug!(target: LOG_TARGET, "spawn account subscriber: {subscription_name}-{pubkey:?}");
                let exit_status = 'outer: loop {
                    let pubsub = match PubsubClient::new(&url).await {
                        Ok(client) => {
                            attempt = 0;
                            client
                        }
                        Err(err) => {
                            warn!(target: LOG_TARGET, "couldn't subscribe {pubkey:?}: {err:?}, retrying...");
                            attempt += 1;
                            if attempt >= max_reconnection_attempts {
                                log::error!(
                                    "{}: Max reconnection attempts reached",
                                    subscription_name
                                );
                                break 'outer Err(crate::SdkError::MaxReconnectionAttemptsReached);
                            }
                            tokio::time::sleep(base_delay).await;
                            continue;
                        }
                    };

                    log::debug!(target: LOG_TARGET, "account subscribed: {subscription_name}-{pubkey:?}");

                    match pubsub
                        .account_subscribe(&pubkey, Some(account_config.clone()))
                        .await
                    {
                        Ok((mut account_updates, account_unsubscribe)) => loop {
                            attempt = 0;
                            tokio::select! {
                                biased;
                                message = account_updates.next() => {
                                    match message {
                                        Some(message) => {
                                            let slot = message.context.slot;
                                            if slot >= latest_slot {
                                                latest_slot = slot;
                                                if let Some(data) = message.value.data.decode() {
                                                    let account_update = AccountUpdate {
                                                        owner: Pubkey::from_str(&message.value.owner).unwrap(),
                                                        lamports: message.value.lamports,
                                                        pubkey,
                                                        data,
                                                        slot,
                                                    };
                                                    handler_fn(&account_update);
                                                }
                                            }
                                        }
                                        None => {
                                            log::warn!("{}: Account stream interrupted", subscription_name);
                                            account_unsubscribe().await;
                                            break;
                                        }
                                    }
                                }
                                _ = &mut unsub_rx => {
                                    log::debug!(target: LOG_TARGET, "{}: Unsubscribing from account stream: {pubkey:?}", subscription_name);
                                    account_unsubscribe().await;
                                    break 'outer Ok(());
                                }
                            }
                        },
                        Err(err) => {
                            log::error!(
                                "{}: Failed to subscribe to account stream: {err:?}, retrying",
                                subscription_name
                            );
                            attempt += 1;
                            if attempt >= max_reconnection_attempts {
                                log::error!("Max reconnection attempts reached.");
                                break 'outer Err(crate::SdkError::MaxReconnectionAttemptsReached);
                            }
                        }
                    }

                    if attempt >= max_reconnection_attempts {
                        log::error!("{}: Max reconnection attempts reached", subscription_name);
                        break 'outer Err(crate::SdkError::MaxReconnectionAttemptsReached);
                    }

                    let delay_duration = base_delay * 2_u32.pow(attempt);
                    log::warn!(
                        "{}: reconnecting in {:?}",
                        subscription_name,
                        delay_duration
                    );
                    tokio::time::sleep(delay_duration).await;
                    attempt += 1;
                };

                if let Err(err) = exit_status {
                    log::warn!(
                        "account subscriber failed ({subscription_name}-{pubkey:?}): {err:?}"
                    );
                }
            }
        });

        Ok(unsub_tx)
    }
}
