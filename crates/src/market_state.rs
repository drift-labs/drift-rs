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
}

/// Optimized storage for drift markets and oracles
pub struct MarketState {
    state: AtomicPtr<Arc<MarketStateData>>,
}

impl MarketState {
    /// Create a new lock-free market state
    pub fn new() -> Self {
        let initial_state = Box::into_raw(Box::new(Arc::new(MarketStateData::default())));
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

    pub fn get_perp_oracle_price(&self, market_index: u16) -> Option<OraclePriceData> {
        let current = self.load();
        current.perp_oracle_prices.get(&market_index).copied()
    }

    pub fn get_spot_oracle_price(&self, market_index: u16) -> Option<OraclePriceData> {
        let current = self.load();
        current.spot_oracle_prices.get(&market_index).copied()
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
        Self::new()
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
