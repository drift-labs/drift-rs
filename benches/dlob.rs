use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use drift_rs::{
    dlob::dlob::DLOB,
    drift_idl::types::{MarketType, Order},
    Pubkey,
};

fn dlob_insert(n: u64) {
    let dlob = DLOB::new();
    let user_account = Pubkey::new_unique();
    let taking_limit_order = Order {
        order_id: 1,
        slot: 1,
        market_index: 0,
        market_type: MarketType::Perp,
        ..Order::default()
    };
    let floating_limit_order = Order {
        order_id: 2,
        oracle_price_offset: 1,
        market_index: 0,
        market_type: MarketType::Perp,
        ..Order::default()
    };
    let resting_limit_order = Order {
        order_id: 3,
        slot: 3,
        market_index: 0,
        market_type: MarketType::Perp,
        ..Order::default()
    };
    let market_order = Order {
        order_id: 4,
        slot: 4,
        market_index: 0,
        market_type: MarketType::Perp,
        ..Order::default()
    };
    let trigger_order = Order {
        order_id: 5,
        slot: 5,
        market_index: 0,
        market_type: MarketType::Perp,
        ..Order::default()
    };

    dlob.insert_order(&taking_limit_order, user_account, 1);
    dlob.insert_order(&floating_limit_order, user_account, 0);
    dlob.insert_order(&resting_limit_order, user_account, 3);
    dlob.insert_order(&market_order, user_account, 4);
    dlob.insert_order(&trigger_order, user_account, 5);

    dlob.get_order(1, user_account);
    dlob.get_order(2, user_account);
    dlob.get_order(3, user_account);
    dlob.get_order(4, user_account);
    dlob.get_order(5, user_account);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("dlob 100", |b| b.iter(|| dlob_insert(black_box(100))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
