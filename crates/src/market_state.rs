//! Unified HashMap for providing market and oracle data for margin calculations
//! replaces the programs AccountLoader types for FFI
use fxhash::FxBuildHasher;

use crate::{
    drift_idl::accounts::{PerpMarket, SpotMarket},
    OraclePriceData,
};
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicPtr, Ordering},
        Arc,
    },
};

/// Internal data structure for market state
#[derive(Clone, Default)]
pub struct MarketStateData {
    pub spot_markets: HashMap<u16, SpotMarket, FxBuildHasher>,
    pub perp_markets: HashMap<u16, PerpMarket, FxBuildHasher>,
    pub spot_oracle_prices: HashMap<u16, OraclePriceData, FxBuildHasher>,
    pub perp_oracle_prices: HashMap<u16, OraclePriceData, FxBuildHasher>,
    pub spot_pyth_prices: HashMap<u16, i64, FxBuildHasher>, // Override spot with pyth price
    pub perp_pyth_prices: HashMap<u16, i64, FxBuildHasher>, // Override perp with pyth price
    pub pyth_oracle_diff_threshold_bps: u64, // Min bps diff to prefer pyth price over oracle. Defaults to 0 (always use pyth when set).
}

impl MarketStateData {
    pub fn set_spot_market(&mut self, market: SpotMarket) {
        self.spot_markets.insert(market.market_index, market);
    }

    pub fn set_perp_market(&mut self, market: PerpMarket) {
        self.perp_markets.insert(market.market_index, market);
    }

    pub fn set_spot_oracle_price(&mut self, market_index: u16, price: OraclePriceData) {
        self.spot_oracle_prices.insert(market_index, price);
    }

    pub fn set_perp_oracle_price(&mut self, market_index: u16, price: OraclePriceData) {
        self.perp_oracle_prices.insert(market_index, price);
    }

    pub fn set_spot_pyth_price(&mut self, market_index: u16, price_data: i64) {
        self.spot_pyth_prices.insert(market_index, price_data);
    }

    pub fn set_perp_pyth_price(&mut self, market_index: u16, price_data: i64) {
        self.perp_pyth_prices.insert(market_index, price_data);
    }
}

/// Optimized storage for drift markets and oracles
pub struct MarketState {
    state: AtomicPtr<Arc<MarketStateData>>,
}

impl MarketState {
    /// Create a lock-free market state with initial data
    pub fn new(data: MarketStateData) -> Self {
        let initial_state = Box::into_raw(Box::new(Arc::new(data)));
        Self {
            state: AtomicPtr::new(initial_state),
        }
    }

    /// Get a lock-free read-only reference to the current market state
    ///
    /// This returns an Arc<MarketStateData> that can be safely used for calculations
    /// without blocking writers. The Arc ensures the data remains valid even if
    /// the state is updated concurrently.
    pub fn load(&self) -> Arc<MarketStateData> {
        unsafe {
            let ptr = self.state.load(Ordering::Acquire);
            (*ptr).clone()
        }
    }

    /// Atomically update the entire market state
    ///
    /// This creates a new Arc<MarketStateData> with the updated data and atomically
    /// replaces the current state. All readers will see the new state on their
    /// next load() call. The old state is properly deallocated.
    fn store(&self, new_state: Arc<MarketStateData>) {
        let new_ptr = Box::into_raw(Box::new(new_state));
        let old_ptr = self.state.swap(new_ptr, Ordering::AcqRel);

        // Deallocate the old state
        if !old_ptr.is_null() {
            unsafe {
                let _ = Box::from_raw(old_ptr);
            }
        }
    }

    /// Update a single spot market
    pub fn set_spot_market(&self, market: SpotMarket) {
        let current = self.load();
        let mut new_data = (*current).clone();
        new_data.set_spot_market(market);
        self.store(Arc::new(new_data));
    }

    /// Update a single perp market
    pub fn set_perp_market(&self, market: PerpMarket) {
        let current = self.load();
        let mut new_data = (*current).clone();
        new_data.set_perp_market(market);
        self.store(Arc::new(new_data));
    }

    /// Update spot oracle price
    pub fn set_spot_oracle_price(&self, market_index: u16, price: OraclePriceData) {
        let current = self.load();
        let mut new_data = (*current).clone();
        new_data.set_spot_oracle_price(market_index, price);
        self.store(Arc::new(new_data));
    }

    /// Update perp oracle price
    pub fn set_perp_oracle_price(&self, market_index: u16, price: OraclePriceData) {
        let current = self.load();
        let mut new_data = (*current).clone();
        new_data.set_perp_oracle_price(market_index, price);
        self.store(Arc::new(new_data));
    }

    /// Update spot pyth price
    pub fn set_spot_pyth_price(&self, market_index: u16, price: i64) {
        let current = self.load();
        let mut new_data = (*current).clone();
        new_data.set_spot_pyth_price(market_index, price);
        self.store(Arc::new(new_data));
    }

    /// Update perp pyth price
    pub fn set_perp_pyth_price(&self, market_index: u16, price: i64) {
        let current = self.load();
        let mut new_data = (*current).clone();
        new_data.set_perp_pyth_price(market_index, price);
        self.store(Arc::new(new_data));
    }

    pub fn get_perp_oracle_price(&self, market_index: u16) -> Option<OraclePriceData> {
        let current = self.load();
        current.perp_oracle_prices.get(&market_index).copied()
    }

    pub fn get_spot_oracle_price(&self, market_index: u16) -> Option<OraclePriceData> {
        let current = self.load();
        current.spot_oracle_prices.get(&market_index).copied()
    }

    pub fn get_spot_pyth_price(&self, market_index: u16) -> Option<OraclePriceData> {
        let current = self.load();
        current
            .spot_pyth_prices
            .get(&market_index)
            .map(|&price| OraclePriceData {
                price,
                confidence: 0,
                delay: 0,
                has_sufficient_number_of_data_points: true,
                sequence_id: None,
            })
    }

    pub fn get_perp_pyth_price(&self, market_index: u16) -> Option<OraclePriceData> {
        let current = self.load();
        current
            .perp_pyth_prices
            .get(&market_index)
            .map(|&price| OraclePriceData {
                price,
                confidence: 0,
                delay: 0,
                has_sufficient_number_of_data_points: true,
                sequence_id: None,
            })
    }
    /// Batch update multiple markets atomically
    ///
    /// This is more efficient than multiple individual updates as it only
    /// creates one new state and performs one atomic swap.
    pub fn batch_update<F>(&self, update_fn: F)
    where
        F: FnOnce(&mut MarketStateData),
    {
        let current = self.load();
        let mut new_data = (*current).clone();
        update_fn(&mut new_data);
        self.store(Arc::new(new_data));
    }
}

impl Default for MarketState {
    fn default() -> Self {
        Self::new(MarketStateData::default())
    }
}

impl Drop for MarketState {
    fn drop(&mut self) {
        let ptr = self.state.load(Ordering::Acquire);
        if !ptr.is_null() {
            unsafe {
                let _ = Box::from_raw(ptr);
            }
        }
    }
}
