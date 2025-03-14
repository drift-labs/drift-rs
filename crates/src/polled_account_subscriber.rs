use std::{sync::Arc, time::Duration};

use log::error;
use solana_account_decoder_client_types::UiAccountEncoding;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_rpc_client_api::config::RpcAccountInfoConfig;
use solana_sdk::pubkey::Pubkey;
use tokio::sync::oneshot;

use crate::UnsubHandle;

#[derive(Clone, Debug)]
pub struct AccountUpdate {
    /// Address of the account
    pub pubkey: Pubkey,
    /// Owner of the account
    pub owner: Pubkey,
    pub lamports: u64,
    /// Serialized account data (e.g. Anchor/Borsh)
    pub data: Vec<u8>,
    /// Slot retrieved
    pub slot: u64,
}

/// Subscribes to account updates at regular polled intervals
pub struct PolledAccountSubscriber {
    pubkey: Pubkey,
    interval: Duration,
    rpc_client: Arc<RpcClient>,
}

impl PolledAccountSubscriber {
    /// Create a new polling account subscriber
    ///
    /// `poll_interval` configurable polling interval
    /// `pubkey` the account to poll
    /// `rpc_client` Provides account fetching implementation
    pub fn new(
        pubkey: Pubkey,
        poll_interval: Duration,
        rpc_client: Arc<RpcClient>,
    ) -> PolledAccountSubscriber {
        Self {
            pubkey,
            interval: poll_interval,
            rpc_client: Arc::clone(&rpc_client),
        }
    }

    /// Start the account subscriber
    ///
    /// `on_update` callback to receive new account values
    ///
    /// Returns channel for unsubscribing
    pub fn subscribe<F>(&self, on_update: F) -> UnsubHandle
    where
        F: 'static + Send + Fn(&AccountUpdate),
    {
        let (unsub_tx, mut unsub_rx) = oneshot::channel();

        tokio::spawn({
            let mut interval = tokio::time::interval(self.interval);
            let pubkey = self.pubkey;
            let rpc_client = Arc::clone(&self.rpc_client);

            let config = RpcAccountInfoConfig {
                encoding: Some(UiAccountEncoding::Base64Zstd),
                commitment: Some(rpc_client.commitment()),
                ..Default::default()
            };
            async move {
                loop {
                    tokio::select! {
                        biased;
                        _ = interval.tick() => {
                            match rpc_client.get_account_with_config(&pubkey, config.clone()).await {
                                Ok(response) => {
                                    if let Some(new_account) = response.value {
                                        on_update(
                                            &AccountUpdate {
                                                owner: new_account.owner,
                                                lamports: new_account.lamports,
                                                pubkey,
                                                data: new_account.data.clone(),
                                                slot: response.context.slot,
                                            }
                                        );
                                    }
                                }
                                Err(err) => error!("{err:?}"),
                            }
                        }
                        _ = &mut unsub_rx => {
                            break;
                        }
                    }
                }
            }
        });

        unsub_tx
    }
}

#[cfg(test)]
mod tests {
    use anchor_lang::AccountSerialize;
    use serde_json::json;
    use solana_account_decoder::encode_ui_account;
    use solana_account_decoder_client_types::UiAccountEncoding;
    use solana_rpc_client::rpc_client::Mocks;
    use solana_rpc_client_api::request::RpcRequest;
    use solana_sdk::account::Account;

    use super::*;
    use crate::{accounts::User, SpotPosition};

    #[tokio::test]
    async fn polled_account_subscriber_updates() {
        // mock account response
        let owner = Pubkey::new_unique();
        let sub_account = Pubkey::new_unique();

        let mut mock_user = User {
            authority: owner,
            ..Default::default()
        };
        mock_user.spot_positions[1] = SpotPosition {
            scaled_balance: 12_345,
            market_index: 1,
            ..Default::default()
        };

        let mut buf = Vec::<u8>::default();
        mock_user.try_serialize(&mut buf).expect("serializes");

        let mock_account = Account {
            data: buf,
            ..Default::default()
        };

        let mut response_mocks = Mocks::default();
        let account_response = json!({
            "context": {
                "slot": 12_345,
            },
            "value": encode_ui_account(&sub_account, &mock_account, UiAccountEncoding::Base64Zstd, None, None),
        });
        response_mocks.insert(RpcRequest::GetAccountInfo, account_response);

        let mock_rpc = RpcClient::new_mock_with_mocks(
            "https://api.mainnet-beta.solana.com".into(),
            response_mocks,
        );

        // test
        let subscriber =
            PolledAccountSubscriber::new(sub_account, Duration::from_secs(1), Arc::new(mock_rpc));
        let _unsub = subscriber.subscribe(move |user| {
            assert_eq!(user.data, mock_account.data,);
        });
        let _ = tokio::time::sleep(Duration::from_millis(500)).await;
    }
}
