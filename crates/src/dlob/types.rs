use std::{
    fmt::Debug,
    sync::{atomic::AtomicPtr, Arc},
};

use arrayvec::ArrayVec;
use solana_sdk::pubkey::Pubkey;

use crate::{
    dlob::{Direction, OrderDelta},
    ffi::{calculate_auction_price, OraclePriceData},
    math::standardize_price,
    types::{
        accounts::PerpMarket, MarketId, MarketType, Order, OrderParams, OrderStatus,
        OrderTriggerCondition, OrderType, SdkResult,
    },
};

// Replace the key structs with type aliases
type MarketOrderKey = (u64, u64);
type OracleOrderKey = (i64, u64);
type LimitOrderKey = (u64, u64, u64);
type FloatingLimitOrderKey = (i32, u64, u64);
type TriggerOrderKey = (u64, u64);

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Copy, PartialEq)]
#[repr(u8)]
pub enum OrderKind {
    /// auction fixed price offset
    Market,
    /// auction oracle offset
    Oracle,
    /// oracle limit order undergoing initial auction (taking)
    FloatingLimitAuction,
    /// fixed limit order undergoing initial auction (taking)
    LimitAuction,
    /// resting limit order
    Limit,
    /// resting oracle limit order
    FloatingLimit,
    /// trigger order that will result in Market or Oracle auction order (untriggered)
    TriggerMarket,
    /// trigger order that will result in Limit/Market auction order (untriggered)
    TriggerLimit,
    /// Triggered oracle order
    OracleTriggered,
    /// Triggered market order
    MarketTriggered,
    /// Triggered limit order
    LimitTriggered,
}

impl OrderKind {
    /// Returns true if the order is a taker order
    pub fn is_taker(&self) -> bool {
        !self.is_maker()
    }
    /// Returns true if the order is a maker order
    pub fn is_maker(&self) -> bool {
        matches!(self, OrderKind::Limit | OrderKind::FloatingLimit)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Copy, PartialEq)]
pub struct OrderMetadata {
    pub max_ts: u64,
    pub order_id: u32,
    pub user: Pubkey,
    pub kind: OrderKind,
}

impl OrderMetadata {
    pub fn new(user: Pubkey, kind: OrderKind, order_id: u32, max_ts: u64) -> Self {
        Self {
            user,
            kind,
            order_id,
            max_ts,
        }
    }
}

/// Minimal taker order info
#[derive(Copy, Clone, Debug)]
pub struct TakerOrder {
    pub price: u64,
    pub size: u64,
    pub market_index: u16,
    pub direction: Direction,
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

#[derive(Clone, Debug, Default)]
/// Orderbook crosses and top maker info
pub struct CrossesAndTopMakers {
    //  best maker accounts on ask side
    pub top_maker_asks: ArrayVec<Pubkey, 3>,
    //  best maker accounts on bid side
    pub top_maker_bids: ArrayVec<Pubkey, 3>,
    // top of book limit cross, if any
    pub limit_crosses: Option<(OrderMetadata, OrderMetadata)>,
    pub vamm_taker_ask: Option<OrderMetadata>,
    pub vamm_taker_bid: Option<OrderMetadata>,
    //  taker crosses and maker orders
    pub crosses: Vec<(OrderMetadata, MakerCrosses)>,
}

/// Best fills for a taker order
/// Returns (candidates, is_partial)
#[derive(Clone, Debug, Default)]
pub struct MakerCrosses {
    /// (metadata, maker_price, fill_size)
    pub orders: ArrayVec<(OrderMetadata, u64, u64), 16>,
    /// Slot crosses were found
    pub slot: u64,
    // true if crosses VAMM quote
    pub has_vamm_cross: bool,
    pub is_partial: bool,
    pub taker_direction: Direction,
}

impl MakerCrosses {
    /// Returns True if there were no crosses found
    pub fn is_empty(&self) -> bool {
        self.orders.is_empty() && !self.has_vamm_cross
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum DLOBEvent {
    /// dlob slot and/or oracle update
    SlotAndOracleUpdate {
        slot: u64,
        oracle_price: u64,
        market: MarketId,
    },
    /// user order deltas
    Deltas { deltas: Vec<OrderDelta>, slot: u64 },
}

/// Order with dynamic price calculation
pub(crate) trait DynamicPrice {
    fn get_price(&self, slot: u64, oracle_price: u64, tick_size: u64) -> u64;
    fn size(&self) -> u64;
}

// Subset of order fields for sorting
pub(crate) trait OrderKey {
    type Key: Ord + Clone + Debug;
    fn key(&self) -> Self::Key;
}

impl OrderKey for MarketOrder {
    type Key = MarketOrderKey;
    fn key(&self) -> Self::Key {
        (self.slot, self.id)
    }
}

impl MarketOrder {
    /// Check if this auction order has completed
    pub fn is_auction_complete(&self, current_slot: u64) -> bool {
        current_slot.saturating_sub(self.slot) > self.duration as u64
    }

    /// Convert to LimitOrder when auction completes
    pub fn to_limit_order(&self) -> LimitOrder {
        LimitOrder {
            id: self.id,
            size: self.size,
            price: self.price,
            slot: self.slot,
            max_ts: self.max_ts,
            post_only: false,
            reduce_only: self.reduce_only,
        }
    }
}

impl OrderKey for OracleOrder {
    type Key = OracleOrderKey;
    fn key(&self) -> Self::Key {
        (self.end_price_offset, self.id)
    }
}

impl OracleOrder {
    /// Check if this auction order has completed
    pub fn is_auction_complete(&self, current_slot: u64) -> bool {
        (self.slot + self.duration as u64) <= current_slot
    }

    /// Convert to FloatingLimitOrder when auction completes
    pub fn to_floating_limit_order(&self) -> FloatingLimitOrder {
        FloatingLimitOrder {
            id: self.id,
            slot: self.slot,
            size: self.size,
            offset_price: self.oracle_price_offset,
            max_ts: self.max_ts,
            post_only: false,
            reduce_only: self.reduce_only,
        }
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
        // nb: trigger order slot updates when triggered so is unreliable as a sort key
        (self.price, self.id)
    }
}

#[derive(Default, Clone, PartialEq, Debug)]
pub(crate) struct MarketOrder {
    pub id: u64,
    pub size: u64,
    pub start_price: i64,
    pub end_price: i64,
    pub price: u64, // the limit price post auction
    pub duration: u8,
    pub slot: u64,
    pub max_ts: u64,
    pub is_limit: bool,
    pub direction: Direction,
    pub reduce_only: bool,
}

#[derive(Default, Clone, PartialEq, Debug)]
pub(crate) struct OracleOrder {
    pub id: u64,
    pub size: u64,
    pub start_price_offset: i64,
    pub end_price_offset: i64,
    pub oracle_price_offset: i32,
    pub max_ts: u64,
    pub slot: u64,
    pub duration: u8,
    pub is_limit: bool,
    pub direction: Direction,
    pub reduce_only: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct LimitOrderView {
    /// Internal order id
    pub id: u64,
    /// Price of the order
    pub price: u64,
    /// Size of the order
    pub size: u64,
    /// Slot of the order
    pub slot: u64,
    /// Whether the order is post-only
    pub post_only: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub(crate) struct LimitOrder {
    pub id: u64,
    pub size: u64,
    pub price: u64,
    pub slot: u64,
    pub max_ts: u64,
    pub post_only: bool,
    pub reduce_only: bool,
}

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub(crate) struct FloatingLimitOrder {
    pub id: u64,
    pub size: u64,
    pub slot: u64,
    pub max_ts: u64,
    pub offset_price: i32,
    pub post_only: bool,
    pub reduce_only: bool,
}

#[allow(dead_code)]
#[derive(Default, Debug, Clone)]
pub(crate) struct TriggerOrder {
    pub id: u64,
    pub size: u64,
    /// static trigger price
    pub price: u64,
    pub slot: u64,
    pub max_ts: u64,
    pub condition: OrderTriggerCondition,
    pub direction: Direction,
    pub kind: OrderType,
    pub bit_flags: u8,
    pub reduce_only: bool,
}

impl TriggerOrder {
    /// Returns true if the order would trigger at the given `oracle_price`
    pub fn will_trigger_at(&self, oracle_price: u64) -> bool {
        oracle_price != 0
            && match self.condition {
                OrderTriggerCondition::Above => oracle_price > self.price,
                OrderTriggerCondition::Below => oracle_price < self.price,
                _ => true, // technically unreachable
            }
    }
    /// Returns order price if it were triggered at `slot` with the current market parameters, `oracle_price` and `perp_market`
    pub fn get_price(
        &self,
        slot: u64,
        oracle_price: u64,
        perp_market: Option<&PerpMarket>,
    ) -> SdkResult<u64> {
        // TODO: safe trigger order can fill against VAMM
        // if slot.saturating_sub(order.slot) > 150 && self.reduce_only {
        //     order.add_bit_flag(OrderBitFlag::SafeTriggerOrder);
        // }
        if let Some(market) = perp_market {
            let mut order = Order {
                slot, // slot is the current slot (i.e simulate trigger and place)
                direction: self.direction,
                order_type: self.kind,
                market_index: market.market_index,
                market_type: MarketType::Perp,
                base_asset_amount: self.size,
                status: OrderStatus::Open,
                trigger_condition: match self.condition {
                    OrderTriggerCondition::Above => OrderTriggerCondition::TriggeredAbove,
                    OrderTriggerCondition::Below => OrderTriggerCondition::TriggeredBelow,
                    _ => self.condition,
                },
                bit_flags: self.bit_flags,
                ..Default::default()
            };
            let (auction_duration, auction_start, auction_end) =
                crate::ffi::calculate_auction_params_for_trigger_order(
                    &order,
                    &OraclePriceData {
                        price: oracle_price as i64,
                        confidence: 0,
                        delay: 0,
                        has_sufficient_number_of_data_points: true,
                        sequence_id: None,
                    },
                    Some(market),
                )
                .unwrap();
            order.auction_duration = auction_duration;
            order.auction_start_price = auction_start;
            order.auction_end_price = auction_end;

            if matches!(order.order_type, OrderType::TriggerMarket) {
                order.bit_flags |= Order::ORACLE_TRIGGER_MARKET_FLAG;
            }

            return calculate_auction_price(
                &order,
                slot,
                market.amm.order_tick_size,
                Some(oracle_price as i64),
                false,
            );
        }

        todo!("implement spot market trigger price");
    }
}

impl DynamicPrice for MarketOrder {
    fn size(&self) -> u64 {
        self.size
    }
    fn get_price(&self, slot: u64, _oracle_price: u64, tick_size: u64) -> u64 {
        let slots_elapsed = slot.saturating_sub(self.slot) as i64;
        let delta_denominator = self.duration as i64;
        let delta_numerator = slots_elapsed.min(delta_denominator);

        if delta_denominator == 0 {
            return self.end_price as u64;
        }

        let price = if self.direction == Direction::Long {
            let delta = (self.end_price.saturating_sub(self.start_price) * delta_numerator)
                / delta_denominator;
            self.start_price.saturating_add(delta)
        } else {
            let delta = (self.start_price.saturating_sub(self.end_price) * delta_numerator)
                / delta_denominator;
            self.start_price.saturating_sub(delta)
        };

        let price = price.max(tick_size as i64);
        standardize_price(price as u64, tick_size, self.direction)
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
            price: order.price,
            duration: order.auction_duration,
            direction: order.direction,
            slot: order.slot,
            is_limit: matches!(order.order_type, OrderType::Limit | OrderType::TriggerLimit),
            max_ts: order.max_ts as u64,
            reduce_only: order.reduce_only,
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
            oracle_price_offset: order.oracle_price_offset,
            duration: order.auction_duration,
            slot: order.slot,
            is_limit: order.order_type == OrderType::Limit,
            direction: order.direction,
            max_ts: order.max_ts as u64,
            reduce_only: order.reduce_only,
        }
    }
}

impl LimitOrder {
    pub fn get_price(&self) -> u64 {
        self.price
    }
    pub fn is_expired(&self, now_unix_seconds: u64) -> bool {
        self.max_ts > now_unix_seconds
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
            max_ts: order.max_ts as u64,
            post_only: order.post_only,
            reduce_only: order.reduce_only,
        }
    }
}

impl FloatingLimitOrder {
    pub fn is_expired(&self, now_unix_seconds: u64) -> bool {
        self.max_ts > now_unix_seconds
    }
    pub fn get_price(&self, oracle_price: u64, tick_size: u64) -> u64 {
        (oracle_price as i64 + self.offset_price as i64).max(tick_size as i64) as u64
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
            max_ts: order.max_ts as u64,
            post_only: order.post_only,
            reduce_only: order.reduce_only,
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
            max_ts: order.max_ts.unsigned_abs(),
            slot: order.slot,
            direction: order.direction,
            kind: order.order_type,
            bit_flags: order.bit_flags,
            reduce_only: order.reduce_only,
        }
    }
}

/// Double-buffered snapshot of T
///
/// Provides lock-free reads/write API
pub struct Snapshot<T: Default + Clone> {
    a: AtomicPtr<T>,
    b: AtomicPtr<T>,
}

impl<T: Default + Clone> Default for Snapshot<T> {
    fn default() -> Self {
        Self::new(T::default(), T::default())
    }
}

impl<T: Default + Clone> Snapshot<T> {
    /// Create a new double buffer from two initial values.
    pub fn new(a: T, b: T) -> Self {
        let a = Arc::into_raw(Arc::new(a)) as *mut T;
        let b = Arc::into_raw(Arc::new(b)) as *mut T;
        Self {
            a: AtomicPtr::new(a),
            b: AtomicPtr::new(b),
        }
    }

    /// Read the snapshot
    #[inline]
    pub fn read(&self) -> Arc<T> {
        unsafe {
            let ptr = self.a.load(std::sync::atomic::Ordering::Acquire);
            Arc::increment_strong_count(ptr);
            Arc::from_raw(ptr)
        }
    }

    /// Write the snapshot
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut T),
    {
        let b_ptr = self.b.load(std::sync::atomic::Ordering::Relaxed);
        let b = unsafe { Arc::from_raw(b_ptr as *const T) };
        let mut b_mut = Arc::try_unwrap(b).unwrap_or_else(|arc| (*arc).clone());
        f(&mut b_mut);
        let new_b = Arc::into_raw(Arc::new(b_mut)) as *mut T;
        self.b.store(new_b, std::sync::atomic::Ordering::Relaxed);
        self.swap();
    }

    /// atomic swap of a/b pointers.
    #[inline]
    fn swap(&self) {
        let a_ptr = self.a.load(std::sync::atomic::Ordering::Acquire);
        let b_ptr = self.b.load(std::sync::atomic::Ordering::Acquire);
        self.a.store(b_ptr, std::sync::atomic::Ordering::Release);
        self.b.store(a_ptr, std::sync::atomic::Ordering::Release);
    }
}

impl<T: Default + Clone> Drop for Snapshot<T> {
    fn drop(&mut self) {
        unsafe {
            let a_ptr = self.a.load(std::sync::atomic::Ordering::Relaxed);
            let b_ptr = self.b.load(std::sync::atomic::Ordering::Relaxed);

            // Only drop non-null pointers to avoid double-free
            if !a_ptr.is_null() {
                drop(Arc::from_raw(a_ptr));
            }
            if !b_ptr.is_null() {
                drop(Arc::from_raw(b_ptr));
            }
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct CrossingOrder {
    pub order_view: LimitOrderView,
    pub metadata: OrderMetadata,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct CrossingRegion {
    pub slot: u64,
    pub crossing_bids: Vec<CrossingOrder>,
    pub crossing_asks: Vec<CrossingOrder>,
}

#[derive(Debug, Clone)]
pub struct L3Order {
    /// point in time limit price of the order at some slot & oracle price
    pub price: u64,
    /// base asset amount of the order
    pub size: u64,
    /// order expiry ts
    pub max_ts: u64,
    /// program assigned order id
    pub order_id: u32,
    pub kind: OrderKind,
    /// user subaccount of the order
    pub user: Pubkey,
    /// internal order flags
    pub(crate) flags: u8,
}

impl L3Order {
    /// when set indicates order is reduce only
    pub(crate) const RO_FLAG: u8 = 0b1000_0000;
    /// When set indicates order direction is long
    pub(crate) const IS_LONG: u8 = 0b0100_0000;
    /// when set and order kind is trigger, this bit indicates 'trigger above'
    /// conversely, 'trigger below' when unset
    pub(crate) const IS_TRIGGER_ABOVE: u8 = 0b0010_0000;
    /// True if this is a long order, false otherwise
    pub fn is_long(&self) -> bool {
        self.flags & Self::IS_LONG > 0
    }
    /// True if this is a reduce only order, false otherwise
    pub fn is_reduce_only(&self) -> bool {
        self.flags & Self::RO_FLAG > 0
    }
    pub fn is_trigger_above(&self) -> bool {
        self.flags & Self::IS_TRIGGER_ABOVE > 0
    }
    /// Calculate the 'limit' price of an _untriggered_ perp trigger order
    ///
    /// i.e. if order was triggered onchain immediately at the current `slot` and `oracle_price`
    pub fn post_trigger_price(
        &self,
        slot: u64,
        oracle_price: u64,
        perp_market: &PerpMarket,
    ) -> Option<u64> {
        if matches!(
            self.kind,
            OrderKind::TriggerMarket | OrderKind::TriggerLimit
        ) {
            let condition = if self.flags & Self::IS_TRIGGER_ABOVE > 0 {
                OrderTriggerCondition::Above
            } else {
                OrderTriggerCondition::Below
            };
            let order = TriggerOrder {
                id: 0,
                size: self.size,
                reduce_only: self.is_reduce_only(),
                max_ts: self.max_ts,
                direction: if self.is_long() {
                    Direction::Long
                } else {
                    Direction::Short
                },
                condition,
                slot,
                ..Default::default()
            };
            order.get_price(slot, oracle_price, Some(perp_market)).ok()
        } else {
            None
        }
    }
    /// True if order is maker only
    pub fn is_maker(&self) -> bool {
        self.kind.is_maker()
    }
    /// True if order is taker only
    pub fn is_taker(&self) -> bool {
        self.kind.is_taker()
    }
}
