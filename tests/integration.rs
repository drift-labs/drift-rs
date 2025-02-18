use drift_rs::{
    event_subscriber::RpcClient,
    math::constants::{BASE_PRECISION_I64, LAMPORTS_PER_SOL_I64, PRICE_PRECISION_U64},
    types::{accounts::User, Context, MarketId, NewOrder, PostOnlyParam},
    utils::test_envs::{devnet_endpoint, mainnet_endpoint, test_keypair},
    DriftClient, TransactionBuilder, Wallet,
};
use futures_util::StreamExt;
use solana_sdk::signature::Keypair;

#[tokio::test]
async fn client_sync_subscribe_all_devnet() {
    let client = DriftClient::new(
        Context::DevNet,
        RpcClient::new(devnet_endpoint()),
        Keypair::new().into(),
    )
    .await
    .expect("connects");

    tokio::try_join!(
        client.subscribe_all_markets(),
        client.subscribe_all_oracles(),
    )
    .expect("subscribes");
    let all_markets = client.get_all_market_ids();
    for market in all_markets {
        let price = client.oracle_price(market).await.expect("ok");
        assert!(price > 0);
        dbg!(market, price);
    }
}

#[tokio::test]
async fn client_sync_subscribe_devnet() {
    let _ = env_logger::try_init();
    let client = DriftClient::new(
        Context::DevNet,
        RpcClient::new(devnet_endpoint()),
        Keypair::new().into(),
    )
    .await
    .expect("connects");
    let markets = [
        MarketId::spot(1),
        MarketId::spot(2),
        MarketId::perp(0),
        MarketId::perp(1),
        MarketId::perp(2),
    ];
    tokio::try_join!(
        client.subscribe_markets(&markets),
        client.subscribe_oracles(&markets),
    )
    .expect("subscribes");

    let price = client.oracle_price(MarketId::perp(1)).await.expect("ok");
    assert!(price > 0);
    dbg!(price);
    let price = client.oracle_price(MarketId::spot(2)).await.expect("ok");
    assert!(price > 0);
    dbg!(price);
}

#[tokio::test]
async fn client_sync_subscribe_mainnet() {
    let _ = env_logger::try_init();
    let client = DriftClient::new(
        Context::MainNet,
        RpcClient::new(mainnet_endpoint()),
        Keypair::new().into(),
    )
    .await
    .expect("connects");
    let markets = [
        MarketId::spot(1),
        MarketId::spot(2),
        MarketId::perp(0),
        MarketId::perp(1),
        MarketId::perp(2),
    ];
    tokio::try_join!(
        client.subscribe_markets(&markets),
        client.subscribe_oracles(&markets),
    )
    .expect("subscribes");

    let price = client.oracle_price(MarketId::perp(1)).await.expect("ok");
    assert!(price > 0);
    dbg!(price);
    let price = client.oracle_price(MarketId::spot(2)).await.expect("ok");
    assert!(price > 0);
    dbg!(price);
}

#[tokio::test]
async fn place_and_cancel_orders() {
    let _ = env_logger::try_init();
    let btc_perp = MarketId::perp(1);
    let sol_spot = MarketId::spot(1);

    let wallet: Wallet = test_keypair().into();
    let client = DriftClient::new(
        Context::DevNet,
        RpcClient::new(devnet_endpoint()),
        wallet.clone(),
    )
    .await
    .expect("connects");

    let user: User = client
        .get_user_account(&wallet.default_sub_account())
        .await
        .expect("exists");
    let tx = TransactionBuilder::new(
        client.program_data(),
        wallet.default_sub_account(),
        std::borrow::Cow::Borrowed(&user),
        false,
    )
    .cancel_all_orders()
    .place_orders(vec![
        NewOrder::limit(btc_perp)
            .amount(1 * BASE_PRECISION_I64)
            .price(40 * PRICE_PRECISION_U64)
            .post_only(PostOnlyParam::MustPostOnly)
            .build(),
        NewOrder::limit(sol_spot)
            .amount(-1 * LAMPORTS_PER_SOL_I64)
            .price(400 * PRICE_PRECISION_U64)
            .post_only(PostOnlyParam::MustPostOnly)
            .build(),
    ])
    .cancel_orders(btc_perp.to_parts(), None)
    .cancel_orders(sol_spot.to_parts(), None)
    .build();

    dbg!(tx.clone());

    let result = client.sign_and_send(tx).await;
    dbg!(&result);
    assert!(result.is_ok());
    client.unsubscribe().await.unwrap();
}

#[ignore]
#[tokio::test]
async fn place_and_take() {
    let wallet: Wallet = test_keypair().into();
    let client = DriftClient::new(
        Context::DevNet,
        RpcClient::new(devnet_endpoint()),
        wallet.clone(),
    )
    .await
    .expect("connects");

    let sol_perp = client.market_lookup("sol-perp").expect("exists");

    let order = NewOrder::limit(sol_perp)
        .amount(1 * BASE_PRECISION_I64)
        .price(40 * PRICE_PRECISION_U64)
        .build();
    let tx = client
        .init_tx(&wallet.default_sub_account(), false)
        .await
        .unwrap()
        .place_and_take(order, None, None, None, None)
        .build();

    let result = client.sign_and_send(tx).await;
    dbg!(&result);
    // TODO: add a place and make to match against
    assert!(result.is_err());
}

#[tokio::test]
async fn client_subscribe_fastlane_orders() {
    let _ = env_logger::try_init();
    let client = DriftClient::new(
        Context::DevNet,
        RpcClient::new(devnet_endpoint()),
        Keypair::new().into(),
    )
    .await
    .expect("connects");

    let all_markets = client.get_all_perp_market_ids();
    let mut order_stream = client
        .subscribe_fastlane_orders(all_markets.as_slice())
        .await
        .unwrap();
    let mut recv_count = 0;
    while let Some(fastlane_order) = order_stream.next().await {
        if recv_count > 5 {
            break;
        }
        dbg!(fastlane_order.order_uuid());
        recv_count += 1;
    }
}
