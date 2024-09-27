use super::{account_map_builder::AccountsListBuilder, constants::PRICE_PRECISION};
use crate::{
    ffi::{
        calculate_margin_requirement_and_total_collateral_and_liability_info, MarginContextMode,
    },
    types::accounts::User,
    DriftClient, SdkError, SdkResult,
};

pub fn get_leverage(client: &DriftClient, user: &User) -> SdkResult<u128> {
    let mut builder = AccountsListBuilder::default();
    let mut accounts = builder.build(client, user)?;
    let margin_calculation = calculate_margin_requirement_and_total_collateral_and_liability_info(
        user,
        &mut accounts,
        MarginContextMode::StandardMaintenance,
    )?;

    let net_asset_value = calculate_net_asset_value(
        margin_calculation.total_collateral,
        margin_calculation.total_spot_liability_value,
    );

    if net_asset_value == i128::MIN {
        return Err(SdkError::MathError(
            "Net asset value is less than i128::MIN".to_string(),
        ));
    }

    let total_liability_value = margin_calculation
        .total_perp_liability_value
        .checked_add(margin_calculation.total_spot_liability_value)
        .expect("fits u128");

    let leverage = calculate_leverage(total_liability_value, net_asset_value);

    Ok(leverage)
}

pub fn get_spot_asset_value(client: &DriftClient, user: &User) -> SdkResult<i128> {
    let mut builder = AccountsListBuilder::default();
    let mut accounts = builder.build(client, user)?;

    let margin_calculation = calculate_margin_requirement_and_total_collateral_and_liability_info(
        user,
        &mut accounts,
        MarginContextMode::StandardMaintenance,
    )?;

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

#[cfg(feature = "rpc_tests")]
mod tests {
    use solana_sdk::signature::Keypair;

    use super::*;
    use crate::{
        utils::test_envs::{mainnet_endpoint, test_keypair},
        Context, RpcAccountProvider, Wallet,
    };

    #[tokio::test]
    async fn test_get_spot_market_value() {
        let wallet: Wallet = test_keypair().into();
        let pubkey = wallet.authority().clone();
        let drift_client = DriftClient::new(
            Context::MainNet,
            RpcAccountProvider::new(&mainnet_endpoint()),
            wallet,
        )
        .await
        .expect("drift client");
        drift_client.subscribe().await.expect("subscribe");

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        let mut user = crate::user::DriftUser::new(
            Wallet::derive_user_account(&pubkey, 0, &constants::PROGRAM_ID),
            drift_client.clone(),
        )
        .await
        .expect("drift user");
        user.subscribe().await.expect("subscribe");

        let spot_asset_value = get_spot_asset_value(&drift_client, &user.get_user_account())
            .expect("spot asset value");
        println!("spot_asset_value: {}", spot_asset_value);
    }

    #[tokio::test]
    async fn test_leverage() {
        let wallet: Wallet = test_keypair().into();
        let pubkey = wallet.authority().clone();
        let drift_client = DriftClient::new(
            Context::MainNet,
            RpcAccountProvider::new & (mainnet_endpoint()),
            wallet,
        )
        .await
        .expect("drift client");
        drift_client.subscribe().await.expect("subscribe");

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        let mut user = crate::user::DriftUser::new(
            Wallet::derive_user_account(&pubkey, 0, &constants::PROGRAM_ID),
            drift_client.clone(),
        )
        .await
        .expect("drift user");
        user.subscribe().await.expect("subscribe");

        let leverage = get_leverage(&drift_client, &user.get_user_account()).expect("leverage");
        println!("leverage: {}", leverage);
    }
}
