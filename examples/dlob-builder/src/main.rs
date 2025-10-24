//! Example DLOB subscription/builder with web server
use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Html, Json},
    routing::get,
    Router,
};
use drift_rs::{
    dlob::builder::DLOBBuilder,
    dlob::DLOB,
    types::{MarketId, MarketType},
    Context, DriftClient, GrpcSubscribeOpts, RpcClient,
};
use serde::{Deserialize, Serialize};
use solana_commitment_config::CommitmentLevel;
use solana_keypair::Keypair;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

#[derive(Serialize, Deserialize)]
struct OrderbookLevel {
    price: u64,
    size: u64,
}

#[derive(Serialize, Deserialize)]
struct L2Response {
    slot: u64,
    oracle_price: u64,
    asks: Vec<OrderbookLevel>,
    bids: Vec<OrderbookLevel>,
    market_index: u16,
}

#[derive(Deserialize)]
struct L2Query {
    market_index: u16,
}

#[derive(Serialize, Deserialize)]
struct L3OrderResponse {
    price: u64,
    size: u64,
    max_ts: u64,
    order_id: u32,
    reduce_only: bool,
    kind: String,
    user: String,
}

#[derive(Serialize, Deserialize)]
struct L3Response {
    slot: u64,
    oracle_price: u64,
    bids: Vec<L3OrderResponse>,
    asks: Vec<L3OrderResponse>,
    market_index: u16,
}

#[derive(Deserialize)]
struct L3Query {
    market_index: u16,
}

async fn get_l2_orderbook(
    Query(params): Query<L2Query>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<L2Response>, StatusCode> {
    // Get the L2 snapshot from the DLOB
    let price = state
        .drift
        .try_get_oracle_price_data_and_slot(MarketId::perp(params.market_index))
        .unwrap()
        .data
        .price as u64;
    let l2_book = state
        .dlob
        .get_l2_book(params.market_index, MarketType::Perp, price);
    // Convert BTreeMap to Vec<OrderbookLevel> for JSON serialization
    let asks: Vec<OrderbookLevel> = l2_book
        .asks
        .iter()
        .map(|(price, size)| OrderbookLevel {
            price: *price,
            size: *size,
        })
        .collect();

    let bids: Vec<OrderbookLevel> = l2_book
        .bids
        .iter()
        .map(|(price, size)| OrderbookLevel {
            price: *price,
            size: *size,
        })
        .collect();

    Ok(Json(L2Response {
        slot: l2_book.slot,
        oracle_price: l2_book.oracle_price,
        asks,
        bids,
        market_index: params.market_index,
    }))
}

async fn get_l3_orderbook(
    Query(params): Query<L3Query>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<L3Response>, StatusCode> {
    // Get the L3 snapshot from the DLOB
    let price = state
        .drift
        .try_get_oracle_price_data_and_slot(MarketId::perp(params.market_index))
        .unwrap()
        .data
        .price as u64;
    let l3_book = state
        .dlob
        .get_l3_book(params.market_index, MarketType::Perp, price);
    // Convert L3Order to L3OrderResponse for JSON serialization
    let convert_order = |order: &drift_rs::dlob::L3Order| L3OrderResponse {
        price: order.price,
        size: order.size,
        max_ts: order.max_ts,
        order_id: order.order_id,
        reduce_only: order.reduce_only,
        kind: format!("{:?}", order.kind),
        user: order.user.to_string(),
    };

    let bids: Vec<L3OrderResponse> = l3_book.bids.iter().map(convert_order).collect();
    let asks: Vec<L3OrderResponse> = l3_book.asks.iter().map(convert_order).collect();

    Ok(Json(L3Response {
        slot: l3_book.slot,
        oracle_price: l3_book.oracle_price,
        bids,
        asks,
        market_index: params.market_index,
    }))
}

async fn clob_ui() -> Html<&'static str> {
    Html(include_str!("clob.html"))
}

struct AppState {
    drift: DriftClient,
    dlob: &'static DLOB,
}

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();
    let _ = env_logger::init();

    let rpc_url = std::env::var("RPC_URL")
        .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".to_string());
    let drift = DriftClient::new(
        Context::MainNet,
        RpcClient::new(rpc_url),
        Keypair::new().into(),
    )
    .await
    .expect("initialized client");

    let account_map = drift.backend().account_map();
    println!("syncing initial User accounts/orders");
    account_map
        .sync_user_accounts(vec![drift_rs::memcmp::get_user_with_order_filter()])
        .await
        .expect("synced user accounts");

    let dlob_builder = DLOBBuilder::new(account_map);

    println!("starting gRPC subscription to live order changes");
    let grpc_url = std::env::var("GRPC_URL").expect("GRPC_URL set");
    let grpc_x_token = std::env::var("GRPC_X_TOKEN").expect("GRPC_X_TOKEN set");
    let res = drift
        .grpc_subscribe(
            grpc_url,
            grpc_x_token,
            GrpcSubscribeOpts::default()
                .commitment(CommitmentLevel::Processed)
                .usermap_on()
                .on_user_account(dlob_builder.account_update_handler(account_map))
                .on_slot(dlob_builder.slot_update_handler()),
            true,
        )
        .await;

    if let Err(err) = res {
        eprintln!("{err}");
        std::process::exit(1);
    }

    let state = Arc::new(AppState {
        dlob: dlob_builder.dlob(),
        drift,
    });

    // Build the web server
    let app = Router::new()
        .route("/", get(clob_ui))
        .route("/l2", get(get_l2_orderbook))
        .route("/l3", get(get_l3_orderbook))
        .layer(ServiceBuilder::new().layer(CorsLayer::permissive()))
        .with_state(state);

    println!("Starting web server on http://localhost:8080");
    println!("CLOB UI: http://localhost:8080/");
    println!("L2 API: curl 'http://localhost:8080/l2?market_index=0'");
    println!("L3 API: curl 'http://localhost:8080/l3?market_index=0'");

    // Start the web server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
