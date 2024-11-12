use std::{
    sync::{atomic::AtomicU64, Arc, Mutex},
    time::Duration,
};

use futures_util::StreamExt;
use log::{debug, error, warn};
use solana_client::nonblocking::pubsub_client::PubsubClient;
use solana_sdk::clock::Slot;
use tokio::sync::{
    mpsc::{self},
    oneshot,
};

use crate::{
    async_utils::{retry_policy, spawn_retry_task},
    types::{SdkError, SdkResult},
};

/// Max. time for slot subscriber to run without an update
const SLOT_STALENESS_THRESHOLD: Duration = Duration::from_secs(4);

const LOG_TARGET: &str = "slotsub";

/// Subscribes to network slot number increases
///
/// ```example
/// let slot_subscriber = SlotSubscriber::new("http://rpc.example.com");
/// slot_subscriber.subscribe(move |slot| {
///     dbg!("new slot", slot);
/// }).expect("subd");
///
/// // get latest slot
/// let latest_slot = slot_subscriber.current_slot();
/// ```
///
pub struct SlotSubscriber {
    current_slot: Arc<AtomicU64>,
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
            current_slot: Arc::default(),
            url,
            unsub: Mutex::new(None),
        }
    }

    /// Returns the latest slot
    pub fn current_slot(&self) -> Slot {
        self.current_slot.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub fn subscribe<F>(&mut self, handler_fn: F) -> SdkResult<()>
    where
        F: 'static + Send + Fn(SlotUpdate),
    {
        if self.is_subscribed() {
            return Ok(());
        }
        self.subscribe_ws(handler_fn)
    }

    fn subscribe_ws<F>(&mut self, handler_fn: F) -> SdkResult<()>
    where
        F: 'static + Send + Fn(SlotUpdate),
    {
        let (slot_tx, mut slot_rx) = mpsc::channel(8);
        let url = self.url.clone();

        let join_handle = spawn_retry_task(
            move || {
                let task = SlotSubscriberTask {
                    endpoint: url.clone(),
                    slot_tx: slot_tx.clone(),
                };
                task.slot_subscribe()
            },
            retry_policy::forever(1),
        );

        let (unsub_tx, mut unsub_rx) = oneshot::channel::<()>();
        {
            let mut guard = self.unsub.try_lock().expect("uncontested");
            *guard = Some(unsub_tx);
        }

        let current_slot = Arc::clone(&self.current_slot);
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    biased;
                    new_slot = slot_rx.recv() => {
                        match new_slot {
                            Some(new_slot) => {
                                current_slot.store(new_slot, std::sync::atomic::Ordering::Relaxed);
                                handler_fn(SlotUpdate::new(new_slot));
                            }
                            None => {

                            }
                        }
                    }
                    _ = &mut unsub_rx => {
                        debug!("unsubscribed");
                        break;
                    }
                }
            }
            join_handle.abort();
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

struct SlotSubscriberTask {
    endpoint: String,
    slot_tx: mpsc::Sender<Slot>,
}

impl SlotSubscriberTask {
    async fn slot_subscribe(self) {
        debug!(target: LOG_TARGET, "start task");
        let pubsub = match PubsubClient::new(&self.endpoint).await {
            Ok(p) => p,
            Err(err) => {
                debug!(target: LOG_TARGET, "connect failed: {err:?}");
                return;
            }
        };
        let (mut slot_updates, unsubscriber) = match pubsub.slot_subscribe().await {
            Ok(s) => s,
            Err(err) => {
                debug!(target: LOG_TARGET, "subscribe failed: {err:?}");
                return;
            }
        };

        let mut current_slot = 0;
        while let Ok(Some(message)) =
            tokio::time::timeout(SLOT_STALENESS_THRESHOLD, slot_updates.next()).await
        {
            if message.slot >= current_slot {
                current_slot = message.slot;
                self.slot_tx.try_send(current_slot).expect("sent");
            }
        }
        warn!(target: LOG_TARGET, "slot stream stale or disconnected");
        unsubscriber().await;
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
