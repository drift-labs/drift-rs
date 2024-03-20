use crate::{
    dlob::dlob::DLOB,
    event_emitter::{Event, EventEmitter},
    slot_subscriber::SlotSubscriber,
    usermap::UserMap,
    SdkResult,
};

pub struct DLOBBuilder {
    slot_subscriber: SlotSubscriber,
    usermap: UserMap,
    rebuild_frequency: u64,
    dlob: DLOB,
    event_emitter: EventEmitter,
}

impl DLOBBuilder {
    pub fn new(
        slot_subscriber: SlotSubscriber,
        usermap: UserMap,
        rebuild_frequency: u64,
    ) -> DLOBBuilder {
        DLOBBuilder {
            slot_subscriber,
            usermap,
            rebuild_frequency,
            dlob: DLOB::new(),
            event_emitter: EventEmitter::new(),
        }
    }

    pub async fn start_building(&mut self) -> SdkResult<()> {
        self.slot_subscriber.subscribe().await?;
        self.usermap.subscribe().await?;

        self.build();

        loop {
            self.build();
            tokio::time::sleep(tokio::time::Duration::from_secs(self.rebuild_frequency)).await;
        }
    }

    pub fn build(&mut self) {
        self.dlob
            .build_from_usermap(&self.usermap, self.slot_subscriber.current_slot());
        self.event_emitter
            .emit("dlob_update", Box::new(self.dlob.clone()));
    }

    pub fn get_dlob(&self) -> DLOB {
        self.dlob.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::get_ws_url;
    use solana_sdk::commitment_config::CommitmentConfig;
    use solana_sdk::commitment_config::CommitmentLevel;
    use std::time::Instant;

    #[tokio::test]
    #[cfg(rpc_tests)]
    async fn test_dlob_builder() {
        let endpoint = "rpc_url".to_string();
        let commitment = CommitmentConfig {
            commitment: CommitmentLevel::Processed,
        };

        let slot_subscriber = SlotSubscriber::new(get_ws_url(&endpoint.clone()).unwrap());
        let usermap = UserMap::new(commitment, endpoint, true);
        let mut dlob_builder = DLOBBuilder::new(slot_subscriber, usermap, 30);

        dlob_builder
            .event_emitter
            .clone()
            .subscribe("dlob_update", move |event| {
                if let Some(_) = event.as_any().downcast_ref::<DLOB>() {
                    dbg!("update received");
                }
            });

        dlob_builder.start_building().await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_secs(120)).await;
    }

    #[tokio::test]
    #[cfg(rpc_tests)]
    async fn test_build_time() {
        let endpoint = "url".to_string();
        let commitment = CommitmentConfig {
            commitment: CommitmentLevel::Processed,
        };

        let mut slot_subscriber = SlotSubscriber::new(get_ws_url(&endpoint.clone()).unwrap());
        let mut usermap = UserMap::new(commitment, endpoint, true);
        let _ = slot_subscriber.subscribe().await;
        let _ = usermap.subscribe().await;

        let mut dlob_builder = DLOBBuilder::new(slot_subscriber, usermap, 30);

        let start = Instant::now();
        dlob_builder.build();
        let duration = start.elapsed();
        dbg!(duration);
    }
}
