//!
//! FFI shims
//! Defines wrapper types for ergonomic access to drift-program logic
//!
use solana_program::clock::Slot;
use solana_sdk::{account::Account, pubkey::Pubkey};

pub use self::abi_types::*;
use crate::{
    drift_idl::{
        accounts,
        errors::ErrorCode,
        types::{self, ContractType, MarginRequirementType, OracleSource},
    },
    SdkResult,
};

// Declarations of exported functions from `drift-ffi` lib
// the types here must be C abi safe/compatible
//
// DEV: the types here are deliberately received as those defined in `::abi_types`-
// which are equivalent to the drift-ffi exported types directly from drift program crate
// the result is that this code can use its own solana-program/* crates without restriction from the version used by drift program
extern "C" {
    #[allow(improper_ctypes)]
    // tuple ok: compile the ffi crate with same rust version = same layout
    pub fn math_calculate_margin_requirement_and_total_collateral_and_liability_info(
        user: &accounts::User,
        accounts: &mut AccountsList,
        mode: MarginContextMode,
    ) -> FfiResult<MarginCalculation>;

    #[allow(improper_ctypes)]
    // tuple ok: compile the ffi crate with same rust version = same layout
    pub fn oracle_get_oracle_price(
        orace_source: &OracleSource,
        oracle_account: &mut (Pubkey, Account),
        slot: Slot,
    ) -> FfiResult<OraclePriceData>;

    #[allow(improper_ctypes)]
    pub fn order_is_limit_order(order: &types::Order) -> bool;
    #[allow(improper_ctypes)]
    pub fn order_is_resting_limit_order(order: &types::Order, slot: Slot) -> FfiResult<bool>;

    #[allow(improper_ctypes)]
    pub fn perp_market_get_margin_ratio(
        market: &accounts::PerpMarket,
        size: u128,
        margin_type: MarginRequirementType,
    ) -> FfiResult<u32>;
    #[allow(improper_ctypes)]
    pub fn perp_market_get_open_interest(market: &accounts::PerpMarket) -> u128;

    #[allow(improper_ctypes)]
    pub fn perp_position_get_unrealized_pnl(
        position: &types::PerpPosition,
        oracle_price: i64,
    ) -> FfiResult<i128>;
    pub fn perp_position_is_available(position: &types::PerpPosition) -> bool;
    pub fn perp_position_is_open_position(position: &types::PerpPosition) -> bool;
    #[allow(improper_ctypes)]
    pub fn perp_position_worst_case_base_asset_amount(
        position: &types::PerpPosition,
        oracle_price: i64,
        contract_type: ContractType,
    ) -> FfiResult<i128>;
    #[allow(improper_ctypes)]
    pub fn perp_position_simulate_settled_lp_position(
        position: &types::PerpPosition,
        market: &accounts::PerpMarket,
        oracle_price: i64,
    ) -> FfiResult<types::PerpPosition>;

    #[allow(improper_ctypes)]
    pub fn spot_market_get_asset_weight(
        market: &accounts::SpotMarket,
        size: u128,
        oracle_price: i64,
        margin_requirement_type: MarginRequirementType,
    ) -> FfiResult<u32>;
    #[allow(improper_ctypes)]
    pub fn spot_market_get_liability_weight(
        market: &accounts::SpotMarket,
        size: u128,
        margin_requirement_type: MarginRequirementType,
    ) -> FfiResult<u32>;
    #[allow(improper_ctypes)]
    pub fn spot_position_is_available(position: &types::SpotPosition) -> bool;
    #[allow(improper_ctypes)]
    pub fn spot_position_get_signed_token_amount(
        position: &types::SpotPosition,
        market: &accounts::SpotMarket,
    ) -> FfiResult<i128>;
    #[allow(improper_ctypes)]
    pub fn spot_position_get_token_amount(
        position: &types::SpotPosition,
        market: &accounts::SpotMarket,
    ) -> FfiResult<u128>;
    #[allow(improper_ctypes)]
    pub fn user_get_spot_position(
        user: &accounts::User,
        market_index: u16,
    ) -> FfiResult<&types::SpotPosition>;
    #[allow(improper_ctypes)]
    pub fn user_get_perp_position(
        user: &accounts::User,
        market_index: u16,
    ) -> FfiResult<&types::PerpPosition>;
}

//
// Shims for SDK
//
pub fn get_oracle_price(
    orace_source: &OracleSource,
    oracle_account: &mut (Pubkey, Account),
    slot: Slot,
) -> SdkResult<OraclePriceData> {
    to_sdk_result(unsafe { oracle_get_oracle_price(orace_source, oracle_account, slot) })
}

pub fn calculate_margin_requirement_and_total_collateral_and_liability_info(
    user: &accounts::User,
    accounts: &mut AccountsList,
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
    pub fn get_token_amount(&self, market: &accounts::SpotMarket) -> SdkResult<u128> {
        to_sdk_result(unsafe { spot_position_get_token_amount(&self.0, market) })
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
    pub fn get_unrealized_pnl(&self, oracle_price: i64) -> SdkResult<i128> {
        to_sdk_result(unsafe { perp_position_get_unrealized_pnl(&self.0, oracle_price) })
    }
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
        // TODO: no clone
        to_sdk_result(unsafe { user_get_spot_position(&self.0, market_index) })
            .map(|p| SpotPosition(*p))
    }
    pub fn get_perp_position(&self, market_index: u16) -> SdkResult<PerpPosition> {
        to_sdk_result(unsafe { user_get_perp_position(&self.0, market_index) })
            .map(|p| PerpPosition(*p))
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
    pub fn is_resting_limit_order(&self, slot: Slot) -> SdkResult<bool> {
        to_sdk_result(unsafe { order_is_resting_limit_order(&self.0, slot) })
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
    pub fn get_open_interest(&self) -> u128 {
        unsafe { perp_market_get_open_interest(&self.0) }
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
            let error_code = unsafe {
                std::mem::transmute::<u32, ErrorCode>(code - anchor_lang::error::ERROR_CODE_OFFSET)
            };
            Err(crate::SdkError::Anchor(Box::new(error_code.into())))
        }
    }
}

pub mod abi_types {
    //! cross-boundary FFI types
    use abi_stable::std_types::RResult;
    use solana_sdk::{account::Account, clock::Slot, pubkey::Pubkey};
    use type_layout::TypeLayout;

    use crate::drift_idl::types::MarginRequirementType;

    /// FFI safe version of (pubkey, account)
    #[repr(C)]
    pub struct AccountWithKey {
        pub key: Pubkey,
        pub account: Account,
    }

    impl From<(Pubkey, Account)> for AccountWithKey {
        fn from(value: (Pubkey, Account)) -> Self {
            Self {
                key: value.0,
                account: value.1,
            }
        }
    }

    /// FFI equivalent of an `AccountMap`
    #[repr(C)]
    pub struct AccountsList<'a> {
        pub perp_markets: &'a mut [AccountWithKey],
        pub spot_markets: &'a mut [AccountWithKey],
        pub oracles: &'a mut [AccountWithKey],
        pub latest_slot: Slot,
    }

    impl<'a> AccountsList<'a> {
        #[cfg(test)]
        pub fn new(
            perp_markets: &'a mut [AccountWithKey],
            spot_markets: &'a mut [AccountWithKey],
            oracles: &'a mut [AccountWithKey],
        ) -> Self {
            Self {
                perp_markets,
                spot_markets,
                oracles,
                latest_slot: 0,
            }
        }
    }

    /// FFI safe equivalent of `MarginContext`
    #[repr(C)]
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum MarginContextMode {
        StandardMaintenance,
        StandardInitial,
        StandardCustom(MarginRequirementType),
    }

    // TODO: simplified version of MarginCalculation
    // can pipe through fill struct if needed
    #[repr(C, align(16))]
    #[derive(Copy, Clone, Debug, PartialEq, TypeLayout)]
    pub struct MarginCalculation {
        pub total_collateral: i128,
        pub margin_requirement: u128,
        pub all_oracles_valid: bool,
        pub with_perp_isolated_liability: bool,
        pub with_spot_isolated_liability: bool,
        pub total_spot_asset_value: i128,
        pub total_spot_liability_value: u128,
        pub total_perp_liability_value: u128,
        pub total_perp_pnl: i128,
        pub open_orders_margin_requirement: u128,
    }

    impl MarginCalculation {
        pub fn get_free_collateral(&self) -> u128 {
            (self.total_collateral - self.margin_requirement as i128) // safe cast, margin requirement >= 0
                .max(0) as u128
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

/// Defines an upgrade from plain IDL generated type into an FFI version with drift program functionality available
pub trait IntoFfi {
    type Output;
    /// Convert self into an FFI type with drift-program functionality
    fn ffi(&self) -> Self::Output;
}

#[cfg(test)]
mod tests {
    use anchor_lang::Discriminator;
    use solana_sdk::{account::Account, pubkey::Pubkey};
    use type_layout::TypeLayout;

    use super::{
        AccountWithKey, AccountsList, IntoFfi, MarginCalculation, MarginContextMode, SpotPosition,
    };
    use crate::{
        constants::{self},
        drift_idl::{
            accounts::{
                PerpMarket as DriftPerpMarket, SpotMarket as DriftSpotMarket, User as DriftUser,
            },
            types::{
                ContractType, MarginRequirementType, OracleSource, Order as DriftOrder, OrderType,
                PerpPosition as DriftPerpPosition, SpotBalanceType,
                SpotPosition as DriftSpotPosition,
            },
        },
        ffi::{
            calculate_margin_requirement_and_total_collateral_and_liability_info, get_oracle_price,
        },
        math::constants::{
            BASE_PRECISION_I64, LIQUIDATION_FEE_PRECISION, MARGIN_PRECISION, QUOTE_PRECISION,
            QUOTE_PRECISION_I64, SPOT_BALANCE_PRECISION, SPOT_BALANCE_PRECISION_U64,
            SPOT_CUMULATIVE_INTEREST_PRECISION, SPOT_WEIGHT_PRECISION,
        },
    };

    const _SOL_PYTH_PRICE_STR: &str = include_str!("../../res/sol-oracle-pyth.hex");
    /// encoded pyth price account for SOL, see math/liquidation.rs tests
    const SOL_PYTH_PRICE: std::cell::LazyCell<Vec<u8>> =
        std::cell::LazyCell::new(|| hex::decode(_SOL_PYTH_PRICE_STR).unwrap());

    fn sol_spot_market() -> DriftSpotMarket {
        DriftSpotMarket {
            market_index: 1,
            oracle_source: OracleSource::Pyth,
            oracle: solana_sdk::pubkey!("J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix"),
            cumulative_deposit_interest: SPOT_CUMULATIVE_INTEREST_PRECISION.into(),
            cumulative_borrow_interest: SPOT_CUMULATIVE_INTEREST_PRECISION.into(),
            decimals: 9,
            initial_asset_weight: 8 * SPOT_WEIGHT_PRECISION / 10,
            maintenance_asset_weight: 9 * SPOT_WEIGHT_PRECISION / 10,
            initial_liability_weight: 12 * SPOT_WEIGHT_PRECISION / 10,
            maintenance_liability_weight: 11 * SPOT_WEIGHT_PRECISION / 10,
            liquidator_fee: LIQUIDATION_FEE_PRECISION / 1000,
            deposit_balance: (1_000 * SPOT_BALANCE_PRECISION).into(),
            ..Default::default()
        }
    }

    #[test]
    fn ffi_deser_1_76_0_spot_market() {
        // smoke test for deserializing program data (where u128/i128 alignment is 8)
        let buf = hex_literal::hex!("64b1086ba84141270000000000000000000000000000000000000000000000000000000000000000fe650f0367d4a7ef9815a593ea15d36593f0643aaaf0149bb04be67ab851decd0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010a5d4e800000000000000000000000000000000000000000000000000000000e40b5402000000000000000000000000e40b54020000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000401f000028230000e02e0000f82a000000000000e803000000000000000000000000000000000000090000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
        let actual: &DriftSpotMarket = bytemuck::from_bytes::<DriftSpotMarket>(&buf.as_ref()[8..]); // ignore dscriminator

        assert_eq!(actual, &sol_spot_market());
    }

    #[test]
    fn ffi_spot_position_is_available() {
        let default_position = DriftSpotPosition::default();
        let spot_position = SpotPosition(default_position);

        assert!(spot_position.is_available());
    }

    #[test]
    fn ffi_spot_position_get_signed_token_amount() {
        let spot_position = DriftSpotPosition {
            scaled_balance: (123 * SPOT_BALANCE_PRECISION) as u64,
            market_index: 1,
            balance_type: SpotBalanceType::Deposit,
            ..Default::default()
        };

        let result = spot_position
            .ffi()
            .get_signed_token_amount(&sol_spot_market());
        assert_eq!(result.unwrap(), 123 * SPOT_BALANCE_PRECISION as i128);
    }

    #[test]
    fn ffi_spot_market_get_asset_weight() {
        let spot_market = DriftSpotMarket {
            initial_asset_weight: 9_000,
            initial_liability_weight: 11_000,
            decimals: 6,
            imf_factor: 0,
            ..Default::default()
        };
        let size = 1_000 * QUOTE_PRECISION;
        let price = QUOTE_PRECISION as i64;
        let asset_weight = spot_market
            .ffi()
            .get_asset_weight(size, price, MarginRequirementType::Initial)
            .unwrap();
        assert_eq!(asset_weight, 9_000);
    }

    #[test]
    fn ffi_spot_market_get_liability_weight() {
        let spot_market = DriftSpotMarket {
            initial_asset_weight: 9_000,
            initial_liability_weight: 11_000,
            decimals: 6,
            imf_factor: 0,
            ..Default::default()
        };

        let size = 1_000 * QUOTE_PRECISION;
        let liability_weight = spot_market
            .ffi()
            .get_liability_weight(size, MarginRequirementType::Initial)
            .unwrap();
        assert_eq!(liability_weight, 11_000);
    }

    #[test]
    fn ffi_user_get_spot_position() {
        let mut user = DriftUser::default();
        user.spot_positions[1] = DriftSpotPosition {
            market_index: 1,
            scaled_balance: 1_000 * SPOT_BALANCE_PRECISION_U64,
            balance_type: SpotBalanceType::Deposit,
            ..Default::default()
        };

        let result = user.ffi().get_spot_position(1);
        assert!(result.is_ok());
        let spot_position = result.unwrap();
        assert_eq!(spot_position.0.market_index, 1);
        assert_eq!(
            spot_position.0.scaled_balance,
            1_000 * SPOT_BALANCE_PRECISION_U64
        );
        assert_eq!(spot_position.0.balance_type, SpotBalanceType::Deposit);

        // Test for non-existent market index
        let result = user.ffi().get_spot_position(5);
        assert!(result.is_err());
    }

    #[test]
    fn ffi_user_get_perp_position() {
        let mut user = DriftUser::default();
        user.perp_positions[2] = DriftPerpPosition {
            market_index: 2,
            base_asset_amount: 500,
            quote_asset_amount: 1_000,
            lp_shares: 1_000,
            ..Default::default()
        };

        let result = user.ffi().get_perp_position(2);
        assert!(result.is_ok());
        let perp_position = result.unwrap();
        assert_eq!(perp_position.0.market_index, 2);
        assert_eq!(perp_position.0.base_asset_amount, 500);
        assert_eq!(perp_position.0.quote_asset_amount, 1_000);

        // Test for non-existent market index
        let result = user.ffi().get_perp_position(5);
        assert!(result.is_err());
    }

    #[test]
    fn ffi_perp_position_is_available() {
        let position = DriftPerpPosition::default();
        assert!(position.ffi().is_available());

        let position = DriftPerpPosition {
            base_asset_amount: 100,
            ..Default::default()
        };
        assert!(!position.ffi().is_available());
    }

    #[test]
    fn ffi_perp_position_is_open_position() {
        let position = DriftPerpPosition::default();
        assert!(!position.ffi().is_open_position());

        let position = DriftPerpPosition {
            base_asset_amount: 100,
            ..Default::default()
        };
        assert!(position.ffi().is_open_position());
    }

    #[test]
    fn ffi_perp_position_worst_case_base_asset_amount() {
        let position = DriftPerpPosition {
            base_asset_amount: 1_000 * BASE_PRECISION_I64,
            quote_asset_amount: 5_000 * QUOTE_PRECISION_I64,
            market_index: 1,
            ..Default::default()
        };
        let oracle_price = 10 * QUOTE_PRECISION_I64;
        let contract_type = ContractType::Perpetual;

        let result = position
            .ffi()
            .worst_case_base_asset_amount(oracle_price, contract_type);
        assert!(result.is_ok());
        let worst_case_amount = result.unwrap();
        assert!(worst_case_amount >= 1000); // The worst case should be at least the current base asset amount
    }

    #[test]
    fn ffi_perp_position_simulate_settled_lp_position() {
        let position = DriftPerpPosition {
            base_asset_amount: 1_000 * BASE_PRECISION_I64,
            quote_asset_amount: 5_000 * QUOTE_PRECISION_I64,
            last_cumulative_funding_rate: 100.into(),
            ..Default::default()
        };
        let market = DriftPerpMarket {
            amm: crate::drift_idl::types::AMM {
                cumulative_funding_rate_long: 120.into(),
                cumulative_funding_rate_short: 80.into(),
                ..Default::default()
            },
            ..Default::default()
        };
        let oracle_price = 10 * QUOTE_PRECISION_I64;

        let result = position
            .ffi()
            .simulate_settled_lp_position(&market, oracle_price);
        assert!(result.is_ok());
        let simulated_position = result.unwrap();
        assert!(simulated_position.quote_asset_amount > 1_000);
    }

    #[test]
    fn ffi_get_oracle_price() {
        let oracle_pubkey = Pubkey::new_unique();
        let oracle_account = Account {
            // encoded from pyth Price, see liquidation tests
            data: SOL_PYTH_PRICE.clone(),
            owner: constants::ids::pyth_program::ID,
            ..Default::default()
        };

        let oracle_source = OracleSource::Pyth;
        let slot = 12_345;

        let result = get_oracle_price(&oracle_source, &mut (oracle_pubkey, oracle_account), slot);

        // Assert the result
        assert!(result.is_ok());
        let oracle_price_data = result.unwrap();

        dbg!(oracle_price_data.price);
        assert!(oracle_price_data.price == 60 * QUOTE_PRECISION as i64);
    }

    #[test]
    fn ffi_order_is_limit_order() {
        // Test a limit order
        for (order_type, is_limit) in [
            (OrderType::Limit, true),
            (OrderType::Market, false),
            (OrderType::TriggerLimit, true),
            (OrderType::TriggerMarket, false),
        ]
        .into_iter()
        {
            let limit_order = DriftOrder {
                order_type,
                slot: 100,
                ..Default::default()
            };
            let ffi_limit_order = limit_order.ffi();
            assert_eq!(ffi_limit_order.is_limit_order(), is_limit);
            assert_eq!(
                ffi_limit_order.is_resting_limit_order(100).unwrap(),
                is_limit
            );
        }
    }

    #[test]
    fn ffi_perp_market_get_margin_ratio() {
        let perp_market = DriftPerpMarket {
            margin_ratio_initial: 1_000 * MARGIN_PRECISION, // 10%
            margin_ratio_maintenance: 500,                  // 5%
            imf_factor: 0,                                  // No impact for simplicity
            ..Default::default()
        };

        let size = 1_000 * MARGIN_PRECISION as u128; // Assuming MARGIN_PRECISION is defined

        // Test initial margin ratio
        let result = perp_market
            .ffi()
            .get_margin_ratio(size, MarginRequirementType::Initial);
        assert!(result.is_ok());
        let initial_margin_ratio = result.unwrap();
        assert_eq!(initial_margin_ratio, 1_000 * MARGIN_PRECISION); // 10%

        // Test maintenance margin ratio
        let result = perp_market
            .ffi()
            .get_margin_ratio(size, MarginRequirementType::Maintenance);
        assert!(result.is_ok());
        let maintenance_margin_ratio = result.unwrap();
        assert_eq!(maintenance_margin_ratio, 500); // 5%
    }

    #[test]
    fn ffi_test_calculate_margin_requirement_and_total_collateral_and_liability_info() {
        // smoke test for ffi compatability, logic tested in `math::` module
        let btc_perp_index = 1_u16;
        let mut user = DriftUser::default();
        user.spot_positions[1] = DriftSpotPosition {
            market_index: 1,
            scaled_balance: (1_000 * SPOT_BALANCE_PRECISION) as u64,
            balance_type: SpotBalanceType::Deposit,
            ..Default::default()
        };
        user.perp_positions[0] = DriftPerpPosition {
            market_index: btc_perp_index,
            base_asset_amount: 100 * BASE_PRECISION_I64 as i64,
            quote_asset_amount: -5_000 * QUOTE_PRECISION as i64,
            ..Default::default()
        };

        // Create mock accounts
        let mut perp_markets = vec![AccountWithKey {
            key: Pubkey::new_unique(),
            account: Account {
                owner: crate::constants::PROGRAM_ID,
                data: [
                    DriftPerpMarket::DISCRIMINATOR.as_slice(),
                    bytemuck::bytes_of(&DriftPerpMarket {
                        market_index: btc_perp_index,
                        ..Default::default()
                    }),
                ]
                .concat()
                .to_vec(),
                ..Default::default()
            },
        }];
        let spot_market = sol_spot_market();
        let mut spot_markets = vec![AccountWithKey {
            key: Pubkey::new_unique(),
            account: Account {
                owner: crate::constants::PROGRAM_ID,
                data: [
                    DriftSpotMarket::DISCRIMINATOR.as_slice(),
                    bytemuck::bytes_of(&spot_market),
                ]
                .concat()
                .to_vec(),
                ..Default::default()
            },
        }];

        let mut oracles = [AccountWithKey {
            key: Pubkey::new_unique(),
            account: Account {
                // encoded from pyth Price, see liquidation tests
                data: SOL_PYTH_PRICE.clone(),
                owner: constants::ids::pyth_program::ID,
                ..Default::default()
            },
        }];
        let mut accounts = AccountsList::new(&mut perp_markets, &mut spot_markets, &mut oracles);

        let modes = [
            MarginContextMode::StandardMaintenance,
            MarginContextMode::StandardInitial,
            MarginContextMode::StandardCustom(MarginRequirementType::Initial),
        ];

        // no panics is enough
        for mode in modes.iter() {
            let _ = calculate_margin_requirement_and_total_collateral_and_liability_info(
                &user,
                &mut accounts,
                *mode,
            );
        }
    }

    #[test]
    fn layouts() {
        dbg!(MarginCalculation::type_layout());
    }
}
