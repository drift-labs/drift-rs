//! Example swift maker bot
//!
//! The `TODO:` comments should be altered depending on individual maker strategy
//!
use drift_rs::{
    swift_order_subscriber::SignedOrderInfo,
    types::{MarketId, OrderParams, OrderType, PositionDirection, PostOnlyParam},
    DriftClient, Pubkey, RpcClient, Wallet,
};
use futures_util::StreamExt;
use solana_sdk::signature::Keypair;

#[tokio::main]
async fn main() {
    let _ = env_logger::init();
    let wallet: Wallet =
        Keypair::from_base58_string(&std::env::var("PRIVATE_KEY").expect("base58 PRIVATE_KEY set"))
            .into();

    // choose a sub-account for order placement
    let filler_subaccount = wallet.default_sub_account();

    let drift = DriftClient::new(
        drift_rs::types::Context::DevNet,
        RpcClient::new("https://api.devnet.solana.com".into()),
        wallet,
    )
    .await
    .expect("initialized client");
    let _ = drift
        .subscribe_blockhashes()
        .await
        .expect("subscribed blockhashes");

    // choose some markets by symbol
    let market_ids: Vec<MarketId> = ["sui-perp", "eth-perp", "xrp-perp"]
        .iter()
        .map(|m| drift.market_lookup(m).unwrap())
        .collect();

    let mut swift_order_stream = drift
        .subscribe_swift_orders(&market_ids)
        .await
        .expect("subscribed swift orders");

    // watch orders
    loop {
        tokio::select! {
            biased;
            _ = tokio::signal::ctrl_c() => {
                println!("swift maker shutting down...");
                break;
            }
            swift_order = swift_order_stream.next() => {
                match swift_order {
                    Some(order) => {
                        let _handle = tokio::spawn(try_fill(drift.clone(), filler_subaccount, order));
                    }
                    None => {
                        println!("swift order stream finished");
                        break;
                    }
                }
            }
        }
    }
}

/// Try to fill a swift order
async fn try_fill(drift: DriftClient, filler_subaccount: Pubkey, swift_order: SignedOrderInfo) {
    // TODO: filter `swift_order.order_params()` depending on strategy params
    println!("new swift order: {swift_order:?}");
    let taker_order = swift_order.order_params();
    let taker_subaccount = Wallet::derive_user_account(
        &swift_order.taker_authority,
        swift_order.taker_subaccount_id(),
    );

    // fetching taker accounts inline
    // TODO: for better fills maintain a gRPC map of user accounts
    let (taker_account_data, taker_stats) = tokio::try_join!(
        drift.get_user_account(&taker_subaccount),
        drift.get_user_stats(&swift_order.taker_authority)
    )
    .unwrap();

    // built the taker tx
    // It places the swift order for the taker and fills it
    let tx_builder = drift.init_tx(&filler_subaccount, false).await.unwrap();
    let tx = tx_builder
        .place_and_make_swift_order(
            OrderParams {
                order_type: OrderType::Limit,
                market_index: taker_order.market_index,
                market_type: taker_order.market_type,
                direction: match taker_order.direction {
                    PositionDirection::Long => PositionDirection::Short,
                    PositionDirection::Short => PositionDirection::Long,
                },
                // TODO: fill at price depending on strategy params
                // always fill at the best price for the taker
                price: taker_order
                    .auction_start_price
                    .expect("start price set")
                    .unsigned_abs(),
                // try fill the whole order amount
                base_asset_amount: taker_order.base_asset_amount,
                post_only: PostOnlyParam::MustPostOnly,
                immediate_or_cancel: true,
                ..Default::default()
            },
            &swift_order,
            &taker_account_data,
            &taker_stats.referrer,
        )
        .build();

    match drift.sign_and_send(tx).await {
        Ok(sig) => {
            println!("sent fill: {sig}");
        }
        Err(err) => {
            println!("fill failed: {err}");
        }
    }
}
