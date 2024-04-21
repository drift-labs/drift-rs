//! liquidation and margin helpers
//!

use std::ops::Neg;

use drift::{
    instructions::optional_accounts::AccountMaps,
    math::{
        constants::{
            AMM_RESERVE_PRECISION_I128, BASE_PRECISION_I128, MARGIN_PRECISION,
            QUOTE_PRECISION_I128, QUOTE_PRECISION_I64, SPOT_WEIGHT_PRECISION,
        },
        margin::{
            calculate_margin_requirement_and_total_collateral_and_liability_info,
            MarginRequirementType,
        },
    },
    state::{
        margin_calculation::MarginContext,
        perp_market::PerpMarket,
        spot_market::SpotMarket,
        user::{MarketType, PerpPosition, SpotPosition, User},
    },
};

use crate::{
    math::account_map_builder::AccountMapBuilder, AccountProvider, DriftClient, SdkError, SdkResult,
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
    // TODO: this does a decent amount of rpc queries, it could make sense to cache it e.g. for calculating multiple perp positions
    let mut accounts_builder = AccountMapBuilder::default();
    let mut account_maps = accounts_builder.build(client, user)?;
    let position = user
        .get_perp_position(market_index)
        .map_err(|_| SdkError::NoPosiiton(market_index))?;
    let unrealized_pnl = calculate_unrealized_pnl_inner(position, market_index, &mut account_maps)?;
    let liquidation_price =
        calculate_liquidation_price_inner(user, market_index, &mut account_maps)?;

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
    if let Ok(position) = user.get_perp_position(market_index) {
        let mut accounts_builder = AccountMapBuilder::default();
        let mut account_maps = accounts_builder.build(client, user)?;
        calculate_unrealized_pnl_inner(position, market_index, &mut account_maps)
    } else {
        Err(SdkError::NoPosiiton(market_index))
    }
}

pub fn calculate_unrealized_pnl_inner(
    position: &PerpPosition,
    market_index: u16,
    account_maps: &mut AccountMaps<'_>,
) -> SdkResult<i128> {
    let AccountMaps {
        perp_market_map,
        ref mut oracle_map,
        ..
    } = account_maps;

    let perp_market = perp_market_map
        .get_ref(&market_index)
        .map_err(|_| SdkError::InvalidAccount)?;

    let oracle = oracle_map
        .get_price_data(&perp_market.amm.oracle)
        .map_err(|_| SdkError::InvalidOracle)?;

    let base_asset_value = (position.base_asset_amount as i128 * oracle.price.max(0) as i128)
        / AMM_RESERVE_PRECISION_I128;
    let pnl = base_asset_value + position.quote_entry_amount as i128;

    Ok(pnl)
}

/// Calculate the liquidation price of a user's perp position (given by `market_index`)
///
/// Returns the liquidaton price (PRICE_PRECISION / 1e6)
pub async fn calculate_liquidation_price<'a, T: AccountProvider>(
    client: &DriftClient<T>,
    user: &User,
    market_index: u16,
) -> SdkResult<i64> {
    // TODO: this does a decent amount of rpc queries, it could make sense to cache it e.g. for calculating multiple perp positions
    let mut accounts_builder = AccountMapBuilder::default();
    let mut account_maps = accounts_builder.build(client, user)?;
    calculate_liquidation_price_inner(user, market_index, &mut account_maps)
}

pub fn calculate_liquidation_price_inner(
    user: &User,
    market_index: u16,
    account_maps: &mut AccountMaps<'_>,
) -> SdkResult<i64> {
    let AccountMaps {
        perp_market_map,
        spot_market_map,
        ref mut oracle_map,
    } = account_maps;

    let margin_calculation = calculate_margin_requirement_and_total_collateral_and_liability_info(
        user,
        perp_market_map,
        spot_market_map,
        oracle_map,
        MarginContext::standard(MarginRequirementType::Maintenance),
    )
    .map_err(|err| SdkError::Anchor(Box::new(err.into())))?;

    // calculate perp free collateral delta
    let perp_market = perp_market_map
        .get_ref(&market_index)
        .map_err(|err| SdkError::Anchor(Box::new(err.into())))?;

    let max_confidence_interval_multiplier =
        perp_market
            .get_max_confidence_interval_multiplier()
            .map_err(|err| SdkError::Anchor(Box::new(err.into())))?;

    let (oracle_price_data, _oracle_validity) = oracle_map
        .get_price_data_and_validity(
            MarketType::Perp,
            perp_market.market_index,
            &perp_market.amm.oracle,
            perp_market
                .amm
                .historical_oracle_data
                .last_oracle_price_twap,
            max_confidence_interval_multiplier,
        )
        .map_err(|err| SdkError::Anchor(Box::new(err.into())))?;

    let perp_position = user
        .get_perp_position(market_index)
        .map_err(|_| SdkError::NoPosiiton(market_index))?;

    let perp_position_with_lp = perp_position
        .simulate_settled_lp_position(&perp_market, oracle_price_data.price)
        .map_err(|err| SdkError::Anchor(Box::new(err.into())))?;

    let perp_free_collateral_delta =
        calculate_perp_free_collateral_delta(&perp_position_with_lp, &perp_market);

    // user holding spot asset case
    let mut spot_free_collateral_delta = 0;
    if let Some((spot_market_index, _)) = spot_market_map
        .0
        .iter()
        .find(|x| x.1.load().is_ok_and(|x| x.oracle == perp_market.amm.oracle))
    {
        if let Ok(spot_position) = user.get_spot_position(*spot_market_index) {
            if !spot_position.is_available() {
                let market = spot_market_map.get_ref(spot_market_index).unwrap();
                spot_free_collateral_delta =
                    calculate_spot_free_collateral_delta(spot_position, &market);
            }
        }
    }

    // calculate liquidation price
    // what price delta causes free collateral == 0
    let free_collateral = margin_calculation.get_free_collateral().unwrap();
    let free_collateral_delta = perp_free_collateral_delta + spot_free_collateral_delta;
    if free_collateral == 0 {
        return Ok(-1);
    }
    let liquidation_price_delta =
        (free_collateral as i64 * QUOTE_PRECISION_I64) / free_collateral_delta;

    let oracle_price_data = *oracle_map.get_price_data(&perp_market.amm.oracle).unwrap();
    let liquidation_price = oracle_price_data.price - liquidation_price_delta;
    if liquidation_price < 0 {
        Ok(-1)
    } else {
        Ok(liquidation_price)
    }
}

fn calculate_perp_free_collateral_delta(position: &PerpPosition, market: &PerpMarket) -> i64 {
    let current_base_asset_amount = position.base_asset_amount;

    let worst_case_base_amount = position.worst_case_base_asset_amount().unwrap();
    let margin_ratio = market
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

fn calculate_spot_free_collateral_delta(position: &SpotPosition, market: &SpotMarket) -> i64 {
    let market_precision = 10_i128.pow(market.decimals);
    let signed_token_amount = position.get_signed_token_amount(market).unwrap();
    let delta = if signed_token_amount > 0 {
        let weight = market
            .get_asset_weight(
                signed_token_amount.unsigned_abs(),
                0, // unused by Maintenance margin type, hence 0
                &MarginRequirementType::Maintenance,
            )
            .unwrap() as i128;
        (((QUOTE_PRECISION_I128 * weight) / SPOT_WEIGHT_PRECISION as i128) * signed_token_amount)
            / market_precision
    } else {
        let weight = market
            .get_liability_weight(
                signed_token_amount.unsigned_abs(),
                &MarginRequirementType::Maintenance,
            )
            .unwrap() as i128;
        (((QUOTE_PRECISION_I128.neg() * weight) / SPOT_WEIGHT_PRECISION as i128)
            * signed_token_amount.abs())
            / market_precision
    };

    delta.try_into().expect("ftis i64")
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use anchor_lang::prelude::AccountInfo;
    use anchor_lang::{Owner, ZeroCopy};
    use bytes::BytesMut;
    use drift::ids::pyth_program;
    use drift::state::oracle_map::OracleMap;
    use drift::state::perp_market_map::{MarketSet, PerpMarketMap};
    use drift::state::spot_market_map::SpotMarketMap;
    use drift::{
        math::constants::{
            AMM_RESERVE_PRECISION, BASE_PRECISION_I64, LIQUIDATION_FEE_PRECISION, PEG_PRECISION,
            SPOT_BALANCE_PRECISION, SPOT_BALANCE_PRECISION_U64, SPOT_CUMULATIVE_INTEREST_PRECISION,
        },
        state::{
            oracle::{HistoricalOracleData, OracleSource},
            perp_market::{MarketStatus, AMM},
            user::SpotPosition,
        },
    };
    use pyth::pc;
    use solana_sdk::pubkey::Pubkey;

    use super::*;
    use crate::{constants, MarketId, RpcAccountProvider, Wallet};

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
            historical_oracle_data: HistoricalOracleData::default_quote_oracle(),
            ..SpotMarket::default()
        }
    }

    #[ignore]
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
        let user = client
            .get_user_account(&wallet.sub_account(2))
            .await
            .unwrap();

        dbg!(calculate_liquidation_price_and_unrealized_pnl(&client, &user, 24).unwrap());
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

        let mut sol_oracle_price = get_pyth_price(100, 6);
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
            build_account_map(&mut [sol_perp], &mut [usdc_spot], &mut [sol_oracle]);

        let liquidation_price =
            calculate_liquidation_price_inner(&user, sol_perp_index, &mut accounts_map).unwrap();
        dbg!(liquidation_price);
        assert_eq!(liquidation_price, 119_047_619);
    }

    #[test]
    fn liquidation_price_long() {
        let sol_perp_index = 0;
        let mut user = User::default();
        user.perp_positions[0] = PerpPosition {
            market_index: sol_perp_index,
            base_asset_amount: 5 * BASE_PRECISION_I64,
            quote_asset_amount: -5 * (100 * QUOTE_PRECISION_I64),
            ..Default::default()
        };
        user.spot_positions[0] = SpotPosition {
            market_index: MarketId::QUOTE_SPOT.index,
            scaled_balance: 250_u64 * SPOT_BALANCE_PRECISION_U64,
            ..Default::default()
        };
        let mut sol_oracle_price = get_pyth_price(100, 6);
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
            build_account_map(&mut [sol_perp], &mut [usdc_spot], &mut [sol_oracle]);
        let liquidation_price =
            calculate_liquidation_price_inner(&user, sol_perp_index, &mut accounts_map).unwrap();
        dbg!(liquidation_price);
        assert_eq!(liquidation_price, 52_631_579);
    }

    #[test]
    fn liquidation_price_short_with_spot_balance() {
        let btc_perp_index = 1;
        let mut user = User::default();
        user.perp_positions[0] = PerpPosition {
            market_index: btc_perp_index,
            base_asset_amount: -250_000_000, // 0.25btc
            ..Default::default()
        };
        user.spot_positions[0] = SpotPosition {
            market_index: 1,
            scaled_balance: 200 * SPOT_BALANCE_PRECISION_U64,
            ..Default::default()
        };
        let mut sol_oracle_price = get_pyth_price(100, 6);
        crate::create_account_info!(sol_oracle_price, &SOL_ORACLE, &pyth_program::ID, sol_oracle);
        let mut btc_oracle_price = get_pyth_price(40_000, 6);
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
        let mut accounts_map = build_account_map(
            &mut [btc_perp],
            &mut [usdc_spot, sol_spot],
            &mut [sol_oracle, btc_oracle],
        );
        let liquidation_price =
            calculate_liquidation_price_inner(&user, btc_perp_index, &mut accounts_map).unwrap();
        assert_eq!(liquidation_price, 68_571_428_571);
    }

    #[test]
    fn liquidation_price_long_with_spot_balance() {
        let sol_perp_index = 0;
        let mut user = User::default();
        user.perp_positions[0] = PerpPosition {
            market_index: sol_perp_index,
            base_asset_amount: 5 * BASE_PRECISION_I64,
            quote_asset_amount: -5 * (100 * QUOTE_PRECISION_I64),
            ..Default::default()
        };
        user.spot_positions[0] = SpotPosition {
            market_index: 1,
            scaled_balance: 2 * SPOT_BALANCE_PRECISION_U64,
            ..Default::default()
        };
        let mut sol_oracle_price = get_pyth_price(100, 6);
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
        let mut accounts_map = build_account_map(
            &mut [sol_perp],
            &mut [usdc_spot, sol_spot],
            &mut [sol_oracle],
        );
        let liquidation_price =
            calculate_liquidation_price_inner(&user, sol_perp_index, &mut accounts_map).unwrap();
        dbg!(liquidation_price);
        assert_eq!(liquidation_price, 76_335_878);
    }

    #[test]
    fn liquidation_price_no_positions() {
        let user = User::default();
        let mut accounts_map = build_account_map(&mut [], &mut [], &mut []);
        assert!(calculate_liquidation_price_inner(&user, 0, &mut accounts_map).is_err());
    }

    #[test]
    fn unrealized_pnl_short() {
        let sol_perp_index = 0;
        let position = PerpPosition {
            market_index: sol_perp_index,
            base_asset_amount: -1 * BASE_PRECISION_I64,
            quote_entry_amount: 80 * QUOTE_PRECISION_I64,
            ..Default::default()
        };
        let mut sol_oracle_price = get_pyth_price(60, 6);

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
            build_account_map(&mut [sol_perp], &mut [usdc_spot], &mut [sol_oracle]);
        let unrealized_pnl =
            calculate_unrealized_pnl_inner(&position, sol_perp_index, &mut accounts_map).unwrap();
        dbg!(unrealized_pnl);
        // entry at $80, upnl at $60
        assert_eq!(unrealized_pnl, 20_i128 * QUOTE_PRECISION_I64 as i128);
    }

    #[test]
    fn liquidation_price_hedged_short() {
        let sol_perp_index = 0;
        let sol_spot_index = 1;
        let mut user = User::default();
        user.perp_positions[0] = PerpPosition {
            market_index: sol_perp_index,
            base_asset_amount: -10 * BASE_PRECISION_I64,
            quote_entry_amount: 80 * QUOTE_PRECISION_I64,
            ..Default::default()
        };
        user.spot_positions[0] = SpotPosition {
            market_index: sol_spot_index,
            scaled_balance: 10 * SPOT_BALANCE_PRECISION as u64,
            ..Default::default()
        };
        let mut sol_oracle_price = get_pyth_price(60, 6);

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
        let mut accounts_map = build_account_map(
            &mut [sol_perp],
            &mut [usdc_spot, sol_spot],
            &mut [sol_oracle],
        );
        let liq_price =
            calculate_liquidation_price_inner(&user, sol_perp_index, &mut accounts_map).unwrap();
        dbg!(liq_price);
        // price down but fully hedged
        assert_eq!(liq_price, -1);
    }

    #[test]
    fn unrealized_pnl_long() {
        let sol_perp_index = 0;
        let position = PerpPosition {
            market_index: sol_perp_index,
            base_asset_amount: 1 * BASE_PRECISION_I64,
            quote_entry_amount: -80 * QUOTE_PRECISION_I64,
            ..Default::default()
        };
        let mut sol_oracle_price = get_pyth_price(100, 6);

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
            build_account_map(&mut [sol_perp], &mut [usdc_spot], &mut [sol_oracle]);
        let unrealized_pnl =
            calculate_unrealized_pnl_inner(&position, sol_perp_index, &mut accounts_map).unwrap();

        dbg!(unrealized_pnl);
        // entry at $80, upnl at $100
        assert_eq!(unrealized_pnl, 20_i128 * QUOTE_PRECISION_I64 as i128);
    }

    fn build_account_map<'a>(
        perp: &mut [AccountInfo<'a>],
        spot: &mut [AccountInfo<'a>],
        oracle: &mut [AccountInfo<'a>],
    ) -> AccountMaps<'a> {
        AccountMaps {
            perp_market_map: PerpMarketMap::load(
                &MarketSet::default(),
                &mut perp.iter().peekable(),
            )
            .unwrap(),
            spot_market_map: SpotMarketMap::load(
                &MarketSet::default(),
                &mut spot.iter().peekable(),
            )
            .unwrap(),
            oracle_map: OracleMap::load(&mut oracle.iter().peekable(), 0, None).unwrap(),
        }
    }

    // helpers from drift-program test_utils.
    // TODO: re-export from there
    fn get_pyth_price(price: i64, expo: i32) -> pc::Price {
        let mut pyth_price = pc::Price::default();
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

    pub fn get_anchor_account_bytes<T: ZeroCopy + Owner>(account: &mut T) -> BytesMut {
        let mut bytes = BytesMut::new();
        bytes.extend_from_slice(&T::discriminator());
        let data = bytemuck::bytes_of_mut(account);
        bytes.extend_from_slice(data);
        bytes
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
            let owner = $type::owner();
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
