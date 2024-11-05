use std::time::Instant;

use anchor_lang::AnchorDeserialize;
use futures_util::StreamExt;
use log::{debug, error, warn};
use solana_account_decoder::UiAccountEncoding;
use solana_client::{
    nonblocking::pubsub_client::PubsubClient,
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    rpc_filter::RpcFilterType,
};
use solana_sdk::commitment_config::CommitmentConfig;
use tokio::sync::oneshot;

use crate::{
    constants,
    types::{DataAndSlot, SdkError},
    UnsubHandle,
};

#[derive(Clone, Debug)]
pub struct ProgramAccountUpdate<T: AnchorDeserialize + Send> {
    pub pubkey: String,
    pub data_and_slot: DataAndSlot<T>,
    pub now: Instant,
}

impl<T: AnchorDeserialize + Send> ProgramAccountUpdate<T> {
    pub fn new(pubkey: String, data_and_slot: DataAndSlot<T>, now: Instant) -> Self {
        Self {
            pubkey,
            data_and_slot,
            now,
        }
    }
}

#[derive(Clone)]
pub struct WebsocketProgramAccountOptions {
    pub filters: Vec<RpcFilterType>,
    pub commitment: CommitmentConfig,
    pub encoding: UiAccountEncoding,
}

pub struct WebsocketProgramAccountSubscriber {
    url: String,
    pub(crate) options: WebsocketProgramAccountOptions,
}

impl WebsocketProgramAccountSubscriber {
    pub fn new(url: String, options: WebsocketProgramAccountOptions) -> Self {
        WebsocketProgramAccountSubscriber { url, options }
    }

    /// Start a GPA subscription task
    ///
    /// `subscription_name` some user defined identifier for the subscription
    /// `handler_fn` handles updates from the subscription task
    pub fn subscribe<T, F>(&self, subscription_name: &'static str, handler_fn: F) -> UnsubHandle
    where
        T: AnchorDeserialize + Clone + Send + 'static,
        F: 'static + Send + Fn(&ProgramAccountUpdate<T>),
    {
        let account_config = RpcAccountInfoConfig {
            commitment: Some(self.options.commitment),
            encoding: Some(self.options.encoding),
            ..RpcAccountInfoConfig::default()
        };
        let config = RpcProgramAccountsConfig {
            filters: Some(self.options.filters.clone()),
            account_config,
            ..RpcProgramAccountsConfig::default()
        };

        let (unsub_tx, mut unsub_rx) = oneshot::channel::<()>();
        let mut attempt = 0;
        let max_reconnection_attempts = 20;
        let base_delay = tokio::time::Duration::from_secs(5);
        let url = self.url.clone();

        tokio::spawn(async move {
            let mut latest_slot = 0;
            let result = 'outer: loop {
                let pubsub = PubsubClient::new(&url).await.expect("connects");
                match pubsub
                    .program_subscribe(&constants::PROGRAM_ID, Some(config.clone()))
                    .await
                {
                    Ok((mut accounts, unsubscriber)) => loop {
                        attempt = 0;
                        tokio::select! {
                            biased;
                            message = accounts.next() => {
                                match message {
                                    Some(message) => {
                                        let slot = message.context.slot;
                                        if slot >= latest_slot {
                                            latest_slot = slot;
                                            let pubkey = message.value.pubkey;
                                            let data = &message.value.account.data.decode().expect("account has data");
                                            match T::deserialize(&mut &data[8..]) {
                                                Ok(data) => {
                                                    let data_and_slot = DataAndSlot::<T> { slot, data };
                                                    handler_fn(&ProgramAccountUpdate::new(pubkey, data_and_slot, Instant::now()));
                                                },
                                                Err(err) => {
                                                    // The account at this pubkey does not match `T`
                                                    panic!("invalid account data: {err:?}");
                                                }
                                            }
                                        }
                                    },
                                    None => {
                                        warn!("stream ended: {subscription_name}");
                                        unsubscriber().await;
                                        break;
                                    }
                                }
                            }
                            _ = &mut unsub_rx => {
                                warn!("unsubscribing: {subscription_name}");
                                unsubscriber().await;
                                break 'outer Ok(())
                            }
                        }
                    },
                    Err(_) => {
                        error!("Failed to subscribe to program stream, retrying.");
                        attempt += 1;
                        if attempt >= max_reconnection_attempts {
                            error!("Max reconnection attempts reached.");
                            break 'outer Err(SdkError::MaxReconnectionAttemptsReached);
                        }
                    }
                }

                if attempt >= max_reconnection_attempts {
                    error!("Max reconnection attempts reached.");
                    break 'outer Err(SdkError::MaxReconnectionAttemptsReached);
                }

                let delay_duration = base_delay * 2_u32.pow(attempt);
                debug!(
                    "{}: Reconnecting in {:?}",
                    subscription_name, delay_duration
                );
                tokio::time::sleep(delay_duration).await;
                attempt += 1;
            };

            if result.is_err() {
                panic!("subscription task failed");
            }
        });

        unsub_tx
    }
}

#[cfg(feature = "rpc_tests")]
mod tests {
    use super::*;
    use crate::{
        drift_idl::accounts::User,
        memcmp::{get_non_idle_user_filter, get_user_filter},
        utils::test_envs::mainnet_endpoint,
    };

    #[tokio::test]
    async fn test_subscribe() {
        let filters = vec![get_user_filter(), get_non_idle_user_filter()];
        let commitment = CommitmentConfig::confirmed();
        let options = WebsocketProgramAccountOptions {
            filters,
            commitment,
            encoding: UiAccountEncoding::Base64,
        };
        let subscription_name = "Test";

        let mut ws_subscriber =
            WebsocketProgramAccountSubscriber::<User>::new(mainnet_endpoint(), options);

        let _ = ws_subscriber.subscribe().await;
        dbg!("sub'd");

        ws_subscriber.event_emitter.clone().subscribe(move |event| {
            dbg!(event);
        });

        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        let _ = ws_subscriber.unsubscribe().await;
        dbg!("unsub'd");
    }
}
