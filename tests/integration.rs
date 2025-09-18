use std::time::Duration;

use anchor_lang::Discriminator;
use drift_rs::{
    constants::DEFAULT_PUBKEY,
    event_subscriber::RpcClient,
    grpc::grpc_subscriber::AccountFilter,
    math::constants::{BASE_PRECISION_I64, LAMPORTS_PER_SOL_I64, PRICE_PRECISION_U64},
    types::{accounts::User, Context, MarketId, NewOrder, PostOnlyParam, SettlePnlMode},
    utils::test_envs::{devnet_endpoint, mainnet_endpoint, test_keypair},
    DriftClient, GrpcSubscribeOpts, Pubkey, TransactionBuilder, Wallet,
};
use futures_util::StreamExt;
use solana_sdk::{clock::Slot, signature::Keypair};

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
        MarketId::perp(4),
        MarketId::spot(32),
    ];
    tokio::try_join!(
        client.subscribe_markets(&markets),
        client.subscribe_oracles(&markets),
    )
    .expect("subscribes");

    let price = client.oracle_price(MarketId::perp(1)).await.expect("ok");
    assert!(price > 0);
    dbg!(price);
    let price = client.oracle_price(MarketId::perp(4)).await.expect("ok");
    assert!(price > 0);
    dbg!(price);
    let price = client.oracle_price(MarketId::spot(32)).await.expect("ok");
    assert!(price > 0);
    dbg!(price);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 5)]
async fn client_sync_subscribe_mainnet_grpc() {
    let _ = env_logger::try_init();
    let client = DriftClient::new(
        Context::MainNet,
        RpcClient::new(mainnet_endpoint()),
        Keypair::new().into(),
    )
    .await
    .expect("connects");

    let (slot_update_tx, mut slot_update_rx) = tokio::sync::mpsc::channel::<Slot>(1);
    let (user_update_tx, mut user_update_rx) = tokio::sync::mpsc::channel::<Pubkey>(1);

    assert!(client
        .grpc_subscribe(
            "https://api.rpcpool.com".into(),
            std::env::var("TEST_GRPC_X_TOKEN").expect("TEST_GRPC_X_TOKEN set"),
            GrpcSubscribeOpts::default()
                .usermap_on()
                .on_slot(move |new_slot| {
                    println!("slot: {new_slot}");
                    let _ = slot_update_tx.try_send(new_slot);
                })
                .on_account(
                    AccountFilter::partial().with_discriminator(User::DISCRIMINATOR),
                    move |account| {
                        println!("account: {}", account.pubkey);
                        let _ = user_update_tx.try_send(account.pubkey);
                    }
                ),
            true,
        )
        .await
        .is_ok());

    // oracle map subscribed
    for market in client.get_all_spot_market_ids() {
        let rpc_fetched_price = client.oracle_price(market).await.unwrap();
        log::info!("fetching market: {market:?}");
        let x = client.try_get_oracle_price_data_and_slot(market).unwrap();
        assert!(x.data.price == rpc_fetched_price);
    }

    for market in client.get_all_perp_market_ids() {
        let rpc_fetched_price = client.oracle_price(market).await.unwrap();
        log::info!("fetching market: {market:?}");
        let x = client.try_get_oracle_price_data_and_slot(market).unwrap();
        assert!(x.data.price == rpc_fetched_price);
    }

    // wait for updates
    tokio::time::sleep(Duration::from_secs(4)).await;

    // markets available
    assert!(client.try_get_perp_market_account(0).is_ok());
    assert!(client.try_get_spot_market_account(1).is_ok());

    // slot update received
    assert!(slot_update_rx.try_recv().is_ok_and(|s| s > 0));

    // user update received
    assert!(user_update_rx.try_recv().is_ok_and(|u| u != DEFAULT_PUBKEY));

    client.grpc_unsubscribe();
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
        .place_and_take(order, &[], None, None, None)
        .build();

    let result = client.sign_and_send(tx).await;
    dbg!(&result);
    // TODO: add a place and make to match against
    assert!(result.is_err());
}

#[tokio::test]
async fn client_subscribe_swift_orders() {
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
        .subscribe_swift_orders(all_markets.as_slice(), Some(true), None)
        .await
        .unwrap();
    let mut recv_count = 0;
    while let Some(swift_order) = order_stream.next().await {
        if recv_count > 5 {
            break;
        }
        dbg!(swift_order.order_uuid());
        recv_count += 1;
    }
}

#[tokio::test]
async fn oracle_source_mixed_precision() {
    let _ = env_logger::try_init();
    let client = DriftClient::new(
        Context::MainNet,
        RpcClient::new(mainnet_endpoint()),
        Keypair::new().into(),
    )
    .await
    .expect("connects");

    let price = client
        .get_oracle_price_data_and_slot(MarketId::perp(4))
        .await
        .unwrap()
        .data
        .price;
    println!("Bonk: {price}");
    assert!(price % 100_000 > 0);

    tokio::time::sleep(Duration::from_secs(1)).await;
    assert!(client.subscribe_oracles(&[MarketId::perp(4)]).await.is_ok());

    let price = client
        .try_get_oracle_price_data_and_slot(MarketId::perp(4))
        .unwrap()
        .data
        .price;

    println!("Bonk: {price}");
    assert!(price % 100_000 > 0);
}

#[tokio::test]
async fn settle_pnl_txs() {
    let wallet = Wallet::read_only(
        "DxoRJ4f5XRMvXU9SGuM4ZziBFUxbhB3ubur5sVZEvue2"
            .parse()
            .unwrap(),
    );
    let client = DriftClient::new(
        Context::MainNet,
        RpcClient::new(mainnet_endpoint()),
        wallet.clone(),
    )
    .await
    .expect("connects");

    let doge_perp = client.market_lookup("doge-perp").expect("exists");

    let tx = client
        .init_tx(&wallet.default_sub_account(), false)
        .await
        .unwrap()
        .settle_pnl(doge_perp.index(), None, None)
        .build();

    let result = client.simulate_tx(tx).await;
    dbg!(&result);
    assert!(result.is_ok_and(|x| x.err.is_none()));

    let sol_perp = client.market_lookup("sol-perp").expect("exists");
    let tx = client
        .init_tx(&wallet.default_sub_account(), false)
        .await
        .unwrap()
        .with_priority_fee(1, Some(2 * 200_000))
        .settle_pnl_multi(
            &[sol_perp.index(), doge_perp.index()],
            SettlePnlMode::MustSettle,
            None,
            None,
        )
        .build();

    let result = client.simulate_tx(tx).await;
    dbg!(&result);
    assert!(result.is_ok_and(|x| x.err.is_none()));
}

#[tokio::test]
async fn pyth_pull_update_atomic() {
    let vaa_proof = "UE5BVQEAAAADuAEAAAAEDQCGP2Fjz2LFIX48Qqm/paFzO/iEtFgH5sC1FHhNroyIC2fuzsISzz9IHbvBrlknA0UvM8r9UHSvsAwaqzquhzFsAALnhRblTgAMLanjq38YctnwqDsdV39WviJ0QAnWgRn+a2i4ljPkbVQl1+MM47qcsua4+1L6jo8i3LPMivVx+HQgAQRRADMkx28oGLPnNZnaZp8wtsxckjtx1GvXi+l9d89Yu1DJnYEGkVF4TzZSKtIQe+5OoUPAaIpnEauGVe0AEeh7AAYzopa5UFEUji5zKjTRRUxlQWAWDq3LuGMeV7osio6L/jyD0jMxpYMR0r/dqIdUi2a4GP0uF15k9yFMkANh7OCRAAgb/WNBqaYjLjZsqRqEFKbG3ZVR2vS5ph7EO6D8TKP2gg3Mk3Mhyr2O21KAboy061vKPbHTjX1w3dq8ipnz6EacAQpOpdNfjx1LhMu7ZaMfSM6IFqHOZnQqGxQOQHwh5lAd50Vj8LVu3rng211UchelGHtROvhN1IapTkVSEhD0dbeeAQs+IYIUBk8EahKpPnD0hk6E2d8ge3gKDcgakWgDhRMunArMASyVWkWw0N3p9FvOobXg4V4L5Tim6L1AhHf5Rj0YAAxsygUAwlhGQPEThxT72eY0HVbi8C1LATsBXrW6jksUNTllCqWWbRwgwDSlgibrk05BKtO1pjFCjkWRZZ+TCvrsAA05LnYl0RwpRYUs31y5Lbk8mZHrFDj02MkTC05rGcjVzmddlNcj5/IIp8Hc44GJFZ4XZO3kx7jW3vuF6RQm6RPmAA6xLKcvzZllJT8kxn/LI4AYUuCIOVyLMG/kVodeXWkOKSrkXr0SNwMFsLfl9xvPk2dCa7SyicGwMTUfKP4P8cyeAQ9Q5G4EDpPCq/A0J3luHRoCnSDpCuCu4zTzESAmRe80aSwDl7tN4wSn369Nu4iD6JSyUx/y3bHF7BgvlyGfQYHjABCZpnivKtKFNYpaLR627OKG//Vv3zol7gdCoMOXRcIxLhwSuhn5QlVHgeoOrHiLtOBlTzpz4bwa8btRxvU43pCgABK2TIKVKUnv5OyTjkQh8N5IMpaRK83UH3hpvsJKejNpJQK2zR/WfCkrYjy6pYQfhenZYHi4GCMQ0ALSh9cojaDlAGaVh0wAAAAAABrhAfrtrFhR4yubI7X5QRqMK6xKrj7U3XuBHdGnLqSqcQAAAAAEHJOLAUFVV1YAAAAAAAknGqgAACcQdoC/4vcjI21wyoVC1q3FUZH0FpwBAFUALy0Xq7weeBvYe0pdUsiyhWiG9cSC+jWTzr9nlQQKsLYAAAAAAICwNAAAAAAAAS4k////+AAAAABmlYdMAAAAAGaVh0sAAAAAAH6C+QAAAAAAAPFsCz2Sz2/hyAOSwCA+M8lRiOs+jGuZp6wcFR4rTAFuR2bAYNycVYQFeCxlkQJrEKDSba6FxQXgPZ7wBb/43EHuHHKQaaGb3NVsxnHFnLHefZDbF235q+aRnadgJfm6gqckqb0IczoHBaSuyrVYfSEbPuyNjXE7V++G/OwwVrwQOWqD6ti/nzLgnQ+qCVpEBto25YvZQzkmfYMKg1tJepxs/Sbgyx2fayAJtK8pRlJIixSTRbLiQX408KCq/ElVNzOSqt6Aw1KrAg81sLzKSjMEqnhbdFxgSzqncj8kPFw=";
    let feed_id =
        hex_literal::hex!("2f2d17abbc1e781bd87b4a5d52c8b2856886f5c482fa3593cebf6795040ab0b6");

    let _ = env_logger::try_init();
    let wallet: Wallet = test_keypair().into();
    let client = DriftClient::new(
        Context::MainNet,
        RpcClient::new(mainnet_endpoint()),
        wallet.clone(),
    )
    .await
    .expect("connects");

    let tx = client
        .init_tx(&wallet.default_sub_account(), false)
        .await
        .unwrap()
        .post_pyth_pull_oracle_update_atomic(vaa_proof, &feed_id, None)
        .build();

    let result = client.simulate_tx(tx).await;
    dbg!(&result);
    assert!(result.is_ok_and(|x| x.err.is_none()));
}
