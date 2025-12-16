//!
//! FFI shims
//! Defines wrapper types for ergonomic access to drift-program logic
//!
use std::time::{SystemTime, UNIX_EPOCH};

use abi_stable::std_types::ROption;
use anchor_lang::{prelude::AccountInfo, Discriminator};
use solana_sdk::{account::Account, clock::Slot, pubkey::Pubkey};

pub use self::abi_types::*;
use crate::{
    constants::{high_leverage_mode_account, PROGRAM_ID},
    drift_idl::{
        accounts,
        errors::ErrorCode,
        types::{self, ContractType, MarginRequirementType, OracleSource},
    },
    market_state::MarketState,
    math::{
        constants::{BID_ASK_SPREAD_PRECISION_I64, PERCENTAGE_PRECISION_I128, QUOTE_PRECISION_I64},
        standardize_price_i64,
    },
    types::{
        accounts::HighLeverageModeConfig, ContractTier, OrderParams, OrderType, PositionDirection,
        ProtectedMakerParams, RevenueShareOrder, SdkError, ValidityGuardRails,
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
    pub fn ffi_version() -> String;
    #[allow(improper_ctypes)]
    pub fn math_calculate_auction_price(
        order: &types::Order,
        slot: Slot,
        tick_size: u64,
        oracle_price: ROption<i64>,
        is_prediction_market: bool,
    ) -> FfiResult<u64>;
    #[allow(improper_ctypes)]
    pub fn math_calculate_margin_requirement_and_total_collateral_and_liability_info(
        user: &accounts::User,
        accounts: &mut AccountsList,
        mode: MarginContextMode,
    ) -> FfiResult<MarginCalculation>;

    #[allow(improper_ctypes)]
    pub fn oracle_get_oracle_price(
        oracle_source: OracleSource,
        oracle_account: &mut (Pubkey, Account),
        slot: Slot,
    ) -> FfiResult<OraclePriceData>;

    #[allow(improper_ctypes)]
    pub fn order_is_limit_order(order: &types::Order) -> bool;
    #[allow(improper_ctypes)]
    pub fn order_is_resting_limit_order(order: &types::Order, slot: Slot) -> FfiResult<bool>;
    #[allow(improper_ctypes)]
    pub fn order_triggered(order: &types::Order) -> bool;
    #[allow(improper_ctypes)]
    pub fn order_get_limit_price(
        order: &types::Order,
        valid_oracle_price: Option<i64>,
        fallback_price: Option<u64>,
        slot: u64,
        tick_size: u64,
        is_prediction_market: bool,
        pmm_params: Option<ProtectedMakerParams>,
    ) -> FfiResult<Option<u64>>;

    #[allow(improper_ctypes)]
    pub fn perp_market_get_margin_ratio(
        market: &accounts::PerpMarket,
        size: u128,
        margin_type: MarginRequirementType,
        high_leverage_mode: bool,
    ) -> FfiResult<u32>;
    #[allow(improper_ctypes)]
    pub fn perp_market_get_open_interest(market: &accounts::PerpMarket) -> u128;
    #[allow(improper_ctypes)]
    pub fn perp_market_get_protected_maker_params(
        market: &accounts::PerpMarket,
    ) -> ProtectedMakerParams;
    #[allow(improper_ctypes)]
    pub fn perp_market_get_trigger_price(
        market: &accounts::PerpMarket,
        oracle_price: i64,
        now: i64,
        use_median_trigger_price: bool,
    ) -> FfiResult<u64>;
    #[allow(improper_ctypes)]
    pub fn perp_market_get_mm_oracle_price_data(
        market: &accounts::PerpMarket,
        oracle_price_data: OraclePriceData,
        clock_slot: Slot,
        oracle_guard_rails: &ValidityGuardRails,
    ) -> FfiResult<MMOraclePriceData>;
    #[allow(improper_ctypes)]
    pub fn perp_market_get_fallback_price(
        market: &accounts::PerpMarket,
        taker_direction: PositionDirection,
        oracle_price: i64,
        seconds_til_expiry: i64,
    ) -> FfiResult<u64>;
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
    #[allow(improper_ctypes)]
    pub fn user_update_perp_position_max_margin_ratio(
        user: &mut accounts::User,
        market_index: u16,
        margin_ratio: u16,
    ) -> FfiResult<()>;
    #[allow(improper_ctypes)]
    pub fn orders_place_perp_order<'a>(
        user: &accounts::User,
        state: &accounts::State,
        order_params: &types::OrderParams,
        accounts: &mut AccountsList,
        high_leverage_mode_config: Option<&'a AccountInfo<'a>>,
        revenue_order_share: &mut Option<&mut RevenueShareOrder>,
    ) -> FfiResult<bool>;
    #[allow(improper_ctypes)]
    pub fn order_params_will_auction_params_sanitize(
        order_params: &types::OrderParams,
        perp_market: &accounts::PerpMarket,
        oracle_price: i64,
        is_signed_msg: bool,
    ) -> FfiResult<bool>;
    #[allow(improper_ctypes)]
    pub fn order_params_update_perp_auction_params(
        order_params: &mut types::OrderParams,
        perp_market: &accounts::PerpMarket,
        oracle_price: i64,
        is_signed_msg: bool,
    );
    #[allow(improper_ctypes)]
    pub fn order_calculate_auction_params_for_trigger_order(
        order: &types::Order,
        oracle_price: &OraclePriceData,
        perp_market: Option<&accounts::PerpMarket>,
    ) -> FfiResult<(u8, i64, i64)>;
}

//
// Shims for SDK
//

/// Returns the linked libdrift_ffi version
pub fn check_ffi_version() -> String {
    unsafe { ffi_version() }
}

pub fn get_oracle_price(
    oracle_source: OracleSource,
    oracle_account: &mut (Pubkey, Account),
    slot: Slot,
) -> SdkResult<OraclePriceData> {
    if oracle_account.1.data.is_empty() {
        return Err(SdkError::NoAccountData(oracle_account.0));
    }
    to_sdk_result(unsafe { oracle_get_oracle_price(oracle_source, oracle_account, slot) })
}

pub fn calculate_auction_price(
    order: &types::Order,
    slot: Slot,
    tick_size: u64,
    oracle_price: Option<i64>,
    is_prediction_market: bool,
) -> SdkResult<u64> {
    let res = unsafe {
        math_calculate_auction_price(
            order,
            slot,
            tick_size,
            oracle_price.into(),
            is_prediction_market,
        )
    };
    to_sdk_result(res)
}

impl OrderParams {
    pub fn update_perp_auction_params(
        &mut self,
        perp_market: &accounts::PerpMarket,
        oracle_price: i64,
        is_signed_msg: bool,
    ) {
        unsafe {
            order_params_update_perp_auction_params(self, perp_market, oracle_price, is_signed_msg)
        }
    }
}

impl MarketState {
    /// Calculate margin requirement for user
    ///
    /// this is a more lightweight/optimized version of `calculate_margin_requirement_and_total_collateral_and_liability_info`
    pub fn calculate_simplified_margin_requirement(
        &self,
        user: &accounts::User,
        margin_type: MarginRequirementType,
        margin_buffer: Option<u32>,
    ) -> crate::SdkResult<SimplifiedMarginCalculation> {
        let state = self.load();
        let result = unsafe {
            margin_calculate_simplified_margin_requirement(
                user,
                &state,
                margin_type,
                margin_buffer.unwrap_or(0),
            )
        };

        to_sdk_result(result)
    }
    /// Calculate margin requirement for user
    ///
    /// incremental version allows partial updates e.g. when specific positions or oracle prices change
    pub fn calculate_incremental_margin_requirement(
        &self,
        user: &accounts::User,
        margin_type: MarginRequirementType,
        margin_buffer: Option<u32>,
    ) -> IncrementalMarginCalculation {
        let state = self.load();
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        unsafe {
            incremental_margin_calculation_from_user(
                user,
                &state,
                margin_type,
                ts,
                margin_buffer.unwrap_or(0),
            )
        }
    }
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

/// Simulates the program's `place_perp_order` ix
/// Useful to verify an order can be placed given factors such as available margin, etc.
///
/// Returns `true` if the order could be placed
pub fn simulate_place_perp_order<'a>(
    user: &mut accounts::User,
    accounts: &mut AccountsList,
    state: &accounts::State,
    order_params: &types::OrderParams,
    high_leverage_mode_config: Option<&mut accounts::HighLeverageModeConfig>,
    max_margin_ratio: Option<u16>,
    revenue_share_order: &mut Option<&mut RevenueShareOrder>,
) -> SdkResult<bool> {
    if order_params.high_leverage_mode() && high_leverage_mode_config.is_none() {
        return Err(SdkError::Generic(
            "HLM config account must be provided".to_owned(),
        ));
    }

    if let Some(max_margin_ratio) = max_margin_ratio {
        user.update_perp_position_max_margin_ratio(order_params.market_index, max_margin_ratio)?;
    }

    let mut lamports = 0;
    let res = match high_leverage_mode_config {
        Some(hlm) => {
            let mut data = HighLeverageModeConfig::DISCRIMINATOR.to_vec();
            data.extend_from_slice(bytemuck::bytes_of(hlm));

            let hlm = AccountInfo::new(
                high_leverage_mode_account(),
                false,
                true,
                &mut lamports,
                data.as_mut_slice(),
                &PROGRAM_ID,
                false,
                u64::MAX,
            );
            unsafe {
                orders_place_perp_order(
                    user,
                    state,
                    order_params,
                    accounts,
                    Some(&hlm),
                    revenue_share_order,
                )
            }
        }
        None => unsafe {
            orders_place_perp_order(
                user,
                state,
                order_params,
                accounts,
                None,
                revenue_share_order,
            )
        },
    };
    to_sdk_result(res)
}

/// Simulates using the program's update_perp_auction_params func to determine if
/// an order's auction params will get sanitized
///
/// Returns `true` if the order's auctions will get sanitized
pub fn simulate_will_auction_params_sanitize(
    order_params: &types::OrderParams,
    perp_market: &accounts::PerpMarket,
    oracle_price: i64,
    is_signed_msg: bool,
) -> SdkResult<bool> {
    let res = unsafe {
        order_params_will_auction_params_sanitize(
            order_params,
            perp_market,
            oracle_price,
            is_signed_msg,
        )
    };
    to_sdk_result(res)
}

impl types::OrderParams {
    pub fn get_auction_params(
        &self,
        oracle_price_data: &OraclePriceData,
        tick_size: u64,
        min_auction_duration: u8,
    ) -> Option<(i64, i64, u8)> {
        if !matches!(
            self.order_type,
            OrderType::Market | OrderType::Oracle | OrderType::Limit
        ) {
            return Some((0_i64, 0_i64, 0_u8));
        }

        if self.order_type == OrderType::Limit {
            return match (
                self.auction_start_price,
                self.auction_end_price,
                self.auction_duration,
            ) {
                (Some(auction_start_price), Some(auction_end_price), Some(auction_duration)) => {
                    let auction_duration = if auction_duration == 0 {
                        auction_duration
                    } else {
                        // if auction is non-zero, force it to be at least min_auction_duration
                        auction_duration.max(min_auction_duration)
                    };

                    Some((
                        standardize_price_i64(auction_start_price, tick_size, self.direction),
                        standardize_price_i64(auction_end_price, tick_size, self.direction),
                        auction_duration,
                    ))
                }
                _ => Some((0_i64, 0_i64, 0_u8)),
            };
        }

        let auction_duration = self.auction_duration.unwrap_or(0).max(min_auction_duration);

        let (auction_start_price, auction_end_price) =
            match (self.auction_start_price, self.auction_end_price) {
                (Some(auction_start_price), Some(auction_end_price)) => {
                    (auction_start_price, auction_end_price)
                }
                _ if self.order_type == OrderType::Oracle => return None,
                _ => calculate_auction_prices(oracle_price_data, self.direction, self.price)?,
            };

        Some((
            standardize_price_i64(auction_start_price, tick_size, self.direction),
            standardize_price_i64(auction_end_price, tick_size, self.direction),
            auction_duration,
        ))
    }
}
pub const AUCTION_DERIVE_PRICE_FRACTION: i64 = 200;
pub fn calculate_auction_prices(
    oracle_price_data: &OraclePriceData,
    direction: PositionDirection,
    limit_price: u64,
) -> Option<(i64, i64)> {
    let oracle_price = oracle_price_data.price;
    let limit_price = limit_price as i64;
    if limit_price > 0 {
        let (auction_start_price, auction_end_price) = match direction {
            // Long and limit price is better than oracle price
            PositionDirection::Long if limit_price < oracle_price => {
                let limit_derive_start_price =
                    limit_price.checked_sub(limit_price / AUCTION_DERIVE_PRICE_FRACTION)?;
                let oracle_derive_start_price =
                    oracle_price.checked_sub(oracle_price / AUCTION_DERIVE_PRICE_FRACTION)?;

                (
                    limit_derive_start_price.min(oracle_derive_start_price),
                    limit_price,
                )
            }
            // Long and limit price is worse than oracle price
            PositionDirection::Long if limit_price >= oracle_price => {
                let oracle_derive_end_price =
                    oracle_price.checked_add(oracle_price / AUCTION_DERIVE_PRICE_FRACTION)?;

                (oracle_price, limit_price.min(oracle_derive_end_price))
            }
            // Short and limit price is better than oracle price
            PositionDirection::Short if limit_price > oracle_price => {
                let limit_derive_start_price =
                    limit_price.checked_add(limit_price / AUCTION_DERIVE_PRICE_FRACTION)?;
                let oracle_derive_start_price =
                    oracle_price.checked_add(oracle_price / AUCTION_DERIVE_PRICE_FRACTION)?;

                (
                    limit_derive_start_price.max(oracle_derive_start_price),
                    limit_price,
                )
            }
            // Short and limit price is worse than oracle price
            PositionDirection::Short if limit_price <= oracle_price => {
                let oracle_derive_end_price =
                    oracle_price.checked_sub(oracle_price / AUCTION_DERIVE_PRICE_FRACTION)?;

                (oracle_price, limit_price.max(oracle_derive_end_price))
            }
            _ => unreachable!(),
        };

        return Some((auction_start_price, auction_end_price));
    }

    let auction_end_price = match direction {
        PositionDirection::Long => {
            oracle_price.checked_add(oracle_price / AUCTION_DERIVE_PRICE_FRACTION)?
        }
        PositionDirection::Short => {
            oracle_price.checked_sub(oracle_price / AUCTION_DERIVE_PRICE_FRACTION)?
        }
    };

    Some((oracle_price, auction_end_price))
}

impl types::SpotPosition {
    pub fn is_available(&self) -> bool {
        unsafe { spot_position_is_available(self) }
    }
    pub fn get_signed_token_amount(&self, market: &accounts::SpotMarket) -> SdkResult<i128> {
        to_sdk_result(unsafe { spot_position_get_signed_token_amount(self, market) })
    }
    pub fn get_token_amount(&self, market: &accounts::SpotMarket) -> SdkResult<u128> {
        to_sdk_result(unsafe { spot_position_get_token_amount(self, market) })
    }
}

impl types::PerpPosition {
    pub fn get_unrealized_pnl(&self, oracle_price: i64) -> SdkResult<i128> {
        to_sdk_result(unsafe { perp_position_get_unrealized_pnl(self, oracle_price) })
    }
    pub fn is_available(&self) -> bool {
        unsafe { perp_position_is_available(self) }
    }
    pub fn is_open_position(&self) -> bool {
        unsafe { perp_position_is_open_position(self) }
    }
    pub fn worst_case_base_asset_amount(
        &self,
        oracle_price: i64,
        contract_type: ContractType,
    ) -> SdkResult<i128> {
        to_sdk_result(unsafe {
            perp_position_worst_case_base_asset_amount(self, oracle_price, contract_type)
        })
    }
}

impl accounts::User {
    pub const STATUS_BEING_LIQUIDATED: u8 = 0b00000001;
    pub const STATUS_BANKRUPT: u8 = 0b00000010;
    pub const STATUS_REDUCE_ONLY: u8 = 0b00000100;
    pub const STATUS_ADVANCED_LP: u8 = 0b00001000;
    pub const STATUS_PROTECTED_MAKER_ORDERS: u8 = 0b00010000;

    pub fn get_spot_position(&self, market_index: u16) -> SdkResult<types::SpotPosition> {
        // TODO: no clone
        to_sdk_result(unsafe { user_get_spot_position(self, market_index) }).copied()
    }
    pub fn get_perp_position(&self, market_index: u16) -> SdkResult<types::PerpPosition> {
        to_sdk_result(unsafe { user_get_perp_position(self, market_index) }).copied()
    }
    pub fn is_being_liquidated(&self) -> bool {
        self.status & (Self::STATUS_BEING_LIQUIDATED | Self::STATUS_BANKRUPT) > 0
    }
    pub fn is_bankrupt(&self) -> bool {
        (self.status & Self::STATUS_BANKRUPT) > 0
    }
    pub fn is_reduce_only(&self) -> bool {
        (self.status & Self::STATUS_REDUCE_ONLY) > 0
    }
    pub fn is_advanced_lp(&self) -> bool {
        (self.status & Self::STATUS_ADVANCED_LP) > 0
    }
    pub fn is_protected_maker(&self) -> bool {
        (self.status & Self::STATUS_PROTECTED_MAKER_ORDERS) > 0
    }

    pub fn update_perp_position_max_margin_ratio(
        &mut self,
        market_index: u16,
        max_margin_ratio: u16,
    ) -> SdkResult<()> {
        to_sdk_result(unsafe {
            user_update_perp_position_max_margin_ratio(self, market_index, max_margin_ratio)
        })
    }
}

impl types::Order {
    pub fn is_limit_order(&self) -> bool {
        unsafe { order_is_limit_order(self) }
    }
    pub fn is_resting_limit_order(&self, slot: Slot) -> SdkResult<bool> {
        to_sdk_result(unsafe { order_is_resting_limit_order(self, slot) })
    }
    pub fn triggered(&self) -> bool {
        unsafe { order_triggered(self) }
    }
    pub fn get_limit_price(
        &self,
        valid_oracle_price: Option<i64>,
        fallback_price: Option<u64>,
        slot: u64,
        tick_size: u64,
        is_prediction_market: bool,
        pmm_params: Option<ProtectedMakerParams>,
    ) -> SdkResult<Option<u64>> {
        to_sdk_result(unsafe {
            order_get_limit_price(
                self,
                valid_oracle_price,
                fallback_price,
                slot,
                tick_size,
                is_prediction_market,
                pmm_params,
            )
        })
    }
}

impl accounts::SpotMarket {
    pub fn get_asset_weight(
        &self,
        size: u128,
        oracle_price: i64,
        margin_requirement_type: MarginRequirementType,
    ) -> SdkResult<u32> {
        to_sdk_result(unsafe {
            spot_market_get_asset_weight(self, size, oracle_price, margin_requirement_type)
        })
    }
    pub fn get_liability_weight(
        &self,
        size: u128,
        margin_requirement_type: MarginRequirementType,
    ) -> SdkResult<u32> {
        to_sdk_result(unsafe {
            spot_market_get_liability_weight(self, size, margin_requirement_type)
        })
    }
}

impl accounts::PerpMarket {
    /// Return VAMM fallback price
    pub fn fallback_price(
        &self,
        taker_direction: PositionDirection,
        oracle_price: i64,
        seconds_til_expiry: i64,
    ) -> SdkResult<u64> {
        to_sdk_result(unsafe {
            perp_market_get_fallback_price(self, taker_direction, oracle_price, seconds_til_expiry)
        })
    }
    pub fn get_mm_oracle_price_data(
        &self,
        oracle_price_data: OraclePriceData,
        clock_slot: Slot,
        validity_guard_rails: &ValidityGuardRails,
    ) -> SdkResult<MMOraclePriceData> {
        to_sdk_result(unsafe {
            perp_market_get_mm_oracle_price_data(
                self,
                oracle_price_data,
                clock_slot,
                validity_guard_rails,
            )
        })
    }
    pub fn get_trigger_price(
        &self,
        oracle_price: i64,
        now: i64,
        use_median_trigger_price: bool,
    ) -> SdkResult<u64> {
        to_sdk_result(unsafe {
            perp_market_get_trigger_price(self, oracle_price, now, use_median_trigger_price)
        })
    }
    pub fn get_margin_ratio(
        &self,
        size: u128,
        margin_requirement_type: MarginRequirementType,
        high_leverage_mode: bool,
    ) -> SdkResult<u32> {
        to_sdk_result(unsafe {
            perp_market_get_margin_ratio(self, size, margin_requirement_type, high_leverage_mode)
        })
    }
    pub fn get_open_interest(&self) -> u128 {
        unsafe { perp_market_get_open_interest(self) }
    }
    pub fn get_protected_maker_params(&self) -> ProtectedMakerParams {
        unsafe { perp_market_get_protected_maker_params(self) }
    }
    pub fn has_too_much_drawdown(&self) -> bool {
        pub const DEFAULT_REVENUE_SINCE_LAST_FUNDING_SPREAD_RETREAT: i64 =
            -25 * QUOTE_PRECISION_I64; //$25 loss
        let quote_drawdown_limit_breached = match self.contract_tier {
            ContractTier::A | ContractTier::B => {
                self.amm.net_revenue_since_last_funding
                    <= DEFAULT_REVENUE_SINCE_LAST_FUNDING_SPREAD_RETREAT * 400
            }
            _ => {
                self.amm.net_revenue_since_last_funding
                    <= DEFAULT_REVENUE_SINCE_LAST_FUNDING_SPREAD_RETREAT * 200
            }
        };

        if quote_drawdown_limit_breached {
            let net_revenue_since_last_funding = self.amm.net_revenue_since_last_funding as i128;
            let percent_drawdown = (net_revenue_since_last_funding * PERCENTAGE_PRECISION_I128)
                / (self.amm.total_fee_minus_distributions.as_i128().max(1_i128));

            let percent_drawdown_limit_breached = match self.contract_tier {
                ContractTier::A => percent_drawdown <= -PERCENTAGE_PRECISION_I128 / 50,
                ContractTier::B => percent_drawdown <= -PERCENTAGE_PRECISION_I128 / 33,
                ContractTier::C => percent_drawdown <= -PERCENTAGE_PRECISION_I128 / 25,
                _ => percent_drawdown <= -PERCENTAGE_PRECISION_I128 / 20,
            };

            if percent_drawdown_limit_breached {
                return true;
            }
        }

        false
    }
    /// Return AMM's reserve price
    pub fn reserve_price(&self) -> u64 {
        // (quote_asset_reserve / base_asset_reserve) * peg / PEG_PRECISION
        if self.amm.base_asset_reserve.as_u128() == 0 {
            return 0;
        }
        let peg_quote_asset_amount =
            self.amm.quote_asset_reserve.as_u128() * self.amm.peg_multiplier.as_u128();
        peg_quote_asset_amount.saturating_div(self.amm.base_asset_reserve.as_u128()) as u64
    }

    /// Return AMM's bid price
    ///
    /// ## Params
    ///
    /// * `reserve_price` - optional reserve price, default: AMM current reserve price
    ///
    pub fn bid_price(&self, reserve_price: Option<u64>) -> u64 {
        let adjusted_spread = (-(self.amm.short_spread as i32)) + self.amm.reference_price_offset;
        let multiplier = BID_ASK_SPREAD_PRECISION_I64 + adjusted_spread as i64;
        let reserve_price = reserve_price.unwrap_or(self.reserve_price());

        (reserve_price * multiplier.unsigned_abs()) / BID_ASK_SPREAD_PRECISION_I64 as u64
    }

    /// Return AMM's ask price
    ///
    /// ## Params
    ///
    /// * `reserve_price` - optional reserve price, default: AMM current reserve price
    ///
    pub fn ask_price(&self, reserve_price: Option<u64>) -> u64 {
        let adjusted_spread = self.amm.long_spread as i32 + self.amm.reference_price_offset;
        let multiplier = BID_ASK_SPREAD_PRECISION_I64 + adjusted_spread as i64;
        let reserve_price = reserve_price.unwrap_or(self.reserve_price());

        (reserve_price * multiplier.unsigned_abs()) / BID_ASK_SPREAD_PRECISION_I64 as u64
    }
}

impl types::MarginMode {
    /// Returns true if the margin mode is high leverage mode or high leverage maintenance mode
    pub fn is_high_leverage_mode(&self, margin_type: MarginRequirementType) -> bool {
        matches!(
            (self, margin_type),
            (types::MarginMode::HighLeverage, _)
                | (
                    types::MarginMode::HighLeverageMaintenance,
                    MarginRequirementType::Maintenance
                )
        )
    }
}

/// Calculates auction params for a trigger order using the FFI, returning (duration, start_price, end_price)
pub fn calculate_auction_params_for_trigger_order(
    order: &types::Order,
    oracle_price: &OraclePriceData,
    perp_market: Option<&accounts::PerpMarket>,
) -> SdkResult<(u8, i64, i64)> {
    to_sdk_result(unsafe {
        order_calculate_auction_params_for_trigger_order(order, oracle_price, perp_market)
    })
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

impl IncrementalMarginCalculation {
    /// Return free collateral amount
    pub fn free_collateral(&self) -> i128 {
        self.total_collateral - self.margin_requirement as i128
    }
    /// Create a new cached margin calculation from a user account
    pub fn from_user(
        user: &accounts::User,
        market_state: &MarketState,
        margin_type: MarginRequirementType,
        timestamp: u64,
        margin_buffer: Option<u32>,
    ) -> Self {
        let m = market_state.load();
        unsafe {
            incremental_margin_calculation_from_user(
                user,
                &m,
                margin_type,
                timestamp,
                margin_buffer.unwrap_or(0),
            )
        }
    }

    /// Create a new cached margin calculation from a user account with current timestamp
    pub fn from_user_now(
        user: &accounts::User,
        market_state: &MarketState,
        margin_type: MarginRequirementType,
        margin_buffer: Option<u32>,
    ) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Self::from_user(user, market_state, margin_type, timestamp, margin_buffer)
    }

    /// Update the cached calculation with a spot position change
    pub fn update_spot_position(
        &mut self,
        spot_position: &types::SpotPosition,
        market_state: &MarketState,
        timestamp: u64,
    ) {
        let m = market_state.load();
        unsafe {
            incremental_margin_calculation_update_spot_position(self, spot_position, &m, timestamp);
        }
    }

    /// Update the cached calculation with a perp position change
    pub fn update_perp_position(
        &mut self,
        perp_position: &types::PerpPosition,
        market_state: &MarketState,
        timestamp: u64,
    ) {
        let m = market_state.load();
        unsafe {
            incremental_margin_calculation_update_perp_position(self, perp_position, &m, timestamp);
        }
    }
}

pub mod abi_types {
    //! cross-boundary FFI types
    use abi_stable::std_types::RResult;
    use solana_sdk::{account::Account, clock::Slot, pubkey::Pubkey};

    use crate::{drift_idl::types::MarginRequirementType, types::OracleValidity, OracleGuardRails};

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
    /// Its used as input for drift program math functions
    #[repr(C)]
    pub struct AccountsList<'a> {
        // accounts
        pub perp_markets: &'a mut [AccountWithKey],
        pub spot_markets: &'a mut [AccountWithKey],
        pub oracles: &'a mut [AccountWithKey],
        // context
        pub oracle_guard_rails: Option<OracleGuardRails>,
        pub latest_slot: Slot,
    }

    #[cfg(test)]
    impl<'a> AccountsList<'a> {
        pub fn new(
            perp_markets: &'a mut [AccountWithKey],
            spot_markets: &'a mut [AccountWithKey],
            oracles: &'a mut [AccountWithKey],
        ) -> Self {
            Self {
                perp_markets,
                spot_markets,
                oracles,
                oracle_guard_rails: None,
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

    #[repr(C, align(16))]
    #[derive(Copy, Clone, Debug, PartialEq)]
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

    /// FFI equivalent of `OraclePriceData`
    #[derive(Default, Clone, Copy, Debug)]
    pub struct OraclePriceData {
        pub price: i64,
        pub confidence: u64,
        pub delay: i64,
        pub has_sufficient_number_of_data_points: bool,
        pub sequence_id: Option<u64>,
    }

    /// MMOraclePriceData, not defined in IDL
    #[derive(Default, Clone, Copy, Debug)]
    pub struct MMOraclePriceData {
        pub mm_oracle_price: i64,
        pub mm_oracle_delay: i64,
        pub mm_oracle_validity: OracleValidity,
        pub mm_exchange_diff_bps: u128,
        pub exchange_oracle_price_data: OraclePriceData,
        pub safe_oracle_price_data: OraclePriceData,
    }

    /// C-ABI compatible result type for drift FFI calls
    pub type FfiResult<T> = RResult<T, u32>;

    /// FFI-compatible simplified margin calculation result
    #[repr(C, align(16))]
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct SimplifiedMarginCalculation {
        pub total_collateral: i128,
        pub total_collateral_buffer: i128,
        pub margin_requirement: u128,
        pub margin_requirement_plus_buffer: u128,
    }

    /// FFI-compatible incremental margin calculation
    ///
    /// This struct must match the FFI-side struct exactly for proper alignment
    #[repr(C, align(16))]
    #[derive(Clone)]
    pub struct IncrementalMarginCalculation {
        pub total_collateral: i128,
        pub total_collateral_buffer: i128,
        pub margin_requirement: u128,
        pub margin_requirement_plus_buffer: u128,
        pub spot_collateral: [PositionCollateral; 8],
        pub perp_collateral: [PositionCollateral; 8],
        pub last_updated: u64,
        pub user_custom_margin_ratio: u32,
        pub margin_buffer: u32,
        pub margin_type: MarginRequirementType,
        pub user_high_leverage_mode: bool,
        pub user_pool_id: u8,
    }

    impl std::fmt::Debug for IncrementalMarginCalculation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let total_collateral_formatted =
                format!("{:.2}", self.total_collateral as f64 / 1_000_000.0);
            let margin_requirement_formatted =
                format!("{:.2}", self.margin_requirement as f64 / 1_000_000.0);

            let total_collateral_buffer_formatted =
                format!("{:.2}", self.total_collateral_buffer as f64 / 1_000_000.0);
            let margin_requirement_buffer_formatted = format!(
                "{:.2}",
                self.margin_requirement_plus_buffer as f64 / 1_000_000.0
            );

            f.debug_struct("CachedMarginCalculation")
                .field("total_collateral", &total_collateral_formatted)
                .field(
                    "total_collateral_buffer",
                    &total_collateral_buffer_formatted,
                )
                .field("margin_requirement", &margin_requirement_formatted)
                .field(
                    "margin_requirement_buffer",
                    &margin_requirement_buffer_formatted,
                )
                .field("margin_type", &self.margin_type)
                .field("user_high_leverage_mode", &self.user_high_leverage_mode)
                .field("user_custom_margin_ratio", &self.user_custom_margin_ratio)
                .field("last_updated", &self.last_updated)
                .finish()
        }
    }

    /// FFI-compatible position collateral contribution
    /// This struct must match the FFI-side struct exactly for proper alignment
    #[repr(C, align(16))]
    #[derive(Clone, Copy, Default)]
    pub struct PositionCollateral {
        pub collateral_value: i128,
        pub collateral_buffer: i128,
        pub liability_value: u128,
        pub liability_buffer: u128,
        pub last_updated: u64,
        pub market_index: u16,
    }

    impl std::fmt::Debug for PositionCollateral {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut fields = vec![format!("market_index: {}", self.market_index)];

            if self.collateral_value > 0 {
                let asset_formatted = format!("{:.2}", self.collateral_value as f64 / 1_000_000.0);
                fields.push(format!("+{}", asset_formatted));
            }

            if self.liability_value > 0 {
                let liability_formatted =
                    format!("{:.2}", self.liability_value as f64 / 1_000_000.0);
                fields.push(format!("-{}", liability_formatted));
            }

            if self.last_updated > 0 {
                fields.push(format!("last_updated: {}", self.last_updated));
            }

            write!(f, "PositionCollateral {{ {} }}", fields.join(", "))
        }
    }
}

#[cfg(test)]
mod tests {
    use anchor_lang::Discriminator;
    use solana_sdk::{account::Account, pubkey::Pubkey};

    use super::{
        margin_calculate_simplified_margin_requirement, simulate_place_perp_order, AccountWithKey,
        AccountsList, FfiResult, MarginContextMode,
    };
    use crate::{
        accounts::State,
        constants::{self, ids::pyth_program},
        create_account_info,
        drift_idl::{
            accounts::{PerpMarket, SpotMarket, User},
            types::{
                ContractType, MarginRequirementType, OracleSource, Order, OrderParams,
                OrderTriggerCondition, OrderType, PerpPosition, PostOnlyParam, SpotBalanceType,
                SpotPosition,
            },
        },
        ffi::{
            calculate_auction_price,
            calculate_margin_requirement_and_total_collateral_and_liability_info,
            check_ffi_version, get_oracle_price, IncrementalMarginCalculation, OraclePriceData,
        },
        math::constants::{
            BASE_PRECISION, BASE_PRECISION_I64, LIQUIDATION_FEE_PRECISION, MARGIN_PRECISION,
            PERCENTAGE_PRECISION, PRICE_PRECISION_I64, PRICE_PRECISION_U64, QUOTE_PRECISION,
            QUOTE_PRECISION_I64, SPOT_BALANCE_PRECISION, SPOT_BALANCE_PRECISION_U64,
            SPOT_CUMULATIVE_INTEREST_PRECISION, SPOT_WEIGHT_PRECISION,
        },
        types::{accounts::HighLeverageModeConfig, ContractTier, MarketType, ValidityGuardRails},
        utils::test_utils::{get_account_bytes, get_pyth_price},
        HistoricalOracleData, MarketStatus, PositionDirection, AMM,
    };

    fn sol_spot_market() -> SpotMarket {
        SpotMarket {
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
            order_step_size: 1_000,
            order_tick_size: 1_000,
            historical_oracle_data: HistoricalOracleData {
                last_oracle_price_twap5min: 240_000_000_000,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    fn usdc_spot_market() -> SpotMarket {
        SpotMarket {
            market_index: 0,
            oracle_source: OracleSource::QuoteAsset,
            cumulative_deposit_interest: SPOT_CUMULATIVE_INTEREST_PRECISION.into(),
            decimals: 6,
            initial_asset_weight: SPOT_WEIGHT_PRECISION,
            maintenance_asset_weight: SPOT_WEIGHT_PRECISION,
            deposit_balance: (100_000 * SPOT_BALANCE_PRECISION).into(),
            liquidator_fee: 0,
            order_step_size: 1_000,
            order_tick_size: 1_000,
            historical_oracle_data: HistoricalOracleData {
                last_oracle_price_twap5min: 1_000_000,
                ..Default::default()
            },
            ..SpotMarket::default()
        }
    }

    #[test]
    fn ffi_check_version() {
        let drift_ffi_sys = include_str!("../drift-ffi-sys/Cargo.toml");
        let cargo_toml: toml::Value = drift_ffi_sys.parse().unwrap();
        let expected_version = cargo_toml["package"]["version"].as_str();
        assert_eq!(&check_ffi_version(), expected_version.unwrap());
    }

    #[test]
    fn ffi_deser_1_76_0_spot_market() {
        // smoke test for deserializing program data (where u128/i128 alignment is 8)
        let spot_market_borsh =
            hex::decode(include_str!("../../res/spot_market_1_76_0.hex")).unwrap();
        let actual: &SpotMarket = bytemuck::from_bytes::<SpotMarket>(&spot_market_borsh[8..]); // ignore dscriminator

        assert_eq!(actual, &sol_spot_market());
    }

    #[test]
    fn ffi_spot_position_is_available() {
        let spot_position = SpotPosition::default();
        assert!(spot_position.is_available());
    }

    #[test]
    fn ffi_spot_position_get_signed_token_amount() {
        let spot_position = SpotPosition {
            scaled_balance: (123 * SPOT_BALANCE_PRECISION) as u64,
            market_index: 1,
            balance_type: SpotBalanceType::Deposit,
            ..Default::default()
        };

        let result = spot_position.get_signed_token_amount(&sol_spot_market());
        assert_eq!(result.unwrap(), 123 * SPOT_BALANCE_PRECISION as i128);
    }

    #[test]
    fn ffi_spot_market_get_asset_weight() {
        let spot_market = SpotMarket {
            initial_asset_weight: 9_000,
            initial_liability_weight: 11_000,
            decimals: 6,
            imf_factor: 0,
            ..Default::default()
        };
        let size = 1_000 * QUOTE_PRECISION;
        let price = QUOTE_PRECISION as i64;
        let asset_weight = spot_market
            .get_asset_weight(size, price, MarginRequirementType::Initial)
            .unwrap();
        assert_eq!(asset_weight, 9_000);
    }

    #[test]
    fn ffi_spot_market_get_liability_weight() {
        let spot_market = SpotMarket {
            initial_asset_weight: 9_000,
            initial_liability_weight: 11_000,
            decimals: 6,
            imf_factor: 0,
            ..Default::default()
        };

        let size = 1_000 * QUOTE_PRECISION;
        let liability_weight = spot_market
            .get_liability_weight(size, MarginRequirementType::Initial)
            .unwrap();
        assert_eq!(liability_weight, 11_000);
    }

    #[test]
    fn ffi_user_get_spot_position() {
        let mut user = User::default();
        user.spot_positions[1] = SpotPosition {
            market_index: 1,
            scaled_balance: 1_000 * SPOT_BALANCE_PRECISION_U64,
            balance_type: SpotBalanceType::Deposit,
            ..Default::default()
        };

        let result = user.get_spot_position(1);
        assert!(result.is_ok());
        let spot_position = result.unwrap();
        assert_eq!(spot_position.market_index, 1);
        assert_eq!(
            spot_position.scaled_balance,
            1_000 * SPOT_BALANCE_PRECISION_U64
        );
        assert_eq!(spot_position.balance_type, SpotBalanceType::Deposit);

        // Test for non-existent market index
        let result = user.get_spot_position(5);
        assert!(result.is_err());
    }

    #[test]
    fn ffi_user_get_perp_position() {
        let mut user = User::default();
        user.perp_positions[2] = PerpPosition {
            market_index: 2,
            base_asset_amount: 500,
            quote_asset_amount: 1_000,
            lp_shares: 1_000,
            ..Default::default()
        };

        let result = user.get_perp_position(2);
        assert!(result.is_ok());
        let perp_position = result.unwrap();
        assert_eq!(perp_position.market_index, 2);
        assert_eq!(perp_position.base_asset_amount, 500);
        assert_eq!(perp_position.quote_asset_amount, 1_000);

        // Test for non-existent market index
        let result = user.get_perp_position(5);
        assert!(result.is_err());
    }

    #[test]
    fn ffi_perp_position_is_available() {
        let position = PerpPosition::default();
        assert!(position.is_available());

        let position = PerpPosition {
            base_asset_amount: 100,
            ..Default::default()
        };
        assert!(!position.is_available());
    }

    #[test]
    fn ffi_perp_position_is_open_position() {
        let position = PerpPosition::default();
        assert!(!position.is_open_position());

        let position = PerpPosition {
            base_asset_amount: 100,
            ..Default::default()
        };
        assert!(position.is_open_position());
    }

    #[test]
    fn ffi_perp_position_worst_case_base_asset_amount() {
        let position = PerpPosition {
            base_asset_amount: 1_000 * BASE_PRECISION_I64,
            quote_asset_amount: 5_000 * QUOTE_PRECISION_I64,
            market_index: 1,
            ..Default::default()
        };
        let oracle_price = 10 * QUOTE_PRECISION_I64;
        let contract_type = ContractType::Perpetual;

        let result = position.worst_case_base_asset_amount(oracle_price, contract_type);
        assert!(result.is_ok());
        let worst_case_amount = result.unwrap();
        assert!(worst_case_amount >= 1000); // The worst case should be at least the current base asset amount
    }

    #[test]
    fn ffi_get_oracle_price() {
        let oracle_pubkey = Pubkey::new_unique();
        let oracle_account = Account {
            data: get_account_bytes(&mut get_pyth_price(240, 9)).to_vec(),
            owner: constants::ids::pyth_program::ID,
            ..Default::default()
        };

        let oracle_source = OracleSource::Pyth;
        let slot = 12_345;

        let result = get_oracle_price(oracle_source, &mut (oracle_pubkey, oracle_account), slot);

        // Assert the result
        assert!(result.is_ok());
        let oracle_price_data = result.unwrap();

        dbg!(oracle_price_data.price);
        assert!(oracle_price_data.price == 240 * QUOTE_PRECISION as i64);
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
            let limit_order = Order {
                order_type,
                slot: 100,
                ..Default::default()
            };
            let ffi_limit_order = limit_order;
            assert_eq!(ffi_limit_order.is_limit_order(), is_limit);
            assert_eq!(
                ffi_limit_order.is_resting_limit_order(100).unwrap(),
                is_limit
            );
        }
    }

    #[test]
    fn ffi_perp_market_get_mm_oracle_data_basic() {
        let perp_market = PerpMarket {
            market_index: 1,
            contract_tier: ContractTier::A,
            amm: AMM {
                mm_oracle_price: 1_000_123,
                mm_oracle_slot: 12345,
                ..Default::default()
            },
            ..Default::default()
        };
        let oracle_price_data = OraclePriceData {
            price: 1_000_000,
            confidence: 99 * PERCENTAGE_PRECISION as u64,
            delay: 2,
            has_sufficient_number_of_data_points: true,
            sequence_id: None,
        };
        let clock_slot = 12345;
        let validity_guard_rails = ValidityGuardRails::default();

        let result = perp_market.get_mm_oracle_price_data(
            oracle_price_data,
            clock_slot,
            &validity_guard_rails,
        );
        assert!(result.is_ok(), "Should succeed for valid input");
        let mm_oracle_data = result.unwrap();
        assert!(mm_oracle_data.safe_oracle_price_data.price > 0);
    }

    #[test]
    fn ffi_perp_market_fallback_price() {
        use crate::math::constants::{AMM_RESERVE_PRECISION, PEG_PRECISION};

        // Use properly scaled AMM values
        let default_reserves = 100 * AMM_RESERVE_PRECISION;
        let perp_market = PerpMarket {
            market_index: 1,
            contract_tier: ContractTier::A,
            amm: AMM {
                max_fill_reserve_fraction: 1,
                base_asset_reserve: default_reserves.into(),
                quote_asset_reserve: default_reserves.into(),
                sqrt_k: default_reserves.into(),
                peg_multiplier: PEG_PRECISION.into(),
                terminal_quote_asset_reserve: default_reserves.into(),
                concentration_coef: 5u128.into(),
                long_spread: 100,  // 1% spread
                short_spread: 100, // 1% spread
                max_base_asset_reserve: (u64::MAX as u128).into(),
                min_base_asset_reserve: 0u128.into(),
                order_step_size: 1,
                order_tick_size: 1,
                max_spread: 1000,
                historical_oracle_data: HistoricalOracleData {
                    last_oracle_price: crate::math::constants::PRICE_PRECISION_I64,
                    ..Default::default()
                },
                last_oracle_valid: true,
                ..Default::default()
            },
            ..Default::default()
        };
        let oracle_price = 10_000_000i64; // $10.00 with PRICE_PRECISION
        let seconds_til_expiry = 3600i64; // 1 hour

        // Test fallback price for Long direction (buying)
        let result_long =
            perp_market.fallback_price(PositionDirection::Long, oracle_price, seconds_til_expiry);
        assert!(result_long.is_ok(), "Should succeed for Long direction");
        let fallback_price_long = result_long.unwrap();
        assert!(fallback_price_long > 0, "Fallback price should be positive");

        // Test fallback price for Short direction (selling)
        let result_short =
            perp_market.fallback_price(PositionDirection::Short, oracle_price, seconds_til_expiry);
        assert!(result_short.is_ok(), "Should succeed for Short direction");
        let fallback_price_short = result_short.unwrap();
        assert!(
            fallback_price_short > 0,
            "Fallback price should be positive"
        );

        // For Long (buying), fallback price should typically be higher than oracle (ask price)
        // For Short (selling), fallback price should typically be lower than oracle (bid price)
        // Note: This depends on AMM state, but generally holds true
        assert!(
            fallback_price_long >= fallback_price_short,
            "Long fallback price should be >= Short fallback price"
        );
    }

    #[test]
    fn ffi_perp_market_get_margin_ratio() {
        let perp_market = PerpMarket {
            margin_ratio_initial: 1_000 * MARGIN_PRECISION, // 10%
            margin_ratio_maintenance: 500,                  // 5%
            imf_factor: 0,                                  // No impact for simplicity
            // enable HL mode for this market
            high_leverage_margin_ratio_maintenance: 1_234,
            high_leverage_margin_ratio_initial: 4_321,
            ..Default::default()
        };

        let size = 1_000 * MARGIN_PRECISION as u128; // Assuming MARGIN_PRECISION is defined

        // Test initial margin ratio
        let result = perp_market.get_margin_ratio(size, MarginRequirementType::Initial, false);
        assert!(result.is_ok());
        let initial_margin_ratio = result.unwrap();
        assert_eq!(initial_margin_ratio, 1_000 * MARGIN_PRECISION); // 10%

        // Test maintenance margin ratio
        let result = perp_market.get_margin_ratio(size, MarginRequirementType::Maintenance, false);
        assert!(result.is_ok());
        let maintenance_margin_ratio = result.unwrap();
        assert_eq!(maintenance_margin_ratio, 500); // 5%

        // HL mode
        let result = perp_market.get_margin_ratio(size, MarginRequirementType::Maintenance, true);
        assert!(result.is_ok());
        let maintenance_margin_ratio = result.unwrap();
        assert_eq!(maintenance_margin_ratio, 1_234); // 5%
    }

    #[test]
    fn ffi_order_params_update_perp_auction_params_populates_fields() {
        let market_index = 3u16;
        // PerpMarket with non-zero AMM fields to exercise FFI struct layout
        let perp_market = PerpMarket {
            market_index,
            status: MarketStatus::Active,
            contract_tier: ContractTier::A,
            amm: AMM {
                order_step_size: 2_000,
                order_tick_size: 1_000,
                base_asset_reserve: 10_000u128.into(),
                quote_asset_reserve: 20_000u128.into(),
                sqrt_k: 100u128.into(),
                peg_multiplier: 1_000_000u128.into(),
                terminal_quote_asset_reserve: 19_000u128.into(),
                concentration_coef: 5u128.into(),
                max_open_interest: 1_000_000u128.into(),
                mm_oracle_price: 1_234_567,
                mm_oracle_slot: 9_999,
                ..Default::default()
            },
            ..Default::default()
        };

        // Start with empty auction params and let program fill them
        let mut params = OrderParams {
            order_type: OrderType::Limit,
            market_type: MarketType::Perp,
            direction: PositionDirection::Short,
            user_order_id: 7,
            base_asset_amount: 50_000,
            price: 200_000,
            market_index,
            reduce_only: false,
            post_only: PostOnlyParam::None,
            bit_flags: 0,
            max_ts: None,
            trigger_price: None,
            trigger_condition: OrderTriggerCondition::Below,
            oracle_price_offset: None,
            auction_duration: None,
            auction_start_price: None,
            auction_end_price: None,
        };

        // Call through FFI
        let oracle_price = 199_500i64;

        // Debug: Check AMM fields before FFI call
        eprintln!("Before FFI call - AMM fields:");
        eprintln!("  mm_oracle_price: {}", perp_market.amm.mm_oracle_price);
        eprintln!("  order_step_size: {}", perp_market.amm.order_step_size);
        eprintln!("  order_tick_size: {}", perp_market.amm.order_tick_size);
        eprintln!(
            "  base_asset_reserve: {:?}",
            perp_market.amm.base_asset_reserve.as_u128()
        );
        eprintln!(
            "  quote_asset_reserve: {:?}",
            perp_market.amm.quote_asset_reserve.as_u128()
        );

        params.update_perp_auction_params(&perp_market, oracle_price, true);

        // Debug: Check if auction params were populated
        eprintln!("After FFI call - auction params:");
        eprintln!("  auction_duration: {:?}", params.auction_duration);
        eprintln!("  auction_start_price: {:?}", params.auction_start_price);
        eprintln!("  auction_end_price: {:?}", params.auction_end_price);

        // Expect auction params to be populated and non-zero
        assert!(params.auction_duration.is_some());
        assert!(params.auction_start_price.is_some());
        assert!(params.auction_end_price.is_some());

        let dur = params.auction_duration.unwrap();
        let start = params.auction_start_price.unwrap();
        let end = params.auction_end_price.unwrap();
        assert!(dur > 0);
        assert!(start != 0);
        assert!(end != 0);
    }

    #[test]
    fn ffi_order_params_update_perp_auction_params_reads_amm_fields() {
        let market_index = 4u16;

        let perp_market_a = PerpMarket {
            market_index,
            status: MarketStatus::Active,
            contract_tier: ContractTier::A,
            amm: AMM {
                order_step_size: 1_000,
                order_tick_size: 1_000,
                base_asset_reserve: 50_000u128.into(),
                quote_asset_reserve: 80_000u128.into(),
                sqrt_k: 200u128.into(),
                peg_multiplier: 1_100_000u128.into(),
                terminal_quote_asset_reserve: 79_000u128.into(),
                concentration_coef: 7u128.into(),
                max_open_interest: 2_000_000u128.into(),
                mm_oracle_price: 2_222_222,
                mm_oracle_slot: 10_001,
                ..Default::default()
            },
            ..Default::default()
        };

        let perp_market_b = PerpMarket {
            market_index,
            status: MarketStatus::Active,
            contract_tier: ContractTier::A,
            amm: AMM {
                order_step_size: 8_000, // different
                order_tick_size: 4_000, // different
                base_asset_reserve: 60_000u128.into(),
                quote_asset_reserve: 90_000u128.into(),
                sqrt_k: 250u128.into(),
                peg_multiplier: 900_000u128.into(),
                terminal_quote_asset_reserve: 88_000u128.into(),
                concentration_coef: 9u128.into(),
                max_open_interest: 3_000_000u128.into(),
                mm_oracle_price: 3_333_333,
                mm_oracle_slot: 10_005,
                ..Default::default()
            },
            ..Default::default()
        };

        let base_params = || OrderParams {
            order_type: OrderType::Limit,
            market_type: MarketType::Perp,
            direction: PositionDirection::Short,
            user_order_id: 8,
            base_asset_amount: 50_000,
            price: 200_000,
            market_index,
            reduce_only: false,
            post_only: PostOnlyParam::None,
            bit_flags: 0,
            max_ts: None,
            trigger_price: None,
            trigger_condition: OrderTriggerCondition::Below,
            oracle_price_offset: None,
            auction_duration: None,
            auction_start_price: None,
            auction_end_price: None,
        };

        let oracle_price = 199_500i64;

        // Debug: Check AMM fields before FFI calls
        eprintln!("Before FFI calls - AMM fields:");
        eprintln!(
            "  Market A - mm_oracle_price: {}, order_step_size: {}",
            perp_market_a.amm.mm_oracle_price, perp_market_a.amm.order_step_size
        );
        eprintln!(
            "  Market B - mm_oracle_price: {}, order_step_size: {}",
            perp_market_b.amm.mm_oracle_price, perp_market_b.amm.order_step_size
        );

        let mut a = base_params();
        a.update_perp_auction_params(&perp_market_a, oracle_price, false);

        let mut b = base_params();
        b.update_perp_auction_params(&perp_market_b, oracle_price, false);

        // Debug: Check results
        eprintln!("After FFI calls:");
        eprintln!(
            "  Market A - auction_duration: {:?}, auction_start_price: {:?}",
            a.auction_duration, a.auction_start_price
        );
        eprintln!(
            "  Market B - auction_duration: {:?}, auction_start_price: {:?}",
            b.auction_duration, b.auction_start_price
        );

        // If AMM fields are read correctly across FFI, results should differ
        assert!(a.auction_duration.is_some() && b.auction_duration.is_some());
        assert!(a.auction_start_price.is_some() && b.auction_start_price.is_some());
        assert!(a.auction_end_price.is_some() && b.auction_end_price.is_some());

        let a_tuple = (
            a.auction_duration.unwrap(),
            a.auction_start_price.unwrap(),
            a.auction_end_price.unwrap(),
        );
        let b_tuple = (
            b.auction_duration.unwrap(),
            b.auction_start_price.unwrap(),
            b.auction_end_price.unwrap(),
        );
        assert_ne!(
            a_tuple, b_tuple,
            "auction params should reflect differing AMM fields"
        );
    }

    #[test]
    fn ffi_test_calculate_margin_requirement_and_total_collateral_and_liability_info() {
        // smoke test for ffi compatibility, logic tested in `math::` module
        let btc_perp_index = 1_u16;
        let mut user = User::default();
        user.spot_positions[1] = SpotPosition {
            market_index: 1,
            scaled_balance: (1_000 * SPOT_BALANCE_PRECISION) as u64,
            balance_type: SpotBalanceType::Deposit,
            ..Default::default()
        };
        user.perp_positions[0] = PerpPosition {
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
                    PerpMarket::DISCRIMINATOR,
                    bytemuck::bytes_of(&PerpMarket {
                        market_index: btc_perp_index,
                        ..Default::default()
                    }),
                ]
                .concat()
                .to_vec(),
                ..Default::default()
            },
        }];
        // Set up both USDC and SOL spot markets
        let usdc_spot_market = usdc_spot_market();
        let sol_spot_market = sol_spot_market();
        let mut spot_markets = vec![
            AccountWithKey {
                key: Pubkey::new_unique(),
                account: Account {
                    owner: crate::constants::PROGRAM_ID,
                    data: [
                        SpotMarket::DISCRIMINATOR,
                        bytemuck::bytes_of(&usdc_spot_market),
                    ]
                    .concat()
                    .to_vec(),
                    ..Default::default()
                },
            },
            AccountWithKey {
                key: Pubkey::new_unique(),
                account: Account {
                    owner: crate::constants::PROGRAM_ID,
                    data: [
                        SpotMarket::DISCRIMINATOR,
                        bytemuck::bytes_of(&sol_spot_market),
                    ]
                    .concat()
                    .to_vec(),
                    ..Default::default()
                },
            },
        ];

        // Set up oracles for both markets
        // USDC oracle (market index 0) - using quote asset oracle
        let usdc_oracle = AccountWithKey {
            key: usdc_spot_market.oracle,
            account: Account {
                data: get_account_bytes(&mut get_pyth_price(1, 6)).to_vec(), // 1 USD
                owner: constants::ids::pyth_program::ID,
                ..Default::default()
            },
        };

        // SOL oracle (market index 1) - using the specific oracle pubkey
        let sol_oracle = AccountWithKey {
            key: sol_spot_market.oracle,
            account: Account {
                data: get_account_bytes(&mut get_pyth_price(240, 9)).to_vec(),
                owner: constants::ids::pyth_program::ID,
                ..Default::default()
            },
        };

        let mut oracles = [usdc_oracle, sol_oracle];
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
    fn ffi_simulate_place_perp_order() {
        // smoke test for ffi compatibility, logic tested in `math::` module
        let btc_perp_index = 1_u16;
        let mut user = User::default();
        user.spot_positions[1] = SpotPosition {
            market_index: 1,
            scaled_balance: (1_000 * SPOT_BALANCE_PRECISION) as u64,
            balance_type: SpotBalanceType::Deposit,
            ..Default::default()
        };
        user.perp_positions[0] = PerpPosition {
            market_index: btc_perp_index,
            base_asset_amount: 100 * BASE_PRECISION_I64 as i64,
            quote_asset_amount: -5_000 * QUOTE_PRECISION as i64,
            ..Default::default()
        };
        user.perp_positions[1] = PerpPosition {
            market_index: 0,
            base_asset_amount: 100 * BASE_PRECISION_I64 as i64,
            quote_asset_amount: -5_000 * QUOTE_PRECISION as i64,
            ..Default::default()
        };

        // Create mock accounts
        let mut perp_markets = vec![
            AccountWithKey {
                key: Pubkey::new_unique(),
                account: Account {
                    owner: crate::constants::PROGRAM_ID,
                    data: [
                        PerpMarket::DISCRIMINATOR,
                        bytemuck::bytes_of(&PerpMarket {
                            market_index: btc_perp_index,
                            status: MarketStatus::Active,
                            amm: AMM {
                                order_step_size: 1_000,
                                order_tick_size: 1_000,
                                ..Default::default()
                            },
                            ..Default::default()
                        }),
                    ]
                    .concat()
                    .to_vec(),
                    ..Default::default()
                },
            },
            AccountWithKey {
                key: Pubkey::new_unique(),
                account: Account {
                    owner: crate::constants::PROGRAM_ID,
                    data: [
                        PerpMarket::DISCRIMINATOR,
                        bytemuck::bytes_of(&PerpMarket {
                            market_index: 0,
                            status: MarketStatus::Active,
                            amm: AMM {
                                order_step_size: 1_000,
                                order_tick_size: 1_000,
                                ..Default::default()
                            },
                            ..Default::default()
                        }),
                    ]
                    .concat()
                    .to_vec(),
                    ..Default::default()
                },
            },
        ];
        let mut spot_markets = vec![
            AccountWithKey {
                key: Pubkey::new_unique(),
                account: Account {
                    owner: crate::constants::PROGRAM_ID,
                    data: [
                        SpotMarket::DISCRIMINATOR,
                        bytemuck::bytes_of(&sol_spot_market()),
                    ]
                    .concat()
                    .to_vec(),
                    ..Default::default()
                },
            },
            AccountWithKey {
                key: Pubkey::new_unique(),
                account: Account {
                    owner: crate::constants::PROGRAM_ID,
                    data: [
                        SpotMarket::DISCRIMINATOR,
                        bytemuck::bytes_of(&usdc_spot_market()),
                    ]
                    .concat()
                    .to_vec(),
                    ..Default::default()
                },
            },
        ];

        create_account_info!(
            get_pyth_price(240, 9),
            &sol_spot_market().oracle,
            pyth_program::ID,
            sol_oracle
        );
        create_account_info!(
            get_pyth_price(1, 6),
            &usdc_spot_market().oracle,
            pyth_program::ID,
            usdc_oracle
        );

        let mut oracles = [sol_oracle, usdc_oracle];
        let mut accounts = AccountsList::new(&mut perp_markets, &mut spot_markets, &mut oracles);

        let res = simulate_place_perp_order(
            &mut user,
            &mut accounts,
            &State::default(),
            &OrderParams {
                market_index: 1,
                market_type: MarketType::Perp,
                direction: PositionDirection::Short,
                base_asset_amount: 123 * BASE_PRECISION as u64,
                order_type: OrderType::Market,
                ..Default::default()
            },
            None,
            None,
            &mut None,
        );
        assert!(res.is_ok_and(|truthy| truthy));

        let res = simulate_place_perp_order(
            &mut user,
            &mut accounts,
            &State::default(),
            &OrderParams {
                market_index: 1,
                market_type: MarketType::Perp,
                direction: PositionDirection::Short,
                base_asset_amount: 1_234 * BASE_PRECISION as u64,
                order_type: OrderType::Market,
                bit_flags: 0b0000_0010,
                ..Default::default()
            },
            Some(&mut HighLeverageModeConfig {
                max_users: 5,
                current_users: 2,
                reduce_only: 0,
                padding1: Default::default(),
                current_maintenance_users: 0,
                padding2: Default::default(),
            }),
            None,
            &mut None,
        );
        dbg!(&res);
        assert!(res.is_ok_and(|truthy| truthy));
    }

    #[test]
    fn ffi_simulate_place_perp_order_with_max_margin_ratio() {
        // smoke test for ffi compatibility, logic tested in `math::` module
        let btc_perp_index = 1_u16;
        let mut user = User::default();
        user.spot_positions[1] = SpotPosition {
            market_index: 1,
            scaled_balance: (1_000 * SPOT_BALANCE_PRECISION) as u64,
            balance_type: SpotBalanceType::Deposit,
            ..Default::default()
        };
        user.perp_positions[0] = PerpPosition {
            market_index: btc_perp_index,
            base_asset_amount: 100 * BASE_PRECISION_I64 as i64,
            quote_asset_amount: -5_000 * QUOTE_PRECISION as i64,
            ..Default::default()
        };
        user.perp_positions[1] = PerpPosition {
            market_index: 0,
            base_asset_amount: 100 * BASE_PRECISION_I64 as i64,
            quote_asset_amount: -5_000 * QUOTE_PRECISION as i64,
            ..Default::default()
        };

        // Create mock accounts
        let mut perp_markets = vec![
            AccountWithKey {
                key: Pubkey::new_unique(),
                account: Account {
                    owner: crate::constants::PROGRAM_ID,
                    data: [
                        PerpMarket::DISCRIMINATOR,
                        bytemuck::bytes_of(&PerpMarket {
                            market_index: btc_perp_index,
                            status: MarketStatus::Active,
                            amm: AMM {
                                order_step_size: 1_000,
                                order_tick_size: 1_000,
                                ..Default::default()
                            },
                            ..Default::default()
                        }),
                    ]
                    .concat()
                    .to_vec(),
                    ..Default::default()
                },
            },
            AccountWithKey {
                key: Pubkey::new_unique(),
                account: Account {
                    owner: crate::constants::PROGRAM_ID,
                    data: [
                        PerpMarket::DISCRIMINATOR,
                        bytemuck::bytes_of(&PerpMarket {
                            market_index: 0,
                            status: MarketStatus::Active,
                            amm: AMM {
                                order_step_size: 1_000,
                                order_tick_size: 1_000,
                                ..Default::default()
                            },
                            ..Default::default()
                        }),
                    ]
                    .concat()
                    .to_vec(),
                    ..Default::default()
                },
            },
        ];
        let mut spot_markets = vec![
            AccountWithKey {
                key: Pubkey::new_unique(),
                account: Account {
                    owner: crate::constants::PROGRAM_ID,
                    data: [
                        SpotMarket::DISCRIMINATOR,
                        bytemuck::bytes_of(&sol_spot_market()),
                    ]
                    .concat()
                    .to_vec(),
                    ..Default::default()
                },
            },
            AccountWithKey {
                key: Pubkey::new_unique(),
                account: Account {
                    owner: crate::constants::PROGRAM_ID,
                    data: [
                        SpotMarket::DISCRIMINATOR,
                        bytemuck::bytes_of(&usdc_spot_market()),
                    ]
                    .concat()
                    .to_vec(),
                    ..Default::default()
                },
            },
        ];

        create_account_info!(
            get_pyth_price(240, 9),
            &sol_spot_market().oracle,
            pyth_program::ID,
            sol_oracle
        );
        create_account_info!(
            get_pyth_price(1, 6),
            &usdc_spot_market().oracle,
            pyth_program::ID,
            usdc_oracle
        );

        let mut oracles = [sol_oracle, usdc_oracle];
        let mut accounts = AccountsList::new(&mut perp_markets, &mut spot_markets, &mut oracles);

        let res = simulate_place_perp_order(
            &mut user,
            &mut accounts,
            &State::default(),
            &OrderParams {
                market_index: 1,
                market_type: MarketType::Perp,
                direction: PositionDirection::Short,
                base_asset_amount: 123 * BASE_PRECISION as u64,
                order_type: OrderType::Market,
                ..Default::default()
            },
            None,
            Some(2),
            &mut None,
        );
        assert!(res.is_ok_and(|truthy| truthy));

        let res: Result<bool, crate::types::SdkError> = simulate_place_perp_order(
            &mut user,
            &mut accounts,
            &State::default(),
            &OrderParams {
                market_index: 1,
                market_type: MarketType::Perp,
                direction: PositionDirection::Short,
                base_asset_amount: 1_234 * BASE_PRECISION as u64,
                order_type: OrderType::Market,
                bit_flags: 0b0000_0010,
                ..Default::default()
            },
            Some(&mut HighLeverageModeConfig {
                max_users: 5,
                current_users: 2,
                reduce_only: 0,
                padding1: Default::default(),
                current_maintenance_users: 0,
                padding2: Default::default(),
            }),
            None,
            &mut None,
        );
        dbg!(&res);
        assert!(res.is_ok_and(|truthy| truthy));
    }

    #[test]
    fn ffi_calculate_auction_price() {
        let price = calculate_auction_price(
            &Order {
                price: 123_456,
                order_type: OrderType::Limit,
                direction: PositionDirection::Long,
                ..Default::default()
            },
            0,
            1_000,
            None,
            false,
        );
        assert_eq!(price.unwrap(), 0,);

        let price = calculate_auction_price(
            &Order {
                slot: 1,
                auction_duration: 10,
                auction_start_price: 90 * PRICE_PRECISION_I64,
                auction_end_price: 100 * PRICE_PRECISION_I64,
                oracle_price_offset: 555,
                order_type: OrderType::Oracle,
                direction: PositionDirection::Long,
                ..Default::default()
            },
            5,
            3,
            Some(100 * PRICE_PRECISION_I64),
            false,
        );
        assert!(price.is_ok_and(|p| p > 0));
    }

    #[test]
    fn ffi_order_get_limit_price() {
        let tick_size = 1_000;
        let oracle_price = Some(100 * PRICE_PRECISION_I64);
        let fallback_price = Some(95 * PRICE_PRECISION_U64);
        let slot = 100;

        // Test cases
        let cases = [
            // Case 1: Basic limit order with price
            (
                Order {
                    price: 95 * PRICE_PRECISION_U64,
                    order_type: OrderType::Limit,
                    direction: PositionDirection::Long,
                    ..Default::default()
                },
                "Basic limit order",
            ),
            // Case 2: Order with auction parameters
            (
                Order {
                    slot: 90,
                    auction_duration: 20,
                    auction_start_price: 90 * PRICE_PRECISION_I64,
                    auction_end_price: 100 * PRICE_PRECISION_I64,
                    order_type: OrderType::Limit,
                    direction: PositionDirection::Long,
                    ..Default::default()
                },
                "Order with auction parameters",
            ),
            // Case 3: Order with oracle price offset
            (
                Order {
                    oracle_price_offset: 5 * PRICE_PRECISION_I64 as i32,
                    order_type: OrderType::Limit,
                    direction: PositionDirection::Long,
                    ..Default::default()
                },
                "Order with oracle price offset",
            ),
            // Case 4: Order with zero price and fallback
            (
                Order {
                    price: 0,
                    order_type: OrderType::Limit,
                    direction: PositionDirection::Long,
                    ..Default::default()
                },
                "Order with zero price and fallback",
            ),
        ];

        for (order, case_name) in cases {
            let result = order
                .get_limit_price(oracle_price, fallback_price, slot, tick_size, false, None)
                .unwrap();
            assert!(result.is_some(), "{} should return a price", case_name);
            let price = result.unwrap();
            assert!(price > 0, "{} should return a positive price", case_name);
        }
    }

    #[test]
    fn ffi_perp_market_get_protected_maker_params() {
        let perp_market = PerpMarket {
            protected_maker_limit_price_divisor: 100,
            protected_maker_dynamic_divisor: 2,
            amm: AMM {
                oracle_std: 10_000,
                mark_std: 5_000,
                order_tick_size: 1_000,
                ..Default::default()
            },
            ..Default::default()
        };

        let params = perp_market.get_protected_maker_params();

        // Verify the structure matches what we expect
        assert_eq!(params.limit_price_divisor, 100);
        assert_eq!(params.dynamic_offset, 5_000); // max(10_000, 5_000) / 2
        assert_eq!(params.tick_size, 1_000);
    }

    #[test]
    fn ffi_calculate_auction_params_for_trigger_order() {
        use crate::{
            drift_idl::{
                accounts::PerpMarket,
                types::{Order, OrderType, PositionDirection},
            },
            ffi::{abi_types::OraclePriceData, calculate_auction_params_for_trigger_order},
        };
        let order = Order {
            order_type: OrderType::TriggerMarket,
            direction: PositionDirection::Long,
            slot: 1,
            auction_duration: 10,
            auction_start_price: 90_000,
            auction_end_price: 100_000,
            market_index: 0,
            market_type: MarketType::Perp,
            ..Default::default()
        };
        let oracle_price = OraclePriceData {
            price: 2 * PRICE_PRECISION_I64,
            confidence: 100,
            delay: 0,
            has_sufficient_number_of_data_points: true,
            sequence_id: None,
        };
        let perp_market = PerpMarket {
            contract_tier: ContractTier::A,
            market_index: 0,
            ..Default::default()
        };
        let result =
            calculate_auction_params_for_trigger_order(&order, &oracle_price, Some(&perp_market));
        assert!(result.is_ok(), "FFI call should succeed");
        let (duration, start, end) = result.unwrap();
        assert_eq!(duration, 20);
        assert!(start > 0);
        assert!(end > 0);
    }

    #[test]
    fn ffi_test_calculate_simplified_margin_requirement() {
        // Test the simplified margin requirement FFI function
        // This should match the results from the existing margin calculation test
        let btc_perp_index = 1_u16;
        let mut user = User::default();
        user.spot_positions[1] = SpotPosition {
            market_index: 1,
            scaled_balance: (1_000 * SPOT_BALANCE_PRECISION) as u64,
            balance_type: SpotBalanceType::Deposit,
            ..Default::default()
        };
        user.perp_positions[0] = PerpPosition {
            market_index: btc_perp_index,
            base_asset_amount: 100 * BASE_PRECISION_I64 as i64,
            quote_asset_amount: -5_000 * QUOTE_PRECISION as i64,
            ..Default::default()
        };

        // Create market state data similar to the existing test
        let mut market_state_data = crate::market_state::MarketStateData::default();

        // Add USDC spot market (market index 0) - required for quote asset
        let usdc_spot_market = usdc_spot_market();
        market_state_data.set_spot_market(usdc_spot_market);

        // Add SOL spot market (market index 1)
        let sol_spot_market = sol_spot_market();
        market_state_data.set_spot_market(sol_spot_market);

        // Add perp market with proper configuration
        let perp_market = PerpMarket {
            market_index: btc_perp_index,
            margin_ratio_initial: 1_000 * MARGIN_PRECISION, // 10%
            margin_ratio_maintenance: 500,                  // 5%
            amm: AMM {
                ..Default::default()
            },
            ..Default::default()
        };
        market_state_data.set_perp_market(perp_market);

        // Add oracle prices
        let sol_oracle_price = OraclePriceData {
            price: 240 * QUOTE_PRECISION as i64,
            confidence: 99 * PERCENTAGE_PRECISION as u64,
            delay: 2,
            has_sufficient_number_of_data_points: true,
            sequence_id: None,
        };

        // Add oracle prices
        let btc_oracle_price = OraclePriceData {
            price: 120_000 * QUOTE_PRECISION as i64,
            confidence: 99 * PERCENTAGE_PRECISION as u64,
            delay: 2,
            has_sufficient_number_of_data_points: true,
            sequence_id: None,
        };

        // USDC oracle price (market index 0) - required for quote asset
        let usdc_oracle_price = OraclePriceData {
            price: QUOTE_PRECISION as i64, // 1 USD
            confidence: 1,
            delay: 0,
            has_sufficient_number_of_data_points: true,
            sequence_id: None,
        };

        market_state_data.set_spot_oracle_price(0, usdc_oracle_price); // USDC
        market_state_data.set_spot_oracle_price(1, sol_oracle_price); // SOL
        market_state_data.set_perp_oracle_price(btc_perp_index, btc_oracle_price);

        // Test different margin requirement types
        let margin_types = [
            MarginRequirementType::Initial,
            MarginRequirementType::Maintenance,
        ];

        for margin_type in margin_types.iter() {
            let result = unsafe {
                margin_calculate_simplified_margin_requirement(
                    &user,
                    &market_state_data,
                    *margin_type,
                    0,
                )
            };

            // Verify the FFI call succeeds
            assert!(
                matches!(result, FfiResult::ROk(_)),
                "FFI call should succeed for margin type: {:?}",
                margin_type
            );

            let result = match result {
                FfiResult::ROk(data) => data,
                FfiResult::RErr(_) => panic!("FFI call failed for margin type: {:?}", margin_type),
            };

            // Verify we get reasonable values
            assert!(
                result.total_collateral != 0,
                "Total collateral should not be zero for margin type: {:?}",
                margin_type
            );
            assert!(
                result.margin_requirement > 0,
                "Margin requirement should be positive for margin type: {:?}",
                margin_type
            );
        }
    }

    #[test]
    fn ffi_test_incremental_margin_calculation() {
        // Test the cached margin calculation FFI functions
        let btc_perp_index = 1_u16;
        let mut user = User::default();
        user.spot_positions[1] = SpotPosition {
            market_index: 1,
            scaled_balance: (100 * SPOT_BALANCE_PRECISION) as u64, // Smaller amount to avoid overflow
            balance_type: SpotBalanceType::Deposit,
            ..Default::default()
        };
        user.perp_positions[0] = PerpPosition {
            market_index: btc_perp_index,
            base_asset_amount: 10 * BASE_PRECISION_I64 as i64, // Smaller amount
            quote_asset_amount: -500 * QUOTE_PRECISION as i64, // Smaller amount
            ..Default::default()
        };

        // Create market state data
        let market_state = crate::market_state::MarketState::default();

        // Add USDC spot market (market index 0) - required for quote asset
        let usdc_spot_market = usdc_spot_market();
        market_state.set_spot_market(usdc_spot_market);

        // Add SOL spot market (market index 1)
        let sol_spot_market = sol_spot_market();
        market_state.set_spot_market(sol_spot_market);

        // Add perp market with proper configuration
        let perp_market = PerpMarket {
            market_index: btc_perp_index,
            margin_ratio_initial: 1_000 * MARGIN_PRECISION, // 10%
            margin_ratio_maintenance: 500,                  // 5%
            amm: AMM {
                ..Default::default()
            },
            ..Default::default()
        };
        market_state.set_perp_market(perp_market);

        // Add oracle prices
        let sol_oracle_price = OraclePriceData {
            price: 240 * QUOTE_PRECISION as i64,
            confidence: 99 * PERCENTAGE_PRECISION as u64,
            delay: 2,
            has_sufficient_number_of_data_points: true,
            sequence_id: None,
        };

        let btc_oracle_price = OraclePriceData {
            price: 120_000 * QUOTE_PRECISION as i64,
            confidence: 99 * PERCENTAGE_PRECISION as u64,
            delay: 2,
            has_sufficient_number_of_data_points: true,
            sequence_id: None,
        };

        // USDC oracle price (market index 0) - required for quote asset
        let usdc_oracle_price = OraclePriceData {
            price: QUOTE_PRECISION as i64, // 1 USD
            confidence: 1,
            delay: 0,
            has_sufficient_number_of_data_points: true,
            sequence_id: None,
        };

        market_state.set_spot_oracle_price(0, usdc_oracle_price); // USDC
        market_state.set_spot_oracle_price(1, sol_oracle_price); // SOL
        market_state.set_perp_oracle_price(btc_perp_index, btc_oracle_price);

        let timestamp = 1_000_000;

        // Test different margin requirement types
        let margin_types = [MarginRequirementType::Maintenance];

        for margin_type in margin_types.iter() {
            // Test 1: Create cached margin calculation from user using FFI
            let mut calculator = IncrementalMarginCalculation::from_user(
                &user,
                &market_state,
                *margin_type,
                timestamp,
                None,
            );

            // Verify we get reasonable initial values
            assert!(
                calculator.total_collateral != 0,
                "Total collateral should not be zero for margin type: {:?}",
                margin_type
            );

            // Test 2: Update spot position using FFI
            let mut updated_spot_position = user.spot_positions[1].clone();
            updated_spot_position.scaled_balance = (200 * SPOT_BALANCE_PRECISION) as u64; // Double the balance

            let free_collateral_before = calculator.free_collateral();
            calculator.update_spot_position(&updated_spot_position, &market_state, timestamp + 1);

            // Verify the update affected the calculation
            assert!(
                calculator.free_collateral() != free_collateral_before,
                "free collateral should change after perp position update"
            );
            // The FFI function should have been called (values might not change due to implementation)
            // Just verify the function completed without error

            // Test 3: Update perp position using FFI
            let mut updated_perp_position = user.perp_positions[0].clone();
            updated_perp_position.base_asset_amount = 20 * BASE_PRECISION_I64 as i64; // Double the position

            calculator.update_perp_position(&updated_perp_position, &market_state, timestamp + 2);

            dbg!(&calculator);

            assert!(
                calculator.free_collateral() != free_collateral_before,
                "free collateral should change after perp position update"
            );

            // Verify the update affected the calculation
            assert!(
                calculator.total_collateral != 0,
                "Total collateral should still be non-zero after perp position update"
            );
            // The FFI function should have been called (values might not change due to implementation)
            // Just verify the function completed without error

            // Test 5: Verify metadata is accessible (FFI might not preserve margin type exactly)
            assert!(
                calculator.margin_type == *margin_type,
                "Margin type should match the input: expected {:?}, got {:?}",
                margin_type,
                calculator.margin_type
            );

            // Test 6: Verify other fields are accessible
            // Note: Free collateral can be negative if margin requirement exceeds total collateral
            assert!(
                calculator.free_collateral() != 0 || calculator.total_collateral == 0,
                "Free collateral should be calculated correctly"
            );
        }
    }
}

// Simplified Margin Calculation FFI declarations
extern "C" {
    #[allow(improper_ctypes)]
    pub fn margin_calculate_simplified_margin_requirement(
        user: &accounts::User,
        market_state: &crate::market_state::MarketStateData,
        margin_type: MarginRequirementType,
        margin_buffer: u32,
    ) -> FfiResult<SimplifiedMarginCalculation>;

    // Cached Margin Calculation FFI declarations
    #[allow(improper_ctypes)]
    pub fn incremental_margin_calculation_from_user(
        user: &accounts::User,
        market_state: &crate::market_state::MarketStateData,
        margin_type: MarginRequirementType,
        timestamp: u64,
        margin_buffer: u32,
    ) -> IncrementalMarginCalculation;

    #[allow(improper_ctypes)]
    pub fn incremental_margin_calculation_update_spot_position(
        cached: &mut IncrementalMarginCalculation,
        spot_position: &types::SpotPosition,
        market_state: &crate::market_state::MarketStateData,
        timestamp: u64,
    );

    #[allow(improper_ctypes)]
    pub fn incremental_margin_calculation_update_perp_position(
        cached: &mut IncrementalMarginCalculation,
        perp_position: &types::PerpPosition,
        market_state: &crate::market_state::MarketStateData,
        timestamp: u64,
    );
}
