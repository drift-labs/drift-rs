use std::any::Any;

use anchor_lang::AccountDeserialize;
use futures_util::StreamExt;
use log::{debug, error, warn};
use solana_account_decoder::UiAccountEncoding;
use solana_client::{
    nonblocking::pubsub_client::PubsubClient,
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    rpc_filter::RpcFilterType,
};
use solana_sdk::commitment_config::CommitmentConfig;

use crate::{
    event_emitter::{Event, EventEmitter},
    types::{DataAndSlot, SdkError, SdkResult},
    utils::decode,
};

#[derive(Clone, Debug)]
pub struct ProgramAccountUpdate<T: Clone + Send + AccountDeserialize + 'static> {
    pub pubkey: String,
    pub data_and_slot: DataAndSlot<T>,
    pub now: std::time::Instant,
}

impl<T: Clone + Send + AccountDeserialize + 'static> ProgramAccountUpdate<T> {
    pub fn new(pubkey: String, data_and_slot: DataAndSlot<T>, now: std::time::Instant) -> Self {
        Self {
            pubkey,
            data_and_slot,
            now,
        }
    }
}

impl<T: Clone + Send + AccountDeserialize + 'static> Event for ProgramAccountUpdate<T> {
    fn box_clone(&self) -> Box<dyn Event> {
        Box::new((*self).clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct WebsocketProgramAccountOptions {
    pub filters: Vec<RpcFilterType>,
    pub commitment: CommitmentConfig,
    pub encoding: UiAccountEncoding,
}

pub struct WebsocketProgramAccountSubscriber {
    subscription_name: &'static str,
    url: String,
    pub(crate) options: WebsocketProgramAccountOptions,
    pub subscribed: bool,
    pub event_emitter: EventEmitter,
    unsubscriber: Option<tokio::sync::mpsc::Sender<()>>,
}

impl WebsocketProgramAccountSubscriber {
    pub fn new(
        subscription_name: &'static str,
        url: String,
        options: WebsocketProgramAccountOptions,
        event_emitter: EventEmitter,
    ) -> Self {
        WebsocketProgramAccountSubscriber {
            subscription_name,
            url,
            options,
            subscribed: false,
            event_emitter,
            unsubscriber: None,
        }
    }

    pub async fn subscribe<T>(&mut self) -> SdkResult<()>
    where
        T: AccountDeserialize + Clone + Send + 'static,
    {
        if self.subscribed {
            return Ok(());
        }
        self.subscribed = true;
        self.subscribe_ws::<T>().await?;

        Ok(())
    }

    async fn subscribe_ws<T>(&mut self) -> SdkResult<()>
    where
        T: AccountDeserialize + Clone + Send + 'static,
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

        let (unsub_tx, mut unsub_rx) = tokio::sync::mpsc::channel::<()>(1);
        self.unsubscriber = Some(unsub_tx);

        let mut attempt = 0;
        let max_reconnection_attempts = 20;
        let base_delay = tokio::time::Duration::from_secs(5);

        let url = self.url.clone();
        tokio::spawn({
            let event_emitter = self.event_emitter.clone();
            let mut latest_slot = 0;
            let subscription_name = self.subscription_name;
            async move {
                loop {
                    let pubsub = PubsubClient::new(&url).await?;
                    match pubsub
                        .program_subscribe(&drift::ID, Some(config.clone()))
                        .await
                    {
                        Ok((mut accounts, unsubscriber)) => loop {
                            attempt = 0;
                            tokio::select! {
                                message = accounts.next() => {
                                    match message {
                                        Some(message) => {
                                            let slot = message.context.slot;
                                            if slot >= latest_slot {
                                                latest_slot = slot;
                                                let pubkey = message.value.pubkey;
                                                let account_data = message.value.account.data;
                                                match decode(account_data) {
                                                    Ok(data) => {
                                                        let data_and_slot = DataAndSlot::<T> { slot, data };
                                                        event_emitter.emit(subscription_name, Box::new(ProgramAccountUpdate::new(pubkey, data_and_slot, std::time::Instant::now())));
                                                    },
                                                    Err(e) => {
                                                        error!("Error decoding account data {e}");
                                                    }
                                                }
                                            }
                                        }
                                        None => {
                                            warn!("{} stream ended", subscription_name);
                                            unsubscriber().await;
                                            break;
                                        }
                                    }
                                }
                                _ = unsub_rx.recv() => {
                                    debug!("Unsubscribing.");
                                    unsubscriber().await;
                                    return Ok(());
                                }
                            }
                        },
                        Err(_) => {
                            error!("Failed to subscribe to program stream, retrying.");
                            attempt += 1;
                            if attempt >= max_reconnection_attempts {
                                error!("Max reconnection attempts reached.");
                                return Err(SdkError::MaxReconnectionAttemptsReached);
                            }
                        }
                    }

                    if attempt >= max_reconnection_attempts {
                        error!("Max reconnection attempts reached.");
                        return Err(SdkError::MaxReconnectionAttemptsReached);
                    }

                    let delay_duration = base_delay * 2_u32.pow(attempt);
                    debug!(
                        "{}: Reconnecting in {:?}",
                        subscription_name, delay_duration
                    );
                    tokio::time::sleep(delay_duration).await;
                    attempt += 1;
                }
            }
        });

        Ok(())
    }

    pub async fn unsubscribe(&mut self) -> SdkResult<()> {
        if self.subscribed && self.unsubscriber.is_some() {
            if let Err(e) = self.unsubscriber.as_ref().unwrap().send(()).await {
                error!("Failed to send unsubscribe signal: {:?}", e);
                return Err(SdkError::CouldntUnsubscribe(e));
            }
            self.subscribed = false;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "rpc_tests")]
    #[tokio::test]
    async fn test_subscribe() {
        use std::str::FromStr;

        use anchor_client::Cluster;
        use drift::state::user::User;

        use super::*;
        use crate::memcmp::{get_non_idle_user_filter, get_user_filter};

        let filters = vec![get_user_filter(), get_non_idle_user_filter()];
        let commitment = CommitmentConfig::confirmed();
        let options = WebsocketProgramAccountOptions {
            filters,
            commitment,
            encoding: UiAccountEncoding::Base64,
        };
        let endpoint = "https://api.devnet.solana.com";
        let cluster = Cluster::from_str(endpoint).unwrap();
        let url = cluster.ws_url().to_string();
        let subscription_name = "Test";

        let mut ws_subscriber = WebsocketProgramAccountSubscriber::new(
            subscription_name,
            url,
            options,
            EventEmitter::new(),
        );

        let _ = ws_subscriber.subscribe::<User>().await;
        dbg!("sub'd");

        ws_subscriber
            .event_emitter
            .clone()
            .subscribe("Test", move |event| {
                if let Some(event) = event.as_any().downcast_ref::<ProgramAccountUpdate<User>>() {
                    dbg!(event);
                }
            });

        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        let _ = ws_subscriber.unsubscribe().await;
        dbg!("unsub'd");
    }
}
