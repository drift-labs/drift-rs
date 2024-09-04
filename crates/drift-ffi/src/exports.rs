use core::slice;

use abi_stable::std_types::RResult::{RErr, ROk};
pub use drift_program::state::user::User;
use drift_program::{
    math::margin::MarginRequirementType,
    state::{
        margin_calculation::MarginContext,
        oracle::{get_oracle_price as get_oracle_price_, OracleSource},
        oracle_map::OracleMap,
        perp_market::{ContractType, PerpMarket},
        perp_market_map::PerpMarketMap,
        spot_market::SpotMarket,
        spot_market_map::SpotMarketMap,
        user::{Order, PerpPosition, SpotPosition},
    },
};
use solana_program::{account_info::AccountInfo, clock::Slot};

use crate::types::{FfiResult, MarginCalculation, MarginContextMode, OraclePriceData};

#[no_mangle]
pub extern "C" fn oracle_get_oracle_price(
    oracle_source: &OracleSource,
    price_oracle: &AccountInfo,
    clock_slot: Slot,
) -> FfiResult<OraclePriceData> {
    to_ffi_result(
        get_oracle_price_(oracle_source, price_oracle, clock_slot)
            .map(|o| unsafe { std::mem::transmute(o) }),
    )
}

#[no_mangle]
pub extern "C" fn math_calculate_margin_requirement_and_total_collateral_and_liability_info<'a>(
    user: &User,
    accounts: &'a mut AccountsList<'a>,
    margin_context: MarginContextMode,
) -> FfiResult<MarginCalculation> {
    let margin_calculation = drift_program::math::margin::calculate_margin_requirement_and_total_collateral_and_liability_info(
        user,
        &PerpMarketMap::load(&Default::default(), &mut accounts.perp_markets.as_mut_slice().iter().peekable()).unwrap(),
        &SpotMarketMap::load(&Default::default(), &mut accounts.spot_markets.as_mut_slice().iter().peekable()).unwrap(),
        &mut OracleMap::load(&mut accounts.oracles.as_mut_slice().iter().peekable(), accounts.latest_slot, None).unwrap(),
        margin_context.into(),
    );

    let m = margin_calculation.map(|m| MarginCalculation {
        total_collateral: m.total_collateral,
        margin_requirement: m.margin_requirement,
        all_oracles_valid: m.all_oracles_valid,
        with_perp_isolated_liability: m.with_perp_isolated_liability,
        with_spot_isolated_liability: m.with_spot_isolated_liability,
        total_spot_asset_value: m.total_spot_asset_value,
        total_spot_liability_value: m.total_spot_liability_value,
        total_perp_liability_value: m.total_perp_liability_value,
        total_perp_pnl: m.total_perp_pnl,
        open_orders_margin_requirement: m.open_orders_margin_requirement,
    });

    to_ffi_result(m)
}

#[no_mangle]
pub extern "C" fn order_is_limit_order(ptr: *const Order) -> bool {
    unsafe { *ptr }.is_limit_order()
}

#[no_mangle]
pub extern "C" fn perp_market_get_margin_ratio(
    ptr: *const PerpMarket,
    size: u128,
    margin_type: crate::types::MarginRequirementType,
) -> FfiResult<u32> {
    to_ffi_result(unsafe { *ptr }.get_margin_ratio(size, margin_type.into()))
}

#[no_mangle]
pub extern "C" fn perp_position_is_available(ptr: *const PerpPosition) -> bool {
    unsafe { *ptr }.is_available()
}

#[no_mangle]
pub extern "C" fn perp_position_is_open_position(ptr: *const PerpPosition) -> bool {
    unsafe { *ptr }.is_open_position()
}

#[no_mangle]
pub extern "C" fn perp_position_worst_case_base_asset_amount(
    ptr: *const PerpPosition,
    oracle_price: i64,
    contract_type: crate::types::ContractType,
) -> FfiResult<i128> {
    to_ffi_result(unsafe { *ptr }.worst_case_base_asset_amount(oracle_price, contract_type.into()))
}

#[no_mangle]
pub extern "C" fn perp_position_simulate_settled_lp_position(
    ptr: *const PerpPosition,
    market: *const PerpMarket,
    oracle_price: i64,
) -> FfiResult<PerpPosition> {
    let position = unsafe { *ptr };
    let market = unsafe { *market };
    to_ffi_result(position.simulate_settled_lp_position(&market, oracle_price))
}

#[no_mangle]
pub extern "C" fn spot_market_get_asset_weight(
    ptr: *const SpotMarket,
    size: u128,
    oracle_price: i64,
    margin_requirement_type: crate::types::MarginRequirementType,
) -> FfiResult<u32> {
    to_ffi_result(unsafe { *ptr }.get_asset_weight(
        size,
        oracle_price,
        &margin_requirement_type.into(),
    ))
}

#[no_mangle]
pub extern "C" fn spot_market_get_liability_weight(
    ptr: *const SpotMarket,
    size: u128,
    margin_requirement_type: crate::types::MarginRequirementType,
) -> FfiResult<u32> {
    to_ffi_result(unsafe { *ptr }.get_liability_weight(size, &margin_requirement_type.into()))
}

#[no_mangle]
pub extern "C" fn spot_position_is_available(ptr: *const SpotPosition) -> bool {
    unsafe { *ptr }.is_available()
}

#[no_mangle]
pub extern "C" fn spot_position_get_signed_token_amount(
    ptr: *const SpotPosition,
    market: *const SpotMarket,
) -> FfiResult<i128> {
    let market = unsafe { *market };
    to_ffi_result(unsafe { *ptr }.get_signed_token_amount(&market))
}

#[no_mangle]
pub extern "C" fn user_get_spot_position(
    ptr: *const User,
    market_index: u16,
) -> FfiResult<*const SpotPosition> {
    to_ffi_result(
        unsafe { *ptr }
            .get_spot_position(market_index)
            .map(|p| p as *const SpotPosition),
    )
}

#[no_mangle]
pub extern "C" fn user_get_perp_position(
    ptr: *const User,
    market_index: u16,
) -> FfiResult<*const PerpPosition> {
    to_ffi_result(
        unsafe { *ptr }
            .get_perp_position(market_index)
            .map(|p| p as *const PerpPosition),
    )
}

//
// Inbound Types
//
#[repr(C)]
pub struct AccountsList<'a> {
    pub perp_markets: AccountList<'a>,
    pub spot_markets: AccountList<'a>,
    pub oracles: AccountList<'a>,
    pub latest_slot: Slot,
}

#[repr(C)]
pub struct AccountList<'a> {
    accounts: *mut AccountInfo<'a>,
    count: usize,
}

impl<'a> AccountList<'a> {
    pub fn new(accounts: &mut [AccountInfo<'a>]) -> Self {
        Self {
            accounts: accounts.as_mut_ptr(),
            count: accounts.len(),
        }
    }
    fn as_mut_slice(&'a mut self) -> &mut [AccountInfo<'a>] {
        unsafe { slice::from_raw_parts_mut(self.accounts, self.count) }
    }
}

//
// Helpers
//
impl From<MarginContextMode> for MarginContext {
    fn from(value: MarginContextMode) -> Self {
        match value {
            MarginContextMode::StandardMaintenance => {
                MarginContext::standard(MarginRequirementType::Initial)
            }
            MarginContextMode::StandardInitial => {
                MarginContext::standard(MarginRequirementType::Maintenance)
            }
            _ => {
                panic!("unknown margin context mode");
            }
        }
    }
}

impl From<crate::types::MarginRequirementType> for MarginRequirementType {
    fn from(value: crate::types::MarginRequirementType) -> Self {
        match value {
            crate::types::MarginRequirementType::Fill => MarginRequirementType::Fill,
            crate::types::MarginRequirementType::Initial => MarginRequirementType::Initial,
            crate::types::MarginRequirementType::Maintenance => MarginRequirementType::Maintenance,
        }
    }
}

impl From<crate::types::ContractType> for ContractType {
    fn from(value: crate::types::ContractType) -> Self {
        match value {
            crate::types::ContractType::Perpetual => ContractType::Perpetual,
            crate::types::ContractType::Future => ContractType::Future,
            crate::types::ContractType::Prediction => ContractType::Prediction,
        }
    }
}

/// Convert Drift program result into an FFI compatible version
#[inline]
pub(crate) fn to_ffi_result<T>(result: Result<T, drift_program::error::ErrorCode>) -> FfiResult<T> {
    match result {
        Ok(r) => ROk(r),
        Err(err) => RErr(err.into()),
    }
}
