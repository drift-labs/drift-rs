use futures_util::StreamExt;
use solana_account_decoder::{UiAccount, UiAccountEncoding};
use solana_client::{nonblocking::pubsub_client::PubsubClient, rpc_config::RpcAccountInfoConfig};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};

use crate::{
    event_emitter::{Event, EventEmitter},
    SdkResult,
};

#[derive(Clone, Debug)]
pub(crate) struct AccountUpdate {
    pub pubkey: String,
    pub data: UiAccount,
    pub slot: u64,
}

impl Event for AccountUpdate {
    fn box_clone(&self) -> Box<dyn Event> {
        Box::new((*self).clone())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Clone)]
pub struct WebsocketAccountSubscriber {
    subscription_name: &'static str,
    url: String,
    pubkey: Pubkey,
    pub(crate) commitment: CommitmentConfig,
    pub subscribed: bool,
    pub event_emitter: EventEmitter,
    unsubscriber: Option<tokio::sync::mpsc::Sender<()>>,
}

impl WebsocketAccountSubscriber {
    pub fn new(
        subscription_name: &'static str,
        url: String,
        pubkey: Pubkey,
        commitment: CommitmentConfig,
        event_emitter: EventEmitter,
    ) -> Self {
        WebsocketAccountSubscriber {
            subscription_name,
            url,
            pubkey,
            commitment,
            subscribed: false,
            event_emitter,
            unsubscriber: None,
        }
    }

    pub async fn subscribe(&mut self) -> SdkResult<()> {
        if self.subscribed {
            return Ok(());
        }

        self.subscribed = true;
        self.subscribe_ws().await?;
        Ok(())
    }

    async fn subscribe_ws(&mut self) -> SdkResult<()> {
        let account_config = RpcAccountInfoConfig {
            commitment: Some(self.commitment),
            encoding: Some(UiAccountEncoding::Base64),
            ..RpcAccountInfoConfig::default()
        };

        let pubsub = PubsubClient::new(&self.url).await?;
        let (unsub_tx, mut unsub_rx) = tokio::sync::mpsc::channel::<()>(1);
        self.unsubscriber = Some(unsub_tx);

        let mut attempt = 0;
        let max_reconnection_attempts = 20;
        let base_delay = tokio::time::Duration::from_secs(2);

        tokio::spawn({
            let event_emitter = self.event_emitter.clone();
            let mut latest_slot = 0;
            let subscription_name = self.subscription_name;
            let pubkey = self.pubkey.clone();
            async move {
                loop {
                    let (mut account_updates, account_unsubscribe) = pubsub
                        .account_subscribe(&pubkey, Some(account_config.clone()))
                        .await
                        .unwrap();

                    loop {
                        tokio::select! {
                            message = account_updates.next() => {
                                match message {
                                    Some(message) => {
                                        let slot = message.context.slot;
                                        if slot >= latest_slot {
                                            latest_slot = slot;
                                            let account_update = AccountUpdate {
                                                pubkey: pubkey.to_string(),
                                                data: message.value,
                                                slot,
                                            };
                                            event_emitter.emit(subscription_name, Box::new(account_update));
                                        }
                                    }
                                    None => {
                                        log::warn!("{}: Account stream interrupted", subscription_name);
                                        account_unsubscribe().await;
                                        break;
                                    }
                                }
                            }
                            unsub = unsub_rx.recv() => {
                                if let Some(_) = unsub {
                                    log::debug!("{}: Unsubscribing from account stream", subscription_name);
                                    account_unsubscribe().await;
                                    return Ok(());

                                }
                            }
                        }
                    }

                    if attempt >= max_reconnection_attempts {
                        log::error!("{}: Max reconnection attempts reached", subscription_name);
                        return Err(crate::SdkError::MaxReconnectionAttemptsReached);
                    }

                    let delay_duration = base_delay * 2_u32.pow(attempt);
                    log::debug!(
                        "{}: Reconnecting in {:?}",
                        subscription_name,
                        delay_duration
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
                log::error!("Failed to send unsubscribe signal: {:?}", e);
                return Err(crate::SdkError::CouldntUnsubscribe(e));
            }
            self.subscribed = false;
        }
        Ok(())
    }
}
