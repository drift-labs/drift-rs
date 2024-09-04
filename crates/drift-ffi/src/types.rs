//! cross-boundary FFI types
//!
//! DEV: _must_ not include solana-* crates
use abi_stable::std_types::RResult;

/// FFI type-safe equivalent of `MarginContext`
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MarginContextMode {
    StandardMaintenance,
    StandardInitial,
    StandardCustom(MarginRequirementType),
}

/// FFI type-safe equivalent of `MarginRequirementType`
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MarginRequirementType {
    Initial,
    Fill,
    Maintenance,
}

/// FFI type-safe equivalent of `ContractType`
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ContractType {
    Perpetual,
    Future,
    Prediction,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MarginCalculation {
    /// PRICE_PRECISION
    pub total_collateral: i128,
    /// PRICE_PRECISION
    pub margin_requirement: u128,
    pub all_oracles_valid: bool,
    pub with_perp_isolated_liability: bool,
    pub with_spot_isolated_liability: bool,
    pub total_spot_asset_value: i128,
    /// PRICE_PRECISION
    pub total_spot_liability_value: u128,
    /// PRICE_PRECISION
    pub total_perp_liability_value: u128,
    /// PRICE_PRECISION
    pub total_perp_pnl: i128,
    /// PRICE_PRECISION
    pub open_orders_margin_requirement: u128,
}

impl MarginCalculation {
    pub fn get_free_collateral(&self) -> u128 {
        (self.total_collateral - self.margin_requirement as i128).max(0) as u128
    }
}

#[repr(C)]
#[derive(Default, Clone, Copy, Debug)]
pub struct OraclePriceData {
    pub price: i64,
    pub confidence: u64,
    pub delay: i64,
    pub has_sufficient_number_of_data_points: bool,
}

/// C-ABI compatible result type for drift FFI calls
pub type FfiResult<T> = RResult<T, u32>;
