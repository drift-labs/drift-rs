//! Example swift maker bot
//!
//! The `TODO:` comments should be altered depending on individual maker strategy
//!
use std::{
    sync::{atomic::AtomicU64, Arc},
    time::Duration,
};

use anchor_lang::prelude::*;
use drift_rs::{
    dlob::{util::OrderDelta, TakerOrder, DLOB},
    grpc::grpc_subscriber::AccountFilter,
    swift_order_subscriber::SignedOrderInfo,
    types::{
        accounts::User, MarketId, MarketType, OrderParams, OrderType, PositionDirection,
        PostOnlyParam,
    },
    DriftClient, GrpcSubscribeOpts, Pubkey, RpcClient, Wallet,
};
use futures_util::StreamExt;
use solana_sdk::signature::Keypair;

async fn setup_grpc(drift: DriftClient, dlob: &'static DLOB) {
    let latest_slot = Arc::new(AtomicU64::default());

    /// Sync all User accounts with `filter` e.g. non-idle,has-auctions
    let accounts = self.rpc.get_program_accounts_with_config(&PROGRAM_ID, RpcProgramAccountsConfig {
        filters: Some(vec![
            drift_rs::memcmp::get_user_with_order_filter(),
        ]),
        ..Default::default()
    });

    let res = drift
        .grpc_subscribe(
            "https://api.rpcpool.com".into(),
            std::env::var("TEST_GRPC_X_TOKEN").expect("TEST_GRPC_X_TOKEN set"),
            GrpcSubscribeOpts::default()
                .usermap_on()
                .on_slot({
                    let drift = drift.clone();
                    let latest_slot = Arc::clone(&latest_slot);

                    move |new_slot| {
                        latest_slot.store(new_slot, std::sync::atomic::Ordering::Relaxed);

                        // TODO: only updating SOL-PERP
                        if let Some(oracle_price) =
                            drift.try_get_oracle_price_data_and_slot(MarketId::perp(0))
                        {
                            dlob.update_slot_and_oracle_price(
                                0,
                                MarketType::Perp,
                                new_slot,
                                oracle_price.data.price as u64,
                            );
                        }
                    }
                })
                .on_account(
                    AccountFilter::partial().with_discriminator(User::DISCRIMINATOR),
                    {
                        let drift = drift.clone();
                        let latest_slot = Arc::clone(&latest_slot);

                        move |account| {
                            let new_user = User::try_deserialize_unchecked(
                                &mut account.data.to_vec().as_slice(),
                            )
                            .expect("deser user");
                            match drift.try_get_account::<User>(&account.pubkey) {
                                Ok(old_user) => {
                                    for order_delta in drift_rs::dlob::util::compare_user_orders(
                                        account.pubkey,
                                        &old_user,
                                        &new_user,
                                    ) {
                                        match order_delta {
                                            OrderDelta::Create { user, order } => {
                                                dlob.insert_order(&user, order);
                                            }
                                            OrderDelta::Update { user, order } => {
                                                dlob.update_order(&user, order);
                                            }
                                            OrderDelta::Remove { user, order } => {
                                                dlob.remove_order(&user, order);
                                            }
                                        }
                                    }
                                }
                                Err(_err) => {
                                    // assume clean dlob build and insert
                                    for order in new_user.orders {
                                        dlob.insert_order(&account.pubkey, order);
                                    }
                                }
                            }
                        }
                    },
                ),
            true,
        )
        .await;

    // dlob printer
    std::thread::spawn(|| loop {
        let l2_book = dlob.get_l3_snapshot(0, MarketType::Perp);
        dbg!(&l2_book);
        drop(l2_book);
        std::thread::sleep(Duration::from_secs(1));
    });
}

#[tokio::main]
async fn main() {
    let _ = env_logger::init();
    let _ = dotenv::dotenv();
    let wallet: Wallet =
        Keypair::from_base58_string(&std::env::var("PRIVATE_KEY").expect("base58 PRIVATE_KEY set"))
            .into();

    // choose a sub-account for order placement
    let filler_subaccount = wallet.default_sub_account();

    let use_mainnet = std::env::var("MAINNET").is_ok();
    let context = if use_mainnet {
        drift_rs::types::Context::MainNet
    } else {
        drift_rs::types::Context::DevNet
    };
    let rpc_url =
        std::env::var("RPC_URL").unwrap_or_else(|_| "https://api.devnet.solana.com".to_string());
    let drift = DriftClient::new(context, RpcClient::new(rpc_url), wallet)
        .await
        .expect("initialized client");

    let dlob: &'static DLOB = Box::leak(Box::new(DLOB::default()));
    setup_grpc(drift.clone(), dlob).await;

    let _ = drift
        .subscribe_blockhashes()
        .await
        .expect("subscribed blockhashes");

    // subscribe to filler account (used when building Txs)
    let _ = drift
        .subscribe_account(&filler_subaccount)
        .await
        .expect("subscribed");

    // choose some markets by symbol
    let market_ids: Vec<MarketId> = ["sol-perp", "eth-perp"]
        .iter()
        .map(|m| drift.market_lookup(m).expect("market found"))
        .collect();

    let mut swift_order_stream = drift
        .subscribe_swift_orders(&market_ids, None)
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
                        // let _handle = tokio::spawn(try_fill(drift.clone(), filler_subaccount, order));
                        // let res = dlob.find_crosses_for_taker_order(slot, oracle_price, TakerOrder::from_order_params(order.order_params(), current_price));
                        // dbg!("crosses", res);
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
    let taker_subaccount = swift_order.taker_subaccount();

    // fetching taker accounts inline
    // TODO: for better fills maintain a gRPC map of user accounts
    let (taker_account_data, taker_stats, tx_builder) = tokio::try_join!(
        drift.get_user_account(&taker_subaccount), // always hits RPC
        drift.get_user_stats(&swift_order.taker_authority), // always hits RPC
        drift.init_tx(&filler_subaccount, false)
    )
    .unwrap();

    // built the taker tx
    // It places the swift order for the taker and fills it
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
                // this always attempts to fill at the best price for the _taker_
                price: taker_order
                    .auction_start_price
                    .expect("start price set")
                    .unsigned_abs(),
                // try fill the whole order amount
                base_asset_amount: taker_order.base_asset_amount,
                post_only: PostOnlyParam::MustPostOnly,
                bit_flags: 0,
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
