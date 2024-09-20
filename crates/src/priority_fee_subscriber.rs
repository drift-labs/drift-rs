use std::{
    sync::{Arc, Mutex, RwLock},
    time::Duration,
};

use log::warn;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{clock::Slot, pubkey::Pubkey};
use tokio::sync::oneshot;

pub const DEFAULT_REFRESH_FREQUENCY: Duration = Duration::from_millis(5 * 400);
pub const DEFAULT_SLOT_WINDOW: Slot = 30;

/// Subscribes to network priority fees given some accounts
pub struct PriorityFeeSubscriber {
    config: PriorityFeeSubscriberConfig,
    /// Accounts to lock for the priority fee calculation
    writeable_accounts: Vec<Pubkey>,
    rpc_client: RpcClient,
    latest_fees: RwLock<Vec<u64>>,
    /// unsubscriber handler
    unsub: Mutex<Option<oneshot::Sender<()>>>,
}

/// Options for `PriorityFeeSubscriber`
pub struct PriorityFeeSubscriberConfig {
    /// how frequently to re-poll the priority fee
    pub refresh_frequency: Option<Duration>,
    /// # of historic slots to consider in the fee calculation
    /// max: 150
    pub window: Option<Slot>,
}

impl PriorityFeeSubscriber {
    /// Create new `PriorityFeeSubscriber` assuming a tx will lock `writeable_accounts`
    pub fn new(endpoint: String, writeable_accounts: &[Pubkey]) -> Self {
        Self::with_config(
            RpcClient::new(endpoint),
            writeable_accounts,
            PriorityFeeSubscriberConfig {
                refresh_frequency: Some(DEFAULT_REFRESH_FREQUENCY),
                window: Some(DEFAULT_SLOT_WINDOW),
            },
        )
    }

    /// Create new `PriorityFeeSubscriber` assuming a tx will lock `writeable_accounts`
    pub fn with_config(
        rpc_client: RpcClient,
        writeable_accounts: &[Pubkey],
        config: PriorityFeeSubscriberConfig,
    ) -> Self {
        Self {
            config,
            writeable_accounts: writeable_accounts.to_vec(),
            rpc_client,
            latest_fees: RwLock::new(Default::default()),
            unsub: Mutex::default(),
        }
    }

    /// Start the priority fee subscriber task
    ///
    /// Returns a handle to the subscriber for querying results
    pub fn subscribe(self) -> Arc<PriorityFeeSubscriber> {
        let (unsub_tx, mut unsub_rx) = oneshot::channel();
        {
            let mut guard = self.unsub.try_lock().expect("uncontested");
            guard.replace(unsub_tx);
        }

        let arc = Arc::new(self);

        tokio::spawn({
            let this = Arc::clone(&arc);
            async move {
                let mut refresh = tokio::time::interval(
                    this.config
                        .refresh_frequency
                        .unwrap_or(DEFAULT_REFRESH_FREQUENCY),
                );
                let window = this
                    .config
                    .window
                    .unwrap_or(DEFAULT_SLOT_WINDOW)
                    .clamp(5, 150) as usize; // 150 max slots from node cache

                loop {
                    let _ = refresh.tick().await;
                    let response = this
                        .rpc_client
                        .get_recent_prioritization_fees(this.writeable_accounts.as_slice())
                        .await;

                    match response {
                        Ok(response) => {
                            let mut latest_fees: Vec<u64> = response
                                .iter()
                                .take(window)
                                .map(|x| x.prioritization_fee)
                                .collect();
                            latest_fees.sort_unstable();
                            let mut current_fees = this.latest_fees.write().expect("acquired");
                            *current_fees = latest_fees;
                        }
                        Err(err) => {
                            warn!("failed to fetch priority fee: {err:?}");
                        }
                    }

                    if unsub_rx.try_recv().is_ok() {
                        warn!("unsubscribing priority fees");
                        break;
                    }
                }
            }
        });

        arc
    }

    /// Stop the associated network subscription task
    pub fn unsubscribe(&self) {
        let mut guard = self.unsub.lock().expect("acquired");
        if let Some(unsub) = guard.take() {
            if unsub.send(()).is_err() {
                log::error!("couldn't unsubscribe");
            }
        }
    }

    /// Returns the median priority fee in micro-lamports over the lookback window
    pub fn priority_fee(&self) -> u64 {
        self.priority_fee_nth(0.5)
    }

    /// Returns the n-th percentile priority fee in micro-lamports over the lookback window
    /// `precentile` given as decimal 0.0 < n <= 1.0
    pub fn priority_fee_nth(&self, percentile: f32) -> u64 {
        let percentile = percentile.min(1.0);
        let lock = self.latest_fees.read().expect("acquired");
        let idx = ((lock.len() - 1) as f32 * percentile).round() as usize;
        lock[idx]
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use solana_client::{
        rpc_client::Mocks, rpc_request::RpcRequest, rpc_response::RpcPrioritizationFee,
    };

    use super::*;

    #[tokio::test]
    async fn priority_fee_subscribe() {
        let _ = env_logger::try_init();
        let account_one = Pubkey::new_unique();
        let account_two = Pubkey::new_unique();

        let mut response_mocks = Mocks::default();
        let recent_fees: Vec<RpcPrioritizationFee> = [1, 3, 5, 6, 4, 7, 2, 9, 8]
            .into_iter()
            .enumerate()
            .map(|(i, f)| RpcPrioritizationFee {
                slot: i as u64,
                prioritization_fee: f,
            })
            .collect();

        response_mocks.insert(RpcRequest::GetRecentPrioritizationFees, json!(recent_fees));

        let mock_rpc = RpcClient::new_mock_with_mocks(
            "https://api.mainnet-beta.solana.com".into(),
            response_mocks,
        );
        let writeable_accounts = &[account_one, account_two];

        let pf = PriorityFeeSubscriber::with_config(
            mock_rpc,
            writeable_accounts,
            PriorityFeeSubscriberConfig {
                refresh_frequency: Some(Duration::from_secs(5)),
                window: Some(100),
            },
        );

        // test
        let pf = pf.subscribe();
        tokio::time::sleep(Duration::from_millis(100)).await; // wait for subscriber to populate

        let pf_median = pf.priority_fee();
        assert_eq!(pf_median, 5);
        let pf_99 = pf.priority_fee_nth(0.99);
        assert_eq!(pf_99, 9);
        let pf_05 = pf.priority_fee_nth(0.05);
        assert_eq!(pf_05, 1);
    }
}
