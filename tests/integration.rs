use drift::math::constants::{BASE_PRECISION_I64, LAMPORTS_PER_SOL_I64, PRICE_PRECISION_U64};
use drift_sdk::{
    types::{Context, MarketId, NewOrder},
    DriftClient, RpcAccountProvider, Wallet,
};
use solana_sdk::signature::Keypair;

/// keypair for integration tests
fn test_keypair() -> Keypair {
    let private_key = std::env::var("TEST_PRIVATE_KEY").expect("TEST_PRIVATE_KEY set");
    Keypair::from_base58_string(private_key.as_str())
}

#[tokio::test]
async fn get_oracle_prices() {
    let client = DriftClient::new(
        Context::DevNet,
        RpcAccountProvider::new("https://api.devnet.solana.com"),
        Keypair::new().into(),
    )
    .await
    .expect("connects");
    let price = client.oracle_price(MarketId::perp(0)).await.expect("ok");
    assert!(price > 0);
    dbg!(price);
    let price = client.oracle_price(MarketId::spot(1)).await.expect("ok");
    assert!(price > 0);
    dbg!(price);
}

#[tokio::test]
async fn place_and_cancel_orders() {
    let wallet: Wallet = test_keypair().into();
    let client = DriftClient::new(
        Context::DevNet,
        RpcAccountProvider::new("https://api.devnet.solana.com"),
        wallet.clone(),
    )
    .await
    .expect("connects");

    let sol_perp = client.market_lookup("sol-perp").expect("exists");
    let sol_spot = client.market_lookup("sol").expect("exists");

    let tx = client
        .init_tx(&wallet.default_sub_account(), false)
        .await
        .unwrap()
        .cancel_all_orders()
        .place_orders(vec![
            NewOrder::limit(sol_perp)
                .amount(1 * BASE_PRECISION_I64)
                .price(40 * PRICE_PRECISION_U64)
                .post_only(drift_sdk::types::PostOnlyParam::MustPostOnly)
                .build(),
            NewOrder::limit(sol_spot)
                .amount(-1 * LAMPORTS_PER_SOL_I64)
                .price(400 * PRICE_PRECISION_U64)
                .post_only(drift_sdk::types::PostOnlyParam::MustPostOnly)
                .build(),
        ])
        .cancel_all_orders()
        .build();

    dbg!(tx.clone());

    let result = client.sign_and_send(tx).await;
    dbg!(&result);
    assert!(result.is_ok());
}

#[tokio::test]
async fn place_and_take() {
    let wallet: Wallet = test_keypair().into();
    let client = DriftClient::new(
        Context::DevNet,
        RpcAccountProvider::new("https://api.devnet.solana.com"),
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
        .place_and_take(order, None, None, None)
        .build();

    let result = client.sign_and_send(tx).await;
    dbg!(&result);
    // TODO: add a place and make to match against
    assert!(result.is_err());
}
