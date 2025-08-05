//! Example DLOB subscription/builder
use std::time::Duration;

use drift_rs::{
    dlob::builder::DLOBBuilder, types::MarketType, Context, DriftClient, GrpcSubscribeOpts,
    RpcClient,
};
use solana_commitment_config::CommitmentLevel;
use solana_keypair::Keypair;

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();

    let rpc_url = std::env::var("RPC_URL")
        .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());
    let drift = DriftClient::new(
        Context::MainNet,
        RpcClient::new(rpc_url),
        Keypair::new().into(),
    )
    .await
    .expect("initialized client");

    let markets = drift.get_all_perp_market_ids();
    let account_map = drift.backend().account_map();
    println!("syncing initial User accounts/orders");
    account_map
        .sync_user_accounts(vec![drift_rs::memcmp::get_user_with_order_filter()])
        .await
        .expect("synced user accounts");

    let dlob_builder = DLOBBuilder::new(markets, account_map);

    println!("starting gRPC subscription to live order changes");
    let grpc_url = std::env::var("GRPC_URL").expect("GRPC_URL set");
    let grpc_x_token = std::env::var("GRPC_X_TOKEN").expect("GRPC_X_TOKEN set");
    let _res = drift
        .grpc_subscribe(
            grpc_url,
            grpc_x_token,
            GrpcSubscribeOpts::default()
                .commitment(CommitmentLevel::Processed)
                .usermap_on()
                .on_user_account(dlob_builder.account_update_handler(account_map))
                .on_slot(dlob_builder.slot_update_handler(drift.clone())),
            true,
        )
        .await;

    let dlob = dlob_builder.dlob();

    println!("printing sol-perp orderbook");
    for _ in 0..5 {
        println!("{}", dlob.get_l2_snapshot(0, MarketType::Perp));
        let _ = tokio::time::sleep(Duration::from_secs(1)).await;
    }
    println!("example finished");
}
