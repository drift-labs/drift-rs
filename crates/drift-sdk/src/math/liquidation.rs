//! liquidation and margin helpers
//!

use std::ops::Neg;

use crate::{
    drift_abi::{
        accounts::{PerpMarket, User},
        types::PerpPosition,
    },
    ffi::{
        self, calculate_margin_requirement_and_total_collateral_and_liability_info, AccountsList,
        IntoFfi, MarginContextMode, MarginRequirementType,
    },
    math::{
        account_map_builder::AccountsListBuilder,
        constants::{
            AMM_RESERVE_PRECISION_I128, BASE_PRECISION_I128, MARGIN_PRECISION,
            QUOTE_PRECISION_I128, QUOTE_PRECISION_I64, SPOT_WEIGHT_PRECISION,
        },
    },
    AccountProvider, DriftClient, SdkError, SdkResult, SpotMarket,
};

/// Info on a positions liquidation price and unrealized PnL
#[derive(Debug)]
pub struct LiquidationAndPnlInfo {
    // PRICE_PRECISION
    pub liquidation_price: i64,
    // PRICE_PRECISION
    pub unrealized_pnl: i128,
}

/// Calculate the liquidation price and unrealized PnL of a user's perp position (given by `market_index`)
pub fn calculate_liquidation_price_and_unrealized_pnl<T: AccountProvider>(
    client: &DriftClient<T>,
    user: &User,
    market_index: u16,
) -> SdkResult<LiquidationAndPnlInfo> {
    let perp_market = client
        .get_perp_market_account(market_index)
        .ok_or(SdkError::InvalidAccount)?;
    let oracle = client
        .get_oracle_price_data_and_slot(&perp_market.amm.oracle)
        .ok_or(SdkError::InvalidAccount)?;
    let position = user
        .ffi()
        .get_perp_position(market_index)
        .map_err(|_| SdkError::NoPosiiton(market_index))?;

    let unrealized_pnl = calculate_unrealized_pnl_inner(&position.into(), oracle.data.price)?;

    // matching spot market e.g. sol-perp => SOL spot
    let mut builder = AccountsListBuilder::default();
    let mut accounts = builder.build(client, user)?;
    let spot_market = client
        .program_data()
        .spot_market_configs()
        .iter()
        .find(|x| x.oracle == perp_market.amm.oracle);
    let liquidation_price = calculate_liquidation_price_inner(
        user,
        &perp_market,
        spot_market,
        oracle.data.price,
        &mut accounts,
    )?;

    Ok(LiquidationAndPnlInfo {
        unrealized_pnl,
        liquidation_price,
    })
}

/// Calculate the unrealized pnl for user perp position, given by `market_index`
pub fn calculate_unrealized_pnl<T: AccountProvider>(
    client: &DriftClient<T>,
    user: &User,
    market_index: u16,
) -> SdkResult<i128> {
    if let Ok(position) = user.ffi().get_perp_position(market_index) {
        let oracle_price = client
            .get_oracle_price_data_and_slot_for_perp_market(market_index)
            .map(|x| x.data.price)
            .unwrap_or(0);
        calculate_unrealized_pnl_inner(&position.into(), oracle_price)
    } else {
        Err(SdkError::NoPosiiton(market_index))
    }
}

pub fn calculate_unrealized_pnl_inner(
    position: &PerpPosition,
    oracle_price: i64,
) -> SdkResult<i128> {
    let base_asset_value = (position.base_asset_amount as i128 * oracle_price.max(0) as i128)
        / AMM_RESERVE_PRECISION_I128;
    let pnl = base_asset_value + position.quote_entry_amount as i128;

    Ok(pnl)
}

/// Calculate the liquidation price of a user's perp position (given by `market_index`)
/// Returns the liquidaton price (PRICE_PRECISION / 1e6)
pub fn calculate_liquidation_price<T: AccountProvider>(
    client: &DriftClient<T>,
    user: &User,
    market_index: u16,
) -> SdkResult<i64> {
    let mut accounts_builder = AccountsListBuilder::default();
    let mut account_maps = accounts_builder.build(client, user)?;
    let perp_market = client
        .get_perp_market_account(market_index)
        .ok_or(SdkError::InvalidAccount)?;
    let oracle = client
        .get_oracle_price_data_and_slot(&perp_market.amm.oracle)
        .ok_or(SdkError::InvalidAccount)?;
    // matching spot market e.g. sol-perp => SOL spot
    let spot_market = client
        .program_data()
        .spot_market_configs()
        .iter()
        .find(|x| x.oracle == perp_market.amm.oracle);

    calculate_liquidation_price_inner(
        user,
        &perp_market,
        spot_market,
        oracle.data.price,
        &mut account_maps,
    )
}

/// Calculate liquidation price of a users perp postion
/// considers all of the users open positions
///
/// - `perp_market` Market info of the perp position
/// - `spot_market` Corresponding spot market (e.g. SOL-perp => SOL spot)
/// - `accounts` collection of all accounts (markets, oracles) to perform margin calculations
///
pub fn calculate_liquidation_price_inner<'a>(
    user: &'a User,
    perp_market: &PerpMarket,
    spot_market: Option<&SpotMarket>,
    oracle_price: i64,
    accounts: &'a mut AccountsList<'a>,
) -> SdkResult<i64> {
    let margin_calculation = calculate_margin_requirement_and_total_collateral_and_liability_info(
        user,
        accounts,
        MarginContextMode::StandardMaintenance,
    )?;

    // calculate perp free collateral delta
    let perp_position = user
        .ffi()
        .get_perp_position(perp_market.market_index)
        .map_err(|_| SdkError::NoPosiiton(perp_market.market_index))?;

    let perp_position_with_lp =
        perp_position.simulate_settled_lp_position(perp_market, oracle_price)?;

    let perp_free_collateral_delta =
        calculate_perp_free_collateral_delta(&perp_position_with_lp, perp_market, oracle_price);

    // user holding spot asset case
    let mut spot_free_collateral_delta = 0;
    if let Some(spot_market) = spot_market {
        if let Ok(spot_position) = user.ffi().get_spot_position(spot_market.market_index) {
            if !spot_position.is_available() {
                spot_free_collateral_delta =
                    calculate_spot_free_collateral_delta(&spot_position, spot_market);
            }
        }
    }

    // calculate liquidation price
    // what price delta causes free collateral == 0
    let free_collateral = margin_calculation.get_free_collateral();
    let free_collateral_delta = perp_free_collateral_delta + spot_free_collateral_delta;
    if free_collateral == 0 {
        return Ok(-1);
    }
    let liquidation_price_delta =
        (free_collateral as i64 * QUOTE_PRECISION_I64) / free_collateral_delta;

    let liquidation_price = (oracle_price - liquidation_price_delta).max(-1);
    Ok(liquidation_price)
}

fn calculate_perp_free_collateral_delta(
    position: &PerpPosition,
    market: &PerpMarket,
    oracle_price: i64,
) -> i64 {
    let current_base_asset_amount = position.base_asset_amount;

    let worst_case_base_amount = position
        .ffi()
        .worst_case_base_asset_amount(oracle_price, market.contract_type.into())
        .unwrap();
    let margin_ratio = market
        .ffi()
        .get_margin_ratio(
            worst_case_base_amount.unsigned_abs(),
            MarginRequirementType::Maintenance,
        )
        .unwrap();
    let margin_ratio = (margin_ratio as i64 * QUOTE_PRECISION_I64) / MARGIN_PRECISION as i64;

    if worst_case_base_amount == 0 {
        return 0;
    }

    let mut fcd = if current_base_asset_amount > 0 {
        ((QUOTE_PRECISION_I64 - margin_ratio) as i128 * current_base_asset_amount as i128)
            / BASE_PRECISION_I128
    } else {
        ((QUOTE_PRECISION_I64.neg() - margin_ratio) as i128
            * current_base_asset_amount.abs() as i128)
            / BASE_PRECISION_I128
    } as i64;

    let order_base_amount = worst_case_base_amount - current_base_asset_amount as i128;
    if order_base_amount != 0 {
        fcd -= ((margin_ratio as i128 * order_base_amount.abs()) / BASE_PRECISION_I128) as i64;
    }

    fcd
}

fn calculate_spot_free_collateral_delta(position: &ffi::SpotPosition, market: &SpotMarket) -> i64 {
    let market_precision = 10_i128.pow(market.decimals);
    let signed_token_amount = position.get_signed_token_amount(market).unwrap();
    let delta = if signed_token_amount > 0 {
        let weight = market
            .ffi()
            .get_asset_weight(
                signed_token_amount.unsigned_abs(),
                0, // unused by Maintenance margin type, hence 0
                MarginRequirementType::Maintenance,
            )
            .unwrap() as i128;
        (((QUOTE_PRECISION_I128 * weight) / SPOT_WEIGHT_PRECISION as i128) * signed_token_amount)
            / market_precision
    } else {
        let weight = market
            .ffi()
            .get_liability_weight(
                signed_token_amount.unsigned_abs(),
                MarginRequirementType::Maintenance,
            )
            .unwrap() as i128;
        (((QUOTE_PRECISION_I128.neg() * weight) / SPOT_WEIGHT_PRECISION as i128)
            * signed_token_amount.abs())
            / market_precision
    };

    delta.try_into().expect("ftis i64")
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MarginRequirementInfo {
    /// initial margin requirement (PRICE_PRECISION)
    pub initial: u128,
    /// maintenance margin requirement (PRICE_PRECISION)
    pub maintenance: u128,
}

/// Calculate the margin requirements of `user`
pub fn calculate_margin_requirements<T: AccountProvider>(
    client: &DriftClient<T>,
    user: &User,
) -> SdkResult<MarginRequirementInfo> {
    calculate_margin_requirements_inner(
        user,
        &mut AccountsListBuilder::default().build(client, user)?,
    )
}

/// Calculate the margin requirements of `user` (internal)
fn calculate_margin_requirements_inner<'a>(
    user: &'a User,
    accounts: &'a mut AccountsList<'a>,
) -> SdkResult<MarginRequirementInfo> {
    let accounts_ref: &'a mut AccountsList<'a> =
        unsafe { std::mem::transmute(&mut accounts.clone()) };
    let maintenance_result = calculate_margin_requirement_and_total_collateral_and_liability_info(
        user,
        accounts_ref,
        MarginContextMode::StandardMaintenance,
    )?;

    let initial_result = calculate_margin_requirement_and_total_collateral_and_liability_info(
        user,
        accounts,
        MarginContextMode::StandardInitial,
    )?;

    Ok(MarginRequirementInfo {
        maintenance: maintenance_result.margin_requirement,
        initial: initial_result.margin_requirement,
    })
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CollateralInfo {
    /// total collateral (QUOTE_PRECISION)
    pub total: i128,
    /// free collateral (QUOTE_PRECISION)
    pub free: i128,
}

pub fn calculate_collateral<T: AccountProvider>(
    client: &DriftClient<T>,
    user: &User,
    margin_requirement_type: MarginRequirementType,
) -> SdkResult<CollateralInfo> {
    let mut accounts_builder = AccountsListBuilder::default();
    calculate_collateral_inner(
        user,
        &mut accounts_builder.build(client, user)?,
        margin_requirement_type,
    )
}

fn calculate_collateral_inner<'a>(
    user: &'a User,
    accounts: &'a mut AccountsList<'a>,
    margin_requirement_type: MarginRequirementType,
) -> SdkResult<CollateralInfo> {
    let result = calculate_margin_requirement_and_total_collateral_and_liability_info(
        user,
        accounts,
        MarginContextMode::StandardCustom(margin_requirement_type),
    )?;

    Ok(CollateralInfo {
        total: result.total_collateral,
        free: result.get_free_collateral() as i128,
    })
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use anchor_lang::{Owner, ZeroCopy};
    use bytes::BytesMut;
    use solana_program::account_info::AccountInfo;
    use solana_sdk::pubkey::Pubkey;

    use super::*;
    use crate::{
        constants::{self, ids::pyth_program},
        drift_abi::types::{HistoricalOracleData, MarketStatus, OracleSource, SpotPosition, AMM},
        math::constants::{
            AMM_RESERVE_PRECISION, BASE_PRECISION_I64, LIQUIDATION_FEE_PRECISION, PEG_PRECISION,
            PRICE_PRECISION_I64, SPOT_BALANCE_PRECISION, SPOT_BALANCE_PRECISION_U64,
            SPOT_CUMULATIVE_INTEREST_PRECISION,
        },
        MarketId, RpcAccountProvider, Wallet,
    };

    const SOL_ORACLE: Pubkey = solana_sdk::pubkey!("J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix");
    const BTC_ORACLE: Pubkey = solana_sdk::pubkey!("GVXRSBjFk6e6J3NbVPXohDJetcTjaeeuykUpbQF8UoMU");

    fn sol_spot_market() -> SpotMarket {
        SpotMarket {
            market_index: 1,
            oracle_source: OracleSource::Pyth,
            oracle: SOL_ORACLE,
            cumulative_deposit_interest: SPOT_CUMULATIVE_INTEREST_PRECISION,
            cumulative_borrow_interest: SPOT_CUMULATIVE_INTEREST_PRECISION,
            decimals: 9,
            initial_asset_weight: 8 * SPOT_WEIGHT_PRECISION / 10,
            maintenance_asset_weight: 9 * SPOT_WEIGHT_PRECISION / 10,
            initial_liability_weight: 12 * SPOT_WEIGHT_PRECISION / 10,
            maintenance_liability_weight: 11 * SPOT_WEIGHT_PRECISION / 10,
            liquidator_fee: LIQUIDATION_FEE_PRECISION / 1000,
            deposit_balance: 1000 * SPOT_BALANCE_PRECISION,
            ..SpotMarket::default()
        }
    }

    fn sol_perp_market() -> PerpMarket {
        PerpMarket {
            amm: AMM {
                base_asset_reserve: 100 * AMM_RESERVE_PRECISION,
                quote_asset_reserve: 100 * AMM_RESERVE_PRECISION,
                bid_base_asset_reserve: 101 * AMM_RESERVE_PRECISION,
                bid_quote_asset_reserve: 99 * AMM_RESERVE_PRECISION,
                ask_base_asset_reserve: 99 * AMM_RESERVE_PRECISION,
                ask_quote_asset_reserve: 101 * AMM_RESERVE_PRECISION,
                sqrt_k: 100 * AMM_RESERVE_PRECISION,
                peg_multiplier: 100 * PEG_PRECISION,
                order_step_size: 10000000,
                oracle: SOL_ORACLE,
                ..AMM::default()
            },
            market_index: 0,
            margin_ratio_initial: 1000,
            margin_ratio_maintenance: 500,
            unrealized_pnl_maintenance_asset_weight: SPOT_WEIGHT_PRECISION,
            status: MarketStatus::Initialized,
            ..PerpMarket::default()
        }
    }

    fn btc_perp_market() -> PerpMarket {
        PerpMarket {
            amm: AMM {
                base_asset_reserve: 100 * AMM_RESERVE_PRECISION,
                quote_asset_reserve: 100 * AMM_RESERVE_PRECISION,
                bid_base_asset_reserve: 101 * AMM_RESERVE_PRECISION,
                bid_quote_asset_reserve: 99 * AMM_RESERVE_PRECISION,
                ask_base_asset_reserve: 99 * AMM_RESERVE_PRECISION,
                ask_quote_asset_reserve: 101 * AMM_RESERVE_PRECISION,
                sqrt_k: 100 * AMM_RESERVE_PRECISION,
                oracle: BTC_ORACLE,
                ..AMM::default()
            },
            market_index: 1,
            margin_ratio_initial: 1000,
            margin_ratio_maintenance: 500,
            imf_factor: 1000, // 1_000/1_000_000 = .001
            unrealized_pnl_initial_asset_weight: SPOT_WEIGHT_PRECISION,
            unrealized_pnl_maintenance_asset_weight: SPOT_WEIGHT_PRECISION,
            status: MarketStatus::Initialized,
            ..PerpMarket::default()
        }
    }

    fn usdc_spot_market() -> SpotMarket {
        SpotMarket {
            market_index: 0,
            oracle_source: OracleSource::QuoteAsset,
            cumulative_deposit_interest: SPOT_CUMULATIVE_INTEREST_PRECISION,
            decimals: 6,
            initial_asset_weight: SPOT_WEIGHT_PRECISION,
            maintenance_asset_weight: SPOT_WEIGHT_PRECISION,
            deposit_balance: 100_000 * SPOT_BALANCE_PRECISION,
            liquidator_fee: 0,
            historical_oracle_data: HistoricalOracleData {
                last_oracle_price: PRICE_PRECISION_I64,
                last_oracle_conf: 0,
                last_oracle_delay: 0,
                last_oracle_price_twap: PRICE_PRECISION_I64,
                last_oracle_price_twap5min: PRICE_PRECISION_I64,
                ..HistoricalOracleData::default()
            },
            ..SpotMarket::default()
        }
    }

    #[cfg(feature = "rpc_tests")]
    #[tokio::test]
    async fn calculate_liq_price() {
        let wallet = Wallet::read_only(
            Pubkey::from_str("DxoRJ4f5XRMvXU9SGuM4ZziBFUxbhB3ubur5sVZEvue2").unwrap(),
        );
        let client = DriftClient::new(
            crate::Context::MainNet,
            RpcAccountProvider::new("https://api.mainnet-beta.solana.com"),
            wallet.clone(),
        )
        .await
        .unwrap();
        assert!(client.subscribe().await.is_ok());
        let user = client
            .get_user_account(&wallet.sub_account(0))
            .await
            .unwrap();

        dbg!(calculate_liquidation_price_and_unrealized_pnl(&client, &user, 4).unwrap());
    }

    #[cfg(feature = "rpc_tests")]
    #[tokio::test]
    async fn calculate_margin_requirements_works() {
        let wallet = Wallet::read_only(
            Pubkey::from_str("DxoRJ4f5XRMvXU9SGuM4ZziBFUxbhB3ubur5sVZEvue2").unwrap(),
        );
        let client = DriftClient::new(
            crate::Context::MainNet,
            RpcAccountProvider::new("https://api.mainnet-beta.solana.com"),
            wallet.clone(),
        )
        .await
        .unwrap();
        client.subscribe().await.unwrap();
        let user = client
            .get_user_account(&wallet.sub_account(0))
            .await
            .unwrap();

        dbg!(calculate_margin_requirements(&client, &user).await.unwrap());
    }

    #[test]
    fn calculate_margin_requirements_works() {
        let sol_perp_index = 0;
        let btc_perp_index = 1;
        let mut user = User::default();
        user.perp_positions[0] = PerpPosition {
            market_index: sol_perp_index,
            base_asset_amount: -2 * BASE_PRECISION_I64,
            ..Default::default()
        };
        user.perp_positions[1] = PerpPosition {
            market_index: btc_perp_index,
            base_asset_amount: (1 * BASE_PRECISION_I64) / 20,
            ..Default::default()
        };
        user.spot_positions[0] = SpotPosition {
            market_index: MarketId::QUOTE_SPOT.index,
            scaled_balance: 1_000 * SPOT_BALANCE_PRECISION_U64,
            ..Default::default()
        };

        let mut sol_oracle_price = get_pyth_price(100, 6);
        crate::create_account_info!(sol_oracle_price, &SOL_ORACLE, &pyth_program::ID, sol_oracle);
        crate::create_anchor_account_info!(
            sol_perp_market(),
            &constants::PROGRAM_ID,
            PerpMarket,
            sol_perp
        );
        let mut btc_oracle_price = get_pyth_price(50_000, 6);
        crate::create_account_info!(btc_oracle_price, &BTC_ORACLE, &pyth_program::ID, btc_oracle);
        crate::create_anchor_account_info!(
            usdc_spot_market(),
            &constants::PROGRAM_ID,
            SpotMarket,
            usdc_spot
        );
        crate::create_anchor_account_info!(
            btc_perp_market(),
            &constants::PROGRAM_ID,
            PerpMarket,
            btc_perp
        );

        let mut accounts_map = AccountsList::new(
            &mut [sol_perp, btc_perp],
            &mut [usdc_spot],
            &mut [sol_oracle, btc_oracle],
        );

        let margin_info = calculate_margin_requirements_inner(&user, &mut accounts_map).unwrap();
        dbg!(margin_info);

        assert_eq!(
            MarginRequirementInfo {
                initial: 27_000_0000,
                maintenance: 135_000_000
            },
            margin_info
        );
    }

    #[test]
    fn liquidation_price_short() {
        let sol_perp_index = 0;
        let mut user = User::default();
        user.perp_positions[0] = PerpPosition {
            market_index: sol_perp_index,
            base_asset_amount: -2 * BASE_PRECISION_I64,
            ..Default::default()
        };
        user.spot_positions[0] = SpotPosition {
            market_index: MarketId::QUOTE_SPOT.index,
            scaled_balance: 250_u64 * SPOT_BALANCE_PRECISION_U64,
            ..Default::default()
        };

        let sol_usdc_price = 100;

        let mut sol_oracle_price = get_pyth_price(sol_usdc_price, 6);
        crate::create_account_info!(sol_oracle_price, &SOL_ORACLE, &pyth_program::ID, sol_oracle);
        crate::create_anchor_account_info!(
            usdc_spot_market(),
            &constants::PROGRAM_ID,
            SpotMarket,
            usdc_spot
        );
        crate::create_anchor_account_info!(
            sol_perp_market(),
            &constants::PROGRAM_ID,
            PerpMarket,
            sol_perp
        );
        let mut accounts_map =
            AccountsList::new(&mut [sol_perp], &mut [usdc_spot], &mut [sol_oracle]);

        let liquidation_price = calculate_liquidation_price_inner(
            &user,
            &sol_perp_market(),
            Some(&sol_spot_market()),
            sol_usdc_price,
            &mut accounts_map,
        )
        .unwrap();

        dbg!(liquidation_price);
        assert_eq!(liquidation_price, 119_047_619);
    }

    #[test]
    fn liquidation_price_long() {
        let mut user = User::default();
        user.perp_positions[0] = PerpPosition {
            market_index: sol_perp_market().market_index,
            base_asset_amount: 5 * BASE_PRECISION_I64,
            quote_asset_amount: -5 * (100 * QUOTE_PRECISION_I64),
            ..Default::default()
        };
        user.spot_positions[0] = SpotPosition {
            market_index: MarketId::QUOTE_SPOT.index,
            scaled_balance: 250_u64 * SPOT_BALANCE_PRECISION_U64,
            ..Default::default()
        };
        let sol_usdc_price = 100;
        let mut sol_oracle_price = get_pyth_price(sol_usdc_price, 6);
        crate::create_account_info!(sol_oracle_price, &SOL_ORACLE, &pyth_program::ID, sol_oracle);
        crate::create_anchor_account_info!(
            usdc_spot_market(),
            &constants::PROGRAM_ID,
            SpotMarket,
            usdc_spot
        );
        crate::create_anchor_account_info!(
            sol_perp_market(),
            &constants::PROGRAM_ID,
            PerpMarket,
            sol_perp
        );

        let mut accounts_map =
            AccountsList::new(&mut [sol_perp], &mut [usdc_spot], &mut [sol_oracle]);
        let liquidation_price = calculate_liquidation_price_inner(
            &user,
            &sol_perp_market(),
            Some(&sol_spot_market()),
            sol_usdc_price,
            &mut accounts_map,
        )
        .unwrap();

        dbg!(liquidation_price);
        assert_eq!(liquidation_price, 52_631_579);
    }

    #[test]
    fn liquidation_price_short_with_spot_balance() {
        let mut user = User::default();
        user.perp_positions[0] = PerpPosition {
            market_index: btc_perp_market().market_index,
            base_asset_amount: -250_000_000, // 0.25btc
            ..Default::default()
        };
        user.spot_positions[0] = SpotPosition {
            market_index: 1,
            scaled_balance: 200 * SPOT_BALANCE_PRECISION_U64,
            ..Default::default()
        };
        let sol_usdc_price = 100;
        let mut sol_oracle_price = get_pyth_price(sol_usdc_price, 6);
        crate::create_account_info!(sol_oracle_price, &SOL_ORACLE, &pyth_program::ID, sol_oracle);

        let btc_usdc_price = 40_000;
        let mut btc_oracle_price = get_pyth_price(btc_usdc_price, 6);
        crate::create_account_info!(btc_oracle_price, &BTC_ORACLE, &pyth_program::ID, btc_oracle);
        crate::create_anchor_account_info!(
            usdc_spot_market(),
            &constants::PROGRAM_ID,
            SpotMarket,
            usdc_spot
        );
        crate::create_anchor_account_info!(
            sol_spot_market(),
            &constants::PROGRAM_ID,
            SpotMarket,
            sol_spot
        );
        crate::create_anchor_account_info!(
            btc_perp_market(),
            &constants::PROGRAM_ID,
            PerpMarket,
            btc_perp
        );
        let mut accounts_map = AccountsList::new(
            &mut [btc_perp],
            &mut [usdc_spot, sol_spot],
            &mut [sol_oracle, btc_oracle],
        );
        let liquidation_price = calculate_liquidation_price_inner(
            &user,
            &btc_perp_market(),
            None,
            btc_usdc_price,
            &mut accounts_map,
        )
        .unwrap();
        assert_eq!(liquidation_price, 68_571_428_571);
    }

    #[test]
    fn liquidation_price_long_with_spot_balance() {
        let sol_usdc_price = 100;
        let mut user = User::default();
        user.perp_positions[0] = PerpPosition {
            market_index: sol_perp_market().market_index,
            base_asset_amount: 5 * BASE_PRECISION_I64,
            quote_asset_amount: -5 * (100 * QUOTE_PRECISION_I64),
            ..Default::default()
        };
        user.spot_positions[0] = SpotPosition {
            market_index: 1,
            scaled_balance: 2 * SPOT_BALANCE_PRECISION_U64,
            ..Default::default()
        };

        let mut sol_oracle_price = get_pyth_price(sol_usdc_price, 6);
        crate::create_account_info!(sol_oracle_price, &SOL_ORACLE, &pyth_program::ID, sol_oracle);
        crate::create_anchor_account_info!(
            usdc_spot_market(),
            &constants::PROGRAM_ID,
            SpotMarket,
            usdc_spot
        );
        crate::create_anchor_account_info!(
            sol_spot_market(),
            &constants::PROGRAM_ID,
            SpotMarket,
            sol_spot
        );
        crate::create_anchor_account_info!(
            sol_perp_market(),
            &constants::PROGRAM_ID,
            PerpMarket,
            sol_perp
        );
        let mut accounts_map = AccountsList::new(
            &mut [sol_perp],
            &mut [usdc_spot, sol_spot],
            &mut [sol_oracle],
        );

        let liquidation_price = calculate_liquidation_price_inner(
            &user,
            &sol_perp_market(),
            Some(&sol_spot_market()),
            sol_usdc_price,
            &mut accounts_map,
        )
        .unwrap();
        dbg!(liquidation_price);
        assert_eq!(liquidation_price, 76_335_878);
    }

    #[test]
    fn liquidation_price_no_positions() {
        let user = User::default();
        let mut accounts_map = AccountsList::new(&mut [], &mut [], &mut []);
        assert!(calculate_liquidation_price_inner(
            &user,
            &sol_perp_market(),
            None,
            100,
            &mut accounts_map
        )
        .is_err());
    }

    #[test]
    fn unrealized_pnl_short() {
        let position = PerpPosition {
            market_index: sol_perp_market().market_index,
            base_asset_amount: -1 * BASE_PRECISION_I64,
            quote_entry_amount: 80 * QUOTE_PRECISION_I64,
            ..Default::default()
        };
        let sol_usdc_price = 60 * QUOTE_PRECISION_I64;

        let unrealized_pnl = calculate_unrealized_pnl_inner(&position, sol_usdc_price).unwrap();

        dbg!(unrealized_pnl);
        // entry at $80, upnl at $60
        assert_eq!(unrealized_pnl, 20_i128 * QUOTE_PRECISION_I64 as i128);
    }

    #[test]
    fn liquidation_price_hedged_short() {
        let mut user = User::default();
        user.perp_positions[0] = PerpPosition {
            market_index: sol_perp_market().market_index,
            base_asset_amount: -10 * BASE_PRECISION_I64,
            quote_entry_amount: 80 * QUOTE_PRECISION_I64,
            ..Default::default()
        };
        user.spot_positions[0] = SpotPosition {
            market_index: sol_spot_market().market_index,
            scaled_balance: 10 * SPOT_BALANCE_PRECISION as u64,
            ..Default::default()
        };
        let sol_usdc_price = 60;
        let mut sol_oracle_price = get_pyth_price(sol_usdc_price, 6);

        crate::create_account_info!(sol_oracle_price, &SOL_ORACLE, &pyth_program::ID, sol_oracle);
        crate::create_anchor_account_info!(
            usdc_spot_market(),
            &constants::PROGRAM_ID,
            SpotMarket,
            usdc_spot
        );
        crate::create_anchor_account_info!(
            sol_perp_market(),
            &constants::PROGRAM_ID,
            PerpMarket,
            sol_perp
        );
        crate::create_anchor_account_info!(
            sol_spot_market(),
            &constants::PROGRAM_ID,
            SpotMarket,
            sol_spot
        );
        let mut accounts_map = AccountsList::new(
            &mut [sol_perp],
            &mut [usdc_spot, sol_spot],
            &mut [sol_oracle],
        );

        let liq_price = calculate_liquidation_price_inner(
            &user,
            &sol_perp_market(),
            Some(&sol_spot_market()),
            sol_oracle_price.agg.price,
            &mut accounts_map,
        )
        .unwrap();
        dbg!(liq_price);

        // price down but fully hedged
        assert_eq!(liq_price, -1);
    }

    #[test]
    fn unrealized_pnl_long() {
        let position = PerpPosition {
            market_index: sol_perp_market().market_index,
            base_asset_amount: 1 * BASE_PRECISION_I64,
            quote_entry_amount: -80 * QUOTE_PRECISION_I64,
            ..Default::default()
        };
        let sol_usdc_price = 100 * QUOTE_PRECISION_I64;

        let unrealized_pnl = calculate_unrealized_pnl_inner(&position, sol_usdc_price).unwrap();

        dbg!(unrealized_pnl);
        // entry at $80, upnl at $100
        assert_eq!(unrealized_pnl, 20_i128 * QUOTE_PRECISION_I64 as i128);
    }

    // helpers from drift-program test_utils.
    fn get_pyth_price(price: i64, expo: i32) -> pyth_test::Price {
        let mut pyth_price = pyth_test::Price::default();
        let price = price * 10_i64.pow(expo as u32);
        pyth_price.agg.price = price;
        pyth_price.twap = price;
        pyth_price.expo = expo;
        pyth_price
    }

    pub fn get_account_bytes<T: bytemuck::Pod>(account: &mut T) -> BytesMut {
        let mut bytes = BytesMut::new();
        let data = bytemuck::bytes_of_mut(account);
        bytes.extend_from_slice(data);
        bytes
    }

    pub fn get_anchor_account_bytes<T: ZeroCopy>(account: &mut T) -> BytesMut {
        let mut bytes = BytesMut::new();
        bytes.extend_from_slice(&T::discriminator());
        let data = bytemuck::bytes_of_mut(account);
        bytes.extend_from_slice(data);
        bytes
    }

    mod pyth_test {
        //! helper structs for pyth oracle prices
        use bytemuck::{Pod, Zeroable};

        #[derive(Default, Copy, Clone)]
        #[repr(C)]
        pub struct AccKey {
            pub val: [u8; 32],
        }

        #[derive(Copy, Clone, Default)]
        #[repr(C)]
        #[allow(dead_code)]
        pub enum PriceStatus {
            Unknown,
            #[default]
            Trading,
            Halted,
            Auction,
        }

        #[derive(Copy, Clone, Default)]
        #[repr(C)]
        pub enum CorpAction {
            #[default]
            NoCorpAct,
        }

        #[derive(Default, Copy, Clone)]
        #[repr(C)]
        pub struct PriceInfo {
            pub price: i64,
            pub conf: u64,
            pub status: PriceStatus,
            pub corp_act: CorpAction,
            pub pub_slot: u64,
        }
        #[derive(Default, Copy, Clone)]
        #[repr(C)]
        pub struct PriceComp {
            publisher: AccKey,
            agg: PriceInfo,
            latest: PriceInfo,
        }

        #[derive(Copy, Clone, Default)]
        #[repr(C)]
        #[allow(dead_code, clippy::upper_case_acronyms)]
        pub enum PriceType {
            Unknown,
            #[default]
            Price,
            TWAP,
            Volatility,
        }

        #[derive(Default, Copy, Clone)]
        #[repr(C)]
        pub struct Price {
            pub magic: u32,       // Pyth magic number.
            pub ver: u32,         // Program version.
            pub atype: u32,       // Account type.
            pub size: u32,        // Price account size.
            pub ptype: PriceType, // Price or calculation type.
            pub expo: i32,        // Price exponent.
            pub num: u32,         // Number of component prices.
            pub unused: u32,
            pub curr_slot: u64,        // Currently accumulating price slot.
            pub valid_slot: u64,       // Valid slot-time of agg. price.
            pub twap: i64,             // Time-weighted average price.
            pub avol: u64,             // Annualized price volatility.
            pub drv0: i64,             // Space for future derived values.
            pub drv1: i64,             // Space for future derived values.
            pub drv2: i64,             // Space for future derived values.
            pub drv3: i64,             // Space for future derived values.
            pub drv4: i64,             // Space for future derived values.
            pub drv5: i64,             // Space for future derived values.
            pub prod: AccKey,          // Product account key.
            pub next: AccKey,          // Next Price account in linked list.
            pub agg_pub: AccKey,       // Quoter who computed last aggregate price.
            pub agg: PriceInfo,        // Aggregate price info.
            pub comp: [PriceComp; 32], // Price components one per quoter.
        }

        #[cfg(target_endian = "little")]
        unsafe impl Zeroable for Price {}

        #[cfg(target_endian = "little")]
        unsafe impl Pod for Price {}
    }

    #[macro_export]
    macro_rules! create_account_info {
        ($account:expr, $owner:expr, $name: ident) => {
            let key = Pubkey::default();
            let mut lamports = 0;
            let mut data = get_account_bytes(&mut $account);
            let owner = $type::owner();
            let $name = AccountInfo::new(
                &key,
                true,
                false,
                &mut lamports,
                &mut data[..],
                $owner,
                false,
                0,
            );
        };
        ($account:expr, $pubkey:expr, $owner:expr, $name: ident) => {
            let mut lamports = 0;
            let mut data = get_account_bytes(&mut $account);
            let $name = AccountInfo::new(
                $pubkey,
                true,
                false,
                &mut lamports,
                &mut data[..],
                $owner,
                false,
                0,
            );
        };
    }

    #[macro_export]
    macro_rules! create_anchor_account_info {
        ($account:expr, $type:ident, $name: ident) => {
            let key = Pubkey::default();
            let mut lamports = 0;
            let mut data = get_anchor_account_bytes(&mut $account);
            let owner = $type::owner();
            let $name = AccountInfo::new(
                &key,
                true,
                false,
                &mut lamports,
                &mut data[..],
                &owner,
                false,
                0,
            );
        };
        ($account:expr, $pubkey:expr, $type:ident, $name: ident) => {
            let mut lamports = 0;
            let mut data = get_anchor_account_bytes(&mut $account);
            let owner = constants::PROGRAM_ID;
            let $name = AccountInfo::new(
                $pubkey,
                true,
                false,
                &mut lamports,
                &mut data[..],
                &owner,
                false,
                0,
            );
        };
    }
}
