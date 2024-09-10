//!
//! Define FFI for subset of drift program
//!
use abi_stable::std_types::RResult::{RErr, ROk};
use drift_program::{
    math::margin::MarginRequirementType,
    state::{
        oracle::{get_oracle_price as get_oracle_price_, OracleSource},
        oracle_map::OracleMap,
        perp_market::{ContractType, PerpMarket},
        perp_market_map::PerpMarketMap,
        spot_market::SpotMarket,
        spot_market_map::SpotMarketMap,
        user::{Order, PerpPosition, SpotPosition, User},
    },
};
use solana_program::{
    account_info::{Account as _, AccountInfo, IntoAccountInfo},
    clock::Slot,
};
use solana_sdk::{account::Account, pubkey::Pubkey};

use crate::types::{
    compat::{self},
    FfiResult, MarginCalculation, MarginContextMode, OraclePriceData,
};

#[no_mangle]
pub extern "C" fn oracle_get_oracle_price(
    oracle_source: &OracleSource,
    price_oracle: &mut (Pubkey, Account),
    clock_slot: Slot,
) -> FfiResult<OraclePriceData> {
    to_ffi_result(
        get_oracle_price_(oracle_source, &price_oracle.into_account_info(), clock_slot)
            .map(|o| unsafe { std::mem::transmute(o) }),
    )
}

#[no_mangle]
pub extern "C" fn math_calculate_margin_requirement_and_total_collateral_and_liability_info(
    user: &User,
    accounts: &mut AccountsList,
    margin_context: MarginContextMode,
) -> FfiResult<MarginCalculation> {
    let spot_accounts = accounts
        .spot_markets
        .iter_mut()
        .map(IntoAccountInfo::into_account_info)
        .collect::<Vec<_>>();
    let spot_map =
        SpotMarketMap::load(&Default::default(), &mut spot_accounts.iter().peekable()).unwrap();

    let perp_accounts = accounts
        .perp_markets
        .iter_mut()
        .map(IntoAccountInfo::into_account_info)
        .collect::<Vec<_>>();
    let perp_map =
        PerpMarketMap::load(&Default::default(), &mut perp_accounts.iter().peekable()).unwrap();

    let oracle_accounts = accounts
        .oracles
        .iter_mut()
        .map(IntoAccountInfo::into_account_info)
        .collect::<Vec<_>>();
    let mut oracle_map = OracleMap::load(
        &mut oracle_accounts.iter().peekable(),
        accounts.latest_slot,
        None,
    )
    .unwrap();

    let margin_calculation = drift_program::math::margin::calculate_margin_requirement_and_total_collateral_and_liability_info(
        user,
        &perp_map,
        &spot_map,
        &mut oracle_map,
        margin_context.into(),
    );

    let m = margin_calculation.map(|m| MarginCalculation {
        total_collateral: m.total_collateral.into(),
        margin_requirement: m.margin_requirement.into(),
        all_oracles_valid: m.all_oracles_valid,
        with_perp_isolated_liability: m.with_perp_isolated_liability,
        with_spot_isolated_liability: m.with_spot_isolated_liability,
        total_spot_asset_value: m.total_spot_asset_value.into(),
        total_spot_liability_value: m.total_spot_liability_value.into(),
        total_perp_liability_value: m.total_perp_liability_value.into(),
        total_perp_pnl: m.total_perp_pnl.into(),
        open_orders_margin_requirement: m.open_orders_margin_requirement.into(),
    });

    to_ffi_result(m)
}

#[no_mangle]
pub extern "C" fn order_is_limit_order(order: &Order) -> bool {
    order.is_limit_order()
}

#[no_mangle]
pub extern "C" fn order_is_resting_limit_order(order: &Order, slot: u64) -> FfiResult<bool> {
    to_ffi_result(order.is_resting_limit_order(slot))
}

#[no_mangle]
pub extern "C" fn perp_market_get_margin_ratio(
    market: &PerpMarket,
    size: compat::u128,
    margin_type: MarginRequirementType,
) -> FfiResult<u32> {
    to_ffi_result(market.get_margin_ratio(size.0, margin_type))
}

#[no_mangle]
pub extern "C" fn perp_position_is_available(position: &PerpPosition) -> bool {
    position.is_available()
}

#[no_mangle]
pub extern "C" fn perp_position_is_open_position(position: &PerpPosition) -> bool {
    position.is_open_position()
}

#[no_mangle]
pub extern "C" fn perp_position_worst_case_base_asset_amount(
    position: &PerpPosition,
    oracle_price: i64,
    contract_type: ContractType,
) -> FfiResult<compat::i128> {
    let res = position.worst_case_base_asset_amount(oracle_price, contract_type);
    to_ffi_result(res.map(compat::i128))
}

#[no_mangle]
pub extern "C" fn perp_position_simulate_settled_lp_position(
    position: &PerpPosition,
    market: &PerpMarket,
    oracle_price: i64,
) -> FfiResult<PerpPosition> {
    to_ffi_result(position.simulate_settled_lp_position(market, oracle_price))
}

#[no_mangle]
pub extern "C" fn spot_market_get_asset_weight(
    market: &SpotMarket,
    size: compat::u128,
    oracle_price: i64,
    margin_requirement_type: MarginRequirementType,
) -> FfiResult<u32> {
    to_ffi_result(market.get_asset_weight(size.0, oracle_price, &margin_requirement_type))
}

#[no_mangle]
pub extern "C" fn spot_market_get_liability_weight(
    market: &SpotMarket,
    size: compat::u128,
    margin_requirement_type: MarginRequirementType,
) -> FfiResult<u32> {
    to_ffi_result(market.get_liability_weight(size.0, &margin_requirement_type))
}

#[no_mangle]
pub extern "C" fn spot_position_is_available(position: &SpotPosition) -> bool {
    position.is_available()
}

#[no_mangle]
pub extern "C" fn spot_position_get_signed_token_amount(
    position: &SpotPosition,
    market: &SpotMarket,
) -> FfiResult<compat::i128> {
    to_ffi_result(position.get_signed_token_amount(market).map(compat::i128))
}

#[no_mangle]
pub extern "C" fn user_get_spot_position(
    user: &User,
    market_index: u16,
) -> FfiResult<&SpotPosition> {
    to_ffi_result(user.get_spot_position(market_index))
}

#[no_mangle]
pub extern "C" fn user_get_perp_position(
    user: &User,
    market_index: u16,
) -> FfiResult<&PerpPosition> {
    to_ffi_result(user.get_perp_position(market_index))
}

//
// Inbound Types
//
#[repr(C)]
#[derive(Debug)]
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

impl From<AccountWithKey> for (Pubkey, Account) {
    fn from(value: AccountWithKey) -> Self {
        (value.key, value.account)
    }
}

impl<'a> IntoAccountInfo<'a> for &'a mut AccountWithKey {
    fn into_account_info(self) -> solana_sdk::account_info::AccountInfo<'a> {
        let (lamports, data, owner, executable, rent_epoch) = self.account.get();
        AccountInfo::new(
            &self.key, false, false, lamports, data, owner, executable, rent_epoch,
        )
    }
}

/// FFI equivalent of an `AccountMap`
#[repr(C)]
#[derive(Debug)]
pub struct AccountsList<'a> {
    pub perp_markets: &'a mut [AccountWithKey],
    pub spot_markets: &'a mut [AccountWithKey],
    pub oracles: &'a mut [AccountWithKey],
    pub latest_slot: Slot,
}

//
// Helpers
//
/// Convert Drift program result into an FFI compatible version
#[inline]
pub(crate) fn to_ffi_result<T>(result: Result<T, drift_program::error::ErrorCode>) -> FfiResult<T> {
    match result {
        Ok(r) => ROk(r),
        Err(err) => RErr(err.into()),
    }
}

#[cfg(test)]
mod tests {
    use anchor_lang::AccountDeserialize;
    use drift_program::state::spot_market::SpotMarket;

    #[test]
    fn spot_market_deser() {
        let buf = hex_literal::hex!("64b1086ba84141270000000000000000000000000000000000000000000000000000000000000000fe650f0367d4a7ef9815a593ea15d36593f0643aaaf0149bb04be67ab851decd000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000058961b0a0300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010a5d4e800000000000000000000000000000000000000000000000000000000e40b5402000000000000000000000000e40b54020000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000401f000028230000e02e0000f82a000000000000e8030000000000000000000000000000000000000900000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
        let s = SpotMarket::try_deserialize(&mut buf.as_slice()).unwrap();
        dbg!(s);

        let s1: &SpotMarket = bytemuck::from_bytes(&buf[8..std::mem::size_of::<SpotMarket>() + 8]);
        dbg!(s1);

        assert_eq!(&s, s1);
    }
}
