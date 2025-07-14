//! Example place swift taker order
use base64::Engine;
use drift_rs::{
    swift_order_subscriber::SignedOrderType,
    types::{
        MarketType, OrderParams, OrderType, PositionDirection,
        SignedMsgOrderParamsMessage,
    },
    DriftClient, RpcClient, Wallet,
};
use nanoid::nanoid;
use reqwest::header;

#[tokio::main]
async fn main() {
    let _ = env_logger::init();
    let _ = dotenv::dotenv();
    let wallet: Wallet = drift_rs::utils::load_keypair_multi_format(
        &std::env::var("PRIVATE_KEY").expect("base58 PRIVATE_KEY set")
    ).unwrap().into();

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
        market_index: 75,
        market_type: MarketType::Perp,
        order_type: OrderType::Oracle,
        base_asset_amount: 5_000_000_000,
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
    });
    let signed_msg = hex::encode(swift_order.to_borsh());
    let signature = drift.wallet.sign_message(signed_msg.as_bytes()).unwrap();

    let swift_order_request = serde_json::json!({
        "message": signed_msg,
        "taker_authority": wallet.authority().to_string(),
        "signature": base64::prelude::BASE64_STANDARD.encode(signature.as_ref()),
    });

    dbg!(
        &swift_order_request.to_string()
    );

    let swift_url = "https://swift.drift.trade/orders";
    let swift_cli = reqwest::Client::new();
    let req = swift_cli
        .post(swift_url)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36")
        .json(&swift_order_request)
        .build();
    dbg!(&req);
    let res = swift_cli.execute(req.unwrap()).await;
    dbg!(&res);
    let response = res.unwrap().text().await.unwrap();
    dbg!(response);
}

// nlHXHqU0
