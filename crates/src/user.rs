use std::sync::{Arc, RwLock};

use solana_sdk::pubkey::Pubkey;

use crate::{
    drift_idl::accounts::User,
    event_emitter::EventEmitter,
    utils::{decode, get_ws_url},
    websocket_account_subscriber::WebsocketAccountSubscriber,
    DataAndSlot, DriftClient, SdkResult,
};

#[derive(Clone)]
pub struct DriftUser {
    pub pubkey: Pubkey,
    subscription: WebsocketAccountSubscriber,
    data_and_slot: Arc<RwLock<DataAndSlot<User>>>,
    pub sub_account: u16,
}

impl DriftUser {
    pub const SUBSCRIPTION_ID: &'static str = "user";

    pub async fn new(
        pubkey: Pubkey,
        drift_client: &DriftClient,
        sub_account: u16,
    ) -> SdkResult<Self> {
        let subscription = WebsocketAccountSubscriber::new(
            DriftUser::SUBSCRIPTION_ID,
            get_ws_url(&drift_client.inner().url()).expect("valid url"),
            pubkey,
            drift_client.inner().commitment(),
            EventEmitter::new(),
        );

        let user = drift_client.get_user_account(&pubkey).await?;
        let data_and_slot = Arc::new(RwLock::new(DataAndSlot {
            data: user,
            slot: 0,
        }));

        Ok(Self {
            pubkey,
            subscription,
            data_and_slot,
            sub_account,
        })
    }

    pub async fn subscribe(&mut self) -> SdkResult<()> {
        let current_data_and_slot = self.data_and_slot.clone();
        self.subscription.event_emitter.subscribe(move |update| {
            let mut data_and_slot = current_data_and_slot.write().unwrap();
            *data_and_slot = DataAndSlot {
                data: decode::<User>(&update.data.data).expect("valid user data"),
                slot: update.slot,
            };
        });
        self.subscription.subscribe().await?;
        Ok(())
    }

    pub fn get_user_account_and_slot(&self) -> DataAndSlot<User> {
        let reader = self.data_and_slot.read().expect("reader");
        reader.clone()
    }

    pub fn get_user_account(&self) -> User {
        self.get_user_account_and_slot().data
    }
}

#[cfg(feature = "rpc_tests")]
mod tests {
    use solana_sdk::signature::Keypair;

    use super::*;
    use crate::{utils::envs::mainnet_endpoint, Context, RpcAccountProvider};

    #[tokio::test]
    async fn test_user_subscribe() {
        let client = DriftClient::new(
            Context::MainNet,
            RpcAccountProvider::new(&mainnet_endpoint()),
            Keypair::new().into(),
        )
        .await
        .unwrap();

        let pubkey = Pubkey::from_str("DCdMynEZ8QwNniQvwSxU4a6bqvnKRhK39QDCMEVJQJzU").unwrap();
        let mut user = DriftUser::new(pubkey, &client, 0).await.unwrap();
        user.subscribe().await.unwrap();

        loop {
            let data_and_slot = user.get_user_account_and_slot();
            dbg!(data_and_slot.slot);

            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }
}
