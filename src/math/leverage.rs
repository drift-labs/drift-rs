use crate::math::liquidation::AccountMapBuilder;
use crate::{AccountProvider, DriftClient, SdkError, SdkResult};
use drift::instructions::optional_accounts::AccountMaps;
use drift::math::margin::{
    calculate_margin_requirement_and_total_collateral_and_liability_info, MarginRequirementType,
};
use drift::state::margin_calculation::MarginContext;
use drift::state::user::User;

pub async fn get_leverage<T: AccountProvider>(
    client: &DriftClient<T>,
    user: &User,
) -> SdkResult<u128> {
    let mut accounts_builder = AccountMapBuilder::default();
    let mut account_maps = accounts_builder.build(client, user).await?;

    let AccountMaps {
        perp_market_map,
        spot_market_map,
        ref mut oracle_map,
    } = account_maps;

    let margin_calculation = calculate_margin_requirement_and_total_collateral_and_liability_info(
        user,
        &perp_market_map,
        &spot_market_map,
        oracle_map,
        MarginContext::standard(MarginRequirementType::Maintenance),
    )
    .map_err(|err| SdkError::Anchor(Box::new(err.into())))?;

    let net_asset_value = calculate_net_asset_value(
        margin_calculation.total_collateral,
        margin_calculation.total_spot_liability_value,
    );

    if net_asset_value == i128::MIN {
        return Err(SdkError::MathError(
            "Net asset value is less than i128::MIN".to_string(),
        ));
    }

    let total_liability_value = margin_calculation.total_perp_liability_value
        + margin_calculation.total_spot_liability_value;

    let leverage = calculate_leverage(total_liability_value, net_asset_value);

    Ok(leverage)
}

pub async fn get_spot_asset_value<T: AccountProvider>(
    client: &DriftClient<T>,
    user: &User,
) -> SdkResult<i128> {
    let mut accounts_builder = AccountMapBuilder::default();
    let mut account_maps = accounts_builder.build(client, user).await?;

    let AccountMaps {
        perp_market_map,
        spot_market_map,
        ref mut oracle_map,
    } = account_maps;

    let margin_calculation = calculate_margin_requirement_and_total_collateral_and_liability_info(
        user,
        &perp_market_map,
        &spot_market_map,
        oracle_map,
        MarginContext::standard(MarginRequirementType::Maintenance),
    )
    .map_err(|err| SdkError::Anchor(Box::new(err.into())))?;

    Ok(margin_calculation.total_spot_asset_value)
}

fn calculate_net_asset_value(total_collateral: i128, total_spot_liability_value: u128) -> i128 {
    if total_spot_liability_value <= i128::MAX as u128 {
        total_collateral - total_spot_liability_value as i128
    } else {
        let overflow = total_spot_liability_value - i128::MAX as u128;
        if overflow <= total_collateral as u128 + 1 {
            total_collateral - (i128::MAX as u128 - (overflow - 1)) as i128
        } else {
            i128::MIN
        }
    }
}

fn calculate_leverage(total_liability_value: u128, net_asset_value: i128) -> u128 {
    let sign: i128 = if net_asset_value < 0 { -1 } else { 1 };

    let leverage = total_liability_value / net_asset_value.abs() as u128;

    sign as u128 * leverage
}
