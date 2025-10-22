use solana_sdk::pubkey::Pubkey;

use crate::{
    account_map::AccountMap,
    accounts::User,
    dlob::{DLOBNotifier, DLOB},
    grpc::AccountUpdate,
    types::MarketId,
    DriftClient,
};

/// Convenience builder for constructing and managing an event driven [`DLOB`] instance.
/// It should be plugged into a gRPC subscription to receive live order updates and slot changes.
///
/// ```example(no_run)
/// use drift_rs::dlob::builder::DLOBBuilder;
/// use drift_rs::types::MarketId;
///
/// // Define the markets you want to track
/// let market_ids = vec![MarketId::new(1), MarketId::new(2)];
///
/// // Construct the DLOBBuilder
/// let builder = DLOBBuilder::new(market_ids);
///
/// // setup grpc client...
///     let _res = drift
///     .grpc_subscribe(
///         grpc_url,
///         grpc_x_token,
///         GrpcSubscribeOpts::default()
///             .commitment(CommitmentLevel::Processed)
///             .usermap_on()
///             .on_user_account(builder.account_update_handler(drift.backend().account_map()))
///             .on_slot(builder.slot_update_handler(drift.clone())),
///         true, // sync all the accounts on startup (required to populate the usermap)
///     )
///    .await;
///
/// // Access the underlying DLOB
/// let dlob = builder.dlob();
/// ```
pub struct DLOBBuilder<'a> {
    dlob: &'a DLOB,
    notifier: DLOBNotifier,
    market_ids: Vec<MarketId>,
}

impl<'a> DLOBBuilder<'a> {
    /// Initialize a new DLOBBuilder instance
    ///
    /// ## Params
    ///
    /// * `market_ids` - to build DLOB for
    /// * `account_map` - account_map with initial User accounts (i.e orders) to bootstrap orderbook
    ///
    pub fn new(market_ids: Vec<MarketId>, account_map: &AccountMap) -> Self {
        let dlob = Box::leak(Box::new(DLOB::default()));
        let notifier = dlob.spawn_notifier();

        let notifier_ref = notifier.clone();
        account_map.iter_accounts_with::<User>(move |pubkey, user, slot| {
            notifier_ref.user_update(*pubkey, None, &user, slot);
        });

        Self {
            dlob,
            notifier,
            market_ids,
        }
    }

    /// Return the DLOB instance
    pub fn dlob(&self) -> &'a DLOB {
        self.dlob
    }

    /// Returns a handler suitable for use in grpc_subscribe's on_account
    ///
    /// This will notify the DLOB of order changes based on User account updates
    pub fn account_update_handler<'b>(
        &self,
        account_map: &'b AccountMap,
    ) -> impl Fn(&AccountUpdate) + Send + Sync + 'b {
        let notifier = self.notifier.clone();
        move |update| {
            let new_user = crate::utils::deser_zero_copy(update.data);
            let old_user = account_map
                .account_data_and_slot::<User>(&update.pubkey)
                .map(|x| x.data);
            notifier.user_update(update.pubkey, old_user.as_ref(), new_user, update.slot);
        }
    }

    pub fn load_user(&self, pubkey: Pubkey, user: &User, slot: u64) {
        self.notifier.user_update(pubkey, None, user, slot);
    }

    /// Returns a handler suitable for use in grpc_subscribe's on_slot
    ///
    /// This will notify the DLOB of slot/price updates for the given markets and send the slot to slot_tx.
    pub fn slot_update_handler(&self, drift: DriftClient) -> impl Fn(u64) + Send + Sync + 'static {
        let notifier = self.notifier.clone();
        let market_ids = self.market_ids.clone();
        move |new_slot| {
            for market in &market_ids {
                if let Ok(oracle_data) =
                    drift.try_get_mmoracle_for_perp_market(market.index(), new_slot)
                {
                    notifier.slot_update(*market, oracle_data.price as u64, new_slot);
                }
            }
        }
    }
}
