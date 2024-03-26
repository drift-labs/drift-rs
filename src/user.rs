use std::sync::{Arc, RwLock};

use drift::state::user::User;
use solana_sdk::pubkey::Pubkey;

use crate::{
    event_emitter::EventEmitter,
    utils::{decode, get_ws_url},
    websocket_account_subscriber::{AccountUpdate, WebsocketAccountSubscriber},
    AccountProvider, DataAndSlot, DriftClient, SdkResult,
};

#[derive(Clone)]
pub struct DriftUser {
    pub pubkey: Pubkey,
    subscription: WebsocketAccountSubscriber,
    data_and_slot: Arc<RwLock<DataAndSlot<User>>>,
}

impl DriftUser {
    pub async fn new<T: AccountProvider>(
        pubkey: Pubkey,
        drift_client: DriftClient<T>,
    ) -> SdkResult<Self> {
        let subscription = WebsocketAccountSubscriber::new(
            "user",
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
        })
    }

    pub async fn subscribe(&mut self) -> SdkResult<()> {
        let current_data_and_slot = self.data_and_slot.clone();
        self.subscription
            .event_emitter
            .subscribe("user", move |event| {
                if let Some(update) = event.as_any().downcast_ref::<AccountUpdate>() {
                    let new_data =
                        decode::<User>(update.data.data.clone()).expect("valid user data");
                    let slot = update.slot;
                    let mut data_and_slot = current_data_and_slot.write().unwrap();
                    *data_and_slot = DataAndSlot {
                        data: new_data,
                        slot,
                    };
                }
            });
        self.subscription.subscribe().await?;
        Ok(())
    }

    pub fn get_user_account_and_slot(&self) -> DataAndSlot<User> {
        let reader = self.data_and_slot.read().expect("reader");
        let clone = reader.clone();
        clone
    }

    pub fn get_user_account(&self) -> User {
        self.get_user_account_and_slot().data
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::Context;
    use crate::RpcAccountProvider;
    use anchor_lang::accounts::account_loader::AccountLoader;
    use solana_sdk::account_info::AccountInfo;
    use solana_sdk::signature::Keypair;

    #[tokio::test]
    #[cfg(feature = "rpc_tests")]
    async fn test_user_subscribe() {
        let url = "rpc";
        let client = DriftClient::new(
            Context::MainNet,
            RpcAccountProvider::new(&url),
            Keypair::new().into(),
        )
        .await
        .unwrap();

        let pubkey = Pubkey::from_str("DCdMynEZ8QwNniQvwSxU4a6bqvnKRhK39QDCMEVJQJzU").unwrap();
        let mut user = DriftUser::new(pubkey, client).await.unwrap();
        user.subscribe().await.unwrap();

        loop {
            let data_and_slot = user.get_user_account_and_slot();
            dbg!(data_and_slot.slot);

            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }
}
