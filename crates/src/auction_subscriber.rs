// Standard Library Imports

// External Crate Imports
use solana_account_decoder::UiAccountEncoding;
use solana_sdk::commitment_config::CommitmentConfig;

// Internal Crate/Module Imports
use crate::{
    drift_idl::accounts::User,
    event_emitter::EventEmitter,
    memcmp::{get_user_filter, get_user_with_auction_filter},
    types::SdkResult,
    websocket_program_account_subscriber::{
        ProgramAccountUpdate, WebsocketProgramAccountOptions, WebsocketProgramAccountSubscriber,
    },
};

pub struct AuctionSubscriberConfig {
    pub commitment: CommitmentConfig,
    pub resub_timeout_ms: Option<u64>,
    pub url: String,
}

/// To subscribe to auction updates, subscribe to the event_emitter's "auction" event type.
pub struct AuctionSubscriber {
    pub subscriber: WebsocketProgramAccountSubscriber<User>,
    pub event_emitter: EventEmitter<ProgramAccountUpdate<User>>,
}

impl AuctionSubscriber {
    pub const SUBSCRIPTION_ID: &'static str = "auction";

    pub fn new(config: AuctionSubscriberConfig) -> Self {
        let event_emitter = EventEmitter::new();

        let filters = vec![get_user_filter(), get_user_with_auction_filter()];

        let websocket_options = WebsocketProgramAccountOptions {
            filters,
            commitment: config.commitment,
            encoding: UiAccountEncoding::Base64,
        };

        let subscriber = WebsocketProgramAccountSubscriber::new(
            AuctionSubscriber::SUBSCRIPTION_ID,
            config.url,
            websocket_options,
            event_emitter.clone(),
        );

        AuctionSubscriber {
            subscriber,
            event_emitter,
        }
    }

    pub async fn subscribe(&mut self) -> SdkResult<()> {
        if self.subscriber.subscribed {
            return Ok(());
        }

        self.subscriber.subscribe().await?;

        Ok(())
    }

    pub async fn unsubscribe(&mut self) -> SdkResult<()> {
        self.subscriber.unsubscribe().await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use env_logger;

    use super::*;
    use crate::{
        utils::envs::mainnet_endpoint, websocket_program_account_subscriber::ProgramAccountUpdate,
    };

    #[cfg(feature = "rpc_tests")]
    #[tokio::test]
    async fn test_auction_subscriber() {
        env_logger::init();
        let cluster = Cluster::from_str(&"rpc").unwrap();
        let url = cluster.ws_url().to_string();

        let config = AuctionSubscriberConfig {
            commitment: CommitmentConfig::confirmed(),
            resub_timeout_ms: None,
            url,
        };

        let mut auction_subscriber = AuctionSubscriber::new(config);

        let emitter = auction_subscriber.event_emitter.clone();

        emitter.subscribe(AuctionSubscriber::SUBSCRIPTION_ID, move |event| {
            log::info!("{:?}", event.now.elapsed());
        });

        let _ = auction_subscriber.subscribe().await;

        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;

        let _ = auction_subscriber.unsubscribe().await;

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}
