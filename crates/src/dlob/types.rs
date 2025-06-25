use std::sync::{atomic::AtomicPtr, Arc};

use arrayvec::ArrayVec;
use solana_sdk::pubkey::Pubkey;

use crate::{
    dlob::{Direction, OrderDelta},
    ffi::calculate_auction_price,
    math::standardize_price,
    types::{MarketType, Order, OrderParams, OrderTriggerCondition, OrderType},
};

// Replace the key structs with type aliases
type MarketOrderKey = (u64, u64);
type OracleOrderKey = (u64, u64);
type LimitOrderKey = (u64, u64, u64);
type FloatingLimitOrderKey = (i32, u64, u64);
type TriggerOrderKey = (u64, u64, u64);

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum OrderKind {
    /// auction fixed price offset
    Market,
    /// auction oracle offset
    Oracle,
    /// transient state before oracle limit order becomes resting
    FloatingLimitAuction,
    /// transient state before fixed limit order becomes resting
    LimitAuction,
    /// resting limit order
    Limit,
    /// resting oracle limit order
    FloatingLimit,
    Trigger,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OrderMetadata {
    pub order_id: u32,
    pub user: Pubkey,
    pub kind: OrderKind,
}

impl OrderMetadata {
    pub fn new(user: Pubkey, kind: OrderKind, order_id: u32) -> Self {
        Self {
            user,
            kind,
            order_id,
        }
    }
}

/// Minimal taker order info
#[derive(Copy, Clone, Debug)]
pub struct TakerOrder {
    pub price: u64,
    pub size: u64,
    pub direction: Direction,
    pub market_index: u16,
    pub market_type: MarketType,
}

impl TakerOrder {
    pub fn from_order_params(order: OrderParams, price: u64) -> Self {
        Self {
            price,
            size: order.base_asset_amount,
            direction: order.direction,
            market_index: order.market_index,
            market_type: order.market_type,
        }
    }
}

/// Best fills for a taker order
/// Returns (candidates, is_partial)
#[derive(Clone, Debug, Default)]
pub struct MakerCrosses {
    /// (metadata, maker_price, fill_size)
    pub orders: ArrayVec<(OrderMetadata, u64, u64), 16>,
    pub is_partial: bool,
    /// Slot crosses were found
    pub slot: u64,
}

impl MakerCrosses {
    /// Returns True if there were not crosses found
    pub fn is_empty(&self) -> bool {
        self.orders.is_empty()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum DLOBEvent {
    SlotOrPriceUpdate {
        slot: u64,
        market_index: u16,
        market_type: MarketType,
        oracle_price: u64,
    },
    Order {
        delta: OrderDelta,
        slot: u64,
    },
}

/// Order with dynamic price calculation
pub(crate) trait DynamicPrice {
    fn get_price(&self, slot: u64, oracle_price: u64, tick_size: u64) -> u64;
    fn size(&self) -> u64;
}

// Subset of order fields for sorting
pub(crate) trait OrderKey {
    type Key: Ord + Clone;
    fn key(&self) -> Self::Key;
}

impl OrderKey for MarketOrder {
    type Key = MarketOrderKey;
    fn key(&self) -> Self::Key {
        (self.slot, self.id)
    }
}

impl OrderKey for OracleOrder {
    type Key = OracleOrderKey;
    fn key(&self) -> Self::Key {
        (self.slot, self.id)
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

#[derive(Default, Clone, PartialEq, Debug)]
pub(crate) struct MarketOrder {
    pub id: u64,
    pub size: u64,
    pub start_price: i64,
    pub end_price: i64,
    pub duration: u8,
    pub slot: u64,
    pub is_limit: bool,
    pub direction: Direction,
}

#[derive(Default, Clone, PartialEq, Debug)]
pub(crate) struct OracleOrder {
    pub id: u64,
    pub size: u64,
    pub start_price_offset: i64,
    pub end_price_offset: i64,
    pub duration: u8,
    pub slot: u64,
    pub is_limit: bool,
    pub direction: Direction,
}

#[derive(Default, Clone, PartialEq, Eq)]
pub(crate) struct LimitOrder {
    pub id: u64,
    pub size: u64,
    pub price: u64,
    pub slot: u64,
}

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub(crate) struct FloatingLimitOrder {
    pub id: u64,
    pub size: u64,
    pub offset_price: i32,
    pub slot: u64,
}

#[derive(Default, Clone)]
pub(crate) struct TriggerOrder {
    pub id: u64,
    pub size: u64,
    pub price: u64,
    pub condition: OrderTriggerCondition,
    pub slot: u64,
}

impl DynamicPrice for MarketOrder {
    fn size(&self) -> u64 {
        self.size
    }
    fn get_price(&self, slot: u64, _oracle_price: u64, market_tick_size: u64) -> u64 {
        calculate_auction_price(
            &Order {
                slot: self.slot,
                auction_duration: self.duration,
                auction_start_price: self.start_price,
                auction_end_price: self.end_price,
                direction: self.direction,
                order_type: OrderType::Market,
                ..Default::default()
            },
            slot,
            market_tick_size,
            None,
            false,
        )
        .unwrap_or(0)
    }
}

impl From<(u64, Order)> for MarketOrder {
    fn from(value: (u64, Order)) -> Self {
        let (id, order) = value;
        Self {
            id,
            size: order.base_asset_amount - order.base_asset_amount_filled,
            start_price: order.auction_start_price,
            end_price: order.auction_end_price,
            duration: order.auction_duration,
            direction: order.direction,
            slot: order.slot,
            is_limit: order.order_type == OrderType::Limit,
        }
    }
}

impl DynamicPrice for OracleOrder {
    fn size(&self) -> u64 {
        self.size
    }
    fn get_price(&self, slot: u64, oracle_price: u64, tick_size: u64) -> u64 {
        let slots_elapsed = slot.saturating_sub(self.slot) as i64;
        let delta_denominator = self.duration as i64;
        let delta_numerator = slots_elapsed.min(delta_denominator);

        if delta_denominator == 0 {
            let price = ((oracle_price as i64 + self.end_price_offset) as u64).max(tick_size);

            return standardize_price(price, tick_size, self.direction);
        }

        let price_offset = if self.direction == Direction::Long {
            let delta = (self
                .end_price_offset
                .saturating_sub(self.start_price_offset)
                * delta_numerator)
                / delta_denominator;
            self.start_price_offset.saturating_add(delta)
        } else {
            let delta = (self
                .start_price_offset
                .saturating_sub(self.end_price_offset)
                * delta_numerator)
                / delta_denominator;
            self.start_price_offset.saturating_sub(delta)
        };

        let price = ((oracle_price as i64 + price_offset) as u64).max(tick_size);

        standardize_price(price, tick_size, self.direction)
    }
}

impl From<(u64, Order)> for OracleOrder {
    fn from(value: (u64, Order)) -> Self {
        let (id, order) = value;
        Self {
            id,
            size: order.base_asset_amount - order.base_asset_amount_filled,
            start_price_offset: order.auction_start_price,
            end_price_offset: order.auction_end_price,
            duration: order.auction_duration,
            slot: order.slot,
            is_limit: order.order_type == OrderType::Limit,
            direction: order.direction,
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
    fn get_price(&self, _slot: u64, oracle_price: u64, _market_tick_size: u64) -> u64 {
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
