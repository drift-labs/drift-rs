use std::sync::{Arc, Mutex};

use futures_util::StreamExt;
use log::{debug, error, warn};
use solana_client::nonblocking::pubsub_client::PubsubClient;
use tokio::sync::oneshot;

use crate::types::{SdkError, SdkResult};

/// To subscribe to slot updates, subscribe to the event_emitter's "slot" event type.
pub struct SlotSubscriber {
    current_slot: Arc<Mutex<u64>>,
    url: String,
    unsub: Mutex<Option<oneshot::Sender<()>>>,
}

#[derive(Clone, Debug)]
pub struct SlotUpdate {
    pub latest_slot: u64,
}

impl SlotUpdate {
    pub fn new(latest_slot: u64) -> Self {
        Self { latest_slot }
    }
}

impl SlotSubscriber {
    pub const SUBSCRIPTION_ID: &'static str = "slot";

    pub fn is_subscribed(&self) -> bool {
        let guard = self.unsub.lock().expect("acquired");
        guard.is_some()
    }

    pub fn new(url: String) -> Self {
        Self {
            current_slot: Arc::new(Mutex::new(0)),
            url,
            unsub: Mutex::new(None),
        }
    }

    pub fn current_slot(&self) -> u64 {
        let slot_guard = self.current_slot.lock().unwrap();
        *slot_guard
    }

    pub async fn subscribe<F>(&self, handler_fn: F) -> SdkResult<()>
    where
        F: 'static + Send + Fn(SlotUpdate),
    {
        if self.is_subscribed() {
            return Ok(());
        }
        self.subscribe_ws(handler_fn).await?;
        Ok(())
    }

    async fn subscribe_ws<F>(&self, handler_fn: F) -> SdkResult<()>
    where
        F: 'static + Send + Fn(SlotUpdate),
    {
        let pubsub = PubsubClient::new(&self.url).await?;
        let (unsub_tx, mut unsub_rx) = oneshot::channel::<()>();
        {
            let mut guard = self.unsub.try_lock().expect("uncontested");
            *guard = Some(unsub_tx);
        }
        let current_slot = self.current_slot.clone();

        tokio::spawn(async move {
            let (mut slot_updates, unsubscriber) = pubsub.slot_subscribe().await.unwrap();
            loop {
                tokio::select! {
                    biased;
                    message = slot_updates.next() => {
                        match message {
                            Some(message) => {
                                let slot = message.slot;
                                let mut current_slot_guard = current_slot.lock().unwrap();
                                if slot >= *current_slot_guard {
                                    *current_slot_guard = slot;
                                    handler_fn(SlotUpdate::new(slot));
                                }
                            }
                            None => {
                                warn!("Slot stream ended");
                                unsubscriber().await;
                                break;
                            }
                        }
                    }
                    _ = &mut unsub_rx => {
                        debug!("Unsubscribing.");
                        unsubscriber().await;
                        break;
                    }
                }
            }
        });

        Ok(())
    }

    pub async fn unsubscribe(&self) -> SdkResult<()> {
        let mut guard = self.unsub.lock().expect("acquired");
        if let Some(unsub) = guard.take() {
            if unsub.send(()).is_err() {
                error!("Failed to send unsubscribe signal");
                return Err(SdkError::CouldntUnsubscribe);
            }
        }

        Ok(())
    }
}

#[cfg(feature = "rpc_tests")]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::utils::test_envs::mainnet_endpoint;

    #[tokio::test]
    async fn test_subscribe() {
        let cluster = Cluster::from_str(&mainnet_endpoint()).unwrap();
        let url = cluster.ws_url().to_string();

        let mut slot_subscriber = SlotSubscriber::new(url);
        let _ = slot_subscriber.subscribe().await;

        slot_subscriber.event_emitter.clone().subscribe(
            SlotSubscriber::SUBSCRIPTION_ID,
            move |event| {
                if let Some(event) = event.as_any().downcast_ref::<SlotUpdate>() {
                    dbg!(event);
                }
            },
        );
        dbg!("sub'd");

        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        let _ = slot_subscriber.unsubscribe().await;
        dbg!("unsub'd");
    }
}
