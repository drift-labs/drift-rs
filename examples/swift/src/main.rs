//! Example swift maker bot
//!
//! The `TODO:` comments should be altered depending on individual maker strategy
//!
use std::time::Duration;

use anchor_lang::prelude::*;
use drift_rs::{
    constants::PROGRAM_ID,
    dlob::{util::OrderDelta, DLOBEvent, TakerOrder, DLOB},
    ffi::calculate_auction_price,
    grpc::grpc_subscriber::AccountFilter,
    swift_order_subscriber::SignedOrderInfo,
    types::{
        accounts::User, MarketId, Order, OrderParams, OrderStatus, OrderType, PositionDirection,
        PostOnlyParam,
    },
    DriftClient, GrpcSubscribeOpts, Pubkey, RpcClient, Wallet,
};
use futures_util::StreamExt;
use solana_account_decoder_client_types::UiAccountEncoding;
use solana_rpc_client_api::config::{RpcAccountInfoConfig, RpcProgramAccountsConfig};
use solana_sdk::signature::Keypair;

/// Active market IDs to fill for
const MARKET_IDS: [MarketId; 5] = [
    MarketId::perp(0),
    MarketId::perp(1),
    MarketId::perp(2),
    MarketId::perp(9),
    MarketId::perp(59),
];

async fn setup_grpc(drift: DriftClient, dlob: &'static DLOB) -> tokio::sync::mpsc::Receiver<u64> {
    // TODO: IncrementalDLOBBuilder
    let dlob_notifier = dlob.spawn_notifier();

    // Sync all User accounts with `filter` e.g. non-idle,has-auctions
    let sync_result = drift
        .rpc()
        .get_program_accounts_with_config(
            &PROGRAM_ID,
            RpcProgramAccountsConfig {
                filters: Some(vec![drift_rs::memcmp::get_user_with_order_filter()]),
                account_config: RpcAccountInfoConfig {
                    encoding: Some(UiAccountEncoding::Base64Zstd),
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .await;

    // synced accounts
    match sync_result {
        Ok(accounts) => {
            for (pubkey, account) in accounts {
                let user: &User = drift_rs::utils::deser_user(&account.data);
                for order in user.orders {
                    if order.status == OrderStatus::Open
                        && order.base_asset_amount > order.base_asset_amount_filled
                    {
                        dlob_notifier.send(DLOBEvent::Order {
                            delta: OrderDelta::Create {
                                user: pubkey,
                                order,
                            },
                            slot: 0, // initial sync value
                        });
                    }
                }
            }
        }
        Err(err) => {
            log::error!(target: "dlob", "dlob sync error: {err:?}");
        }
    }

    log::info!(target: "dlob", "synced initial orders");

    let (slot_tx, slot_rx) = tokio::sync::mpsc::channel(64);

    let res = drift
        .grpc_subscribe(
            "https://api.rpcpool.com".into(),
            std::env::var("TEST_GRPC_X_TOKEN").expect("TEST_GRPC_X_TOKEN set"),
            GrpcSubscribeOpts::default()
                .usermap_on()
                .on_slot({
                    let dlob = dlob_notifier.clone();
                    let drift = drift.clone();

                    move |new_slot| {
                        // TODO: only updating auction orders for active market IDs
                        // still building the orderbook for every market
                        // filter updates for non-active markets
                        for market in MARKET_IDS {
                            if let Some(oracle_price) =
                                drift.try_get_oracle_price_data_and_slot(market)
                            {
                                dlob.send(DLOBEvent::SlotOrPriceUpdate {
                                    slot: new_slot,
                                    market_index: market.index(),
                                    market_type: market.kind(),
                                    oracle_price: oracle_price.data.price as u64,
                                });
                            }
                        }
                        slot_tx.try_send(new_slot);
                    }
                })
                .on_account(
                    AccountFilter::partial().with_discriminator(User::DISCRIMINATOR),
                    {
                        let dlob = dlob_notifier.clone();
                        let drift = drift.clone();

                        move |update| {
                            let new_user = drift_rs::utils::deser_user(update.data);
                            match drift
                                .account_map()
                                .account_data_and_slot::<User>(&update.pubkey)
                            {
                                Some(stored) => {
                                    if stored.slot < update.slot {
                                        let user_order_deltas =
                                            drift_rs::dlob::util::compare_user_orders(
                                                update.pubkey,
                                                &stored.data,
                                                &new_user,
                                            );
                                        for delta in user_order_deltas {
                                            dlob.send(DLOBEvent::Order {
                                                delta,
                                                slot: update.slot,
                                            });
                                        }
                                    }
                                    // TODO: insert new user data here, right now we rely on grpc_subscriber to populate account map elsewhere
                                }
                                None => {
                                    // assume clean dlob build and insert
                                    for order in new_user.orders {
                                        if order.status == OrderStatus::Open
                                            && order.base_asset_amount
                                                > order.base_asset_amount_filled
                                        {
                                            dlob_notifier.send(DLOBEvent::Order {
                                                delta: OrderDelta::Create {
                                                    user: update.pubkey,
                                                    order,
                                                },
                                                slot: update.slot,
                                            });
                                        }
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
    // std::thread::spawn(|| loop {
    //     let l2_book = dlob.get_l2_snapshot(0, MarketType::Perp);
    //     dbg!(&l2_book);
    //     drop(l2_book);
    //     std::thread::sleep(Duration::from_secs(1));
    // });

    slot_rx
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
    let mut slot_rx = setup_grpc(drift.clone(), dlob).await;

    let _ = drift
        .subscribe_blockhashes()
        .await
        .expect("subscribed blockhashes");

    let mut swift_order_stream = drift
        .subscribe_swift_orders(&MARKET_IDS, Some(true))
        .await
        .expect("subscribed swift orders");

    // watch orders
    // select on:
    // - pyth price update
    // - slot update
    // - new swift order

    /*
       assume a placer bot is running:
        - swift filler (tries to fill immediately at slot offset or slot offset + 1)

       assume that later than 1 slot the order is placed onchain already
        - every slot uncross auctions with resting limit orders
        - JIT filler (tries to fill onchain auctions at slot offset > 1)
    */

    // TODO:
    // - build cross ixs/txs
    // - handle retrying txs / rate-limits (light version)

    let mut slot = 0;
    loop {
        tokio::select! {
            biased;
            _ = tokio::signal::ctrl_c() => {
                println!("swift maker shutting down...");
                break;
            }
            swift_order = swift_order_stream.next() => {
                log::info!(target: "swift", "new swift order: {swift_order:?}");
                match swift_order {
                    Some(order) => {
                        let order_params = order.order_params();
                        let tick_size = drift.program_data().perp_market_config_by_index(order_params.market_index).unwrap().amm.order_tick_size;
                        // TODO: run on intra-slot oracle price updates
                        let oracle_price = drift.try_get_oracle_price_data_and_slot(MarketId::perp(order_params.market_index)).expect("got oracle price").data.price;

                        let order = Order {
                            slot,
                            price: order_params.price,
                            base_asset_amount: order_params.base_asset_amount,
                            trigger_price: order_params.trigger_price.unwrap_or_default(),
                            auction_duration: order_params.auction_duration.unwrap_or_default(),
                            auction_start_price: order_params.auction_start_price.unwrap_or_default(),
                            auction_end_price: order_params.auction_end_price.unwrap_or_default(),
                            max_ts: order_params.max_ts.unwrap_or_default(),
                            oracle_price_offset: order_params.oracle_price_offset.unwrap_or_default(),
                            market_index: order_params.market_index,
                            order_type: order_params.order_type,
                            market_type: order_params.market_type,
                            user_order_id: order_params.user_order_id,
                            direction: order_params.direction,
                            reduce_only: order_params.reduce_only,
                            post_only: order_params.post_only != PostOnlyParam::None,
                            immediate_or_cancel: order_params.immediate_or_cancel(),
                            trigger_condition: order_params.trigger_condition,
                            bit_flags: order_params.bit_flags,
                            ..Default::default()
                        };

                        let price = match order_params.order_type {
                            OrderType::Market | OrderType::Oracle => {
                                match calculate_auction_price(&order, slot, tick_size, Some(oracle_price), false) {
                                    Ok(p) => p,
                                    Err(err) => {
                                        log::warn!(target: "dlob", "could not get auction price {err:?}, params: {order_params:?}, skipping...");
                                        continue;
                                    }
                                }
                            }
                            OrderType::Limit => {
                                // assuming PMM is not a taker
                                match order.get_limit_price(Some(oracle_price), Some(oracle_price as u64), slot, tick_size, false, None) {
                                    Ok(Some(p)) => p,
                                    _ => {
                                        log::warn!(target: "dlob", "could not get limit price: {order_params:?}, skipping...");
                                        continue;
                                    },
                                }
                            }
                            _ => {
                                log::warn!("invalid swift order type");
                                unreachable!();
                            }
                        };
                        let taker_order = TakerOrder::from_order_params(order_params, price);

                        let lookahead = 1;
                        for offset in 0..=lookahead {
                            if let Ok(crosses) = dlob.find_crosses_for_taker_order(slot + offset, oracle_price as u64, taker_order) {
                                if !crosses.is_empty() {
                                    log::info!(target: "swift", "found resting cross|offset={offset}|crosses={crosses:?}");
                                    // TODO: build fills
                                    break;
                                }
                            }
                        }
                    }
                    None => {
                        log::error!("swift order stream finished");
                        break;
                    }
                }
            }
            new_slot = slot_rx.recv() => {
                slot = new_slot.expect("got slot update");
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
