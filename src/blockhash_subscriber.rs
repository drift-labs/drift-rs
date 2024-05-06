use std::sync::Arc;

use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::hash::Hash;
use tokio::sync::RwLock;

use crate::SdkResult;

pub struct BlockhashSubscriber {
    latest_blockhash: Hash,
    last_twenty_hashes: Vec<Hash>,
    refresh_frequency: u64,
    rpc_client: RpcClient,
}

impl BlockhashSubscriber {
    pub fn new(refresh_frequency: u64, endpoint: String) -> Self {
        BlockhashSubscriber {
            latest_blockhash: Hash::default(),
            last_twenty_hashes: Vec::with_capacity(20),
            refresh_frequency,
            rpc_client: RpcClient::new(endpoint),
        }
    }

    pub async fn subscribe(blockhash_subscriber: Arc<RwLock<Self>>) -> SdkResult<()> {
        let blockhash_subscriber = blockhash_subscriber.clone();
        let blockhash_subscriber_reader = blockhash_subscriber.read().await;
        let refresh_frequency = blockhash_subscriber_reader.refresh_frequency;
        drop(blockhash_subscriber_reader);

        tokio::spawn(async move {
            loop {
                let mut blockhash_subscriber_writer = blockhash_subscriber.write().await;
                let blockhash = blockhash_subscriber_writer
                    .rpc_client
                    .get_latest_blockhash()
                    .await
                    .expect("blockhash");
                blockhash_subscriber_writer
                    .last_twenty_hashes
                    .push(blockhash);
                blockhash_subscriber_writer.latest_blockhash = blockhash;
                if blockhash_subscriber_writer.last_twenty_hashes.len() > 20 {
                    blockhash_subscriber_writer.last_twenty_hashes.remove(0);
                }
                drop(blockhash_subscriber_writer);
                tokio::time::sleep(tokio::time::Duration::from_secs(refresh_frequency)).await;
            }
        });

        Ok(())
    }

    pub fn get_latest_blockhash(&self) -> Hash {
        self.latest_blockhash
    }

    pub fn get_valid_blockhash(&self) -> Hash {
        *self
            .last_twenty_hashes
            .first()
            .unwrap_or(&self.latest_blockhash)
    }
}

#[cfg(test)]
mod tests {

    #[cfg(feature = "rpc_tests")]
    #[tokio::test]
    async fn test_blockhash_subscribe() {
        use super::*;

        let rpc = "rpc";
        let blockhash_subscriber =
            Arc::new(RwLock::new(BlockhashSubscriber::new(2, rpc.to_string())));
        BlockhashSubscriber::subscribe(blockhash_subscriber.clone())
            .await
            .expect("subscribe blockhash");

        for i in 0..=10 {
            let blockhash_subscriber_reader = blockhash_subscriber.read().await;
            let latest_blockhash = blockhash_subscriber_reader.get_latest_blockhash();
            let valid_blockhash = blockhash_subscriber_reader.get_valid_blockhash();
            drop(blockhash_subscriber_reader);
            dbg!(
                "{}: Latest blockhash: {:?}, Valid blockhash: {:?}",
                i,
                latest_blockhash,
                valid_blockhash
            );
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        }
    }
}
