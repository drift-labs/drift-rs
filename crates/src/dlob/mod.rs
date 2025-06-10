use ahash::AHasher;
use std::cmp::Reverse;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicBool, AtomicPtr};
use std::sync::Arc;

use dashmap::DashMap;
use solana_sdk::pubkey::Pubkey;

use crate::types::{Order, OrderTriggerCondition, OrderType, PositionDirection};

type Direction = PositionDirection;

/// Helper function to generate unique order Id hash
fn order_hash(user: &Pubkey, order_id: u32) -> u64 {
    let mut hasher = AHasher::default();
    user.hash(&mut hasher);
    order_id.hash(&mut hasher);
    hasher.finish()
}

pub struct Snapshot<T: Default> {
    inner: AtomicPtr<T>,
}

impl<T: Default> Snapshot<T> {
    pub fn new(initial: Arc<T>) -> Self {
        let ptr = Arc::into_raw(initial) as *mut T;
        Self {
            inner: AtomicPtr::new(ptr),
        }
    }

    /// Get a cloned Arc<T> for readers (lock-free)
    pub fn get(&self) -> Arc<T> {
        let ptr = self.inner.load(std::sync::atomic::Ordering::Acquire);
        // SAFETY: we never deallocate this pointer while in use
        unsafe {
            Arc::increment_strong_count(ptr);
            Arc::from_raw(ptr)
        }
    }

    /// Atomically replace the snapshot (writer-only)
    pub fn update(&self, new_book: Arc<T>) {
        let new_ptr = Arc::into_raw(new_book) as *mut T;
        let old_ptr = self
            .inner
            .swap(new_ptr, std::sync::atomic::Ordering::Release);

        // SAFETY: we must drop the old Arc so it doesn't leak
        unsafe { drop(Arc::from_raw(old_ptr)) };
    }
}

impl<T: Default> Default for Snapshot<T> {
    fn default() -> Self {
        Self::new(Arc::new(T::default()))
    }
}

impl<T: Default> Drop for Snapshot<T> {
    fn drop(&mut self) {
        let ptr = self.inner.load(std::sync::atomic::Ordering::Acquire);
        // SAFETY: we own the pointer and can safely drop it
        unsafe { drop(Arc::from_raw(ptr)) };
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

        // Get top 5 asks in reverse order (highest to lowest)
        let asks: Vec<_> = self.asks.iter().rev().take(5).collect();
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
    pub fn from_limit_orders(resting_limit_orders: &Orders<LimitOrder>) -> Self {
        let mut bids: BTreeMap<u64, u64> = BTreeMap::new();
        let mut asks: BTreeMap<u64, u64> = BTreeMap::new();
        for (price_rev, order) in &resting_limit_orders.bids {
            let price = price_rev.0 .0;
            *bids.entry(price).or_insert(0) += order.size;
        }

        for (price, order) in &resting_limit_orders.asks {
            *asks.entry(price.0).or_insert(0) += order.size;
        }

        Self { bids, asks }
    }

    /// Add dynamic order types to this `L2Book`
    fn insert_dynamic_orders<T: OrderKey + DynamicPrice>(
        &mut self,
        orders: &DynamicOrders<T>,
        slot: u64,
        oracle_price: u64,
    ) {
        for order in &orders.bids {
            let price = order.get_price(slot, oracle_price);
            *self.bids.entry(price).or_insert(0) += order.size();
        }

        for order in &orders.asks {
            let price = order.get_price(slot, oracle_price);
            *self.asks.entry(price).or_insert(0) += order.size();
        }
    }
}

// Replace the key structs with type aliases
type MarketOrderKey = (i64, u64, u64);
type OracleOrderKey = (i64, u64, u64);
type LimitOrderKey = (u64, u64, u64);
type FloatingLimitOrderKey = (i32, u64, u64);
type TriggerOrderKey = (u64, u64, u64);

/// Order with dynamic price calculation
trait DynamicPrice {
    fn get_price(&self, slot: u64, oracle_price: u64) -> u64;
    fn size(&self) -> u64;
}

// Subset of order fields for sorting
trait OrderKey {
    type Key: Ord + Clone;
    fn key(&self) -> Self::Key;
}

impl OrderKey for MarketOrder {
    type Key = MarketOrderKey;
    fn key(&self) -> Self::Key {
        (self.start_price, self.slot, self.id)
    }
}

impl OrderKey for OracleOrder {
    type Key = OracleOrderKey;
    fn key(&self) -> Self::Key {
        (self.start_price_offset, self.slot, self.id)
    }
}

impl OrderKey for LimitOrder {
    type Key = LimitOrderKey;
    fn key(&self) -> Self::Key {
        (self.price, self.slot, self.id)
    }
}

impl OrderKey for FloatingLimitOrder {
    type Key = FloatingLimitOrderKey;
    fn key(&self) -> Self::Key {
        (self.offset_price, self.slot, self.id)
    }
}

impl OrderKey for TriggerOrder {
    type Key = TriggerOrderKey;
    fn key(&self) -> Self::Key {
        (self.price, self.slot, self.id)
    }
}

#[derive(Default, Clone, PartialEq, Eq)]
struct MarketOrder {
    id: u64,
    size: u64,
    start_price: i64,
    end_price: i64,
    duration: u8,
    price_slope: i64,
    slot: u64,
}

#[derive(Default, Clone, PartialEq, Eq)]
struct OracleOrder {
    id: u64,
    size: u64,
    start_price_offset: i64,
    end_price_offset: i64,
    duration: u8,
    price_slope: i64,
    slot: u64,
}

#[derive(Default, Clone, PartialEq, Eq)]
struct LimitOrder {
    id: u64,
    size: u64,
    price: u64,
    slot: u64,
}

#[derive(Default, Clone, PartialEq, Eq)]
struct FloatingLimitOrder {
    id: u64,
    size: u64,
    offset_price: i32,
    slot: u64,
}

#[derive(Default, Clone)]
struct TriggerOrder {
    id: u64,
    size: u64,
    price: u64,
    condition: OrderTriggerCondition,
    slot: u64,
}

impl DynamicPrice for MarketOrder {
    fn size(&self) -> u64 {
        self.size
    }
    fn get_price(&self, slot: u64, _oracle_price: u64) -> u64 {
        if slot >= self.slot + self.duration as u64 {
            self.end_price as u64
        } else {
            let slots_elapsed = slot - self.slot;
            (self.start_price + (self.price_slope * slots_elapsed as i64)) as u64
        }
    }
}

impl From<(u64, Order)> for MarketOrder {
    fn from(value: (u64, Order)) -> Self {
        let (id, order) = value;
        let price_diff = order.auction_end_price - order.auction_start_price;
        let price_slope = price_diff
            .checked_div(order.auction_duration as i64)
            .unwrap_or(0);

        Self {
            id,
            size: order.base_asset_amount - order.base_asset_amount_filled,
            start_price: order.auction_start_price,
            end_price: order.auction_end_price,
            duration: order.auction_duration,
            price_slope,
            slot: order.slot,
        }
    }
}

impl DynamicPrice for OracleOrder {
    fn size(&self) -> u64 {
        self.size
    }
    fn get_price(&self, slot: u64, oracle_price: u64) -> u64 {
        if slot >= self.slot + self.duration as u64 {
            oracle_price + self.end_price_offset as u64
        } else {
            let slots_elapsed = slot - self.slot;
            oracle_price
                + (self.start_price_offset + (self.price_slope * slots_elapsed as i64)) as u64
        }
    }
}

impl From<(u64, Order)> for OracleOrder {
    fn from(value: (u64, Order)) -> Self {
        let (id, order) = value;
        let price_diff = order.auction_end_price - order.auction_start_price;
        let price_slope = price_diff
            .checked_div(order.auction_duration as i64)
            .unwrap_or(0);

        Self {
            id,
            size: order.base_asset_amount - order.base_asset_amount_filled,
            start_price_offset: order.auction_start_price,
            end_price_offset: order.auction_end_price,
            duration: order.auction_duration,
            price_slope,
            slot: order.slot,
        }
    }
}

impl LimitOrder {
    pub fn get_price(&self) -> u64 {
        self.price
    }
}

impl From<(u64, Order)> for LimitOrder {
    fn from(value: (u64, Order)) -> Self {
        let (id, order) = value;
        Self {
            id,
            size: order.base_asset_amount - order.base_asset_amount_filled,
            price: order.price,
            slot: order.slot,
        }
    }
}

impl DynamicPrice for FloatingLimitOrder {
    fn size(&self) -> u64 {
        self.size
    }
    fn get_price(&self, _slot: u64, oracle_price: u64) -> u64 {
        (oracle_price as i64 + self.offset_price as i64) as u64
    }
}

impl From<(u64, Order)> for FloatingLimitOrder {
    fn from(value: (u64, Order)) -> Self {
        let (id, order) = value;
        Self {
            id,
            size: order.base_asset_amount - order.base_asset_amount_filled,
            offset_price: order.oracle_price_offset,
            slot: order.slot,
        }
    }
}

impl From<(u64, Order)> for TriggerOrder {
    fn from(value: (u64, Order)) -> Self {
        let (id, order) = value;
        Self {
            id,
            size: order.base_asset_amount,
            price: order.trigger_price,
            condition: order.trigger_condition,
            slot: order.slot,
        }
    }
}

struct DynamicOrders<T: DynamicPrice + OrderKey> {
    pub bids: Vec<T>,
    pub asks: Vec<T>,
    /// True if the orderbook requires sorting before use
    is_dirty: AtomicBool,
}

impl<T: DynamicPrice + OrderKey> Default for DynamicOrders<T> {
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
    T: DynamicPrice + OrderKey + Clone + From<(u64, Order)>,
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
    pub fn sort(&mut self, slot: u64, oracle_price: u64) {
        if self.is_dirty() {
            self.bids
                .sort_by_key(|x| Reverse(x.get_price(slot, oracle_price)));
            self.asks.sort_by_key(|x| x.get_price(slot, oracle_price));
        }
        self.mark_clean();
    }

    pub fn insert(&mut self, order_id: u64, order: Order) {
        match order.direction {
            Direction::Long => {
                let order: T = (order_id, order).into();
                self.bids.push(order);
            }
            Direction::Short => {
                let order: T = (order_id, order).into();
                self.asks.push(order);
            }
        }
        self.mark_dirty();
    }

    pub fn remove(&mut self, order_id: u64, order: Order) -> bool {
        match order.direction {
            Direction::Long => {
                let order: T = (order_id, order).into();
                if let Some(idx) = self.bids.iter().position(|x| x.key() == order.key()) {
                    self.bids.swap_remove(idx);
                    self.mark_dirty();
                    true
                } else {
                    false
                }
            }
            Direction::Short => {
                let order: T = (order_id, order).into();
                if let Some(idx) = self.asks.iter().position(|x| x.key() == order.key()) {
                    self.asks.swap_remove(idx);
                    self.mark_dirty();
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn update(&mut self, order_id: u64, order: Order) -> bool {
        let remaining_size = order.base_asset_amount - order.base_asset_amount_filled;
        if remaining_size == 0 {
            return self.remove(order_id, order);
        }
        match order.direction {
            Direction::Long => {
                let order: T = (order_id, order).into();
                if let Some(o) = self.bids.iter_mut().find(|x| x.key() == order.key()) {
                    *o = order;
                    self.mark_dirty();
                    true
                } else {
                    false
                }
            }
            Direction::Short => {
                let order: T = (order_id, order).into();
                if let Some(o) = self.asks.iter_mut().find(|x| x.key() == order.key()) {
                    *o = order;
                    self.mark_dirty();
                    true
                } else {
                    false
                }
            }
        }
    }
}

struct Orders<T: OrderKey + Clone> {
    pub bids: BTreeMap<Reverse<T::Key>, T>,
    pub asks: BTreeMap<T::Key, T>,
}

impl<T: OrderKey + Clone> Default for Orders<T> {
    fn default() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }
}

impl<T: Clone + From<(u64, Order)> + OrderKey> Orders<T> {
    pub fn insert(&mut self, order_id: u64, order: Order) {
        match order.direction {
            Direction::Long => {
                let order: T = (order_id, order).into();
                self.bids.insert(Reverse(order.key()), order);
            }
            Direction::Short => {
                let order: T = (order_id, order).into();
                self.asks.insert(order.key(), order);
            }
        }
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

    pub fn update(&mut self, order_id: u64, order: Order) -> bool {
        let remaining_size = order.base_asset_amount - order.base_asset_amount_filled;
        match order.direction {
            Direction::Long => {
                let order: T = (order_id, order).into();
                if remaining_size == 0 {
                    self.bids.remove(&Reverse(order.key()));
                    return false;
                }
                if let Some(existing) = self.bids.get_mut(&Reverse(order.key())) {
                    *existing = order;
                    return true;
                }
            }
            Direction::Short => {
                let order: T = (order_id, order).into();
                if remaining_size == 0 {
                    self.asks.remove(&order.key());
                    return false;
                }
                if let Some(existing) = self.asks.get_mut(&order.key()) {
                    *existing = order;
                    return true;
                }
            }
        }
        false
    }
}

#[repr(u8)]
pub enum OrderKind {
    Market,
    Oracle,
    Limit,
    FloatingLimit,
    Trigger,
}

struct OrderMetadata {
    user: Pubkey,
    kind: OrderKind,
    direction: Direction,
    slot: u64,
}

impl OrderMetadata {
    pub fn new(user: Pubkey, kind: OrderKind, direction: Direction, slot: u64) -> Self {
        Self {
            user,
            kind,
            direction,
            slot,
        }
    }
}

/// An orderbook per market
pub struct Orderbook {
    inner: OrderbookData,
    l2_snapshot: Snapshot<L2Book>,
    /// Map from DLOB internal order ID to order metadata
    metadata: DashMap<u64, OrderMetadata, ahash::RandomState>,
}

struct OrderbookData {
    // market auctions with fixed price bounds, changes by slot
    market_orders: DynamicOrders<MarketOrder>,
    // oracle auctions with dynamic price bounds, changes by slot
    oracle_orders: DynamicOrders<OracleOrder>,
    // orders to fill at fixed price
    resting_limit_orders: Orders<LimitOrder>,
    // orders to fill at offset from oracle price
    floating_limit_orders: DynamicOrders<FloatingLimitOrder>,
    // promote to other order types when conditions are met
    trigger_orders: Orders<TriggerOrder>,
}

/*
avoid explosion of orderlists
- taking limit orders
=> auction duration of 0 store in market orders and oracle orders
    -> simple to implement
    -> can't easily do 'taker' limit orders (oldest becomes maker)
=> store in resting_limit and floating_limit
    -> auction complete NON-post-only orders can make for any other taker order type
    -> auction complete NON-post-only orders can make for older NON-post only limit orders too
    -> check for crosses must check post_only=False/auction_complete on each order (overhead)
    -> no expiry step moving from auction to limit, crosses checked naturally
    -> DLOB builder needs to filter this or risk incorrect results
 */

impl Orderbook {
    #[cfg(test)]
    /// inner orderbook structure, useful for testing
    fn inner<'a>(&'a self) -> &'a OrderbookData {
        &self.inner
    }

    /// Get a lock-free snapshot of the L2 order book
    /// This is safe to call from any thread and will always return a consistent view
    pub fn get_l2_snapshot(&self) -> Arc<L2Book> {
        self.l2_snapshot.get()
    }

    /// Evaluate dynamic order prices for some `slot` and `oracle_price`
    pub fn update_slot_and_oracle_price(&mut self, slot: u64, oracle_price: u64) {
        log::debug!(target: "dlob", "update orders @ slot: {slot:?}, oracle: {oracle_price:?}");
        self.inner.market_orders.sort(slot, oracle_price);
        self.inner.oracle_orders.sort(slot, oracle_price);
        self.inner.floating_limit_orders.sort(slot, oracle_price);
        // Update L2 snapshot after sorting dynamic orders
        self.update_l2_view(slot, oracle_price);
    }

    pub fn expire_orders() -> Self {
        todo!("remove expired orders");
    }

    /// Update the L2 snapshot
    fn update_l2_view(&self, slot: u64, oracle_price: u64) {
        let mut l2book = L2Book::from_limit_orders(&self.inner.resting_limit_orders);
        l2book.insert_dynamic_orders(&self.inner.market_orders, slot, oracle_price);
        l2book.insert_dynamic_orders(&self.inner.oracle_orders, slot, oracle_price);
        l2book.insert_dynamic_orders(&self.inner.floating_limit_orders, slot, oracle_price);
        self.l2_snapshot.update(Arc::new(l2book));
    }

    pub fn update_order(&mut self, user: &Pubkey, order: Order) {
        log::trace!(target: "dlob", "update order: {user:?},{order:?}");
        let order_id = order_hash(user, order.order_id);
        if let Some(metadata) = self.metadata.get(&order_id) {
            match metadata.kind {
                OrderKind::Market => {
                    self.inner.market_orders.update(order_id, order);
                }
                OrderKind::Oracle => {
                    self.inner.oracle_orders.update(order_id, order);
                }
                OrderKind::Limit => {
                    // TODO: move expired auction to resting
                    self.inner.resting_limit_orders.update(order_id, order);
                }
                OrderKind::FloatingLimit => {
                    self.inner.floating_limit_orders.update(order_id, order);
                }
                OrderKind::Trigger => {
                    log::trace!(target: "dlob", "skipping unhandled trigger order: {order:?}");
                }
            }
        }
    }

    pub fn insert_order(&mut self, user: &Pubkey, order: Order, slot: u64) {
        log::trace!(target: "dlob", "update order: {user:?},{order:?}");
        let order_id = order_hash(user, order.order_id);
        match order.order_type {
            OrderType::Market => {
                self.inner.market_orders.insert(order_id, order);
                self.metadata.insert(
                    order_id,
                    OrderMetadata::new(*user, OrderKind::Market, order.direction, slot),
                );
            }
            OrderType::Oracle => {
                self.inner.oracle_orders.insert(order_id, order);
                self.metadata.insert(
                    order_id,
                    OrderMetadata::new(*user, OrderKind::Oracle, order.direction, slot),
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
                if !is_post_only {
                    // taker orders but can be maker in some circumstances, namely:
                    // 1) auction is complete and taker order is market/oracle
                    // 2) auction is complete, and taker order is limit and newer
                    match (is_auction, is_floating) {
                        (true, true) => self.inner.oracle_orders.insert(order_id, order),
                        (true, false) => self.inner.market_orders.insert(order_id, order),
                        (false, true) => self.inner.floating_limit_orders.insert(order_id, order),
                        (false, false) => self.inner.resting_limit_orders.insert(order_id, order),
                    }
                } else {
                    // post only cannot have an auction (maker side only)
                    if is_floating {
                        self.inner.floating_limit_orders.insert(order_id, order);
                    } else {
                        self.inner.resting_limit_orders.insert(order_id, order);
                    }
                }

                self.metadata.insert(
                    order_id,
                    OrderMetadata::new(*user, OrderKind::Limit, order.direction, slot),
                );
            }
            OrderType::TriggerLimit | OrderType::TriggerMarket => {
                self.inner.trigger_orders.insert(order_id, order);
                self.metadata.insert(
                    order_id,
                    OrderMetadata::new(*user, OrderKind::Trigger, order.direction, slot),
                );
            }
        }
    }

    fn collect_taker_bids(&self, slot: u64, oracle_price: u64) -> Vec<(u64, u64)> {
        let mut result = Vec::new();
        result.extend(
            self.inner
                .market_orders
                .bids
                .iter()
                .map(|o| (o.get_price(slot, oracle_price), o.size())),
        );
        result.extend(
            self.inner
                .oracle_orders
                .bids
                .iter()
                .map(|o| (o.get_price(slot, oracle_price), o.size())),
        );
        // Sort descending so best price comes first
        result.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        result
    }

    fn collect_taker_asks(&self, slot: u64, oracle_price: u64) -> Vec<(u64, u64)> {
        let mut result = Vec::new();
        result.extend(
            self.inner
                .market_orders
                .asks
                .iter()
                .map(|o| (o.get_price(slot, oracle_price), o.size())),
        );
        result.extend(
            self.inner
                .oracle_orders
                .asks
                .iter()
                .map(|o| (o.get_price(slot, oracle_price), o.size())),
        );
        // Sort ascending so best price comes first
        result.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        result
    }

    fn collect_limit_bids(&self, slot: u64, oracle_price: u64) -> Vec<(u64, u64)> {
        let mut result = Vec::new();

        result.extend(
            self.inner
                .resting_limit_orders
                .bids
                .iter()
                .map(|(_, o)| (o.get_price(), o.size)),
        );
        result.extend(
            self.inner
                .floating_limit_orders
                .bids
                .iter()
                .map(|o| (o.get_price(slot, oracle_price), o.size())),
        );

        // Sort descending for bid priority
        result.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        result
    }

    fn collect_limit_asks(&self, slot: u64, oracle_price: u64) -> Vec<(u64, u64)> {
        let mut result = Vec::new();

        result.extend(
            self.inner
                .resting_limit_orders
                .asks
                .iter()
                .map(|(_, o)| (o.get_price(), o.size)),
        );
        result.extend(
            self.inner
                .floating_limit_orders
                .asks
                .iter()
                .map(|o| (o.get_price(slot, oracle_price), o.size())),
        );

        // Sort ascending for ask priority
        result.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        result
    }

    pub fn find_crosses(&self, slot: u64, oracle_price: u64) -> Vec<(u64, u64, u64, u64)> {
        let taker_bids = self.collect_taker_bids(slot, oracle_price);
        let taker_asks = self.collect_taker_asks(slot, oracle_price);
        let limit_bids = self.collect_limit_bids(slot, oracle_price);
        let limit_asks = self.collect_limit_asks(slot, oracle_price);

        // Helper function to find crosses between two sorted price lists
        let find_crosses = |bids: &[(u64, u64)], asks: &[(u64, u64)]| {
            bids.iter()
                .flat_map(|&(bid_price, bid_size)| {
                    asks.iter()
                        .take_while(move |(ask_price, _)| bid_price >= *ask_price)
                        .map(move |&(ask_price, ask_size)| {
                            (bid_price, ask_price, bid_size, ask_size)
                        })
                })
                .collect::<Vec<_>>()
        };

        // Combine all crosses
        let mut crosses = Vec::new();
        crosses.extend(find_crosses(&taker_bids, &limit_asks));
        crosses.extend(find_crosses(&limit_bids, &taker_asks));
        crosses.extend(find_crosses(&limit_bids, &limit_asks));

        crosses
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::pubkey::Pubkey;

    fn empty_orderbook() -> Orderbook {
        Orderbook {
            inner: OrderbookData {
                market_orders: DynamicOrders::default(),
                oracle_orders: DynamicOrders::default(),
                resting_limit_orders: Orders::default(),
                floating_limit_orders: DynamicOrders::default(),
                trigger_orders: Orders::default(),
            },
            l2_snapshot: Snapshot::default(),
            metadata: DashMap::default(),
        }
    }

    fn create_test_order(
        order_id: u32,
        order_type: OrderType,
        direction: Direction,
        price: i64,
        size: u64,
        slot: u64,
    ) -> Order {
        Order {
            order_id,
            order_type,
            direction,
            base_asset_amount: size,
            base_asset_amount_filled: 0,
            price: price as u64,
            auction_start_price: price,
            auction_end_price: price,
            slot,
            ..Default::default()
        }
    }

    #[test]
    fn test_dlob_market_order_sorting() {
        let mut orderbook = empty_orderbook();
        let user = Pubkey::new_unique();
        let slot = 100;

        // Insert bids in random order
        let mut order = create_test_order(1, OrderType::Market, Direction::Long, 100, 1, slot);
        order.auction_duration = 10;
        orderbook.insert_order(&user, order, slot);
        let mut order = create_test_order(2, OrderType::Market, Direction::Long, 200, 1, slot);
        order.auction_duration = 10;
        orderbook.insert_order(&user, order, slot);
        let mut order = create_test_order(3, OrderType::Market, Direction::Long, 150, 1, slot);
        order.auction_duration = 10;
        orderbook.insert_order(&user, order, slot);

        // Insert asks in random order
        let mut order = create_test_order(4, OrderType::Market, Direction::Short, 300, 1, slot);
        order.auction_duration = 10;
        orderbook.insert_order(&user, order, slot);
        let mut order = create_test_order(5, OrderType::Market, Direction::Short, 250, 1, slot);
        order.auction_duration = 10;
        orderbook.insert_order(&user, order, slot);
        let mut order = create_test_order(6, OrderType::Market, Direction::Short, 350, 1, slot);
        order.auction_duration = 10;
        orderbook.insert_order(&user, order, slot);

        orderbook.update_slot_and_oracle_price(slot, 0);
        let book = orderbook.inner();

        // Verify bids are sorted highest to lowest
        assert!(book
            .market_orders
            .bids
            .iter()
            .map(|x| x.start_price)
            .eq([200, 150, 100]));
        // Verify asks are sorted lowest to highest
        assert!(book
            .market_orders
            .asks
            .iter()
            .map(|x| x.start_price)
            .eq([250, 300, 350]));
    }

    #[test]
    fn test_dlob_limit_order_sorting() {
        let mut orderbook = empty_orderbook();
        let user = Pubkey::new_unique();
        let slot = 100;

        // Insert bids in random order
        let mut order = create_test_order(1, OrderType::Limit, Direction::Long, 100, 1, slot);
        order.post_only = true;
        orderbook.insert_order(&user, order, slot);

        let mut order = create_test_order(2, OrderType::Limit, Direction::Long, 200, 1, slot);
        order.post_only = true;
        orderbook.insert_order(&user, order, slot);

        let mut order = create_test_order(3, OrderType::Limit, Direction::Long, 150, 1, slot);
        order.post_only = true;
        orderbook.insert_order(&user, order, slot);

        // Insert asks in random order
        let mut order = create_test_order(4, OrderType::Limit, Direction::Short, 300, 1, slot);
        order.post_only = true;
        orderbook.insert_order(&user, order, slot);

        let mut order = create_test_order(5, OrderType::Limit, Direction::Short, 250, 1, slot);
        order.post_only = true;
        orderbook.insert_order(&user, order, slot);

        let mut order = create_test_order(6, OrderType::Limit, Direction::Short, 350, 1, slot);
        order.post_only = true;
        orderbook.insert_order(&user, order, slot);

        let data = orderbook.inner();

        // Verify bids are sorted highest to lowest
        let bid_prices: Vec<u64> = data
            .resting_limit_orders
            .bids
            .iter()
            .map(|(_, v)| v.get_price())
            .collect();
        assert_eq!(bid_prices, vec![200, 150, 100]);

        // Verify asks are sorted lowest to highest
        let ask_prices: Vec<u64> = data
            .resting_limit_orders
            .asks
            .iter()
            .map(|(_, v)| v.get_price())
            .collect();
        assert_eq!(ask_prices, vec![250, 300, 350]);
    }

    #[test]
    fn test_dlob_floating_limit_order_sorting() {
        let mut orderbook = empty_orderbook();
        let user = Pubkey::new_unique();
        let slot = 100;

        // Insert bids in random order
        let mut order = create_test_order(1, OrderType::Limit, Direction::Long, 100, 1, slot);
        order.oracle_price_offset = 10;
        orderbook.insert_order(&user, order, slot);

        let mut order = create_test_order(2, OrderType::Limit, Direction::Long, 200, 1, slot);
        order.oracle_price_offset = 30;
        orderbook.insert_order(&user, order, slot);

        let mut order = create_test_order(3, OrderType::Limit, Direction::Long, 150, 1, slot);
        order.oracle_price_offset = 20;
        orderbook.insert_order(&user, order, slot);

        // Insert asks in random order
        let mut order = create_test_order(4, OrderType::Limit, Direction::Short, 300, 1, slot);
        order.oracle_price_offset = -30;
        orderbook.insert_order(&user, order, slot);

        let mut order = create_test_order(5, OrderType::Limit, Direction::Short, 250, 1, slot);
        order.oracle_price_offset = -20;
        orderbook.insert_order(&user, order, slot);

        let mut order = create_test_order(6, OrderType::Limit, Direction::Short, 350, 1, slot);
        order.oracle_price_offset = -10;
        orderbook.insert_order(&user, order, slot);

        orderbook.update_slot_and_oracle_price(slot, 0);
        let book = orderbook.inner();

        // Verify bids are sorted highest to lowest offset
        let bid_offsets: Vec<i32> = book
            .floating_limit_orders
            .bids
            .iter()
            .map(|v| v.offset_price)
            .collect();
        assert_eq!(bid_offsets, vec![30, 20, 10]);

        // Verify asks are sorted lowest to highest offset
        let ask_offsets: Vec<i32> = book
            .floating_limit_orders
            .asks
            .iter()
            .map(|v| v.offset_price)
            .collect();
        assert_eq!(ask_offsets, vec![-30, -20, -10]);
    }

    #[test]
    fn test_dlob_oracle_order_sorting() {
        let mut orderbook = empty_orderbook();
        let user = Pubkey::new_unique();
        let slot = 100;

        // Insert bids in random order
        let mut order = create_test_order(1, OrderType::Oracle, Direction::Long, 100, 1, slot);
        order.oracle_price_offset = 10;
        order.auction_start_price = 10;
        order.auction_end_price = 20;
        order.auction_duration = 10;
        orderbook.insert_order(&user, order, slot);

        let mut order = create_test_order(2, OrderType::Oracle, Direction::Long, 200, 1, slot);
        order.oracle_price_offset = 30;
        order.auction_start_price = 30;
        order.auction_end_price = 40;
        order.auction_duration = 10;
        orderbook.insert_order(&user, order, slot);

        let mut order = create_test_order(3, OrderType::Oracle, Direction::Long, 150, 1, slot);
        order.oracle_price_offset = 20;
        order.auction_start_price = 20;
        order.auction_end_price = 30;
        order.auction_duration = 10;
        orderbook.insert_order(&user, order, slot);

        // Insert asks in random order
        let mut order = create_test_order(4, OrderType::Oracle, Direction::Short, 300, 1, slot);
        order.oracle_price_offset = -30;
        order.auction_start_price = -30;
        order.auction_end_price = -20;
        order.auction_duration = 10;
        orderbook.insert_order(&user, order, slot);

        let mut order = create_test_order(5, OrderType::Oracle, Direction::Short, 250, 1, slot);
        order.oracle_price_offset = -20;
        order.auction_start_price = -20;
        order.auction_end_price = -10;
        order.auction_duration = 10;
        orderbook.insert_order(&user, order, slot);

        let mut order = create_test_order(6, OrderType::Oracle, Direction::Short, 350, 1, slot);
        order.oracle_price_offset = -10;
        order.auction_start_price = -10;
        order.auction_end_price = 0;
        order.auction_duration = 10;
        orderbook.insert_order(&user, order, slot);

        orderbook.update_slot_and_oracle_price(slot, 1);
        let book = orderbook.inner();

        // Verify bids are sorted highest to lowest start price offset
        let bid_offsets: Vec<i64> = book
            .oracle_orders
            .bids
            .iter()
            .map(|v| v.start_price_offset)
            .collect();
        assert_eq!(bid_offsets, vec![30, 20, 10]);

        // Verify asks are sorted lowest to highest start price offset
        let ask_offsets: Vec<i64> = book
            .oracle_orders
            .asks
            .iter()
            .map(|v| v.start_price_offset)
            .collect();
        assert_eq!(ask_offsets, vec![-30, -20, -10]);
    }

    #[test]
    fn test_dlob_same_order_different_users() {
        let mut orderbook = empty_orderbook();
        let user1 = Pubkey::new_unique();
        let user2 = Pubkey::new_unique();
        let slot = 100;

        // Create identical orders for different users
        let mut order1 = create_test_order(1, OrderType::Limit, Direction::Long, 100, 1, slot);
        order1.post_only = true;
        orderbook.insert_order(&user1, order1, slot);

        let mut order2 = create_test_order(1, OrderType::Limit, Direction::Long, 100, 1, slot);
        order2.post_only = true;
        orderbook.insert_order(&user2, order2, slot);

        let data = orderbook.inner();

        // Verify both orders are in the book
        let bid_prices: Vec<u64> = data
            .resting_limit_orders
            .bids
            .iter()
            .map(|(_, v)| v.get_price())
            .collect();
        assert_eq!(bid_prices, vec![100, 100]);

        // Verify the orders have different IDs
        let bid_ids: Vec<u64> = data
            .resting_limit_orders
            .bids
            .iter()
            .map(|(_, v)| v.id)
            .collect();
        assert_ne!(bid_ids[0], bid_ids[1]);
    }

    #[test]
    fn test_dlob_l2_snapshot() {
        let mut orderbook = empty_orderbook();
        let user = Pubkey::new_unique();
        let slot = 100;
        let oracle_price = 1000;

        // Insert resting limit orders
        let mut order = create_test_order(1, OrderType::Limit, Direction::Long, 1100, 2, slot);
        order.post_only = true;
        orderbook.insert_order(&user, order, slot);

        let mut order = create_test_order(2, OrderType::Limit, Direction::Short, 900, 3, slot);
        order.post_only = true;
        orderbook.insert_order(&user, order, slot);

        // Insert market orders (dynamic price)
        let mut order = create_test_order(3, OrderType::Market, Direction::Long, 1050, 4, slot);
        order.auction_duration = 10;
        orderbook.insert_order(&user, order, slot);

        let mut order = create_test_order(4, OrderType::Market, Direction::Short, 950, 5, slot);
        order.auction_duration = 10;
        orderbook.insert_order(&user, order, slot);

        // Insert floating limit orders (dynamic price)
        let mut order = create_test_order(5, OrderType::Limit, Direction::Long, 0, 6, slot);
        order.oracle_price_offset = 100; // Will be 1100 with oracle_price
        orderbook.insert_order(&user, order, slot);

        let mut order = create_test_order(6, OrderType::Limit, Direction::Short, 0, 7, slot);
        order.oracle_price_offset = -100; // Will be 900 with oracle_price
        orderbook.insert_order(&user, order, slot);

        // Update slot and oracle price to calculate dynamic prices
        orderbook.update_slot_and_oracle_price(slot, oracle_price);

        // Update L2 snapshot
        orderbook.update_l2_view(slot, oracle_price);

        // Get the L2 snapshot
        let l2book = orderbook.get_l2_snapshot();

        // Verify bid prices and sizes
        // At 1100: 2 (resting limit) + 6 (floating limit) = 8
        assert_eq!(l2book.bids.get(&1100), Some(&8));
        // At 1050: 4 (market)
        assert_eq!(l2book.bids.get(&1050), Some(&4));

        // Verify ask prices and sizes
        // At 900: 3 (resting limit) + 7 (floating limit) = 10
        assert_eq!(l2book.asks.get(&900), Some(&10));
        // At 950: 5 (market)
        assert_eq!(l2book.asks.get(&950), Some(&5));

        // Verify no other prices exist
        assert_eq!(l2book.bids.len(), 2);
        assert_eq!(l2book.asks.len(), 2);

        // Test snapshot updates
        // Add a new limit order
        let mut order = create_test_order(7, OrderType::Limit, Direction::Long, 1075, 8, slot);
        order.post_only = true;
        orderbook.insert_order(&user, order, slot);

        // Update L2 snapshot
        orderbook.update_l2_view(slot, oracle_price);

        // Get updated L2 snapshot
        let l2book = orderbook.get_l2_snapshot();

        // Verify new order was added
        assert_eq!(l2book.bids.get(&1075), Some(&8));
        assert_eq!(l2book.bids.len(), 3);

        // Modify an existing order
        let mut order = create_test_order(1, OrderType::Limit, Direction::Long, 1100, 4, slot); // Changed size from 2 to 4
        order.post_only = true;
        orderbook.update_order(&user, order);

        // Update L2 snapshot
        orderbook.update_l2_view(slot, oracle_price);

        // Get updated L2 snapshot
        let l2book = orderbook.get_l2_snapshot();

        // Verify order was updated
        assert_eq!(l2book.bids.get(&1100), Some(&10)); // 4 (updated) + 6 (floating limit) = 10
        assert_eq!(l2book.bids.len(), 3);

        // Remove an order
        let mut order = create_test_order(3, OrderType::Market, Direction::Long, 1050, 4, slot);
        order.base_asset_amount_filled = order.base_asset_amount; // Set filled amount equal to total amount
        orderbook.update_order(&user, order);

        // Update L2 snapshot
        orderbook.update_l2_view(slot, oracle_price);

        // Get updated L2 snapshot
        let l2book = orderbook.get_l2_snapshot();

        // Verify order was removed
        assert_eq!(l2book.bids.get(&1050), None);
        assert_eq!(l2book.bids.len(), 2);
        dbg!(&l2book);
    }
}
