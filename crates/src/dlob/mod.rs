use std::{
    cmp::Reverse,
    collections::BTreeMap,
    fmt::Debug,
    iter::Peekable,
    sync::{atomic::AtomicBool, Arc},
    time::{SystemTime, UNIX_EPOCH},
};

use arrayvec::ArrayVec;
use dashmap::DashMap;
use solana_sdk::pubkey::Pubkey;

use crate::{
    constants::ProgramData,
    dlob::util::order_hash,
    types::{
        accounts::PerpMarket, MarketId, MarketType, Order, OrderTriggerCondition, OrderType,
        PositionDirection,
    },
};

use serde::{Deserialize, Serialize};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct CrossingOrder {
    pub order_view: LimitOrderView,
    pub metadata: OrderMetadata,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CrossingRegion {
    pub slot: u64,
    pub crossing_bids: Vec<CrossingOrder>,
    pub crossing_asks: Vec<CrossingOrder>,
}

pub mod builder;
#[cfg(test)]
mod tests;
pub mod types;
pub mod util;

pub use types::*;
pub use util::OrderDelta;

type Direction = PositionDirection;

/// Collection of orders with dynamic prices e.g. oracle auctions
///
/// priority changes with every slot and oracle price change
struct DynamicOrders<T: DynamicPrice + OrderKey + Debug> {
    pub bids: Vec<T>,
    pub asks: Vec<T>,
    /// True if the orderbook requires sorting before use
    is_dirty: AtomicBool,
}

impl<T: DynamicPrice + OrderKey + Debug> Default for DynamicOrders<T> {
    fn default() -> Self {
        Self {
            bids: Vec::new(),
            asks: Vec::new(),
            is_dirty: AtomicBool::new(false),
        }
    }
}

impl<T> DynamicOrders<T>
where
    T: DynamicPrice + OrderKey + Clone + From<(u64, Order)> + Debug,
{
    /// True if the orderbook was updated and needs to be sorted before use
    pub fn is_dirty(&self) -> bool {
        self.is_dirty.load(std::sync::atomic::Ordering::Relaxed)
    }
    /// Make the orderbook as dirty
    fn mark_dirty(&self) {
        self.is_dirty
            .store(true, std::sync::atomic::Ordering::Release);
    }
    /// Mark the orderbook as clean
    fn mark_clean(&self) {
        self.is_dirty
            .store(false, std::sync::atomic::Ordering::Release);
    }
    pub fn sort(&mut self, slot: u64, oracle_price: u64, market_tick_size: u64) {
        if self.is_dirty() {
            log::trace!(target: "dlob", "sorting dynamic orders");
            self.bids
                .sort_by_key(|x| Reverse(x.get_price(slot, oracle_price, market_tick_size)));
            self.asks
                .sort_by_key(|x| x.get_price(slot, oracle_price, market_tick_size));
        }
        self.mark_clean();
    }

    fn insert_raw(&mut self, is_bid: bool, order: T) {
        if is_bid {
            self.bids.push(order);
        } else {
            self.asks.push(order);
        }
        self.mark_dirty();
    }

    pub fn insert(&mut self, order_id: u64, order: Order) {
        self.insert_raw(Direction::Long == order.direction, (order_id, order).into());
    }

    pub fn remove(&mut self, order_id: u64, order: Order) -> bool {
        match order.direction {
            Direction::Long => {
                log::trace!(target: "dlob", "remove order: {}/{}", order.slot, order.order_id);
                let order: T = (order_id, order).into();
                if let Some(idx) = self.bids.iter().position(|x| x.key() == order.key()) {
                    log::trace!(target: "dlob", "DynamicOrders: remove {order_id} at: {idx}");
                    self.bids.swap_remove(idx);
                    log::trace!(target: "bids", "bids: {:?}", self.bids);
                    self.mark_dirty();
                    true
                } else {
                    log::trace!(target: "dlob", "DynamicOrders: remove {order_id} not found");
                    false
                }
            }
            Direction::Short => {
                log::trace!(target: "dlob", "remove order: {}/{}", order.slot, order.order_id);
                let order: T = (order_id, order).into();
                if let Some(idx) = self.asks.iter().position(|x| x.key() == order.key()) {
                    log::trace!(target: "dlob", "DynamicOrders: remove {order_id} at: {idx}");
                    self.asks.swap_remove(idx);
                    log::trace!(target: "dlob", "asks: {:?}", self.asks);
                    self.mark_dirty();
                    true
                } else {
                    log::trace!(target: "dlob", "DynamicOrders: remove {order_id} not found");
                    false
                }
            }
        }
    }

    /// Returns true if the order was updated, false if it was removed
    pub fn update(&mut self, order_id: u64, new_order: Order, old_order: Order) -> bool {
        let remaining_size = new_order.base_asset_amount - new_order.base_asset_amount_filled;
        self.remove(order_id, old_order);

        if remaining_size != 0 {
            self.insert(order_id, new_order);
            true
        } else {
            false
        }
    }
}

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
        let old_order: T = (order_id, old_order).into();
        let old_key = old_order.key();
        match new_order.direction {
            Direction::Long => {
                if self.bids.remove(&Reverse(old_key)).is_some() && remaining_size != 0 {
                    let order: T = (order_id, new_order).into();
                    self.insert_raw(true, order);
                    return true;
                }
            }
            Direction::Short => {
                if self.asks.remove(&old_key).is_some() && remaining_size != 0 {
                    let order: T = (order_id, new_order).into();
                    self.insert_raw(false, order);
                    return true;
                }
            }
        }
        log::warn!(target: "dlob", "update not found: {order_id}, {new_order:?}");
        match new_order.direction {
            Direction::Long => {
                log::warn!(target: "dlob", "bids: {:?}", self.bids);
            }
            Direction::Short => {
                log::warn!(target: "dlob", "asks: {:?}", self.asks);
            }
        }

        false
    }
}

/// Orderbook for a specific market
#[derive(Default)]
struct Orderbook {
    /// market auctions with fixed price bounds, changes by slot
    market_orders: DynamicOrders<MarketOrder>,
    /// oracle auctions with dynamic price bounds, changes by slot
    oracle_orders: DynamicOrders<OracleOrder>,
    /// orders to fill at fixed price
    resting_limit_orders: Orders<LimitOrder>,
    /// orders to fill at offset from oracle price
    floating_limit_orders: DynamicOrders<FloatingLimitOrder>,
    /// list of (un)triggered orders
    /// triggered orders are moved to market_orders or oracle_orders
    trigger_orders: Orders<TriggerOrder>,
    /// L2 book snapshot
    l2_snapshot: Snapshot<L2Book>,
    /// L3 book snapshot
    l3_snapshot: Snapshot<L3Book>,
    /// market tick size
    market_tick_size: u64,
    /// slot where dynamic orders where last checked
    last_modified_slot: u64,
    /// market index of this book
    market_index: u16,
}

impl Orderbook {
    /// Evaluate dynamic order prices for some `slot` and `oracle_price`
    pub fn update_slot_and_oracle_price(&mut self, slot: u64, oracle_price: u64) {
        log::debug!(target: "dlob","update book. market:{},slot:{slot},oracle:{oracle_price}", self.market_index);
        self.expire_auction_orders(slot);
        self.market_orders
            .sort(slot, oracle_price, self.market_tick_size);
        self.oracle_orders
            .sort(slot, oracle_price, self.market_tick_size);
        self.floating_limit_orders
            .sort(slot, oracle_price, self.market_tick_size);

        // Update snapshots after sorting dynamic orders
        self.update_l2_view(slot, oracle_price);
        self.last_modified_slot = slot;
    }

    /// Expire all auctions past current `slot`
    ///
    /// limit orders with finishing auctions are moved to resting orders
    fn expire_auction_orders(&mut self, slot: u64) {
        self.market_orders.asks.retain(|x| {
            let is_auction_complete = (x.slot + x.duration as u64) <= slot;
            if is_auction_complete && x.is_limit && x.size > 0 {
                log::trace!(target: "dlob", "market auction end=>resting (slot: {}): {}", slot, x.id);
                self.resting_limit_orders.insert_raw(
                    false,
                    LimitOrder {
                        id: x.id,
                        size: x.size,
                        price: x.end_price as u64,
                        slot: x.slot,
                        max_ts: x.max_ts,
                        post_only: false,
                    },
                );
            }
            !is_auction_complete
        });
        self.market_orders.bids.retain(|x| {
            let is_auction_complete = (x.slot + x.duration as u64) <= slot;
            if is_auction_complete && x.is_limit && x.size > 0 {
                log::trace!(target: "dlob", "market auction end=>resting: (slot: {}): {}", slot, x.id);
                self.resting_limit_orders.insert_raw(
                    true,
                    LimitOrder {
                        id: x.id,
                        size: x.size,
                        price: x.end_price as u64,
                        slot: x.slot,
                        max_ts: x.max_ts,
                        post_only: false,
                    },
                );
            }
            !is_auction_complete
        });
        self.oracle_orders.asks.retain(|x| {
            let is_auction_complete = (x.slot + x.duration as u64) <= slot;
            if is_auction_complete && x.is_limit && x.size > 0 {
                log::trace!(target: "dlob", "oracle auction end=>resting: (slot: {}): {}", slot, x.id);
                self.floating_limit_orders.insert_raw(
                    false,
                    FloatingLimitOrder {
                        id: x.id,
                        slot: x.slot,
                        size: x.size,
                        offset_price: x.end_price_offset as i32,
                        max_ts: x.max_ts,
                        post_only: false,
                    },
                );
            }
            !is_auction_complete
        });
        self.oracle_orders.bids.retain(|x| {
            let is_auction_complete = (x.slot + x.duration as u64) <= slot;
            if is_auction_complete && x.is_limit && x.size > 0 {
                log::trace!(target: "dlob", "oracle auction end=>resting: (slot: {}): {}", slot, x.id);
                self.floating_limit_orders.insert_raw(
                    true,
                    FloatingLimitOrder {
                        id: x.id,
                        slot: x.slot,
                        size: x.size,
                        offset_price: x.end_price_offset as i32,
                        max_ts: x.max_ts,
                        post_only: false,
                    },
                );
            }
            !is_auction_complete
        });
    }

    /// Update the L2 snapshot
    fn update_l2_view(&self, slot: u64, oracle_price: u64) {
        let mut l2book = L2Book::from_limit_orders(&self.resting_limit_orders);
        l2book.insert_dynamic_orders(
            &self.market_orders,
            slot,
            oracle_price,
            self.market_tick_size,
        );
        l2book.insert_dynamic_orders(
            &self.oracle_orders,
            slot,
            oracle_price,
            self.market_tick_size,
        );
        l2book.insert_dynamic_orders(
            &self.floating_limit_orders,
            slot,
            oracle_price,
            self.market_tick_size,
        );
        self.l2_snapshot.update(Arc::new(l2book));
    }

    /// Update the L3 snapshot
    fn update_l3_view(
        &self,
        slot: u64,
        oracle_price: u64,
        metadata: &DashMap<u64, OrderMetadata, ahash::RandomState>,
    ) {
        let l3book = L3Book::from_orders(
            &self.resting_limit_orders,
            &self.floating_limit_orders,
            metadata,
            slot,
            oracle_price,
        );
        self.l3_snapshot.update(Arc::new(l3book));
    }

    pub fn get_limit_bids(&self, slot: u64, oracle_price: u64) -> Vec<LimitOrderView> {
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
                .iter()
                .filter(|o| !o.is_expired(now_unix_s))
                .map(|o| LimitOrderView {
                    id: o.id,
                    price: o.get_price(slot, oracle_price, self.market_tick_size),
                    size: o.size(),
                    post_only: o.post_only,
                    slot: o.slot,
                }),
        );

        // Sort by price in descending order (best bid first)
        result.sort_by(|a, b| b.price.cmp(&a.price));
        result
    }

    pub fn get_limit_asks(&self, slot: u64, oracle_price: u64) -> Vec<LimitOrderView> {
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
                    post_only: o.post_only,
                    slot: o.slot,
                }),
        );
        result.extend(
            self.floating_limit_orders
                .asks
                .iter()
                .filter(|o| !o.is_expired(now_unix_s))
                .map(|o| LimitOrderView {
                    id: o.id,
                    price: o.get_price(slot, oracle_price, self.market_tick_size),
                    size: o.size(),
                    post_only: o.post_only,
                    slot: o.slot,
                }),
        );

        // Sort by price in ascending order (best ask first)
        result.sort_by(|a, b| a.price.cmp(&b.price));
        result
    }

    pub fn get_taker_asks(
        &self,
        slot: u64,
        oracle_price: u64,
        trigger_price: u64,
        perp_market: Option<&PerpMarket>,
    ) -> Vec<(u64, u64, u64)> {
        let mut result = Vec::with_capacity(
            self.market_orders.asks.len()
                + self.oracle_orders.asks.len()
                + self.trigger_orders.asks.len(),
        );

        result.extend(self.market_orders.asks.iter().map(|o| {
            (
                o.id,
                o.get_price(slot, oracle_price, self.market_tick_size),
                o.size,
            )
        }));
        result.extend(self.oracle_orders.asks.iter().map(|o| {
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

    pub fn get_taker_bids(
        &self,
        slot: u64,
        oracle_price: u64,
        trigger_price: u64,
        perp_market: Option<&PerpMarket>,
    ) -> Vec<(u64, u64, u64)> {
        let mut result = Vec::with_capacity(
            self.market_orders.bids.len()
                + self.oracle_orders.bids.len()
                + self.trigger_orders.bids.len(),
        );

        // TODO: this may use MM oracle
        result.extend(self.market_orders.bids.iter().map(|o| {
            (
                o.id,
                o.get_price(slot, oracle_price, self.market_tick_size),
                o.size,
            )
        }));
        result.extend(self.oracle_orders.bids.iter().map(|o| {
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
}

/// channel for sending order updates to DLOB instance
pub type DLOBNotifier = crossbeam::channel::Sender<DLOBEvent>;

/// Aggregates orderbooks for multiple markets
pub struct DLOB {
    /// Map from market to orderbook
    markets: DashMap<MarketId, Orderbook, ahash::RandomState>,
    /// Map from DLOB internal order ID to order metadata
    metadata: DashMap<u64, OrderMetadata, ahash::RandomState>,
    /// static drift program data e.g market tick sizes
    program_data: &'static ProgramData,
}

impl Default for DLOB {
    fn default() -> Self {
        Self {
            markets: DashMap::default(),
            metadata: DashMap::default(),
            program_data: Box::leak(Box::new(ProgramData::uninitialized())),
        }
    }
}

impl DLOB {
    /// Provides a writer channel into the DLOB which acts as a sink for external events
    pub fn spawn_notifier(&'static self) -> DLOBNotifier {
        let (tx, rx) = crossbeam::channel::bounded(2048);
        std::thread::spawn(move || {
            while let Ok(event) = rx.recv() {
                match event {
                    DLOBEvent::SlotOrPriceUpdate {
                        slot,
                        market_index,
                        market_type,
                        oracle_price,
                    } => {
                        self.update_slot_and_oracle_price(
                            market_index,
                            market_type,
                            slot,
                            oracle_price,
                        );
                    }
                    DLOBEvent::Order { slot: _, delta } => match delta {
                        OrderDelta::Create { user, order } => {
                            log::trace!(target: "dlob", "insert order: {:?}", order.order_id);
                            self.insert_order(&user, order);
                        }
                        OrderDelta::Update {
                            user,
                            new_order,
                            old_order,
                        } => {
                            log::trace!(target: "dlob", "update order: {:?}", old_order.order_id);
                            self.update_order(&user, new_order, old_order);
                        }
                        OrderDelta::Remove { user, order } => {
                            log::trace!(target: "dlob", "remove order: {:?}", order.order_id);
                            self.remove_order(&user, order);
                        }
                    },
                }
            }
            log::error!(target: "DLOB", "notifier thread finished");
        });

        tx
    }
    /// run function on a market Orderbook
    fn with_orderbook_mut(&self, market_id: MarketId, f: impl Fn(&mut Orderbook)) {
        let mut orderbook = self.markets.entry(market_id).or_insert({
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
            Orderbook {
                market_tick_size,
                market_index: market_id.index(),
                ..Default::default()
            }
        });

        f(orderbook.value_mut())
    }

    /// Update orderbook slot and oracle price for market
    fn update_slot_and_oracle_price(
        &self,
        market_index: u16,
        market_type: MarketType,
        slot: u64,
        oracle_price: u64,
    ) {
        self.with_orderbook_mut(MarketId::new(market_index, market_type), |orderbook| {
            orderbook.update_slot_and_oracle_price(slot, oracle_price);
            orderbook.update_l3_view(slot, oracle_price, &self.metadata);
        });
    }

    /// Get a lock-free snapshot of the L2 order book
    /// This is safe to call from any thread and will always return a consistent view
    pub fn get_l2_snapshot(&self, market_index: u16, market_type: MarketType) -> Arc<L2Book> {
        self.markets
            .get(&MarketId::new(market_index, market_type))
            .map(|book| book.l2_snapshot.get())
            .unwrap_or_default()
    }

    /// Get a lock-free snapshot of the L3 order book
    /// This is safe to call from any thread and will always return a consistent view
    pub fn get_l3_snapshot(&self, market_index: u16, market_type: MarketType) -> Arc<L3Book> {
        self.markets
            .get(&MarketId::new(market_index, market_type))
            .map(|book| book.l3_snapshot.get())
            .unwrap_or_default()
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
        let bids = book.get_limit_bids(slot, oracle_price);
        let asks = book.get_limit_asks(slot, oracle_price);

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

    fn update_order(&self, user: &Pubkey, new_order: Order, old_order: Order) {
        let order_id = order_hash(user, new_order.order_id);
        log::trace!(target: "dlob", "update order: {order_id},{:?}", new_order.order_type);

        // If order is fully filled, remove it instead of updating
        if new_order.base_asset_amount <= new_order.base_asset_amount_filled {
            self.remove_order(user, new_order);
            return;
        }

        self.with_orderbook_mut(MarketId::new(new_order.market_index, new_order.market_type), |orderbook| {
            if let Some(metadata) = self.metadata.get(&order_id) {
                log::trace!(target: "dlob", "update ({order_id}): {:?}", metadata.kind);
                match metadata.kind {
                    OrderKind::Market | OrderKind::MarketTriggered => {
                        orderbook.market_orders.update(order_id, new_order, old_order);
                    }
                    OrderKind::LimitAuction | OrderKind::LimitTriggered => {
                        // if the auction completed, check if order moved to resting
                        if !orderbook.market_orders.update(order_id, new_order, old_order) {
                            log::trace!(target: "dlob", "update market limit order: {order_id}");
                            orderbook.resting_limit_orders.update(order_id, new_order, old_order);
                        }
                    }
                    OrderKind::Oracle | OrderKind::OracleTriggered => {
                        orderbook.oracle_orders.update(order_id, new_order, old_order);
                    }
                    OrderKind::FloatingLimitAuction => {
                        // if the auction completed, check if order moved to resting
                        if !orderbook.oracle_orders.update(order_id, new_order, old_order) {
                            log::trace!(target: "dlob", "update oracle limit order: {order_id}");
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
                        log::trace!(target: "dlob", "update trigger market order: {order_id},{:?}", new_order);
                        match new_order.trigger_condition {
                            OrderTriggerCondition::Above | OrderTriggerCondition::Below => {
                                orderbook.trigger_orders.update(order_id, new_order, old_order);
                            }
                            OrderTriggerCondition::TriggeredAbove | OrderTriggerCondition::TriggeredBelow => {
                                // order has been triggered, its an ordinary auction order now
                                orderbook.trigger_orders.remove(order_id, new_order);
                                let mut new_metadata = *metadata.value();
                                drop(metadata);
                                if new_order.is_oracle_trigger_market() {
                                    new_metadata.kind = OrderKind::OracleTriggered;
                                    self.metadata.insert(order_id, new_metadata);
                                    orderbook.oracle_orders.update(order_id, new_order, old_order);
                                } else {
                                    new_metadata.kind = OrderKind::MarketTriggered;
                                    self.metadata.insert(order_id, new_metadata);
                                    orderbook.market_orders.update(order_id, new_order, old_order);
                                }
                            }
                        }
                    }
                    OrderKind::TriggerLimit => {
                        log::trace!(target: "dlob", "update trigger limit order: {order_id},{:?}", new_order);
                        match new_order.trigger_condition {
                            OrderTriggerCondition::Above | OrderTriggerCondition::Below => {
                                orderbook.trigger_orders.update(order_id, new_order, old_order);
                            }
                            OrderTriggerCondition::TriggeredAbove | OrderTriggerCondition::TriggeredBelow => {
                                // order has been triggered, its an ordinary auction order now
                                orderbook.trigger_orders.remove(order_id, new_order);
                                drop(metadata); // drop the borrow
                                self.metadata.entry(order_id).and_modify(|o| o.kind = OrderKind::LimitTriggered);
                                orderbook.market_orders.update(order_id, new_order, old_order);
                            }
                        }
                    }
                }
            }
        });
    }

    fn remove_order(&self, user: &Pubkey, order: Order) {
        let order_id = order_hash(user, order.order_id);
        log::trace!(target: "dlob", "remove order: {order_id}");

        self.with_orderbook_mut(MarketId::new(order.market_index, order.market_type), |orderbook| {
            if let Some((_, metadata)) = self.metadata.remove(&order_id) {
                log::trace!(target: "dlob", "remove ({order_id}): {:?}", metadata.kind);
                match metadata.kind {
                    OrderKind::Market | OrderKind::MarketTriggered => {
                        orderbook.market_orders.remove(order_id, order);
                    }
                    OrderKind::LimitAuction | OrderKind::LimitTriggered => {
                        // if the auction completed, check if order moved to resting
                        if !orderbook.market_orders.remove(order_id, order) {
                            log::trace!(target: "dlob", "remove market limit order: {order_id}");
                            orderbook.resting_limit_orders.remove(order_id, order);
                        }
                    }
                    OrderKind::Oracle | OrderKind::OracleTriggered => {
                        orderbook.oracle_orders.remove(order_id, order);
                    }
                    OrderKind::FloatingLimitAuction => {
                        // if the auction completed, check if order moved to resting
                        if !orderbook.oracle_orders.remove(order_id, order) {
                            log::trace!(target: "dlob", "remove oracle limit order: {order_id}");
                            orderbook.floating_limit_orders.remove(order_id, order);
                        }
                    }
                    OrderKind::Limit => {
                        orderbook.resting_limit_orders.remove(order_id, order);
                    }
                    OrderKind::FloatingLimit => {
                        orderbook.floating_limit_orders.remove(order_id, order);
                    }
                    OrderKind::TriggerMarket | OrderKind::TriggerLimit => {
                        log::trace!(target: "dlob", "trigger order: {order_id},{:?}", order.trigger_condition);
                        match order.trigger_condition {
                            OrderTriggerCondition::Above | OrderTriggerCondition::Below => {
                                if !orderbook.trigger_orders.remove(order_id, order) {
                                    log::trace!(target: "dlob", "remove trigger order fail: {:?}", orderbook.trigger_orders);
                                }
                            }
                            OrderTriggerCondition::TriggeredAbove | OrderTriggerCondition::TriggeredBelow => {
                                log::error!(target: "dlob", "trigger order bad state: {order:?}");
                            }
                        }
                    }
                }
            }
        });
    }

    fn insert_order(&self, user: &Pubkey, order: Order) {
        let order_id = order_hash(user, order.order_id);
        log::trace!(target: "dlob", "insert order: {order_id}");

        if order.base_asset_amount <= order.base_asset_amount_filled {
            log::trace!(target: "dlob", "skipping fully filled order: {order:?}");
            return;
        }

        self.with_orderbook_mut(
            MarketId::new(order.market_index, order.market_type),
            |orderbook| {
                match order.order_type {
                    OrderType::Market => {
                        orderbook.market_orders.insert(order_id, order);
                        self.metadata.insert(
                            order_id,
                            OrderMetadata::new(*user, OrderKind::Market, order.order_id),
                        );
                    }
                    OrderType::Oracle => {
                        orderbook.oracle_orders.insert(order_id, order);
                        self.metadata.insert(
                            order_id,
                            OrderMetadata::new(*user, OrderKind::Oracle, order.order_id),
                        );
                    }
                    OrderType::Limit => {
                        /*
                        making order
                        - limit order with POST_ONLY=true
                        - limit order with POST_ONLY=false, auction completed

                        taking_limit orders can cross, only older

                        LIMIT orders with both POST_ONLY cannot cross
                        LIMIT orders with both POST_ONLY=FALSE can cross, older is maker
                        LIMIT order with 1 POST_ONLY=TRUE can become maker for 1 POST_ONLY=FALSE
                        */
                        let is_floating = order.oracle_price_offset != 0;
                        let is_post_only = order.post_only;
                        let is_auction = order.auction_duration != 0;
                        let order_kind = if !is_post_only {
                            // taker orders but can be maker in some circumstances, namely:
                            // 1) auction is complete and taker order is market/oracle
                            // 2) auction is complete, and taker order is limit and newer
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

                        log::trace!(target: "dlob", "insert limit order: {order_id},{:?}", order_kind);
                        self.metadata.insert(
                            order_id,
                            OrderMetadata::new(*user, order_kind, order.order_id),
                        );
                    }
                    OrderType::TriggerMarket => match order.trigger_condition {
                        OrderTriggerCondition::Above | OrderTriggerCondition::Below => {
                            orderbook.trigger_orders.insert(order_id, order);
                                log::trace!(target: "dlob", "insert trigger market order: {order_id}");
                            self.metadata.insert(
                                order_id,
                                OrderMetadata::new(*user, OrderKind::TriggerMarket, order.order_id),
                            );
                        }
                        OrderTriggerCondition::TriggeredAbove
                        | OrderTriggerCondition::TriggeredBelow => {
                            if order.is_oracle_trigger_market() {
                                log::trace!(target: "dlob", "insert triggered oracle order: {order_id}");
                                orderbook.oracle_orders.insert(order_id, order);
                                self.metadata.insert(
                                    order_id,
                                    OrderMetadata::new(
                                        *user,
                                        OrderKind::OracleTriggered,
                                        order.order_id,
                                    ),
                                );
                            } else {
                                log::trace!(target: "dlob", "insert triggered market order: {order_id}");
                                orderbook.market_orders.insert(order_id, order);
                                self.metadata.insert(
                                    order_id,
                                    OrderMetadata::new(
                                        *user,
                                        OrderKind::MarketTriggered,
                                        order.order_id,
                                    ),
                                );
                            }
                        }
                    },
                    OrderType::TriggerLimit => match order.trigger_condition {
                        OrderTriggerCondition::Above | OrderTriggerCondition::Below => {
                            orderbook.trigger_orders.insert(order_id, order);
                            log::trace!(target: "dlob", "insert trigger limit order: {order_id}");
                            self.metadata.insert(
                                order_id,
                                OrderMetadata::new(*user, OrderKind::TriggerLimit, order.order_id),
                            );
                        }
                        OrderTriggerCondition::TriggeredAbove
                        | OrderTriggerCondition::TriggeredBelow => {
                            log::trace!(target: "dlob", "insert triggered limit order: {order_id}");
                            orderbook.market_orders.insert(order_id, order);
                            self.metadata.insert(
                                order_id,
                                OrderMetadata::new(
                                    *user,
                                    OrderKind::LimitTriggered,
                                    order.order_id,
                                ),
                            );
                        }
                    },
                }
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

    /// At the current slot return all auctions crossing resting limit orders
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
        log::trace!(target: "dlob", "VAMM market={} bid={vamm_bid:?} ask={vamm_ask:?}", market_index);

        let taker_asks = book.get_taker_asks(slot, oracle_price, trigger_price, perp_market);
        let taker_bids = book.get_taker_bids(slot, oracle_price, trigger_price, perp_market);
        let mut resting_asks = book.get_limit_asks(slot, oracle_price);
        let mut resting_bids = book.get_limit_bids(slot, oracle_price);
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
                log::warn!(target: "dlob", "missing metadata for order: {oid}");
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
                log::warn!(target: "dlob", "missing metadata for order: {oid}");
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

    /// At the current slot and oracle price return all auctions crossing resting limit orders
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
                (
                    book.last_modified_slot,
                    book.get_limit_asks(current_slot, oracle_price),
                )
            }
            Direction::Short => {
                let book = self.markets.get(&market).expect("market lob exists");
                (
                    book.last_modified_slot,
                    book.get_limit_bids(current_slot, oracle_price),
                )
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
                    log::debug!(target: "dlob", "reached max number crosses");
                    break;
                }
                if remaining_size == 0 {
                    break;
                }
            } else {
                log::warn!(
                    target: "dlob",
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

#[derive(Debug, Default)]
pub struct L3Order {
    pub price: u64,
    pub size: u64,
    pub order_id: u32,
    pub maker: Pubkey,
}

#[derive(Debug, Default)]
pub struct L3Book {
    pub slot: u64,
    pub oracle_price: u64,
    pub bids: Vec<L3Order>,
    pub asks: Vec<L3Order>,
}

impl L3Book {
    fn from_orders(
        resting_limit_orders: &Orders<LimitOrder>,
        floating_limit_orders: &DynamicOrders<FloatingLimitOrder>,
        metadata: &DashMap<u64, OrderMetadata, ahash::RandomState>,
        slot: u64,
        oracle_price: u64,
    ) -> Self {
        let mut bids = Vec::new();
        let mut asks = Vec::new();

        // Add resting limit orders
        for order in resting_limit_orders.bids.values() {
            if let Some(meta) = metadata.get(&order.id) {
                bids.push(L3Order {
                    price: order.get_price(),
                    size: order.size,
                    maker: meta.user,
                    order_id: meta.order_id,
                });
            }
        }

        for order in resting_limit_orders.asks.values() {
            if let Some(meta) = metadata.get(&order.id) {
                asks.push(L3Order {
                    price: order.get_price(),
                    size: order.size,
                    maker: meta.user,
                    order_id: meta.order_id,
                });
            }
        }

        // Add floating limit orders
        for order in &floating_limit_orders.bids {
            if let Some(meta) = metadata.get(&order.id) {
                bids.push(L3Order {
                    price: order.get_price(slot, oracle_price, 0), // tick_size unused
                    size: order.size(),
                    maker: meta.user,
                    order_id: meta.order_id,
                });
            }
        }

        for order in &floating_limit_orders.asks {
            if let Some(meta) = metadata.get(&order.id) {
                asks.push(L3Order {
                    price: order.get_price(slot, oracle_price, 0), // tick_size unused
                    size: order.size(),
                    maker: meta.user,
                    order_id: meta.order_id,
                });
            }
        }

        // Sort bids in descending order (highest first)
        bids.sort_by(|a, b| b.price.cmp(&a.price));
        // Sort asks in ascending order (lowest first)
        asks.sort_by(|a, b| a.price.cmp(&b.price));

        Self {
            bids,
            asks,
            slot,
            oracle_price,
        }
    }
}

#[derive(Debug, Default)]
pub struct L2Book {
    /// price → aggregated size
    pub bids: BTreeMap<u64, u64>,
    /// price → aggregated size
    pub asks: BTreeMap<u64, u64>,
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
    /// Bootstrap L2Book from resting limit orders
    fn from_limit_orders(resting_limit_orders: &Orders<LimitOrder>) -> Self {
        let mut bids: BTreeMap<u64, u64> = BTreeMap::new();
        let mut asks: BTreeMap<u64, u64> = BTreeMap::new();
        for order in resting_limit_orders.bids.values() {
            *bids.entry(order.price).or_insert(0) += order.size;
        }

        for order in resting_limit_orders.asks.values() {
            *asks.entry(order.price).or_insert(0) += order.size;
        }

        Self { bids, asks }
    }

    /// Add dynamic order types to this `L2Book`
    fn insert_dynamic_orders<T: OrderKey + DynamicPrice + Debug>(
        &mut self,
        orders: &DynamicOrders<T>,
        slot: u64,
        oracle_price: u64,
        market_tick_size: u64,
    ) {
        for order in &orders.bids {
            let price = order.get_price(slot, oracle_price, market_tick_size);
            *self.bids.entry(price).or_insert(0) += order.size();
        }

        for order in &orders.asks {
            let price = order.get_price(slot, oracle_price, market_tick_size);
            *self.asks.entry(price).or_insert(0) += order.size();
        }
    }
}
