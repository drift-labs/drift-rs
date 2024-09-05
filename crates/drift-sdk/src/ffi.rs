//!
//! FFI shims
//! Defines wrapper types for ergonomic access to drift-program logic
//!
use solana_program::clock::Slot;
use solana_sdk::account_info::AccountInfo;

pub use self::abi_types::*;
use crate::{
    drift_abi::{
        accounts,
        errors::ErrorCode,
        types::{self, OracleSource},
    },
    SdkResult,
};

// c-abi compat for 128-bit integers is required (i.e rust >= 1.77.0)
static_assertions::const_assert_eq!(align_of::<i128>(), 16);
static_assertions::const_assert_eq!(align_of::<u128>(), 16);

/// Defines an upgrade from plain IDL generated type into an FFI version with drift program functionality available
pub trait IntoFfi {
    type Output;
    /// Convert self into an FFI type with drift-program functionality
    fn ffi(&self) -> Self::Output;
}

/// FFI equivalent of an `AccountMap`
#[repr(C)]
#[derive(Clone)]
pub struct AccountsList<'a> {
    pub perp_markets: AccountList<'a>,
    pub spot_markets: AccountList<'a>,
    pub oracles: AccountList<'a>,
    pub latest_slot: Slot,
}

impl<'a> AccountsList<'a> {
    #[cfg(test)]
    pub fn new(
        perp_markets: &mut [AccountInfo<'a>],
        spot_markets: &mut [AccountInfo<'a>],
        oracles: &mut [AccountInfo<'a>],
    ) -> Self {
        Self {
            perp_markets: AccountList::new(perp_markets),
            spot_markets: AccountList::new(spot_markets),
            oracles: AccountList::new(oracles),
            latest_slot: 0,
        }
    }
}

/// List of accounts
#[repr(C)]
#[derive(Clone)]
pub struct AccountList<'a> {
    accounts: *mut AccountInfo<'a>,
    count: usize,
}

impl<'a> AccountList<'a> {
    pub fn new(accounts: &mut [AccountInfo<'a>]) -> Self {
        Self {
            count: accounts.len(),
            accounts: accounts.as_mut_ptr(),
        }
    }
    pub fn as_mut_slice(&'a mut self) -> &mut [AccountInfo<'a>] {
        unsafe { core::slice::from_raw_parts_mut(self.accounts, self.count) }
    }
    pub fn as_slice(&'a self) -> &[AccountInfo<'a>] {
        unsafe { core::slice::from_raw_parts(self.accounts, self.count) }
    }
}

// TODO: AccountInfo is not FFI safe...
// `std::rc::Rc<std::cell::RefCell<&mut u64>>`

// Declarations of exported functions from `drift-ffi` lib
// the types here must be C abi safe/compatible
//
// DEV: the types here are deliberately received as those defined in `::abi_types`-
// which are equivalent to the drift-ffi exported types directly from drift program crate
// the result is that this code can use its own solana-program/* crates without restriction from the version used by drift program
extern "C" {
    pub fn math_calculate_margin_requirement_and_total_collateral_and_liability_info<'a>(
        user: &accounts::User,
        accounts: &'a mut AccountsList<'a>,
        mode: MarginContextMode,
    ) -> FfiResult<MarginCalculation>;

    pub fn oracle_get_oracle_price(
        orace_source: &OracleSource,
        account_info: &AccountInfo,
        slot: Slot,
    ) -> FfiResult<OraclePriceData>;

    pub fn order_is_limit_order(ptr: *const types::Order) -> bool;

    #[allow(improper_ctypes)]
    pub fn perp_market_get_margin_ratio(
        ptr: *const accounts::PerpMarket,
        size: u128,
        margin_type: MarginRequirementType,
    ) -> FfiResult<u32>;

    pub fn perp_position_is_available(ptr: *const types::PerpPosition) -> bool;
    pub fn perp_position_is_open_position(ptr: *const types::PerpPosition) -> bool;
    #[allow(improper_ctypes)]
    pub fn perp_position_worst_case_base_asset_amount(
        ptr: *const types::PerpPosition,
        oracle_price: i64,
        contract_type: ContractType,
    ) -> FfiResult<i128>;
    #[allow(improper_ctypes)]
    pub fn perp_position_simulate_settled_lp_position(
        ptr: *const types::PerpPosition,
        market: *const accounts::PerpMarket,
        oracle_price: i64,
    ) -> FfiResult<types::PerpPosition>;

    #[allow(improper_ctypes)]
    pub fn spot_market_get_asset_weight(
        ptr: *const accounts::SpotMarket,
        size: u128,
        oracle_price: i64,
        margin_requirement_type: MarginRequirementType,
    ) -> FfiResult<u32>;
    #[allow(improper_ctypes)]
    pub fn spot_market_get_liability_weight(
        ptr: *const accounts::SpotMarket,
        size: u128,
        margin_requirement_type: MarginRequirementType,
    ) -> FfiResult<u32>;

    pub fn spot_position_is_available(ptr: *const types::SpotPosition) -> bool;
    #[allow(improper_ctypes)]
    pub fn spot_position_get_signed_token_amount(
        ptr: *const types::SpotPosition,
        market: *const accounts::SpotMarket,
    ) -> FfiResult<i128>;

    pub fn user_get_spot_position(
        ptr: *const accounts::User,
        market_index: u16,
    ) -> FfiResult<types::SpotPosition>;
    pub fn user_get_perp_position(
        ptr: *const accounts::User,
        market_index: u16,
    ) -> FfiResult<types::PerpPosition>;
}

//
// shims for sdk
//
pub fn get_oracle_price(
    orace_source: &OracleSource,
    account_info: &AccountInfo,
    slot: Slot,
) -> SdkResult<OraclePriceData> {
    to_sdk_result(unsafe { oracle_get_oracle_price(orace_source, account_info, slot) })
}

pub fn calculate_margin_requirement_and_total_collateral_and_liability_info<'a>(
    user: &'a accounts::User,
    accounts: &'a mut AccountsList<'a>,
    mode: MarginContextMode,
) -> SdkResult<MarginCalculation> {
    let res = unsafe {
        math_calculate_margin_requirement_and_total_collateral_and_liability_info(
            user, accounts, mode,
        )
    };
    to_sdk_result(res)
}

pub struct SpotPosition(types::SpotPosition);

impl SpotPosition {
    pub fn is_available(&self) -> bool {
        unsafe { spot_position_is_available(&self.0) }
    }
    pub fn get_signed_token_amount(&self, market: &accounts::SpotMarket) -> SdkResult<i128> {
        to_sdk_result(unsafe { spot_position_get_signed_token_amount(&self.0, market) })
    }
}

impl IntoFfi for types::SpotPosition {
    type Output = SpotPosition;
    fn ffi(&self) -> Self::Output {
        SpotPosition(*self)
    }
}

impl From<SpotPosition> for types::SpotPosition {
    fn from(value: SpotPosition) -> Self {
        value.0
    }
}

pub struct PerpPosition(types::PerpPosition);

impl PerpPosition {
    pub fn is_available(&self) -> bool {
        unsafe { perp_position_is_available(&self.0) }
    }
    pub fn is_open_position(&self) -> bool {
        unsafe { perp_position_is_open_position(&self.0) }
    }
    pub fn worst_case_base_asset_amount(
        &self,
        oracle_price: i64,
        contract_type: ContractType,
    ) -> SdkResult<i128> {
        to_sdk_result(unsafe {
            perp_position_worst_case_base_asset_amount(&self.0, oracle_price, contract_type)
        })
    }
    pub fn simulate_settled_lp_position(
        &self,
        market: &accounts::PerpMarket,
        oracle_price: i64,
    ) -> SdkResult<types::PerpPosition> {
        to_sdk_result(unsafe {
            perp_position_simulate_settled_lp_position(&self.0, market, oracle_price)
        })
    }
}

impl IntoFfi for types::PerpPosition {
    type Output = PerpPosition;
    fn ffi(&self) -> Self::Output {
        PerpPosition(*self)
    }
}

impl From<PerpPosition> for types::PerpPosition {
    fn from(value: PerpPosition) -> Self {
        value.0
    }
}

pub struct User(accounts::User);

impl User {
    pub fn get_spot_position(&self, market_index: u16) -> SdkResult<SpotPosition> {
        to_sdk_result(unsafe { user_get_spot_position(&self.0, market_index) }).map(SpotPosition)
    }
    pub fn get_perp_position(&self, market_index: u16) -> SdkResult<PerpPosition> {
        to_sdk_result(unsafe { user_get_perp_position(&self.0, market_index) }).map(PerpPosition)
    }
}

impl IntoFfi for accounts::User {
    type Output = User;
    fn ffi(&self) -> Self::Output {
        User(*self)
    }
}

impl From<User> for accounts::User {
    fn from(value: User) -> Self {
        value.0
    }
}

pub struct Order(types::Order);

impl Order {
    pub fn is_limit_order(&self) -> bool {
        unsafe { order_is_limit_order(&self.0) }
    }
}

impl IntoFfi for types::Order {
    type Output = Order;
    fn ffi(&self) -> Self::Output {
        Order(*self)
    }
}

pub struct SpotMarket(accounts::SpotMarket);

impl SpotMarket {
    pub fn get_asset_weight(
        &self,
        size: u128,
        oracle_price: i64,
        margin_requirement_type: MarginRequirementType,
    ) -> SdkResult<u32> {
        to_sdk_result(unsafe {
            spot_market_get_asset_weight(&self.0, size, oracle_price, margin_requirement_type)
        })
    }
    pub fn get_liability_weight(
        &self,
        size: u128,
        margin_requirement_type: MarginRequirementType,
    ) -> SdkResult<u32> {
        to_sdk_result(unsafe {
            spot_market_get_liability_weight(&self.0, size, margin_requirement_type)
        })
    }
}

impl IntoFfi for accounts::SpotMarket {
    type Output = SpotMarket;
    fn ffi(&self) -> Self::Output {
        SpotMarket(*self)
    }
}

impl From<SpotMarket> for accounts::SpotMarket {
    fn from(value: SpotMarket) -> Self {
        value.0
    }
}

pub struct PerpMarket(accounts::PerpMarket);

impl PerpMarket {
    pub fn get_margin_ratio(
        &self,
        size: u128,
        margin_requirement_type: MarginRequirementType,
    ) -> SdkResult<u32> {
        to_sdk_result(unsafe {
            perp_market_get_margin_ratio(&self.0, size, margin_requirement_type)
        })
    }
}

impl IntoFfi for accounts::PerpMarket {
    type Output = PerpMarket;
    fn ffi(&self) -> Self::Output {
        PerpMarket(*self)
    }
}

impl From<PerpMarket> for accounts::PerpMarket {
    fn from(value: PerpMarket) -> Self {
        value.0
    }
}

fn to_sdk_result<T>(value: FfiResult<T>) -> SdkResult<T> {
    match value {
        FfiResult::ROk(t) => Ok(t),
        FfiResult::RErr(code) => {
            let error_code: ErrorCode =
                unsafe { std::mem::transmute(code - anchor_lang::error::ERROR_CODE_OFFSET) };
            Err(crate::SdkError::Anchor(Box::new(error_code.into())))
        }
    }
}

impl From<crate::drift_abi::types::ContractType> for ContractType {
    fn from(value: crate::drift_abi::types::ContractType) -> Self {
        match value {
            crate::drift_abi::types::ContractType::Future => ContractType::Future,
            crate::drift_abi::types::ContractType::Perpetual => ContractType::Perpetual,
            crate::drift_abi::types::ContractType::Prediction => ContractType::Prediction,
        }
    }
}

pub mod abi_types {
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
}
