//! Unified HashMap for providing market and oracle data for margin calculations
//! replaces the programs AccountLoader types for FFI
use fxhash::FxBuildHasher;

use crate::{
    drift_idl::accounts::{PerpMarket, SpotMarket},
    utils::Snapshot,
    OraclePriceData,
};
use std::{collections::HashMap, sync::Arc};

/// Internal data structure for market state
#[derive(Clone, Debug, Default)]
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
#[derive(Default)]
pub struct MarketState {
    state: Snapshot<MarketStateData>,
}

impl MarketState {
    /// Create a new lock-free market state
    pub fn new(data: MarketStateData) -> Self {
        Self {
            state: Snapshot::new(data),
        }
    }

    /// Get a lock-free read-only reference to the current market state
    ///
    /// This returns an Arc<MarketStateData> that can be safely used for calculations
    /// without blocking writers. The Arc ensures the data remains valid even if
    /// the state is updated concurrently.
    pub fn load(&self) -> Arc<MarketStateData> {
        self.state.read()
    }

    /// Update a single spot market
    pub fn set_spot_market(&self, market: SpotMarket) {
        self.state.write(|s| {
            s.spot_markets.insert(market.market_index, market);
        });
    }

    /// Update a single perp market
    pub fn set_perp_market(&self, market: PerpMarket) {
        self.state.write(|s| {
            s.perp_markets.insert(market.market_index, market);
        });
    }

    /// Update spot oracle price
    pub fn set_spot_oracle_price(&self, market_index: u16, price: OraclePriceData) {
        self.state.write(|s| {
            s.spot_oracle_prices.insert(market_index, price);
        });
    }

    /// Update perp oracle price
    pub fn set_perp_oracle_price(&self, market_index: u16, price: OraclePriceData) {
        self.state.write(|s| {
            s.perp_oracle_prices.insert(market_index, price);
        });
    }

    pub fn get_perp_oracle_price(&self, market_index: u16) -> Option<OraclePriceData> {
        let current = self.load();
        current.perp_oracle_prices.get(&market_index).copied()
    }

    pub fn get_spot_oracle_price(&self, market_index: u16) -> Option<OraclePriceData> {
        let current = self.load();
        current.spot_oracle_prices.get(&market_index).copied()
    }

    pub fn get_perp_market(&self, market_index: u16) -> Option<PerpMarket> {
        let current = self.load();
        current.perp_markets.get(&market_index).copied()
    }
}
