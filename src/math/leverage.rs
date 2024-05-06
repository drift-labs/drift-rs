use crate::math::account_map_builder::AccountMapBuilder;
use crate::{AccountProvider, DriftClient, SdkError, SdkResult};
use drift::instructions::optional_accounts::AccountMaps;
use drift::math::constants::PRICE_PRECISION;
use drift::math::margin::{
    calculate_margin_requirement_and_total_collateral_and_liability_info, MarginRequirementType,
};
use drift::state::margin_calculation::MarginContext;
use drift::state::user::User;

pub fn get_leverage<T: AccountProvider>(client: &DriftClient<T>, user: &User) -> SdkResult<u128> {
    let mut accounts_builder = AccountMapBuilder::default();
    let mut account_maps = accounts_builder.build(client, user)?;

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

pub fn get_spot_asset_value<T: AccountProvider>(
    client: &DriftClient<T>,
    user: &User,
) -> SdkResult<i128> {
    let mut accounts_builder = AccountMapBuilder::default();
    let mut account_maps = accounts_builder.build(client, user)?;

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

    Ok(margin_calculation.total_spot_asset_value
        - margin_calculation.total_spot_liability_value as i128)
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

    let leverage = (total_liability_value as f64) / (net_asset_value.abs() as f64);

    sign as u128 * (leverage * PRICE_PRECISION as f64) as u128
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Context, RpcAccountProvider, Wallet};
    use solana_sdk::signature::Keypair;

    const RPC: &str = "https://api.devnet.solana.com";
    const PRIVATE_KEY: &'static str =
        "4ZT38mSeFhzzDRCMTMbwDp7VYWDqNfkvDR42Wv4Hu9cKzbZPJoVapQSrjLbs9aMPrpAMmN1cQinztnP2PzKVjzwX";

    #[tokio::test]
    #[cfg(feature = "rpc_tests")]
    async fn test_get_spot_market_value() {
        let wallet: Wallet = Keypair::from_base58_string(PRIVATE_KEY).into();
        let pubkey = wallet.authority().clone();
        let drift_client = DriftClient::new(Context::DevNet, RpcAccountProvider::new(RPC), wallet)
            .await
            .expect("drift client");
        drift_client.subscribe().await.expect("subscribe");

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        let mut user = crate::user::DriftUser::new(
            Wallet::derive_user_account(&pubkey, 0, &drift::ID),
            &drift_client,
            0,
        )
        .await
        .expect("drift user");
        user.subscribe().await.expect("subscribe");

        let spot_asset_value = get_spot_asset_value(&drift_client, &user.get_user_account())
            .expect("spot asset value");
        println!("spot_asset_value: {}", spot_asset_value);
    }

    #[tokio::test]
    #[cfg(feature = "rpc_tests")]
    async fn test_leverage() {
        let wallet: Wallet = Keypair::from_base58_string(PRIVATE_KEY).into();
        let pubkey = wallet.authority().clone();
        let drift_client = DriftClient::new(Context::DevNet, RpcAccountProvider::new(RPC), wallet)
            .await
            .expect("drift client");
        drift_client.subscribe().await.expect("subscribe");

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        let mut user = crate::user::DriftUser::new(
            Wallet::derive_user_account(&pubkey, 0, &drift::ID),
            &drift_client,
            0,
        )
        .await
        .expect("drift user");
        user.subscribe().await.expect("subscribe");

        let leverage = get_leverage(&drift_client, &user.get_user_account()).expect("leverage");
        println!("leverage: {}", leverage);
    }
}
