//! Example place swift taker order
use argh::FromArgs;
use base64::Engine;
use drift_rs::{
    swift_order_subscriber::SignedOrderType,
    types::{MarketType, OrderParams, OrderType, PositionDirection, SignedMsgOrderParamsMessage},
    Context, DriftClient, RpcClient, Wallet,
};
use nanoid::nanoid;
use reqwest::header;

/// Swift taker client example
#[derive(FromArgs)]
struct SwiftTakerArgs {
    /// make a depositTrade request
    #[argh(switch)]
    deposit_trade: bool,
}

#[tokio::main]
async fn main() {
    let _ = env_logger::init();
    let _ = dotenv::dotenv();
    let wallet: Wallet = drift_rs::utils::load_keypair_multi_format(
        &std::env::var("PRIVATE_KEY").expect("base58 PRIVATE_KEY set"),
    )
    .unwrap()
    .into();
    let args: SwiftTakerArgs = argh::from_env();

    let use_mainnet = std::env::var("MAINNET").is_ok();
    let context = if use_mainnet {
        drift_rs::types::Context::MainNet
    } else {
        drift_rs::types::Context::DevNet
    };
    let rpc_url =
        std::env::var("RPC_URL").unwrap_or_else(|_| "https://api.devnet.solana.com".to_string());
    let drift = DriftClient::new(context, RpcClient::new(rpc_url), wallet.clone())
        .await
        .expect("initialized client");

    let latest_slot = drift.rpc().get_slot().await.expect("get slot") + 200;

    let order_params = OrderParams {
        market_index: 0,
        market_type: MarketType::Perp,
        order_type: OrderType::Oracle,
        base_asset_amount: 100_000_000,
        direction: PositionDirection::Long,
        auction_start_price: Some(1_00),
        auction_end_price: Some(1_000),
        auction_duration: Some(20),
        ..Default::default()
    };
    let swift_order = SignedOrderType::Authority(SignedMsgOrderParamsMessage {
        sub_account_id: 0,
        signed_msg_order_params: order_params,
        slot: latest_slot,
        uuid: nanoid!(8).as_bytes().try_into().unwrap(),
        take_profit_order_params: None,
        stop_loss_order_params: None,
        max_margin_ratio: None,
        builder_idx: None,
        builder_fee_tenth_bps: None,
    });
    let signed_msg = hex::encode(swift_order.to_borsh());
    let signature = drift.wallet.sign_message(signed_msg.as_bytes()).unwrap();

    let swift_order_request = serde_json::json!({
        "message": signed_msg,
        "taker_authority": wallet.authority().to_string(),
        "taker_pubkey": wallet.default_sub_account().to_string(),
        "signature": base64::prelude::BASE64_STANDARD.encode(signature.as_ref()),
    });

    dbg!(&swift_order_request.to_string());

    if args.deposit_trade {
        // SOL deposit, 0 = usdc, 1 = sol
        swift_deposit_trade(&drift, 100_000_000, 1, swift_order_request).await;
    } else {
        swift_place_order(&drift, swift_order_request).await;
    }
}

async fn swift_place_order(drift: &DriftClient, swift_order_request: serde_json::Value) {
    println!("sending swift order: {swift_order_request:?}");
    let swift_url = if drift.context == Context::MainNet {
        "https://swift.drift.trade/orders"
    } else {
        "https://master.swift.drift.trade/orders"
    };
    let swift_cli = reqwest::Client::new();
    let req = swift_cli
        .post(swift_url)
        .header(header::CONTENT_TYPE, "application/json")
        .json(&swift_order_request)
        .build();
    dbg!(&req);
    let res = swift_cli.execute(req.unwrap()).await;
    dbg!(&res);
    let response = res.unwrap().text().await.unwrap();
    dbg!(response);
}

async fn swift_deposit_trade(
    drift: &DriftClient,
    deposit_amount: u64,
    deposit_market_index: u16,
    swift_order_request: serde_json::Value,
) {
    println!("sending swift depositTrade order: {swift_order_request:?}, deposit amount: {deposit_amount}, market: {deposit_market_index}");
    let tx_builder = drift
        .init_tx(&drift.wallet().default_sub_account(), false)
        .await
        .unwrap();
    let unsigned_tx = tx_builder
        .deposit(deposit_amount, deposit_market_index, None, None)
        .build();
    let signed_tx = drift
        .wallet()
        .sign_tx(
            unsigned_tx.clone(),
            drift.get_latest_blockhash().await.unwrap(),
        )
        .unwrap();
    dbg!(&signed_tx.verify_with_results());
    dbg!(&signed_tx);

    let sim_res = drift.simulate_tx(unsigned_tx).await;
    dbg!(sim_res);

    let req = serde_json::json!({
        "deposit_tx": base64::prelude::BASE64_STANDARD.encode(
            bincode::serialize(&signed_tx).unwrap()
        ),
        "swift_order": swift_order_request,
    });

    let swift_url = if drift.context == Context::MainNet {
        "https://swift.drift.trade/depositTrade"
    } else {
        "https://master.swift.drift.trade/depositTrade"
    };
    let swift_cli = reqwest::Client::new();
    let req = swift_cli
        .post(swift_url)
        .header(header::CONTENT_TYPE, "application/json")
        .json(&req)
        .build();
    dbg!(&req);
    let res = swift_cli.execute(req.unwrap()).await;
    dbg!(&res);
    let response = res.unwrap().text().await.unwrap();
    dbg!(response);
}
