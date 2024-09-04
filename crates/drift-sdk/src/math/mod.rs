use crate::drift_abi::{
    errors::ErrorCode,
    types::{MarginCalculationMode, MarginRequirementType, MarketIdentifier},
};

pub mod account_map_builder;
pub mod auction;
pub mod constants;
pub mod leverage;
pub mod liquidation;
pub mod order;

#[derive(Clone, Copy, Debug)]
pub struct MarginContext {
    pub margin_type: MarginRequirementType,
    pub mode: MarginCalculationMode,
    pub strict: bool,
    pub margin_buffer: u128,
    pub fuel_bonus_numerator: i64,
    pub fuel_bonus: u64,
    pub fuel_perp_delta: Option<(u16, i64)>,
    pub fuel_spot_deltas: [(u16, i128); 2],
}

impl MarginContext {
    pub fn standard(margin_type: MarginRequirementType) -> Self {
        Self {
            margin_type,
            mode: MarginCalculationMode::Standard {
                track_open_orders_fraction: false,
            },
            strict: false,
            margin_buffer: 0,
            fuel_bonus_numerator: 0,
            fuel_bonus: 0,
            fuel_perp_delta: None,
            fuel_spot_deltas: [(0, 0); 2],
        }
    }

    pub fn strict(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }

    pub fn margin_buffer(mut self, margin_buffer: u32) -> Self {
        self.margin_buffer = margin_buffer as u128;
        self
    }

    // how to change the user's spot position to match how it was prior to instruction change
    // i.e. diffs are ADDED to perp
    pub fn fuel_perp_delta(mut self, market_index: u16, delta: i64) -> Self {
        self.fuel_perp_delta = Some((market_index, delta));
        self
    }

    pub fn fuel_spot_delta(mut self, market_index: u16, delta: i128) -> Self {
        self.fuel_spot_deltas[0] = (market_index, delta);
        self
    }

    pub fn fuel_spot_deltas(mut self, deltas: [(u16, i128); 2]) -> Self {
        self.fuel_spot_deltas = deltas;
        self
    }

    pub fn track_open_orders_fraction(mut self) -> Result<Self, ErrorCode> {
        match self.mode {
            MarginCalculationMode::Standard {
                track_open_orders_fraction: ref mut track,
            } => {
                *track = true;
            }
            _ => {
                return Err(ErrorCode::InvalidMarginCalculation);
            }
        }
        Ok(self)
    }

    pub fn liquidation(margin_buffer: u32) -> Self {
        Self {
            margin_type: MarginRequirementType::Maintenance,
            mode: MarginCalculationMode::Liquidation {
                market_to_track_margin_requirement: None,
            },
            margin_buffer: margin_buffer as u128,
            strict: false,
            fuel_bonus_numerator: 0,
            fuel_bonus: 0,
            fuel_perp_delta: None,
            fuel_spot_deltas: [(0, 0); 2],
        }
    }

    pub fn track_market_margin_requirement(
        mut self,
        market_identifier: MarketIdentifier,
    ) -> Result<Self, ErrorCode> {
        match self.mode {
            MarginCalculationMode::Liquidation {
                market_to_track_margin_requirement: ref mut market_to_track,
                ..
            } => {
                *market_to_track = Some(market_identifier);
            }
            _ => {
                return Err(ErrorCode::InvalidMarginCalculation);
            }
        }
        Ok(self)
    }
}
