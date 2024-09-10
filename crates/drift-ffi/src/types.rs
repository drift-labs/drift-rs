//! cross-boundary FFI types
use abi_stable::std_types::RResult;
use drift_program::{
    math::margin::MarginRequirementType, state::margin_calculation::MarginContext,
};
use type_layout::TypeLayout;

/// FFI type-safe equivalent of `MarginContext`
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MarginContextMode {
    StandardMaintenance,
    StandardInitial,
    StandardCustom(MarginRequirementType),
}

impl From<MarginContextMode> for MarginContext {
    fn from(value: MarginContextMode) -> Self {
        match value {
            MarginContextMode::StandardMaintenance => {
                MarginContext::standard(MarginRequirementType::Maintenance)
            }
            MarginContextMode::StandardInitial => {
                MarginContext::standard(MarginRequirementType::Initial)
            }
            MarginContextMode::StandardCustom(m) => MarginContext::standard(m),
        }
    }
}

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, PartialEq, TypeLayout)]
pub struct MarginCalculation {
    pub total_collateral: compat::i128,
    pub margin_requirement: compat::u128,
    pub all_oracles_valid: bool,
    pub with_perp_isolated_liability: bool,
    pub with_spot_isolated_liability: bool,
    pub total_spot_asset_value: compat::i128,
    pub total_spot_liability_value: compat::u128,
    pub total_perp_liability_value: compat::u128,
    pub total_perp_pnl: compat::i128,
    pub open_orders_margin_requirement: compat::u128,
}

impl MarginCalculation {
    pub fn get_free_collateral(&self) -> u128 {
        (self.total_collateral.0 - self.margin_requirement.0 as i128) // cast ok, margin_requirement > 0
            .max(0)
            .try_into()
            .expect("fits u128")
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

pub mod compat {
    //! ffi compatibility input types

    /// rust 1.76.0 ffi compatible i128
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[repr(C, align(16))]
    pub struct i128(pub std::primitive::i128);

    impl From<std::primitive::i128> for self::i128 {
        fn from(value: std::primitive::i128) -> Self {
            Self(value)
        }
    }

    /// rust 1.76.0 ffi compatible u128
    #[derive(Copy, Clone, Debug, PartialEq)]
    #[repr(C, align(16))]
    pub struct u128(pub std::primitive::u128);

    impl From<std::primitive::u128> for self::u128 {
        fn from(value: std::primitive::u128) -> Self {
            Self(value)
        }
    }
}
