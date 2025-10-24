use std::{
    cmp::Reverse,
    collections::BTreeMap,
    fmt::Debug,
    iter::Peekable,
    sync::atomic::AtomicU64,
    time::{SystemTime, UNIX_EPOCH},
};

use arrayvec::ArrayVec;
use dashmap::{mapref::one::RefMut, DashMap};
use fxhash::FxBuildHasher;
use solana_sdk::pubkey::Pubkey;

use crate::{
    constants::ProgramData,
    dlob::util::order_hash,
    types::{
        accounts::{PerpMarket, User},
        MarketId, MarketType, Order, OrderStatus, OrderTriggerCondition, OrderType,
        PositionDirection,
    },
};

pub mod builder;
#[cfg(test)]
mod tests;
pub mod types;
pub mod util;

pub use types::*;
pub use util::OrderDelta;

/// log target
const TARGET: &str = "dlob";

type Direction = PositionDirection;
type MetadataMap = DashMap<u64, OrderMetadata, FxBuildHasher>;

/// Collection of orders with fixed prices e.g. resting limit orders
#[derive(Debug)]
struct Orders<T: OrderKey + Debug + Clone> {
    pub bids: BTreeMap<Reverse<T::Key>, T>,
    pub asks: BTreeMap<T::Key, T>,
}

impl<T: OrderKey + Clone + Debug> Default for Orders<T> {
    fn default() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }
}

impl<T: Clone + Debug + From<(u64, Order)> + OrderKey> Orders<T> {
    fn insert_raw(&mut self, is_bid: bool, order: T) {
        if is_bid {
            self.bids.insert(Reverse(order.key()), order);
        } else {
            self.asks.insert(order.key(), order);
        }
    }
    pub fn insert(&mut self, order_id: u64, order: Order) {
        self.insert_raw(Direction::Long == order.direction, (order_id, order).into());
    }

    pub fn remove(&mut self, order_id: u64, order: Order) -> bool {
        match order.direction {
            Direction::Long => {
                let order: T = (order_id, order).into();
                self.bids.remove(&Reverse(order.key())).is_some()
            }
            Direction::Short => {
                let order: T = (order_id, order).into();
                self.asks.remove(&order.key()).is_some()
            }
        }
    }

    pub fn update(&mut self, order_id: u64, new_order: Order, old_order: Order) -> bool {
        let remaining_size = new_order.base_asset_amount - new_order.base_asset_amount_filled;
        let old_order_for_key: T = (order_id, old_order).into();
        let old_key = old_order_for_key.key();
        match new_order.direction {
            Direction::Long => {
                if self.bids.remove(&Reverse(old_key)).is_some() {
                    if remaining_size != 0 {
                        let order: T = (order_id, new_order).into();
                        self.insert_raw(true, order);
                    }
                    return true;
                }
            }
            Direction::Short => {
                if self.asks.remove(&old_key).is_some() {
                    if remaining_size != 0 {
                        let order: T = (order_id, new_order).into();
                        self.insert_raw(false, order);
                    }
                    return true;
                }
            }
        }
        log::warn!(target: TARGET, "update not found: {order_id}, {old_order:?}, {new_order:?}");

        false
    }
}

/// Orderbook for a specific market
///
/// Orders are kept in lists of similar types for efficient comparisons
///
/// aggregated L2 and L3 views are also maintained
#[derive(Default)]
struct Orderbook {
    /// market auctions with fixed price bounds, changes by slot
    market_orders: Orders<MarketOrder>,
    /// oracle auctions with dynamic price bounds, changes by slot
    oracle_orders: Orders<OracleOrder>,
    /// orders to fill at fixed price
    resting_limit_orders: Orders<LimitOrder>,
    /// orders to fill at offset from oracle price
    floating_limit_orders: Orders<FloatingLimitOrder>,
    /// list of (un)triggered orders
    /// triggered orders are moved to market_orders or oracle_orders
    trigger_orders: Orders<TriggerOrder>,
    /// market tick size
    market_tick_size: u64,
    /// slot where dynamic orders where last checked
    last_modified_slot: u64,
    /// market index of this book
    market_index: u16,
}

impl Orderbook {
    /// Create a new Orderbook with object pools for efficient memory management
    pub fn new(market_index: u16, market_tick_size: u64) -> Self {
        Self {
            market_orders: Orders::default(),
            oracle_orders: Orders::default(),
            resting_limit_orders: Orders::default(),
            floating_limit_orders: Orders::default(),
            trigger_orders: Orders::default(),
            market_tick_size,
            last_modified_slot: 0,
            market_index,
        }
    }

    /// Update auction order prices to new `slot`
    pub fn update_slot(&mut self, slot: u64) {
        log::trace!(target: TARGET,"update book slot. market:{},slot:{slot}", self.market_index);
        self.expire_auction_orders(slot);
        self.last_modified_slot = slot;
    }

    /// Expire all auctions past current `slot`
    ///
    /// limit orders with finishing auctions are moved to resting orders
    fn expire_auction_orders(&mut self, slot: u64) {
        self.market_orders.asks.retain(|_, x| {
            let is_auction_complete = x.is_auction_complete(slot);
            if is_auction_complete {
                if x.is_limit && x.size > 0 {
                    log::trace!(target: TARGET, "market auction => resting: {}@{}", x.id, slot);
                    self.resting_limit_orders
                        .insert_raw(false, x.to_limit_order());
                } else {
                    log::trace!(target: TARGET, "market auction expired: {}@{}", x.id, slot);
                }
            }
            !is_auction_complete
        });
        self.market_orders.bids.retain(|_, x| {
            let is_auction_complete = x.is_auction_complete(slot);
            if is_auction_complete {
                if x.is_limit && x.size > 0 {
                    log::trace!(target: TARGET, "market auction => resting:: {}@{}", x.id, slot);
                    self.resting_limit_orders
                        .insert_raw(true, x.to_limit_order());
                } else {
                    log::trace!(target: TARGET, "market auction expired: {}@{}", x.id, slot);
                }
            }
            !is_auction_complete
        });
        self.oracle_orders.asks.retain(|_, x| {
            let is_auction_complete = x.is_auction_complete(slot);
            if is_auction_complete {
                if x.is_limit && x.size > 0 {
                    log::trace!(target: TARGET, "oracle auction => resting:: {}@{}", x.id, slot);
                    self.floating_limit_orders
                        .insert_raw(false, x.to_floating_limit_order());
                } else {
                    log::trace!(target: TARGET, "oracle auction expired:: {}@{}", x.id, slot);
                }
            }
            !is_auction_complete
        });
        self.oracle_orders.bids.retain(|_, x| {
            let is_auction_complete = x.is_auction_complete(slot);
            if is_auction_complete {
                if x.is_limit && x.size > 0 {
                    log::trace!(target: TARGET, "oracle auction => resting:: {}@{}", x.id, slot);
                    self.floating_limit_orders
                        .insert_raw(true, x.to_floating_limit_order());
                } else {
                    log::trace!(target: TARGET, "oracle auction expired:: {}@{}", x.id, slot);
                }
            }
            !is_auction_complete
        });
    }

    pub fn get_maker_bids_l3(&self, oracle_price: u64, metadata: &MetadataMap) -> Vec<L3Order> {
        let mut result = Vec::with_capacity(
            self.resting_limit_orders.bids.len() + self.floating_limit_orders.bids.len(),
        );
        let buffer_s = 4;
        let now_unix_s = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + buffer_s;

        result.extend(
            self.resting_limit_orders
                .bids
                .values()
                .filter(|o| !o.is_expired(now_unix_s))
                .filter_map(|o| {
                    metadata.get(&o.id).map(|meta| L3Order {
                        price: o.get_price(),
                        max_ts: o.max_ts,
                        size: o.size,
                        reduce_only: o.reduce_only,
                        order_id: meta.order_id,
                        user: meta.user,
                        kind: meta.kind,
                    })
                }),
        );
        result.extend(
            self.floating_limit_orders
                .bids
                .values()
                .filter(|o| !o.is_expired(now_unix_s))
                .filter_map(|o| {
                    metadata.get(&o.id).map(|meta| L3Order {
                        price: o.get_price(oracle_price, self.market_tick_size),
                        max_ts: o.max_ts,
                        size: o.size,
                        reduce_only: o.reduce_only,
                        order_id: meta.order_id,
                        user: meta.user,
                        kind: meta.kind,
                    })
                }),
        );

        // Sort by price in descending order (best bid first)
        result.sort_by(|a, b| b.price.cmp(&a.price));
        result
    }

    pub fn get_maker_asks_l3(&self, oracle_price: u64, metadata: &MetadataMap) -> Vec<L3Order> {
        let mut result = Vec::with_capacity(
            self.resting_limit_orders.asks.len() + self.floating_limit_orders.asks.len(),
        );
        let buffer_s = 4;
        let now_unix_s = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + buffer_s;

        result.extend(
            self.resting_limit_orders
                .asks
                .values()
                .filter(|o| !o.is_expired(now_unix_s))
                .filter_map(|o| {
                    metadata.get(&o.id).map(|meta| L3Order {
                        price: o.get_price(),
                        max_ts: o.max_ts,
                        size: o.size,
                        reduce_only: o.reduce_only,
                        order_id: meta.order_id,
                        user: meta.user,
                        kind: meta.kind,
                    })
                }),
        );
        result.extend(
            self.floating_limit_orders
                .asks
                .values()
                .filter(|o| !o.is_expired(now_unix_s))
                .filter_map(|o| {
                    metadata.get(&o.id).map(|meta| L3Order {
                        price: o.get_price(oracle_price, self.market_tick_size),
                        max_ts: o.max_ts,
                        size: o.size,
                        reduce_only: o.reduce_only,
                        order_id: meta.order_id,
                        user: meta.user,
                        kind: meta.kind,
                    })
                }),
        );

        // Sort by price in descending order (best bid first)
        result.sort_by(|a, b| b.price.cmp(&a.price));
        result
    }

    fn get_limit_bids(&self, oracle_price: u64) -> Vec<LimitOrderView> {
        let mut result = Vec::with_capacity(
            self.resting_limit_orders.bids.len() + self.floating_limit_orders.bids.len(),
        );
        let buffer_s = 4;
        let now_unix_s = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + buffer_s;
        result.extend(
            self.resting_limit_orders
                .bids
                .values()
                .filter(|o| !o.is_expired(now_unix_s))
                .map(|o| LimitOrderView {
                    id: o.id,
                    price: o.get_price(),
                    size: o.size,
                    post_only: o.post_only,
                    slot: o.slot,
                }),
        );
        result.extend(
            self.floating_limit_orders
                .bids
                .values()
                .filter(|o| !o.is_expired(now_unix_s))
                .map(|o| LimitOrderView {
                    id: o.id,
                    price: o.get_price(oracle_price, self.market_tick_size),
                    size: o.size,
                    slot: o.slot,
                    post_only: o.post_only,
                }),
        );

        // Sort by price in descending order (best bid first)
        result.sort_by(|a, b| b.price.cmp(&a.price));
        result
    }

    fn get_limit_asks(&self, oracle_price: u64) -> Vec<LimitOrderView> {
        let mut result = Vec::with_capacity(
            self.resting_limit_orders.asks.len() + self.floating_limit_orders.asks.len(),
        );
        let buffer_s = 4;
        let now_unix_s = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + buffer_s;
        result.extend(
            self.resting_limit_orders
                .asks
                .values()
                .filter(|o| !o.is_expired(now_unix_s))
                .map(|o| LimitOrderView {
                    id: o.id,
                    price: o.get_price(),
                    size: o.size,
                    slot: o.slot,
                    post_only: o.post_only,
                }),
        );
        result.extend(
            self.floating_limit_orders
                .asks
                .values()
                .filter(|o| !o.is_expired(now_unix_s))
                .map(|o| LimitOrderView {
                    id: o.id,
                    price: o.get_price(oracle_price, self.market_tick_size),
                    size: o.size,
                    slot: o.slot,
                    post_only: o.post_only,
                }),
        );

        // Sort by price in ascending order (best ask first)
        result.sort_by(|a, b| a.price.cmp(&b.price));
        result
    }

    fn get_taker_asks(
        &self,
        min_slot: u64,
        oracle_price: u64,
        trigger_price: u64,
        perp_market: Option<&PerpMarket>,
    ) -> Vec<(u64, u64, u64)> {
        let mut result = Vec::with_capacity(
            self.market_orders.asks.len()
                + self.oracle_orders.asks.len()
                + self.trigger_orders.asks.len(),
        );
        let slot = min_slot.max(self.last_modified_slot);

        result.extend(self.market_orders.asks.values().map(|o| {
            (
                o.id,
                o.get_price(slot, oracle_price, self.market_tick_size),
                o.size,
            )
        }));
        result.extend(self.oracle_orders.asks.values().map(|o| {
            (
                o.id,
                o.get_price(slot, oracle_price, self.market_tick_size),
                o.size(),
            )
        }));
        result.extend(self.trigger_orders.asks.values().filter_map(|o| {
            // checking untriggered orders that will trigger at current oracle price
            if o.will_trigger_at(trigger_price) {
                o.get_price(slot, oracle_price, perp_market)
                    .ok()
                    .map(|p| (o.id, p, o.size))
            } else {
                None
            }
        }));

        // Sort by price in ascending order (best ask first)
        result.sort_by(|a, b| a.1.cmp(&b.1));
        result
    }

    /// Get taker ask orders (market, oracle, trigger orders)
    /// Orders are sorted by price ascending (best ask first).
    ///
    /// # Parameters
    ///
    /// * `oracle_price` - Current oracle price for dynamic order pricing
    /// * `trigger_price` - Price threshold for trigger order evaluation
    /// * `perp_market` - Optional perp market for trigger order price calculations
    /// * `metadata` - Order metadata map for user and order information
    ///
    /// # Returns
    ///
    /// Returns a `Vec<L3Order>` containing detailed taker ask orders sorted by price ascending.
    pub fn get_taker_asks_l3(
        &self,
        oracle_price: u64,
        trigger_price: u64,
        perp_market: Option<&PerpMarket>,
        metadata: &MetadataMap,
    ) -> Vec<L3Order> {
        let mut result = Vec::with_capacity(
            self.market_orders.asks.len()
                + self.oracle_orders.asks.len()
                + self.trigger_orders.asks.len(),
        );
        let slot = self.last_modified_slot;

        result.extend(self.market_orders.asks.values().filter_map(|o| {
            metadata.get(&o.id).map(|meta| L3Order {
                price: o.get_price(slot, oracle_price, self.market_tick_size),
                max_ts: o.max_ts,
                size: o.size,
                reduce_only: o.reduce_only,
                order_id: meta.order_id,
                user: meta.user,
                kind: meta.kind,
            })
        }));
        result.extend(self.oracle_orders.asks.values().filter_map(|o| {
            metadata.get(&o.id).map(|meta| L3Order {
                price: o.get_price(slot, oracle_price, self.market_tick_size),
                max_ts: o.max_ts,
                size: o.size(),
                reduce_only: o.reduce_only,
                order_id: meta.order_id,
                user: meta.user,
                kind: meta.kind,
            })
        }));
        let unix_now_s = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        result.extend(self.trigger_orders.asks.values().filter_map(|o| {
            // checking untriggered orders that will trigger at current oracle price
            if o.will_trigger_at(trigger_price) {
                o.get_price(slot, oracle_price, perp_market)
                    .ok()
                    .and_then(|price| {
                        metadata.get(&o.id).map(|meta| L3Order {
                            price,
                            max_ts: o.max_ts.max(unix_now_s + 30),
                            size: o.size,
                            reduce_only: o.reduce_only,
                            order_id: meta.order_id,
                            user: meta.user,
                            kind: meta.kind,
                        })
                    })
            } else {
                None
            }
        }));

        // Sort by price in ascending order (best ask first)
        result.sort_by(|a, b| a.price.cmp(&b.price));
        result
    }

    fn get_taker_bids(
        &self,
        min_slot: u64,
        oracle_price: u64,
        trigger_price: u64,
        perp_market: Option<&PerpMarket>,
    ) -> Vec<(u64, u64, u64)> {
        let mut result = Vec::with_capacity(
            self.market_orders.bids.len()
                + self.oracle_orders.bids.len()
                + self.trigger_orders.bids.len(),
        );
        let slot = min_slot.max(self.last_modified_slot);

        result.extend(self.market_orders.bids.values().map(|o| {
            (
                o.id,
                o.get_price(slot, oracle_price, self.market_tick_size),
                o.size,
            )
        }));
        result.extend(self.oracle_orders.bids.values().map(|o| {
            (
                o.id,
                o.get_price(slot, oracle_price, self.market_tick_size),
                o.size(),
            )
        }));

        result.extend(
            self.trigger_orders
                .bids
                .values()
                // rely on trigger order sorting for early exit
                .filter_map(|o| {
                    // checking untriggered orders that will trigger at current oracle price
                    if o.will_trigger_at(trigger_price) {
                        o.get_price(slot, oracle_price, perp_market)
                            .ok()
                            .map(|p| (o.id, p, o.size))
                    } else {
                        None
                    }
                }),
        );

        // Sort by price in descending order (best bid first)
        result.sort_by(|a, b| b.1.cmp(&a.1));
        result
    }

    /// Get taker bid orders (market, oracle, trigger orders)
    /// Orders are sorted by price descending (best bid first).
    ///
    /// # Parameters
    ///
    /// * `oracle_price` - Current oracle price for dynamic order pricing
    /// * `trigger_price` - Price threshold for trigger order evaluation
    /// * `perp_market` - Optional perp market for trigger order price calculations
    /// * `metadata` - Order metadata map for user and order information
    ///
    /// # Returns
    ///
    /// Returns a `Vec<L3Order>` containing detailed taker bid orders sorted by price descending.
    pub fn get_taker_bids_l3(
        &self,
        oracle_price: u64,
        trigger_price: u64,
        perp_market: Option<&PerpMarket>,
        metadata: &MetadataMap,
    ) -> Vec<L3Order> {
        let mut result = Vec::with_capacity(
            self.market_orders.bids.len()
                + self.oracle_orders.bids.len()
                + self.trigger_orders.bids.len(),
        );
        let slot = self.last_modified_slot;

        result.extend(self.market_orders.bids.values().filter_map(|o| {
            metadata.get(&o.id).map(|meta| L3Order {
                price: o.get_price(slot, oracle_price, self.market_tick_size),
                max_ts: o.max_ts,
                size: o.size,
                reduce_only: o.reduce_only,
                order_id: meta.order_id,
                user: meta.user,
                kind: meta.kind,
            })
        }));
        result.extend(self.oracle_orders.bids.values().filter_map(|o| {
            metadata.get(&o.id).map(|meta| L3Order {
                price: o.get_price(slot, oracle_price, self.market_tick_size),
                max_ts: o.max_ts,
                size: o.size(),
                reduce_only: o.reduce_only,
                order_id: meta.order_id,
                user: meta.user,
                kind: meta.kind,
            })
        }));

        let unix_now_s = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        result.extend(
            self.trigger_orders
                .bids
                .values()
                // rely on trigger order sorting for early exit
                .filter_map(|o| {
                    // checking untriggered orders that will trigger at current oracle price
                    if o.will_trigger_at(trigger_price) {
                        o.get_price(slot, oracle_price, perp_market)
                            .ok()
                            .and_then(|price| {
                                metadata.get(&o.id).map(|meta| L3Order {
                                    price,
                                    max_ts: o.max_ts.max(unix_now_s + 30), // 30s is the default max_ts
                                    size: o.size,
                                    reduce_only: o.reduce_only,
                                    order_id: meta.order_id,
                                    user: meta.user,
                                    kind: meta.kind,
                                })
                            })
                    } else {
                        None
                    }
                }),
        );

        // Sort by price in descending order (best bid first)
        result.sort_by(|a, b| b.price.cmp(&a.price));
        result
    }
}

/// Channel for sending User order updates to DLOB instance
#[derive(Clone)]
pub struct DLOBNotifier {
    sender: crossbeam::channel::Sender<DLOBEvent>,
}

impl DLOBNotifier {
    pub fn new(sender: crossbeam::channel::Sender<DLOBEvent>) -> Self {
        Self { sender }
    }

    /// Updates the DLOB with user account changes by comparing old and new user states.
    ///
    /// This method processes user account updates and sends appropriate DLOB events to maintain
    /// the order book state. It handles two scenarios:
    /// 1. **User Update**: When both old and new user states are provided, it compares the orders
    ///    and sends delta events for changes (creates, updates, removes)
    /// 2. **New User**: When only a new user is provided, it creates events for all open orders
    ///
    /// # Parameters
    ///
    /// * `pubkey` - The public key of the user account being updated
    /// * `old_user` - The previous state of the user account, if any. `None` indicates a new user
    /// * `new_user` - The current state of the user account
    /// * `slot` - The slot number when this update occurred
    ///
    /// # Panics
    ///
    /// This method will panic if it cannot send events to the DLOB channel, which typically
    /// indicates the DLOB processing thread has been dropped or the channel is full.
    ///
    /// # Example
    ///
    /// ```rust
    /// // Update existing user
    /// notifier.user_update(user_pubkey, Some(&old_user), &new_user, current_slot);
    ///
    /// // Add new user
    /// notifier.user_update(user_pubkey, None, &new_user, current_slot);
    /// ```
    pub fn user_update(&self, pubkey: Pubkey, old_user: Option<&User>, new_user: &User, slot: u64) {
        let deltas = match old_user {
            Some(old_user) => crate::dlob::util::compare_user_orders(pubkey, &old_user, &new_user),
            None => new_user
                .orders
                .iter()
                .filter(|o| {
                    o.status == OrderStatus::Open
                        && o.base_asset_amount > o.base_asset_amount_filled
                })
                .map(|o| OrderDelta::Create {
                    order: *o,
                    user: pubkey,
                })
                .collect(),
        };
        self.sender
            .send(DLOBEvent::Deltas { deltas, slot })
            .expect("Failed to send DLOB event - channel may be closed");
    }

    #[inline]
    pub fn slot_update(&self, slot: u64) {
        self.sender
            .send(DLOBEvent::SlotUpdate { slot })
            .expect("Failed to send slot update event - channel may be closed");
    }
}

/// Aggregates orderbooks for multiple markets
pub struct DLOB {
    /// Map from market to orderbook
    markets: DashMap<MarketId, Orderbook, FxBuildHasher>,
    /// Map from DLOB internal order ID to order metadata
    metadata: MetadataMap,
    /// static drift program data e.g market tick sizes
    program_data: &'static ProgramData,
    /// last slot update
    last_modified_slot: AtomicU64,
}

impl Default for DLOB {
    fn default() -> Self {
        Self {
            markets: DashMap::default(),
            metadata: DashMap::default(),
            program_data: Box::leak(Box::new(ProgramData::uninitialized())),
            last_modified_slot: Default::default(),
        }
    }
}

impl DLOB {
    /// Provides a writer channel into the DLOB which acts as a sink for external events
    pub fn spawn_notifier(&'static self) -> DLOBNotifier {
        let (tx, rx) = crossbeam::channel::bounded(2048);
        std::thread::spawn(move || {
            dbg!(rx.len());
            while let Ok(event) = rx.recv() {
                match event {
                    DLOBEvent::SlotUpdate { slot } => {
                        self.update_slot(slot);
                    }
                    DLOBEvent::Deltas { slot, deltas } => {
                        for delta in deltas {
                            match delta {
                                OrderDelta::Create { user, order } => {
                                    self.insert_order(&user, order);
                                }
                                OrderDelta::Update {
                                    user,
                                    new_order,
                                    old_order,
                                } => {
                                    self.update_order(&user, slot, new_order, old_order);
                                }
                                OrderDelta::Remove { user, order } => {
                                    self.remove_order(&user, slot, order);
                                }
                            }
                        }
                    }
                }
            }
            log::error!(target: TARGET, "notifier thread finished");
        });

        DLOBNotifier::new(tx)
    }

    /// run function on a market Orderbook
    fn with_orderbook_mut(&self, market_id: &MarketId, f: impl Fn(RefMut<MarketId, Orderbook>)) {
        let ob = self.markets.entry(*market_id).or_insert({
            // initialize book on first write
            let market_tick_size: u64 = match market_id.kind() {
                MarketType::Perp => self
                    .program_data
                    .perp_market_config_by_index(market_id.index())
                    .map(|m| m.amm.order_tick_size)
                    .unwrap_or(1),
                MarketType::Spot => self
                    .program_data
                    .spot_market_config_by_index(market_id.index())
                    .map(|m| m.order_tick_size)
                    .unwrap_or(1),
            };
            Orderbook::new(market_id.index(), market_tick_size)
        });
        f(ob);
    }

    /// Update orderbook slot for all markets
    fn update_slot(&self, slot: u64) {
        let last_modified_slot = self
            .last_modified_slot
            .load(std::sync::atomic::Ordering::Relaxed);

        if slot < last_modified_slot {
            log::warn!(
                target: TARGET, "ignoring out of order slot update: update:{slot},ours:{last_modified_slot}",
            );
            return;
        }

        for mut market in self.markets.iter_mut() {
            market.value_mut().update_slot(slot);
        }

        self.last_modified_slot
            .store(slot, std::sync::atomic::Ordering::Relaxed)
    }

    /// Get L2 Book from current orders with `oracle_price`
    pub fn get_l2_snapshot(
        &self,
        market_index: u16,
        market_type: MarketType,
        oracle_price: u64,
    ) -> L2Book {
        let book = self
            .markets
            .get(&MarketId::new(market_index, market_type))
            .expect("orderbook exists for market");
        L2Book::default().load_orderbook(&book, oracle_price)
    }

    /// Get L3 Book from current orders with `oracle_price`
    pub fn get_l3_snapshot(
        &self,
        market_index: u16,
        market_type: MarketType,
        oracle_price: u64,
    ) -> L3Book {
        let book = self
            .markets
            .get(&MarketId::new(market_index, market_type))
            .expect("orderbook exists for market");
        L3Book::default().load_orderbook(&book, &self.metadata, oracle_price)
    }

    pub fn find_crossing_region(
        &self,
        slot: u64,
        oracle_price: u64,
        market_index: u16,
        market_type: MarketType,
    ) -> Option<CrossingRegion> {
        let market_id = MarketId::new(market_index, market_type);
        let book = self.markets.get(&market_id)?;
        let bids = book.get_limit_bids(oracle_price);
        let asks = book.get_limit_asks(oracle_price);

        if bids.is_empty() || asks.is_empty() {
            return None;
        }
        let best_bid = bids[0].price;
        let best_ask = asks[0].price;
        if best_bid < best_ask {
            return None;
        }

        let crossing_bids = bids
            .iter()
            .take_while(|b| b.price >= best_ask)
            .filter_map(|b| {
                self.metadata.get(&b.id).map(|m| CrossingOrder {
                    order_view: b.clone(),
                    metadata: *m.value(),
                })
            })
            .collect();

        let crossing_asks = asks
            .iter()
            .take_while(|a| a.price <= best_bid)
            .filter_map(|a| {
                self.metadata.get(&a.id).map(|m| CrossingOrder {
                    order_view: a.clone(),
                    metadata: *m.value(),
                })
            })
            .collect();

        Some(CrossingRegion {
            slot,
            crossing_bids,
            crossing_asks,
        })
    }

    fn update_order(&self, user: &Pubkey, slot: u64, new_order: Order, old_order: Order) {
        let order_id = order_hash(user, new_order.order_id);
        log::trace!(target: TARGET, "update order: {order_id}{},{:?}", old_order.order_id, new_order.order_type);

        if new_order.status != OrderStatus::Open {
            log::info!(target: TARGET, "update into remove: {order_id:?}");
            self.remove_order(user, slot, new_order);
            return;
        }

        self.with_orderbook_mut(&MarketId::new(new_order.market_index, new_order.market_type), |mut orderbook| {
            if let Some(metadata) = self.metadata.get(&order_id) {
                log::trace!(target: TARGET, "update ({order_id}): {:?}", metadata.kind);
                match metadata.kind {
                    OrderKind::Market | OrderKind::MarketTriggered => {
                        orderbook.market_orders.update(order_id, new_order, old_order);
                    }
                    OrderKind::Oracle | OrderKind::OracleTriggered => {
                        orderbook.oracle_orders.update(order_id, new_order, old_order);
                    }
                    OrderKind::LimitAuction | OrderKind::LimitTriggered => {
                        // if the auction completed, check if order moved to resting
                        if (new_order.slot + new_order.auction_duration as u64) > slot {
                            log::trace!(target: TARGET, "update limit auction: {order_id}");
                            orderbook.market_orders.update(order_id, new_order, old_order);
                        } else {
                            log::trace!(target: TARGET, "update limit auction (resting): {order_id}");
                            orderbook.market_orders.remove(order_id, old_order);
                            orderbook.resting_limit_orders.update(order_id, new_order, old_order);
                        }
                    }
                    OrderKind::FloatingLimitAuction => {
                        // if the auction completed, check if order moved to resting
                        if (new_order.slot + new_order.auction_duration as u64) > slot {
                            log::trace!(target: TARGET, "update oracle limit: {order_id}");
                            orderbook.oracle_orders.update(order_id, new_order, old_order);
                        } else {
                            log::trace!(target: TARGET, "update oracle limit (resting): {order_id}");
                            orderbook.oracle_orders.remove(order_id, old_order);
                            orderbook.floating_limit_orders.update(order_id, new_order, old_order);
                        }
                    }
                    OrderKind::Limit => {
                        orderbook.resting_limit_orders.update(order_id, new_order, old_order);
                    }
                    OrderKind::FloatingLimit => {
                        orderbook.floating_limit_orders.update(order_id, new_order, old_order);
                    }
                    OrderKind::TriggerMarket => {
                        log::trace!(target: TARGET, "update trigger market order: {order_id},{:?}", new_order);
                        match new_order.trigger_condition {
                            OrderTriggerCondition::Above | OrderTriggerCondition::Below => {
                                orderbook.trigger_orders.update(order_id, new_order, old_order);
                            }
                            OrderTriggerCondition::TriggeredAbove | OrderTriggerCondition::TriggeredBelow => {
                                // order has been triggered, its an ordinary auction order now
                                orderbook.trigger_orders.remove(order_id, old_order);
                                let new_kind = if new_order.is_oracle_trigger_market() {
                                    orderbook.oracle_orders.insert(order_id, new_order);
                                    OrderKind::OracleTriggered
                                } else {
                                    orderbook.market_orders.insert(order_id, new_order);
                                    OrderKind::MarketTriggered
                                };
                                drop(metadata);
                                self.metadata.entry(order_id).and_modify(|o| o.kind = new_kind);
                            }
                        }
                    }
                    OrderKind::TriggerLimit => {
                        log::trace!(target: TARGET, "update trigger limit order: {order_id},{:?}", new_order);
                        match new_order.trigger_condition {
                            OrderTriggerCondition::Above | OrderTriggerCondition::Below => {
                                orderbook.trigger_orders.update(order_id, new_order, old_order);
                            }
                            OrderTriggerCondition::TriggeredAbove | OrderTriggerCondition::TriggeredBelow => {
                                // order has been triggered, its an ordinary auction order now
                                log::trace!(target: TARGET, "trigger limit => market auction: {order_id}");
                                orderbook.trigger_orders.remove(order_id, old_order);
                                orderbook.market_orders.insert(order_id, new_order);
                                drop(metadata); // drop the borrow
                                self.metadata.entry(order_id).and_modify(|o| o.kind = OrderKind::LimitTriggered);
                            }
                        }
                    }
                }
            }
        });
    }

    fn remove_order(&self, user: &Pubkey, slot: u64, order: Order) {
        let order_id = order_hash(user, order.order_id);

        self.with_orderbook_mut(&MarketId::new(order.market_index, order.market_type), |mut orderbook| {
            if let Some((_, metadata)) = self.metadata.remove(&order_id) {
                let mut order_removed;

                log::trace!(target: TARGET, "remove order: {order_id} @ status: {:?}, kind: {:?}/{:?}, slot: {slot}", order.status, metadata.kind, order.order_type);

                match metadata.kind {
                    OrderKind::Market | OrderKind::MarketTriggered => {
                        order_removed = orderbook.market_orders.remove(order_id, order);
                    }
                    OrderKind::Oracle | OrderKind::OracleTriggered => {
                        order_removed = orderbook.oracle_orders.remove(order_id, order);
                    }
                    OrderKind::LimitAuction | OrderKind::LimitTriggered => {
                        // if the auction completed, check if order moved to resting
                        log::trace!(target: TARGET, "remove auction limit order: {order_id}");
                        order_removed = orderbook.market_orders.remove(order_id, order);
                        if !order_removed {
                            log::trace!(target: TARGET, "remove auction limit order (resting): {order_id}");
                            order_removed = orderbook.resting_limit_orders.remove(order_id, order);
                        }
                    }
                    OrderKind::FloatingLimitAuction => {
                        // if the auction completed, check if order moved to resting
                        log::trace!(target: TARGET, "remove oracle order: {order_id}, order.slot: {}, order.duration: {}", order.slot, order.auction_duration);
                        order_removed = orderbook.oracle_orders.remove(order_id, order);
                        if !order_removed {
                            log::trace!(target: TARGET, "remove oraclmargin_infoe limit order: {order_id}");
                            order_removed = orderbook.floating_limit_orders.remove(order_id, order);
                        }
                    }
                    OrderKind::Limit => {
                        order_removed = orderbook.resting_limit_orders.remove(order_id, order);
                    }
                    OrderKind::FloatingLimit => {
                        order_removed = orderbook.floating_limit_orders.remove(order_id, order);
                    }
                    OrderKind::TriggerMarket | OrderKind::TriggerLimit => {
                        log::trace!(target: TARGET, "trigger order: {order_id},{:?}", order.trigger_condition);
                        order_removed = orderbook.trigger_orders.remove(order_id, order);
                    }
                }

                let auction_expired = slot > order.slot + order.auction_duration as u64;
                if !order_removed && !auction_expired  {
                    log::warn!(target: TARGET, "failed to remove order {order_id} from orderbook. order_kind: {:?}, user: {}, order_id: {}", metadata.kind, metadata.user, metadata.order_id);
                }
            }
        });
    }

    fn insert_order(&self, user: &Pubkey, order: Order) {
        let order_id = order_hash(user, order.order_id);
        log::trace!(target: TARGET, "insert order: {order_id}");

        if order.base_asset_amount <= order.base_asset_amount_filled {
            log::trace!(target: TARGET, "skipping fully filled order: {order:?}");
            return;
        }

        self.with_orderbook_mut(
            &MarketId::new(order.market_index, order.market_type),
            |mut orderbook| {
               let kind = match order.order_type {
                    OrderType::Market => {
                        orderbook.market_orders.insert(order_id, order);
                        OrderKind::Market
                    }
                    OrderType::Oracle => {
                        orderbook.oracle_orders.insert(order_id, order);
                        OrderKind::Oracle
                    }
                    OrderType::Limit => {
                        /*
                        maker orders:
                            - limit order with POST_ONLY=true
                            - limit order with POST_ONLY=false and auction is completed*
                        crossing:
                            - limit orders with both POST_ONLY=true cannot cross
                            - limit orders with both POST_ONLY=FALSE can cross, *older order becomes maker
                            - limit order with POST_ONLY=TRUE can become maker for POST_ONLY=FALSE
                        */
                        let is_floating = order.oracle_price_offset != 0;
                        let is_post_only = order.post_only;
                        let is_auction = order.auction_duration != 0;
                        let order_kind = if !is_post_only {
                            match (is_auction, is_floating) {
                                (true, true) => {
                                    orderbook.oracle_orders.insert(order_id, order);
                                    OrderKind::FloatingLimitAuction
                                }
                                (true, false) => {
                                    orderbook.market_orders.insert(order_id, order);
                                    OrderKind::LimitAuction
                                }
                                (false, true) => {
                                    orderbook.floating_limit_orders.insert(order_id, order);
                                    OrderKind::FloatingLimit
                                }
                                (false, false) => {
                                    orderbook.resting_limit_orders.insert(order_id, order);
                                    OrderKind::Limit
                                }
                            }
                        } else {
                            // post only cannot have an auction (maker side only)
                            if is_floating {
                                orderbook.floating_limit_orders.insert(order_id, order);
                                OrderKind::FloatingLimit
                            } else {
                                orderbook.resting_limit_orders.insert(order_id, order);
                                OrderKind::Limit
                            }
                        };
                        log::trace!(target: TARGET, "insert limit order: {order_id},{:?}", order_kind);
                        order_kind
                    }
                    OrderType::TriggerMarket => match order.trigger_condition {
                        OrderTriggerCondition::Above | OrderTriggerCondition::Below => {
                            log::trace!(target: TARGET, "insert trigger market order: {order_id}");
                            orderbook.trigger_orders.insert(order_id, order);
                            OrderKind::TriggerMarket

                        }
                        OrderTriggerCondition::TriggeredAbove | OrderTriggerCondition::TriggeredBelow => {
                            if order.is_oracle_trigger_market() {
                                log::trace!(target: TARGET, "insert triggered oracle order: {order_id}");
                                orderbook.oracle_orders.insert(order_id, order);
                                OrderKind::OracleTriggered
                            } else {
                                log::trace!(target: TARGET, "insert triggered market order: {order_id}");
                                orderbook.market_orders.insert(order_id, order);
                                OrderKind::MarketTriggered
                           }
                        }
                    },
                    OrderType::TriggerLimit => match order.trigger_condition {
                        OrderTriggerCondition::Above | OrderTriggerCondition::Below => {
                            log::trace!(target: TARGET, "insert trigger limit order: {order_id}");
                            orderbook.trigger_orders.insert(order_id, order);
                            OrderKind::TriggerLimit
                        }
                        OrderTriggerCondition::TriggeredAbove
                        | OrderTriggerCondition::TriggeredBelow => {
                            log::trace!(target: TARGET, "insert triggered limit order: {order_id}");
                            orderbook.market_orders.insert(order_id, order);
                            OrderKind::LimitTriggered
                        }
                    },
                };
                self.metadata.insert(order_id, OrderMetadata::new(*user, kind, order.order_id, order.max_ts.unsigned_abs()));
            },
        );
    }

    /// Helper to find a crossing pair of limit orders at the top of the book, if any.
    ///
    /// The crossing orders could from the same user account and so un-fillable
    fn find_limit_cross(
        &self,
        bid: &LimitOrderView,
        ask: &LimitOrderView,
    ) -> Option<(OrderMetadata, OrderMetadata)> {
        if bid.price < ask.price {
            return None;
        }
        let bid_meta = self.metadata.get(&bid.id)?;
        let ask_meta = self.metadata.get(&ask.id)?;
        match (bid.post_only, ask.post_only) {
            (true, false) => Some((*ask_meta.value(), *bid_meta.value())),
            (false, true) => Some((*bid_meta.value(), *ask_meta.value())),
            (false, false) => {
                if bid.slot < ask.slot {
                    Some((*bid_meta.value(), *ask_meta.value()))
                } else {
                    Some((*ask_meta.value(), *bid_meta.value()))
                }
            }
            (true, true) => None,
        }
    }

    /// At the current slot return all auctions crossing resting limit orders (i.e uncross)
    ///
    /// ## Panics
    ///
    /// if market_index,market_type has not been initialized on this dlob instance
    ///
    pub fn find_crosses_for_auctions(
        &self,
        market_index: u16,
        market_type: MarketType,
        slot: u64,
        oracle_price: u64,
        trigger_price: u64,
        perp_market: Option<&PerpMarket>,
    ) -> CrossesAndTopMakers {
        let market = MarketId::new(market_index, market_type);
        let book = self.markets.get(&market).expect("market lob exists");
        let mut all_crosses = Vec::with_capacity(16);

        let vamm_bid = perp_market.map(|m| m.bid_price(None));
        let vamm_ask = perp_market.map(|m| m.ask_price(None));
        log::trace!(target: TARGET, "VAMM market={} bid={vamm_bid:?} ask={vamm_ask:?}", market_index);

        let taker_asks = book.get_taker_asks(slot, oracle_price, trigger_price, perp_market);
        let taker_bids = book.get_taker_bids(slot, oracle_price, trigger_price, perp_market);
        let mut resting_asks = book.get_limit_asks(oracle_price);
        let mut resting_bids = book.get_limit_bids(oracle_price);
        let mut vamm_taker_ask = None;
        let mut vamm_taker_bid = None;

        let mut limit_crosses = None;
        if let (Some(best_bid), Some(best_ask)) = (resting_bids.first(), resting_asks.first()) {
            // check for crossing resting limit orders
            limit_crosses = self.find_limit_cross(best_bid, best_ask);
            // check for VAMM crossing resting limit orders
            if vamm_bid.is_some_and(|v| v > best_ask.price && best_ask.post_only)
                && perp_market.is_some_and(|m| best_ask.size > m.amm.min_order_size)
            {
                vamm_taker_bid = self.metadata.get(&best_ask.id).map(|x| *x.value());
            }
            if vamm_ask.is_some_and(|v| v < best_bid.price && best_bid.post_only)
                && perp_market.is_some_and(|m| best_bid.size > m.amm.min_order_size)
            {
                vamm_taker_ask = self.metadata.get(&best_bid.id).map(|x| *x.value());
            }
        }

        let mut taker_order = TakerOrder {
            price: 0,
            size: 0,
            direction: Direction::Long,
            market_index,
            market_type,
        };

        let top_3_maker_bids: ArrayVec<Pubkey, 3> = resting_bids
            .iter()
            .take(3)
            .filter_map(|o| self.metadata.get(&o.id).map(|m| m.user))
            .collect();

        // Check for crosses between auction bids and resting asks
        for (oid, price, size) in taker_bids {
            taker_order.price = price;
            taker_order.size = size;
            taker_order.direction = Direction::Long;

            let new_crosses = self.find_crosses_for_taker_order_inner(
                slot,
                book.last_modified_slot,
                taker_order,
                resting_asks.iter_mut().peekable(),
                vamm_ask,
            );

            if let Some(metadata) = self.metadata.get(&oid) {
                if !new_crosses.is_empty() {
                    all_crosses.push((*metadata.value(), new_crosses));
                } else {
                    break;
                }
            } else {
                log::warn!(target: TARGET, "missing metadata for order: {oid}");
                continue;
            }
        }

        let top_3_maker_asks: ArrayVec<Pubkey, 3> = resting_asks
            .iter()
            .take(3)
            .filter_map(|o| self.metadata.get(&o.id).map(|m| m.user))
            .collect();

        // Check for crosses between auction asks and resting bids
        for (oid, price, size) in taker_asks {
            taker_order.price = price;
            taker_order.size = size;
            taker_order.direction = Direction::Short;

            let new_crosses = self.find_crosses_for_taker_order_inner(
                slot,
                book.last_modified_slot,
                taker_order,
                resting_bids.iter_mut().peekable(),
                vamm_bid,
            );

            if let Some(metadata) = self.metadata.get(&oid) {
                if !new_crosses.is_empty() {
                    all_crosses.push((*metadata.value(), new_crosses));
                } else {
                    break;
                }
            } else {
                log::warn!(target: TARGET, "missing metadata for order: {oid}");
                continue;
            }
        }

        CrossesAndTopMakers {
            top_maker_asks: top_3_maker_asks,
            top_maker_bids: top_3_maker_bids,
            crosses: all_crosses,
            limit_crosses,
            vamm_taker_ask,
            vamm_taker_bid,
        }
    }

    /// At the current slot and oracle price return all orders crossing a given taker order
    ///
    /// # Parameters
    ///
    /// * `current_slot` - The current slot number, used for time-sensitive order logic.
    /// * `oracle_price` - The current oracle price, used for price calculations and trigger conditions.
    /// * `taker_order` - The taker order for which to find matching maker orders. Contains price, size, direction, and market info.
    /// * `vamm_price` - An optional price from the virtual AMM (vAMM) to consider as a crossing point. If `Some`, will check if the taker order crosses this price.
    ///
    /// ## Panics
    ///
    /// if market_index,market_type has not been initialized on this dlob instance
    ///
    /// # Returns
    ///
    /// Returns a `MakerCrosses` struct containing the list of matched maker orders, the slot, whether the match is partial, and if a vAMM cross occurred.
    pub fn find_crosses_for_taker_order(
        &self,
        current_slot: u64,
        oracle_price: u64,
        taker_order: TakerOrder,
        vamm_price: Option<u64>,
    ) -> MakerCrosses {
        let market = MarketId::new(taker_order.market_index, taker_order.market_type);
        let (book_slot, mut resting_orders) = match taker_order.direction {
            Direction::Long => {
                let book = self.markets.get(&market).expect("market lob exists");
                (book.last_modified_slot, book.get_limit_asks(oracle_price))
            }
            Direction::Short => {
                let book = self.markets.get(&market).expect("market lob exists");
                (book.last_modified_slot, book.get_limit_bids(oracle_price))
            }
        };

        self.find_crosses_for_taker_order_inner(
            current_slot,
            book_slot,
            taker_order,
            resting_orders.iter_mut().peekable(),
            vamm_price,
        )
    }

    /// Find crosses for given `taker_order` consuming or updating `resting_limit_orders` upon finding a match
    fn find_crosses_for_taker_order_inner<'a>(
        &self,
        current_slot: u64,
        resting_order_slot: u64,
        taker_order: TakerOrder,
        mut resting_limit_orders: Peekable<impl Iterator<Item = &'a mut LimitOrderView>>,
        vamm_price: Option<u64>,
    ) -> MakerCrosses {
        let mut candidates = ArrayVec::<(OrderMetadata, u64, u64), 16>::new();
        let mut remaining_size = taker_order.size;

        let price_crosses = match taker_order.direction {
            Direction::Long => |taker_price: u64, maker_price: u64| taker_price >= maker_price,
            Direction::Short => |taker_price: u64, maker_price: u64| taker_price <= maker_price,
        };

        while let Some(peeked) = resting_limit_orders.peek_mut() {
            let LimitOrderView {
                id: internal_order_id,
                price: maker_price,
                size: maker_size,
                ..
            } = peeked;
            if !price_crosses(taker_order.price, *maker_price) {
                break;
            }

            let fill_size = remaining_size.min(*maker_size);

            if let Some(metadata) = self.metadata.get(internal_order_id) {
                candidates.push((*metadata.value(), *maker_price, fill_size));
                remaining_size -= fill_size;

                if fill_size == *maker_size {
                    // Fully consumed — advance the iterator
                    resting_limit_orders.next();
                } else {
                    // Partially filled — decrement in-place, do NOT advance
                    *maker_size -= fill_size;
                }

                if candidates.len() == candidates.capacity() {
                    log::debug!(target: TARGET, "reached max number crosses");
                    break;
                }
                if remaining_size == 0 {
                    break;
                }
            } else {
                log::warn!(
                    target: TARGET,
                    "metadata missing. order:{internal_order_id},check_slot:{current_slot},book_slot:{resting_order_slot}"
                );
                resting_limit_orders.next();
            }
        }

        MakerCrosses {
            has_vamm_cross: vamm_price.is_some_and(|v| price_crosses(taker_order.price, v)),
            orders: candidates,
            slot: current_slot,
            is_partial: remaining_size != 0,
            taker_direction: taker_order.direction,
        }
    }
}

#[derive(Debug, Clone)]
pub struct L3Order {
    /// point in time limit price of the order at some slot & oracle price
    pub price: u64,
    pub size: u64,
    /// order expiry ts
    pub max_ts: u64,
    pub order_id: u32,
    pub reduce_only: bool,
    pub kind: OrderKind,
    /// user subaccount of the order
    pub user: Pubkey,
}

#[derive(Debug, Default, Clone)]
pub struct L3Book {
    pub slot: u64,
    pub oracle_price: u64,
    /// Maker orders (resting limit orders)
    pub bids: Vec<L3Order>,
    /// Maker orders (resting limit orders)
    pub asks: Vec<L3Order>,
    /// Taker orders (market, oracle, trigger orders)
    pub taker_bids: Vec<L3Order>,
    /// Taker orders (market, oracle, trigger orders)
    pub taker_asks: Vec<L3Order>,
}

impl L3Book {
    /// Get the best bid and ask from maker orders (resting limit orders)
    ///
    /// Returns `(best_bid, best_ask)` where each is an `Option<&L3Order>`.
    /// The best bid is the highest price bid, and the best ask is the lowest price ask.
    /// Returns `None` if there are no orders on that side.
    pub fn bbo(&self) -> (Option<&L3Order>, Option<&L3Order>) {
        (self.bids.first(), self.asks.first())
    }

    /// Get the best bid and ask from taker orders (market, oracle, trigger orders)
    ///
    /// Returns `(best_bid, best_ask)` where each is an `Option<&L3Order>`.
    /// These are orders that can immediately execute against resting orders.
    /// Returns `None` if there are no taker orders on that side.
    pub fn taker_bbo(&self) -> (Option<&L3Order>, Option<&L3Order>) {
        (self.taker_bids.first(), self.taker_asks.first())
    }

    /// Get the top N maker bids (resting limit orders) sorted by price descending
    ///
    /// Returns an iterator over the highest-priced maker bids.
    pub fn top_bids(&self, count: usize) -> impl Iterator<Item = &L3Order> {
        self.bids.iter().take(count)
    }

    /// Get the top N maker asks (resting limit orders) sorted by price ascending
    ///
    /// Returns an iterator over the lowest-priced maker asks.
    pub fn top_asks(&self, count: usize) -> impl Iterator<Item = &L3Order> {
        self.asks.iter().take(count)
    }

    /// Get the top N taker bids (market, oracle, trigger orders) sorted by price descending
    ///
    /// Returns an iterator over the highest-priced taker bids.
    /// These are orders that can immediately execute against resting asks.
    pub fn top_taker_bids(&self, count: usize) -> impl Iterator<Item = &L3Order> {
        self.taker_bids.iter().take(count)
    }

    /// Get the top N taker asks (market, oracle, trigger orders) sorted by price ascending
    ///
    /// Returns an iterator over the lowest-priced taker asks.
    /// These are orders that can immediately execute against resting bids.
    pub fn top_taker_asks(&self, count: usize) -> impl Iterator<Item = &L3Order> {
        self.taker_asks.iter().take(count)
    }

    /// Get exactly N maker bids as a fixed-size array slice
    ///
    /// Returns `Some(&[L3Order; N])` if there are at least N bids, `None` otherwise.
    /// Useful when you need exactly N orders and want to avoid dynamic allocation.
    pub fn top_bids_exact<const N: usize>(&self) -> Option<&[L3Order; N]> {
        self.bids.first_chunk()
    }

    /// Get exactly N maker asks as a fixed-size array slice
    ///
    /// Returns `Some(&[L3Order; N])` if there are at least N asks, `None` otherwise.
    /// Useful when you need exactly N orders and want to avoid dynamic allocation.
    pub fn top_asks_exact<const N: usize>(&self) -> Option<&[L3Order; N]> {
        self.asks.first_chunk()
    }
    /// Reset this instance for later reuse
    fn reset(&mut self) {
        self.bids.clear();
        self.asks.clear();
        self.taker_asks.clear();
        self.taker_bids.clear();
        self.slot = 0;
        self.oracle_price = 0;
    }
    /// Populate an `L3Book` instance given an `Orderbook` and metadata
    fn load_orderbook(
        mut self,
        orderbook: &Orderbook,
        metadata: &MetadataMap,
        oracle_price: u64,
    ) -> Self {
        self.reset();
        self.slot = orderbook.last_modified_slot;
        self.oracle_price = oracle_price;
        let market_tick_size = orderbook.market_tick_size;

        // Add resting limit orders
        for order in orderbook.resting_limit_orders.bids.values() {
            if let Some(meta) = metadata.get(&order.id) {
                self.bids.push(L3Order {
                    price: order.get_price(),
                    size: order.size,
                    reduce_only: order.reduce_only,
                    user: meta.user,
                    order_id: meta.order_id,
                    kind: meta.kind,
                    max_ts: order.max_ts,
                });
            }
        }

        for order in orderbook.resting_limit_orders.asks.values() {
            if let Some(meta) = metadata.get(&order.id) {
                self.asks.push(L3Order {
                    price: order.get_price(),
                    size: order.size,
                    reduce_only: order.reduce_only,
                    user: meta.user,
                    order_id: meta.order_id,
                    kind: meta.kind,
                    max_ts: order.max_ts,
                });
            }
        }

        // Add trigger orders
        for order in orderbook
            .trigger_orders
            .bids
            .values()
            .filter(|o| o.price <= oracle_price)
        {
            if let Some(meta) = metadata.get(&order.id) {
                self.bids.push(L3Order {
                    price: order.price,
                    size: order.size,
                    reduce_only: order.reduce_only,
                    user: meta.user,
                    order_id: meta.order_id,
                    kind: meta.kind,
                    max_ts: order.max_ts,
                });
            }
        }

        for order in orderbook
            .trigger_orders
            .asks
            .values()
            .filter(|o| o.price >= oracle_price)
        {
            if let Some(meta) = metadata.get(&order.id) {
                self.asks.push(L3Order {
                    price: order.price,
                    size: order.size,
                    reduce_only: order.reduce_only,
                    user: meta.user,
                    order_id: meta.order_id,
                    kind: meta.kind,
                    max_ts: order.max_ts,
                });
            }
        }

        // Add floating limit orders
        for order in orderbook.floating_limit_orders.bids.values() {
            if let Some(meta) = metadata.get(&order.id) {
                self.bids.push(L3Order {
                    price: order.get_price(oracle_price, market_tick_size),
                    size: order.size,
                    reduce_only: order.reduce_only,
                    user: meta.user,
                    order_id: meta.order_id,
                    max_ts: order.max_ts,
                    kind: meta.kind,
                });
            }
        }

        for order in orderbook.floating_limit_orders.asks.values() {
            if let Some(meta) = metadata.get(&order.id) {
                self.asks.push(L3Order {
                    price: order.get_price(oracle_price, market_tick_size),
                    size: order.size,
                    reduce_only: order.reduce_only,
                    user: meta.user,
                    order_id: meta.order_id,
                    max_ts: order.max_ts,
                    kind: meta.kind,
                });
            }
        }

        // Add market orders as taker orders
        for order in orderbook.market_orders.asks.values() {
            if let Some(meta) = metadata.get(&order.id) {
                self.taker_asks.push(L3Order {
                    price: order.get_price(self.slot, oracle_price, market_tick_size),
                    size: order.size(),
                    reduce_only: order.reduce_only,
                    user: meta.user,
                    order_id: meta.order_id,
                    max_ts: order.max_ts,
                    kind: meta.kind,
                });
            }
        }

        for order in orderbook.market_orders.bids.values() {
            if let Some(meta) = metadata.get(&order.id) {
                self.taker_bids.push(L3Order {
                    price: order.get_price(self.slot, oracle_price, market_tick_size),
                    size: order.size(),
                    reduce_only: order.reduce_only,
                    user: meta.user,
                    order_id: meta.order_id,
                    max_ts: order.max_ts,
                    kind: meta.kind,
                });
            }
        }

        // Add oracle orders as taker orders
        for order in orderbook.oracle_orders.bids.values() {
            if let Some(meta) = metadata.get(&order.id) {
                self.taker_bids.push(L3Order {
                    price: order.get_price(self.slot, oracle_price, market_tick_size),
                    size: order.size(),
                    reduce_only: order.reduce_only,
                    user: meta.user,
                    order_id: meta.order_id,
                    max_ts: order.max_ts,
                    kind: meta.kind,
                });
            }
        }

        for order in orderbook.oracle_orders.asks.values() {
            if let Some(meta) = metadata.get(&order.id) {
                self.taker_asks.push(L3Order {
                    price: order.get_price(self.slot, oracle_price, market_tick_size),
                    size: order.size(),
                    reduce_only: order.reduce_only,
                    user: meta.user,
                    order_id: meta.order_id,
                    max_ts: order.max_ts,
                    kind: meta.kind,
                });
            }
        }

        // Sort bids in descending order (highest first)
        self.bids.sort_by(|a, b| b.price.cmp(&a.price));
        // Sort asks in ascending order (lowest first)
        self.asks.sort_by(|a, b| a.price.cmp(&b.price));
        // Sort taker bids in descending order (highest first)
        self.taker_bids.sort_by(|a, b| b.price.cmp(&a.price));
        // Sort taker asks in ascending order (lowest first)
        self.taker_asks.sort_by(|a, b| a.price.cmp(&b.price));

        self
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct L2Book {
    /// price → aggregated size (maker orders only)
    pub bids: BTreeMap<u64, u64>,
    /// price → aggregated size (maker orders only)
    pub asks: BTreeMap<u64, u64>,
    /// price → aggregated size (taker orders only)
    pub taker_bids: BTreeMap<u64, u64>,
    /// price → aggregated size (taker orders only)
    pub taker_asks: BTreeMap<u64, u64>,
    pub oracle_price: u64,
    pub slot: u64,
}

impl std::fmt::Display for L2Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "L2 Order Book:")?;
        writeln!(f, "-------------")?;

        // Get top 5 asks in order (highest to lowest)
        let asks: Vec<_> = self.asks.iter().take(5).collect();
        for (price, size) in asks {
            writeln!(f, "ASK: {:>10} | {:>10}", price, size)?;
        }

        writeln!(f, "-------------")?;

        // Get top 5 bids (highest to lowest)
        let bids: Vec<_> = self.bids.iter().rev().take(5).collect();
        for (price, size) in bids {
            writeln!(f, "BID: {:>10} | {:>10}", price, size)?;
        }

        Ok(())
    }
}

impl L2Book {
    /// Get the best bid and ask from maker orders (resting limit orders)
    ///
    /// Returns `(best_bid, best_ask)` where each is an `Option<(price, size)>`.
    /// The best bid is the highest price bid, and the best ask is the lowest price ask.
    /// Returns `None` if there are no orders on that side.
    ///
    /// # Example
    /// ```rust
    /// let (bid, ask) = l2_book.bbo();
    /// if let (Some((bid_price, bid_size)), Some((ask_price, ask_size))) = (bid, ask) {
    ///     println!("Best bid: {} @ {}", bid_size, bid_price);
    ///     println!("Best ask: {} @ {}", ask_size, ask_price);
    /// }
    /// ```
    pub fn bbo(&self) -> (Option<(u64, u64)>, Option<(u64, u64)>) {
        (
            self.bids.first_key_value().map(|x| (*x.0, *x.1)),
            self.asks.first_key_value().map(|x| (*x.0, *x.1)),
        )
    }

    /// Get the best bid and ask from taker orders (market, oracle, trigger orders)
    ///
    /// Returns `(best_bid, best_ask)` where each is an `Option<(price, size)>`.
    /// These are orders that can immediately execute against resting orders.
    /// Returns `None` if there are no taker orders on that side.
    ///
    /// # Example
    /// ```rust
    /// let (taker_bid, taker_ask) = l2_book.taker_bbo();
    /// if let Some((price, size)) = taker_bid {
    ///     println!("Best taker bid: {} @ {}", size, price);
    /// }
    /// ```
    pub fn taker_bbo(&self) -> (Option<(u64, u64)>, Option<(u64, u64)>) {
        (
            self.taker_bids.first_key_value().map(|x| (*x.0, *x.1)),
            self.taker_asks.first_key_value().map(|x| (*x.0, *x.1)),
        )
    }

    /// Get the top N maker bids (resting limit orders) sorted by price descending
    ///
    /// Returns a vector of `(price, size)` tuples for the highest-priced maker bids.
    ///
    /// # Example
    /// ```rust
    /// let top_bids = l2_book.top_bids(5);
    /// for (price, size) in top_bids {
    ///     println!("Bid: {} @ {}", size, price);
    /// }
    /// ```
    pub fn top_bids(&self, count: usize) -> Vec<(u64, u64)> {
        self.bids.iter().take(count).map(|x| (*x.0, *x.1)).collect()
    }

    /// Get the top N maker asks (resting limit orders) sorted by price ascending
    ///
    /// Returns a vector of `(price, size)` tuples for the lowest-priced maker asks.
    ///
    /// # Example
    /// ```rust
    /// let top_asks = l2_book.top_asks(5);
    /// for (price, size) in top_asks {
    ///     println!("Ask: {} @ {}", size, price);
    /// }
    /// ```
    pub fn top_asks(&self, count: usize) -> Vec<(u64, u64)> {
        self.asks.iter().take(count).map(|x| (*x.0, *x.1)).collect()
    }

    /// Get the top N taker bids (market, oracle, trigger orders) sorted by price descending
    ///
    /// Returns a vector of `(price, size)` tuples for the highest-priced taker bids.
    /// These are orders that can immediately execute against resting asks.
    ///
    /// # Example
    /// ```rust
    /// let taker_bids = l2_book.top_taker_bids(10);
    /// for (price, size) in taker_bids {
    ///     println!("Taker bid: {} @ {}", size, price);
    /// }
    /// ```
    pub fn top_taker_bids(&self, count: usize) -> Vec<(u64, u64)> {
        self.taker_bids
            .iter()
            .take(count)
            .map(|x| (*x.0, *x.1))
            .collect()
    }

    /// Get the top N taker asks (market, oracle, trigger orders) sorted by price ascending
    ///
    /// Returns a vector of `(price, size)` tuples for the lowest-priced taker asks.
    /// These are orders that can immediately execute against resting bids.
    ///
    /// # Example
    /// ```rust
    /// let taker_asks = l2_book.top_taker_asks(10);
    /// for (price, size) in taker_asks {
    ///     println!("Taker ask: {} @ {}", size, price);
    /// }
    /// ```
    pub fn top_taker_asks(&self, count: usize) -> Vec<(u64, u64)> {
        self.taker_asks
            .iter()
            .take(count)
            .map(|x| (*x.0, *x.1))
            .collect()
    }

    fn reset(&mut self) {
        self.bids.clear();
        self.asks.clear();
        self.taker_asks.clear();
        self.taker_bids.clear();
        self.slot = 0;
        self.oracle_price = 0;
    }

    /// Initialize the L2Book with all order types
    ///
    /// This function consolidates all the orderbook initialization logic into a single call.
    /// It processes resting limit orders, floating limit orders, and dynamic orders (market/oracle)
    /// in the correct order to build a complete L2 orderbook snapshot.
    fn load_orderbook(mut self, orderbook: &Orderbook, oracle_price: u64) -> Self {
        self.reset();
        self.slot = orderbook.last_modified_slot;
        self.oracle_price = oracle_price;
        let market_tick_size = orderbook.market_tick_size;

        // TODO: filter/handle max leverage orders

        // Process resting limit orders (fixed price orders)
        for order in orderbook.resting_limit_orders.bids.values() {
            let size = self.bids.entry(order.price).or_insert(0);
            *size = size.saturating_add(order.size);
        }
        for order in orderbook.resting_limit_orders.asks.values() {
            let size = self.asks.entry(order.price).or_insert(0);
            *size = size.saturating_add(order.size);
        }

        // Process floating limit orders (oracle-relative price orders)
        for order in orderbook.floating_limit_orders.bids.values() {
            let price = order.get_price(oracle_price, market_tick_size);
            let size = self.bids.entry(price).or_insert(0);
            *size = size.saturating_add(order.size);
        }
        for order in orderbook.floating_limit_orders.asks.values() {
            let price = order.get_price(oracle_price, market_tick_size);
            let size = self.asks.entry(price).or_insert(0);
            *size = size.saturating_add(order.size);
        }

        // Process trigger orders as taker orders
        for order in orderbook
            .trigger_orders
            .bids
            .values()
            .filter(|o| o.price <= oracle_price)
        {
            let price = order.price;
            let size = self.taker_bids.entry(price).or_insert(0);
            *size = size.saturating_add(order.size);
        }
        for order in orderbook
            .trigger_orders
            .asks
            .values()
            .filter(|o| o.price >= oracle_price)
        {
            let price = order.price;
            let size = self.taker_asks.entry(price).or_insert(0);
            *size = size.saturating_add(order.size);
        }

        // Process market orders as taker orders
        for order in orderbook.market_orders.bids.values() {
            let price = order.get_price(self.slot, oracle_price, market_tick_size);
            let size = self.taker_bids.entry(price).or_insert(0);
            *size = size.saturating_add(order.size());
        }
        for order in orderbook.market_orders.asks.values() {
            let price = order.get_price(self.slot, oracle_price, market_tick_size);
            let size = self.taker_asks.entry(price).or_insert(0);
            *size = size.saturating_add(order.size());
        }

        // Process oracle orders as taker orders
        for order in orderbook.oracle_orders.bids.values() {
            let price = order.get_price(self.slot, oracle_price, market_tick_size);
            let size = self.taker_bids.entry(price).or_insert(0);
            *size = size.saturating_add(order.size());
        }
        for order in orderbook.oracle_orders.asks.values() {
            let price = order.get_price(self.slot, oracle_price, market_tick_size);
            let size = self.taker_asks.entry(price).or_insert(0);
            *size = size.saturating_add(order.size());
        }

        self
    }
}
