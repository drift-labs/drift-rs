use drift_sdk::{
    event_subscriber::RpcClient,
    get_market_accounts,
    math::constants::{BASE_PRECISION_I64, PRICE_PRECISION_U64},
    types::{accounts::User, Context, MarketId, NewOrder, PostOnlyParam},
    DriftClient, TransactionBuilder, Wallet,
};
use solana_sdk::signature::Keypair;

const LAMPORTS_PER_SOL_I64: i64 = 1_000_000_000;

/// keypair for integration tests
fn test_keypair() -> Keypair {
    let private_key = std::env::var("TEST_PRIVATE_KEY").expect("TEST_PRIVATE_KEY set");
    Keypair::from_base58_string(private_key.as_str())
}

#[ignore]
#[tokio::test]
async fn get_oracle_prices() {
    let client = DriftClient::new(
        Context::DevNet,
        RpcClient::new("https://api.devnet.solana.com".into()),
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
async fn get_market_accounts_works() {
    let client = DriftClient::new(
        Context::DevNet,
        RpcClient::new("https://api.devnet.solana.com".into()),
        Keypair::new().into(),
    )
    .await
    .expect("connects");

    let (spot, perp) = get_market_accounts(client.inner()).await.unwrap();
    assert!(spot.len() > 1);
    assert!(perp.len() > 1);
}

#[tokio::test]
async fn place_and_cancel_orders() {
    let wallet: Wallet = test_keypair().into();
    let client = DriftClient::new(
        Context::DevNet,
        RpcClient::new("https://api.devnet.solana.com".into()),
        wallet.clone(),
    )
    .await
    .expect("connects");

    let sol_perp = client.market_lookup("eth-perp").expect("exists");
    let sol_spot = client.market_lookup("eth").expect("exists");

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
        NewOrder::limit(sol_perp)
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
    .cancel_all_orders()
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
        RpcClient::new("https://api.devnet.solana.com".into()),
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
