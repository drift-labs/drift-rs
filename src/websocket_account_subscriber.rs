use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};

use crate::{event_emitter::EventEmitter, SdkResult};

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
        Ok(())
    }

    pub async fn unsubscribe(&mut self) -> SdkResult<()> {
        Ok(())
    }
}
