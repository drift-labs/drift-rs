use drift_rs::{
    event_subscriber::RpcClient,
    math::constants::{BASE_PRECISION_I64, LAMPORTS_PER_SOL_I64, PRICE_PRECISION_U64},
    types::{accounts::User, Context, MarketId, NewOrder, PostOnlyParam},
    utils::test_envs::{devnet_endpoint, mainnet_endpoint, test_keypair},
    DriftClient, TransactionBuilder, Wallet,
};
use solana_sdk::signature::Keypair;

#[tokio::test]
async fn client_sync_subscribe_devnet() {
    let client = DriftClient::new(
        Context::DevNet,
        RpcClient::new(devnet_endpoint()),
        Keypair::new().into(),
    )
    .await
    .expect("connects");
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
    client.subscribe().await.expect("subscribes");

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
    client.subscribe().await.unwrap();

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
