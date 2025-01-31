use std::{str::FromStr, sync::Arc};

use drift_pubsub_client::PubsubClient;
use futures_util::StreamExt;
use log::warn;
use solana_account_decoder::UiAccountEncoding;
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcAccountInfoConfig};
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
    pubsub: Arc<PubsubClient>,
    pub(crate) pubkey: Pubkey,
    pub(crate) commitment: CommitmentConfig,
}

impl WebsocketAccountSubscriber {
    pub fn new(pubsub: Arc<PubsubClient>, pubkey: Pubkey, commitment: CommitmentConfig) -> Self {
        WebsocketAccountSubscriber {
            pubsub,
            pubkey,
            commitment,
        }
    }

    /// Start a Ws account subscription task
    ///
    /// * `subscription_name` - some user defined identifier for the subscription
    /// * `sync` - true if subscription should fetch account data on start
    /// * `on_update` - function to call on updates from the subscription
    ///
    /// Fetches the account to set the initial value, then uses event based updates
    pub async fn subscribe<F>(
        &self,
        subscription_name: &'static str,
        sync: bool,
        on_update: F,
    ) -> SdkResult<UnsubHandle>
    where
        F: 'static + Send + Fn(&AccountUpdate),
    {
        if sync {
            // seed initial account state
            log::debug!(target: LOG_TARGET, "seeding account: {subscription_name}-{:?}", self.pubkey);
            let owner: Pubkey;
            let rpc = RpcClient::new(get_http_url(self.pubsub.url().as_str())?);
            match rpc
                .get_account_with_commitment(&self.pubkey, self.commitment)
                .await
            {
                Ok(response) => {
                    if let Some(account) = response.value {
                        owner = account.owner;
                        on_update(&AccountUpdate {
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

        let (unsub_tx, mut unsub_rx) = oneshot::channel::<()>();
        let account_config = RpcAccountInfoConfig {
            commitment: Some(self.commitment),
            encoding: Some(UiAccountEncoding::Base64Zstd),
            ..RpcAccountInfoConfig::default()
        };
        let pubkey = self.pubkey;
        let pubsub = Arc::clone(&self.pubsub);

        tokio::spawn(async move {
            log::debug!(target: LOG_TARGET, "spawn account subscriber: {subscription_name}-{:?}", pubkey);
            let (mut account_updates, account_unsubscribe) = pubsub
                .account_subscribe(&pubkey, Some(account_config))
                .await
                .expect("account subd");
            log::debug!(target: LOG_TARGET, "account subscribed: {subscription_name}-{pubkey:?}");
            let mut latest_slot = 0;
            loop {
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
                                        on_update(&account_update);
                                    }
                                }
                            }
                            None => {
                                log::warn!("{}: Account stream interrupted", subscription_name);
                                break;
                            }
                        }
                    }
                    _ = &mut unsub_rx => {
                        log::debug!(target: LOG_TARGET, "{}: Unsubscribing from account stream: {pubkey:?}", subscription_name);
                        break;
                    }
                }
            }
            account_unsubscribe().await;
        });

        Ok(unsub_tx)
    }
}
