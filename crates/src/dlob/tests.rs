use solana_sdk::pubkey::Pubkey;
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;

use crate::{
    dlob::{
        types::DynamicPrice, Direction, OrderKind, OrderMetadata, Orderbook, Snapshot, TakerOrder,
        DLOB,
    },
    types::{MarketId, MarketType, Order, OrderType},
};

fn create_test_order(
    order_id: u32,
    order_type: OrderType,
    direction: Direction,
    price: i64,
    size: u64,
    slot: u64,
) -> Order {
    Order {
        order_id,
        order_type,
        direction,
        base_asset_amount: size,
        base_asset_amount_filled: 0,
        price: price as u64,
        auction_start_price: price,
        auction_end_price: price,
        slot,
        market_index: 0,
        market_type: MarketType::Perp,
        ..Default::default()
    }
}

#[test]
fn dlob_market_order_sorting() {
    let _ = env_logger::try_init();
    let dlob = DLOB::default();
    let user = Pubkey::new_unique();
    let slot = 100;
    let order_size = 100;

    // bootstrap orderbook for market
    dlob.markets.entry(MarketId::perp(0)).or_insert(Orderbook {
        market_index: 0,
        market_tick_size: 10,
        ..Default::default()
    });

    // Insert bids in random order
    let mut order = create_test_order(1, OrderType::Market, Direction::Long, 0, order_size, slot);
    order.auction_start_price = 100_000;
    order.auction_end_price = 200_000;
    order.auction_duration = 10;
    dlob.insert_order(&user, order);
    let mut order = create_test_order(2, OrderType::Market, Direction::Long, 0, order_size, slot);
    order.auction_start_price = 150_000;
    order.auction_end_price = 200_000;
    order.auction_duration = 10;
    dlob.insert_order(&user, order);
    let mut order = create_test_order(3, OrderType::Market, Direction::Long, 0, order_size, slot);
    order.auction_start_price = 99_000;
    order.auction_end_price = 200_000;
    order.auction_duration = 10;
    dlob.insert_order(&user, order);

    // Insert asks in random order
    let mut order = create_test_order(4, OrderType::Market, Direction::Short, 0, order_size, slot);
    order.auction_start_price = 100_000;
    order.auction_end_price = 10_000;
    order.auction_duration = 10;
    dlob.insert_order(&user, order);
    let mut order = create_test_order(5, OrderType::Market, Direction::Short, 0, order_size, slot);
    order.auction_start_price = 99_000;
    order.auction_end_price = 10_000;
    order.auction_duration = 10;
    dlob.insert_order(&user, order);
    let mut order = create_test_order(6, OrderType::Market, Direction::Short, 0, order_size, slot);
    order.auction_start_price = 150_000;
    order.auction_end_price = 10_000;
    order.auction_duration = 5;
    dlob.insert_order(&user, order);

    dlob.update_slot_and_oracle_price(0, MarketType::Perp, slot, 0);
    let book = dlob
        .markets
        .get(&MarketId::new(0, MarketType::Perp))
        .unwrap();

    // Verify bids are sorted highest to lowest
    let market_tick_size = 10;
    let bids: Vec<u64> = book
        .market_orders
        .bids
        .iter()
        .map(|x| x.get_price(slot, 0, market_tick_size))
        .collect();
    assert_eq!(&bids, &[150_000, 100_000, 99_000]);
    // Verify asks are sorted lowest to hi
    let asks: Vec<u64> = book
        .market_orders
        .asks
        .iter()
        .map(|x| x.get_price(slot, 0, market_tick_size))
        .collect();
    assert_eq!(&asks, &[99_000, 100_000, 150_000]);
}

#[test]
fn dlob_limit_order_sorting() {
    let _ = env_logger::try_init();
    let dlob = DLOB::default();
    let user = Pubkey::new_unique();
    let slot = 100;

    // Insert bids in random order
    let mut order = create_test_order(1, OrderType::Limit, Direction::Long, 100, 1, slot);
    order.post_only = true;
    dlob.insert_order(&user, order);

    let mut order = create_test_order(2, OrderType::Limit, Direction::Long, 200, 1, slot);
    order.post_only = true;
    dlob.insert_order(&user, order);

    let mut order = create_test_order(3, OrderType::Limit, Direction::Long, 150, 1, slot);
    order.post_only = true;
    dlob.insert_order(&user, order);

    // Insert asks in random order
    let mut order = create_test_order(4, OrderType::Limit, Direction::Short, 300, 1, slot);
    order.post_only = true;
    dlob.insert_order(&user, order);

    let mut order = create_test_order(5, OrderType::Limit, Direction::Short, 250, 1, slot);
    order.post_only = true;
    dlob.insert_order(&user, order);

    let mut order = create_test_order(6, OrderType::Limit, Direction::Short, 350, 1, slot);
    order.post_only = true;
    dlob.insert_order(&user, order);

    let book = dlob
        .markets
        .get(&MarketId::new(0, MarketType::Perp))
        .unwrap();

    // Verify bids are sorted highest to lowest
    let bid_prices = book.get_limit_bids(1_000_000);
    assert_eq!(
        bid_prices.iter().map(|o| o.price).collect::<Vec<u64>>(),
        vec![200_u64, 150, 100]
    );

    // Verify asks are sorted lowest to highest
    let ask_prices = book.get_limit_asks(1_000_000);
    assert_eq!(
        ask_prices.iter().map(|o| o.price).collect::<Vec<u64>>(),
        vec![250_u64, 300, 350]
    );
}

#[test]
fn dlob_floating_limit_order_sorting() {
    let _ = env_logger::try_init();
    let dlob = DLOB::default();
    let user = Pubkey::new_unique();
    let slot = 100;

    // Insert bids in random order
    let mut order = create_test_order(1, OrderType::Limit, Direction::Long, 100, 1, slot);
    order.oracle_price_offset = 10;
    dlob.insert_order(&user, order);

    let mut order = create_test_order(2, OrderType::Limit, Direction::Long, 200, 1, slot);
    order.oracle_price_offset = 30;
    dlob.insert_order(&user, order);

    let mut order = create_test_order(3, OrderType::Limit, Direction::Long, 150, 1, slot);
    order.oracle_price_offset = 20;
    dlob.insert_order(&user, order);

    // Insert asks in random order
    let mut order = create_test_order(4, OrderType::Limit, Direction::Short, 300, 1, slot);
    order.oracle_price_offset = -30;
    dlob.insert_order(&user, order);

    let mut order = create_test_order(5, OrderType::Limit, Direction::Short, 250, 1, slot);
    order.oracle_price_offset = -20;
    dlob.insert_order(&user, order);

    let mut order = create_test_order(6, OrderType::Limit, Direction::Short, 350, 1, slot);
    order.oracle_price_offset = -10;
    dlob.insert_order(&user, order);

    dlob.update_slot_and_oracle_price(0, MarketType::Perp, slot, 0);
    let book = dlob
        .markets
        .get(&MarketId::new(0, MarketType::Perp))
        .unwrap();

    // Verify bids are sorted highest to lowest offset
    let bid_offsets: Vec<i32> = book
        .floating_limit_orders
        .bids
        .iter()
        .map(|(_, v)| v.offset_price)
        .collect();
    assert_eq!(bid_offsets, vec![30, 20, 10]);

    // Verify asks are sorted lowest to highest offset
    let ask_offsets: Vec<i32> = book
        .floating_limit_orders
        .asks
        .iter()
        .map(|(_, v)| v.offset_price)
        .collect();
    assert_eq!(ask_offsets, vec![-30, -20, -10]);
}

#[test]
fn dlob_oracle_order_sorting() {
    let _ = env_logger::try_init();
    let dlob = DLOB::default();

    dlob.markets.entry(MarketId::perp(0)).or_insert(Orderbook {
        market_index: 0,
        market_tick_size: 10,
        ..Default::default()
    });

    let user = Pubkey::new_unique();
    let slot = 1_000;
    let oracle_price = 100_000 as u64;
    let order_size = 100;

    // Insert bids in random order
    let mut order = create_test_order(1, OrderType::Oracle, Direction::Long, 0, order_size, slot);
    order.auction_start_price = 10_000;
    order.auction_end_price = 50_000;
    order.auction_duration = 10;
    dlob.insert_order(&user, order);

    let mut order = create_test_order(2, OrderType::Oracle, Direction::Long, 0, order_size, slot);
    order.auction_start_price = 40_000;
    order.auction_end_price = 50_000;
    order.auction_duration = 10;
    dlob.insert_order(&user, order);

    let mut order = create_test_order(3, OrderType::Oracle, Direction::Long, 0, order_size, slot);
    order.auction_start_price = 30_000;
    order.auction_end_price = 50_000;
    order.auction_duration = 10;
    dlob.insert_order(&user, order);

    // Insert asks in random order
    let mut order = create_test_order(4, OrderType::Oracle, Direction::Short, 0, order_size, slot);
    order.auction_start_price = 50_000;
    order.auction_end_price = 20_000;
    order.auction_duration = 10;
    dlob.insert_order(&user, order);

    let mut order = create_test_order(5, OrderType::Oracle, Direction::Short, 0, order_size, slot);
    order.auction_start_price = 40_000;
    order.auction_end_price = 30_000;
    order.auction_duration = 10;
    dlob.insert_order(&user, order);

    let mut order = create_test_order(6, OrderType::Oracle, Direction::Short, 0, order_size, slot);
    order.auction_start_price = 30_000;
    order.auction_end_price = 5_000;
    order.auction_duration = 10;
    dlob.insert_order(&user, order);

    // orderbook updated
    dlob.update_slot_and_oracle_price(0, MarketType::Perp, slot, oracle_price);

    let book = dlob
        .markets
        .get(&MarketId::new(0, MarketType::Perp))
        .unwrap();

    // Verify bids are sorted highest to lowest price
    let bids: Vec<u64> = book
        .oracle_orders
        .bids
        .iter()
        .map(|o| o.get_price(slot, oracle_price, 10))
        .collect();
    assert_eq!(&bids, &[140_000, 130_000, 110_000]);

    // Verify asks are sorted lowest to highest price
    let asks: Vec<u64> = book
        .oracle_orders
        .asks
        .iter()
        .map(|v| v.get_price(slot, oracle_price, 10))
        .collect();
    assert_eq!(&asks, &[130_000, 140_000, 150_000]);
}

#[test]
fn dlob_same_order_different_users() {
    let _ = env_logger::try_init();
    let dlob = DLOB::default();
    let user1 = Pubkey::new_unique();
    let user2 = Pubkey::new_unique();
    let slot = 100;

    // Create identical orders for different users
    let mut order1 = create_test_order(1, OrderType::Limit, Direction::Long, 100, 1, slot);
    order1.post_only = true;
    dlob.insert_order(&user1, order1);

    let mut order2 = create_test_order(1, OrderType::Limit, Direction::Long, 100, 1, slot);
    order2.post_only = true;
    dlob.insert_order(&user2, order2);

    let book = dlob
        .markets
        .get(&MarketId::new(0, MarketType::Perp))
        .unwrap();

    // Verify both orders are in the book
    let bid_prices: Vec<u64> = book
        .resting_limit_orders
        .bids
        .iter()
        .map(|(_, v)| v.get_price())
        .collect();
    assert_eq!(bid_prices, vec![100, 100]);

    // Verify the orders have different IDs
    let bid_ids: Vec<u64> = book
        .resting_limit_orders
        .bids
        .iter()
        .map(|(_, v)| v.id)
        .collect();
    assert_ne!(bid_ids[0], bid_ids[1]);
}

#[test]
fn dlob_l2_snapshot() {
    let _ = env_logger::try_init();
    let dlob = DLOB::default();
    let user = Pubkey::new_unique();
    let slot = 100_u64;
    let oracle_price = 1000;

    dlob.markets.entry(MarketId::perp(0)).or_insert(Orderbook {
        market_index: 0,
        market_tick_size: 1,
        ..Default::default()
    });

    // Insert resting limit orders
    let mut order = create_test_order(1, OrderType::Limit, Direction::Long, 1100, 2, slot);
    order.post_only = true;
    dlob.insert_order(&user, order);

    let mut order = create_test_order(2, OrderType::Limit, Direction::Short, 900, 3, slot);
    order.post_only = true;
    dlob.insert_order(&user, order);

    // Insert market orders (dynamic price)
    let mut order = create_test_order(3, OrderType::Market, Direction::Long, 0, 4, slot);
    order.auction_duration = 10;
    order.auction_start_price = 1050;
    order.auction_end_price = 1100;
    dlob.insert_order(&user, order);

    let mut order = create_test_order(4, OrderType::Market, Direction::Short, 0, 5, slot);
    order.auction_duration = 10;
    order.auction_start_price = 950;
    order.auction_end_price = 900;
    dlob.insert_order(&user, order);

    // Insert floating limit orders (dynamic price)
    let mut order = create_test_order(5, OrderType::Limit, Direction::Long, 0, 6, slot);
    order.oracle_price_offset = 100; // Will be 1100 with oracle_price
    dlob.insert_order(&user, order);

    let mut order = create_test_order(6, OrderType::Limit, Direction::Short, 0, 7, slot);
    order.oracle_price_offset = -100; // Will be 900 with oracle_price
    dlob.insert_order(&user, order);

    // Update slot and oracle price to calculate dynamic prices
    dlob.update_slot_and_oracle_price(0, MarketType::Perp, slot, oracle_price);

    // Get the L2 snapshot
    let l2book = dlob.get_l2_snapshot(0, MarketType::Perp).unwrap();

    // Verify bid prices and sizes
    // At 1100: 2 (resting limit) + 6 (floating limit) = 8
    dbg!(&l2book.bids);
    assert_eq!(l2book.bids.get(&1100), Some(&8));
    // At 1050: 4 (market)
    assert_eq!(l2book.bids.get(&1050), Some(&4));

    // Verify ask prices and sizes
    // At 900: 3 (resting limit) + 7 (floating limit) = 10
    dbg!(&l2book.asks);
    assert_eq!(l2book.asks.get(&900), Some(&10));
    // At 950: 5 (market)
    assert_eq!(l2book.asks.get(&950), Some(&5));

    // Verify no other prices exist
    assert_eq!(l2book.bids.len(), 2);
    assert_eq!(l2book.asks.len(), 2);

    // Test snapshot updates
    // Add a new limit order
    let mut order = create_test_order(7, OrderType::Limit, Direction::Long, 1075, 8, slot);
    order.post_only = true;
    dlob.insert_order(&user, order);

    // Get updated L2 snapshot
    dlob.update_slot_and_oracle_price(0, MarketType::Perp, slot, oracle_price);
    let l2book = dlob.get_l2_snapshot(0, MarketType::Perp).unwrap();

    // Verify new order was added
    assert_eq!(l2book.bids.get(&1075), Some(&8));
    assert_eq!(l2book.bids.len(), 3);

    // Modify an existing order
    let old_order = create_test_order(1, OrderType::Limit, Direction::Long, 1100, 2, slot);
    let mut new_order = create_test_order(1, OrderType::Limit, Direction::Long, 1100, 4, slot); // Changed size from 2 to 4
    new_order.post_only = true;
    dlob.update_order(&user, slot, new_order, old_order);

    // Get updated L2 snapshot
    dlob.update_slot_and_oracle_price(0, MarketType::Perp, slot, oracle_price);
    let l2book = dlob.get_l2_snapshot(0, MarketType::Perp).unwrap();

    // Verify order was updated
    assert_eq!(l2book.bids.get(&1100), Some(&10)); // 4 (updated) + 6 (floating limit) = 10
    assert_eq!(l2book.bids.len(), 3);

    // Remove an order
    let old_order = create_test_order(3, OrderType::Market, Direction::Long, 0, 4, slot);
    let mut new_order = create_test_order(3, OrderType::Market, Direction::Long, 1050, 4, slot);
    new_order.base_asset_amount_filled = old_order.base_asset_amount; // Set filled amount equal to total amount
    dlob.update_order(&user, slot, new_order, old_order);

    // Get updated L2 snapshot
    dlob.update_slot_and_oracle_price(0, MarketType::Perp, slot, oracle_price);
    let l2book = dlob.get_l2_snapshot(0, MarketType::Perp).unwrap();

    // Verify order was removed
    assert_eq!(l2book.bids.get(&1050), None);
    assert_eq!(l2book.bids.len(), 2);
}

#[test]
fn dlob_find_crosses_for_taker_order_full_fill() {
    let _ = env_logger::try_init();
    let dlob = DLOB::default();
    let user = Pubkey::new_unique();
    let slot = 100;
    let oracle_price = 1000;

    // Insert resting limit orders
    let mut order = create_test_order(1, OrderType::Limit, Direction::Short, 900, 5, slot);
    order.post_only = true;
    dlob.insert_order(&user, order);

    let mut order = create_test_order(2, OrderType::Limit, Direction::Short, 950, 3, slot);
    order.post_only = true;
    dlob.insert_order(&user, order);

    // Create taker order to buy 7 units at 1000
    let taker_order = TakerOrder {
        price: 1000,
        size: 7,
        direction: Direction::Long,
        market_index: 0,
        market_type: MarketType::Perp,
    };

    let result = dlob.find_crosses_for_taker_order(slot, oracle_price, taker_order, None);

    // Should fill both orders, 5 from first order and 2 from second
    assert_eq!(result.orders.len(), 2);
    assert_eq!(
        result.orders[0],
        (
            OrderMetadata {
                max_ts: 30,
                order_id: 1,
                user,
                kind: OrderKind::Limit
            },
            900,
            5
        )
    );
    assert_eq!(
        result.orders[1],
        (
            OrderMetadata {
                max_ts: 30,
                order_id: 2,
                user,
                kind: OrderKind::Limit
            },
            950,
            2
        )
    );
    assert!(!result.is_partial);
}

#[test]
fn dlob_find_crosses_for_taker_order_partial_fill() {
    let _ = env_logger::try_init();
    let dlob = DLOB::default();
    let user = Pubkey::new_unique();
    let slot = 100;
    let oracle_price = 1000;

    // Insert resting limit orders
    let mut order = create_test_order(1, OrderType::Limit, Direction::Short, 900, 3, slot);
    order.post_only = true;
    dlob.insert_order(&user, order);

    // Create taker order to buy 5 units at 1000
    let taker_order = TakerOrder {
        price: 1000,
        size: 5,
        direction: Direction::Long,
        market_index: 0,
        market_type: MarketType::Perp,
    };

    let result = dlob.find_crosses_for_taker_order(slot, oracle_price, taker_order, None);

    // Should only fill 3 units from the first order
    assert_eq!(result.orders.len(), 1);
    assert_eq!(
        result.orders[0],
        (
            OrderMetadata {
                max_ts: 30,
                order_id: 1,
                user,
                kind: OrderKind::Limit
            },
            900,
            3
        )
    );
    assert!(result.is_partial);
}

#[test]
fn dlob_find_crosses_for_taker_order_no_cross() {
    let _ = env_logger::try_init();
    let dlob = DLOB::default();
    let user = Pubkey::new_unique();
    let slot = 100;
    let oracle_price = 1000;

    // Insert resting limit orders
    let mut order = create_test_order(1, OrderType::Limit, Direction::Short, 1100, 5, slot);
    order.post_only = true;
    dlob.insert_order(&user, order);

    // Create taker order to buy at 1000
    let taker_order = TakerOrder {
        price: 1000,
        size: 5,
        direction: Direction::Long,
        market_index: 0,
        market_type: MarketType::Perp,
    };

    let result = dlob.find_crosses_for_taker_order(slot, oracle_price, taker_order, None);

    // Should not fill any orders
    assert_eq!(result.orders.len(), 0);
    assert!(result.is_partial);
}

#[test]
fn dlob_find_crosses_for_taker_order_vamm_cross() {
    let _ = env_logger::try_init();
    let dlob = DLOB::default();
    let user = Pubkey::new_unique();
    let slot = 100;
    let oracle_price = 1000;

    // Insert resting limit orders
    let mut order = create_test_order(1, OrderType::Limit, Direction::Short, 1100, 5, slot);
    order.post_only = true;
    dlob.insert_order(&user, order);

    // Create taker order to buy at 1000
    let taker_order = TakerOrder {
        price: 1000,
        size: 5,
        direction: Direction::Long,
        market_index: 0,
        market_type: MarketType::Perp,
    };

    let result = dlob.find_crosses_for_taker_order(slot, oracle_price, taker_order, Some(999));

    // Should not fill any orders
    assert!(result.has_vamm_cross);
    assert_eq!(result.orders.len(), 0);
    assert!(!result.is_empty());
    assert!(result.is_partial);
}

#[test]
fn dlob_find_crosses_for_taker_order_floating_limit() {
    let _ = env_logger::try_init();
    let dlob = DLOB::default();
    let user = Pubkey::new_unique();
    let slot = 100;
    let oracle_price = 1000;

    // Insert floating limit order with -50 offset
    let mut order = create_test_order(1, OrderType::Limit, Direction::Short, 0, 5, slot);
    order.oracle_price_offset = -50; // Will be 950 with oracle_price
    order.post_only = true;
    dlob.insert_order(&user, order);

    // Create taker order to buy at 1000
    let taker_order = TakerOrder {
        price: 1000,
        size: 5,
        direction: Direction::Long,
        market_index: 0,
        market_type: MarketType::Perp,
    };

    let result = dlob.find_crosses_for_taker_order(slot, oracle_price, taker_order, None);

    // Should fill the floating limit order
    assert_eq!(result.orders.len(), 1);
    assert_eq!(
        result.orders[0],
        (
            OrderMetadata {
                max_ts: 30,
                order_id: 1,
                user,
                kind: OrderKind::FloatingLimit,
            },
            950, // oracle_price + oracle_price_offset
            5
        )
    );
    assert!(!result.is_partial);
}

#[test]
fn dlob_find_crosses_for_taker_order_price_priority() {
    let _ = env_logger::try_init();
    let dlob = DLOB::default();
    let user = Pubkey::new_unique();
    let slot = 100;
    let oracle_price = 1000;

    // Insert resting limit orders at different prices
    let mut order = create_test_order(1, OrderType::Limit, Direction::Short, 950, 3, slot);
    order.post_only = true;
    dlob.insert_order(&user, order);

    let mut order = create_test_order(2, OrderType::Limit, Direction::Short, 900, 3, slot);
    order.post_only = true;
    dlob.insert_order(&user, order);

    // Create taker order to buy 5 units at 1000
    let taker_order = TakerOrder {
        price: 1000,
        size: 5,
        direction: Direction::Long,
        market_index: 0,
        market_type: MarketType::Perp,
    };

    let result = dlob.find_crosses_for_taker_order(slot, oracle_price, taker_order, None);

    // Should fill the better price first (900)
    assert_eq!(result.orders.len(), 2);
    assert_eq!(
        result.orders[0],
        (
            OrderMetadata {
                max_ts: 30,
                order_id: 2,
                user,
                kind: OrderKind::Limit
            },
            900,
            3
        )
    );
    assert_eq!(
        result.orders[1],
        (
            OrderMetadata {
                max_ts: 30,
                order_id: 1,
                user,
                kind: OrderKind::Limit
            },
            950,
            2
        )
    );
    assert!(!result.is_partial);
}

#[test]
fn dlob_auction_expiry_market_orders() {
    let _ = env_logger::try_init();
    let dlob = DLOB::default();
    let user = Pubkey::new_unique();
    let slot = 100;
    let oracle_price = 1000;

    // Insert market orders with different auction durations
    let mut order = create_test_order(1, OrderType::Limit, Direction::Long, 1100, 2, slot);
    order.auction_duration = 5; // Will expire at slot 105
    dlob.insert_order(&user, order);

    let mut order = create_test_order(2, OrderType::Limit, Direction::Short, 900, 3, slot);
    order.auction_duration = 10; // Will expire at slot 110
    dlob.insert_order(&user, order);

    // Update to slot 104 - no orders should expire
    dlob.update_slot_and_oracle_price(0, MarketType::Perp, 104, oracle_price);
    let book = dlob
        .markets
        .get(&MarketId::new(0, MarketType::Perp))
        .unwrap();
    assert_eq!(book.market_orders.bids.len(), 1);
    assert_eq!(book.market_orders.asks.len(), 1);
    assert_eq!(book.resting_limit_orders.bids.len(), 0);
    assert_eq!(book.resting_limit_orders.asks.len(), 0);
    drop(book);

    // Update to slot 105 - first order should expire
    dlob.update_slot_and_oracle_price(0, MarketType::Perp, 105, oracle_price);
    let book = dlob
        .markets
        .get(&MarketId::new(0, MarketType::Perp))
        .unwrap();
    assert_eq!(book.market_orders.bids.len(), 0);
    assert_eq!(book.market_orders.asks.len(), 1);
    assert_eq!(book.resting_limit_orders.bids.len(), 1);
    assert_eq!(book.resting_limit_orders.asks.len(), 0);
    drop(book);

    // Update to slot 110 - second order should expire
    dlob.update_slot_and_oracle_price(0, MarketType::Perp, 110, oracle_price);
    let book = dlob
        .markets
        .get(&MarketId::new(0, MarketType::Perp))
        .unwrap();
    assert_eq!(book.market_orders.bids.len(), 0);
    assert_eq!(book.market_orders.asks.len(), 0);
    assert_eq!(book.resting_limit_orders.bids.len(), 1);
    assert_eq!(book.resting_limit_orders.asks.len(), 1);
}

#[test]
fn dlob_auction_expiry_oracle_orders() {
    let _ = env_logger::try_init();
    let dlob = DLOB::default();
    let user = Pubkey::new_unique();
    let slot = 100;
    let oracle_price = 1000;

    // Insert oracle orders with different auction durations
    let mut order = create_test_order(1, OrderType::Limit, Direction::Long, 0, 2, slot);
    order.auction_duration = 5; // Will expire at slot 105
    order.oracle_price_offset = 100;
    dlob.insert_order(&user, order);

    let mut order = create_test_order(2, OrderType::Limit, Direction::Short, 0, 3, slot);
    order.auction_duration = 10; // Will expire at slot 110
    order.oracle_price_offset = -100;
    dlob.insert_order(&user, order);

    // Update to slot 104 - no orders should expire
    dlob.update_slot_and_oracle_price(0, MarketType::Perp, 104, oracle_price);
    let book = dlob
        .markets
        .get(&MarketId::new(0, MarketType::Perp))
        .unwrap();
    assert_eq!(book.oracle_orders.bids.len(), 1);
    assert_eq!(book.oracle_orders.asks.len(), 1);
    assert_eq!(book.floating_limit_orders.bids.len(), 0);
    assert_eq!(book.floating_limit_orders.asks.len(), 0);
    drop(book);
    // Update to slot 105 - first order should expire
    dlob.update_slot_and_oracle_price(0, MarketType::Perp, 105, oracle_price);
    let book = dlob
        .markets
        .get(&MarketId::new(0, MarketType::Perp))
        .unwrap();
    assert_eq!(book.oracle_orders.bids.len(), 0);
    assert_eq!(book.oracle_orders.asks.len(), 1);
    assert_eq!(book.floating_limit_orders.bids.len(), 1);
    assert_eq!(book.floating_limit_orders.asks.len(), 0);
    drop(book);

    // Update to slot 110 - second order should expire
    dlob.update_slot_and_oracle_price(0, MarketType::Perp, 110, oracle_price);
    let book = dlob
        .markets
        .get(&MarketId::new(0, MarketType::Perp))
        .unwrap();
    assert_eq!(book.oracle_orders.bids.len(), 0);
    assert_eq!(book.oracle_orders.asks.len(), 0);
    assert_eq!(book.floating_limit_orders.bids.len(), 1);
    assert_eq!(book.floating_limit_orders.asks.len(), 1);
}

#[test]
fn dlob_auction_expiry_non_limit_orders() {
    let _ = env_logger::try_init();
    let _ = env_logger::try_init();
    let dlob = DLOB::default();
    let user = Pubkey::new_unique();
    let slot = 100;
    let oracle_price = 1000;

    // Insert market orders that are not limit orders
    let mut order = create_test_order(1, OrderType::Market, Direction::Long, 1100, 2, slot);
    order.auction_duration = 5; // Will expire at slot 105
    dlob.insert_order(&user, order);

    let mut order = create_test_order(2, OrderType::Market, Direction::Short, 900, 3, slot);
    order.auction_duration = 10; // Will expire at slot 110
    dlob.insert_order(&user, order);

    // Update to slot 104 - no orders should expire
    dlob.update_slot_and_oracle_price(0, MarketType::Perp, 104, oracle_price);
    let book = dlob
        .markets
        .get(&MarketId::new(0, MarketType::Perp))
        .unwrap();
    assert_eq!(book.market_orders.bids.len(), 1);
    assert_eq!(book.market_orders.asks.len(), 1);
    assert_eq!(book.resting_limit_orders.bids.len(), 0);
    assert_eq!(book.resting_limit_orders.asks.len(), 0);
    drop(book);

    // Update to slot 105 - first order should expire and be removed
    dlob.update_slot_and_oracle_price(0, MarketType::Perp, 105, oracle_price);
    let book = dlob
        .markets
        .get(&MarketId::new(0, MarketType::Perp))
        .unwrap();
    assert_eq!(book.market_orders.bids.len(), 0);
    assert_eq!(book.market_orders.asks.len(), 1);
    assert_eq!(book.resting_limit_orders.bids.len(), 0);
    assert_eq!(book.resting_limit_orders.asks.len(), 0);
    drop(book);

    // Update to slot 110 - second order should expire and be removed
    dlob.update_slot_and_oracle_price(0, MarketType::Perp, 110, oracle_price);
    let book = dlob
        .markets
        .get(&MarketId::new(0, MarketType::Perp))
        .unwrap();
    assert_eq!(book.market_orders.bids.len(), 0);
    assert_eq!(book.market_orders.asks.len(), 0);
    assert_eq!(book.resting_limit_orders.bids.len(), 0);
    assert_eq!(book.resting_limit_orders.asks.len(), 0);
}

#[test]
fn dlob_auction_expiry_mixed_orders() {
    let _ = env_logger::try_init();
    let dlob = DLOB::default();
    let user = Pubkey::new_unique();
    let slot = 100;
    let oracle_price = 1000;

    // Insert a mix of market and oracle orders with different durations
    let mut order = create_test_order(1, OrderType::Limit, Direction::Long, 1100, 2, slot);
    order.auction_duration = 5;
    dlob.insert_order(&user, order);

    let mut order = create_test_order(2, OrderType::Limit, Direction::Short, 0, 3, slot);
    order.auction_duration = 5;
    order.oracle_price_offset = -100;
    dlob.insert_order(&user, order);

    let mut order = create_test_order(3, OrderType::Market, Direction::Long, 1100, 2, slot);
    order.auction_duration = 5;
    dlob.insert_order(&user, order);

    let mut order = create_test_order(4, OrderType::Market, Direction::Short, 0, 3, slot);
    order.auction_duration = 5;
    dlob.insert_order(&user, order);

    // Update to slot 105 - all orders should expire
    dlob.update_slot_and_oracle_price(0, MarketType::Perp, 105, oracle_price);
    let book = dlob
        .markets
        .get(&MarketId::new(0, MarketType::Perp))
        .unwrap();

    // Market orders should be moved to resting limit or removed
    assert_eq!(book.market_orders.bids.len(), 0);
    assert_eq!(book.market_orders.asks.len(), 0);

    // Oracle orders should be moved to floating limit or removed
    assert_eq!(book.oracle_orders.bids.len(), 0);
    assert_eq!(book.oracle_orders.asks.len(), 0);
    assert_eq!(book.floating_limit_orders.bids.len(), 0);
    assert_eq!(book.floating_limit_orders.asks.len(), 1);
}

#[test]
fn dlob_zero_size_order_handling() {
    let _ = env_logger::try_init();
    let dlob = DLOB::default();
    let user = Pubkey::new_unique();
    let slot = 100;
    let oracle_price = 1000;

    // Test 1: Update order to be fully filled (should be removed)
    let order = create_test_order(2, OrderType::Limit, Direction::Long, 1000, 10, slot);
    dlob.insert_order(&user, order);
    let mut new_order = order.clone();
    new_order.base_asset_amount_filled = 10; // Fully fill it
    dlob.update_order(&user, slot, new_order, order);

    // Verify no orders in book
    let book = dlob
        .markets
        .get(&MarketId::new(0, MarketType::Perp))
        .unwrap();
    assert_eq!(book.resting_limit_orders.bids.len(), 0);

    // Test 2: Insert fully filled order (should be skipped)
    let mut order = create_test_order(1, OrderType::Limit, Direction::Long, 1000, 10, slot);
    order.base_asset_amount_filled = 10; // Fully filled
    dlob.insert_order(&user, order);

    // Verify order was removed
    assert_eq!(book.resting_limit_orders.bids.len(), 0);

    // Test 3: Auction order expiring with zero size (should be removed)
    let mut order = create_test_order(3, OrderType::Limit, Direction::Long, 1000, 10, slot);
    order.auction_duration = 5;
    order.base_asset_amount_filled = 10; // Fully filled
    dlob.insert_order(&user, order);

    // Update to slot after auction end
    drop(book); // release lock
    dlob.update_slot_and_oracle_price(0, MarketType::Perp, slot + 6, oracle_price);

    let book = dlob
        .markets
        .get(&MarketId::new(0, MarketType::Perp))
        .unwrap();

    // Verify no orders in book
    assert_eq!(book.resting_limit_orders.bids.len(), 0);
    assert_eq!(book.market_orders.bids.len(), 0);
}

#[test]
fn dlob_zero_size_auction_orders() {
    let _ = env_logger::try_init();
    let dlob = DLOB::default();
    let user = Pubkey::new_unique();
    let slot = 100;
    let oracle_price = 1000;

    // Test 1: Market order auction expiring with zero size
    let mut order = create_test_order(1, OrderType::Limit, Direction::Long, 1000, 10, slot);
    order.auction_duration = 5;
    order.base_asset_amount_filled = 10; // Fully filled
    dlob.insert_order(&user, order);

    // Update to slot after auction end
    dlob.update_slot_and_oracle_price(0, MarketType::Perp, slot + 6, oracle_price);

    {
        // Verify no orders in book
        let book = dlob
            .markets
            .get(&MarketId::new(0, MarketType::Perp))
            .unwrap();
        assert_eq!(book.resting_limit_orders.bids.len(), 0);
        assert_eq!(book.market_orders.bids.len(), 0);
    }

    // Test 2: Oracle order auction expiring with zero size
    let mut order = create_test_order(2, OrderType::Limit, Direction::Long, 0, 10, slot);
    order.auction_duration = 5;
    order.oracle_price_offset = 100;
    order.base_asset_amount_filled = 10; // Fully filled
    dlob.insert_order(&user, order);

    // Update to slot after auction end
    dlob.update_slot_and_oracle_price(0, MarketType::Perp, slot + 6, oracle_price);

    // Verify no orders in book
    {
        let book = dlob
            .markets
            .get(&MarketId::new(0, MarketType::Perp))
            .unwrap();
        assert_eq!(book.floating_limit_orders.bids.len(), 0);
        assert_eq!(book.oracle_orders.bids.len(), 0);
    }

    // Test 3: Mixed size auction orders
    let mut order = create_test_order(3, OrderType::Limit, Direction::Long, 1000, 10, slot);
    order.auction_duration = 5;
    order.base_asset_amount_filled = 10; // Fully filled
    dlob.insert_order(&user, order);

    let mut order = create_test_order(4, OrderType::Limit, Direction::Long, 1000, 10, slot);
    order.auction_duration = 5;
    order.base_asset_amount_filled = 5; // Partially filled
    dlob.insert_order(&user, order);

    // Update to slot after auction end
    dlob.update_slot_and_oracle_price(0, MarketType::Perp, slot + 6, oracle_price);

    // Verify only non-zero size order was converted to limit order
    let book = dlob
        .markets
        .get(&MarketId::new(0, MarketType::Perp))
        .unwrap();
    assert_eq!(book.resting_limit_orders.bids.len(), 1);
    assert_eq!(book.market_orders.bids.len(), 0);
}

#[test]
fn dlob_find_crosses_for_auctions_market_orders() {
    let _ = env_logger::try_init();
    let dlob = DLOB::default();
    let market_index = 0;
    let market_type = MarketType::Perp;
    let slot = 100;
    let oracle_price = 1000;

    dlob.markets.entry(MarketId::perp(0)).or_insert(Orderbook {
        market_index,
        market_tick_size: 10,
        ..Default::default()
    });

    // Insert a resting limit ask at price 1000
    let limit_order = create_test_order(1, OrderType::Limit, Direction::Short, 1000, 100, slot);
    dlob.insert_order(&Pubkey::new_unique(), limit_order);

    // Insert a market bid that should cross
    let mut market_order = create_test_order(
        2,
        OrderType::Market,
        Direction::Long, // This is correct as it's a bid
        1100,            // Higher price than limit ask
        50,
        slot,
    );
    market_order.auction_duration = 10;
    market_order.auction_start_price = 1100;
    market_order.auction_end_price = 1200;
    dlob.insert_order(&Pubkey::new_unique(), market_order);

    let crosses = dlob.find_crosses_for_auctions(
        market_index,
        market_type,
        slot,
        oracle_price,
        oracle_price,
        None,
    );
    assert_eq!(crosses.crosses.len(), 1);
    assert!(!crosses.crosses[0].1.is_empty());
    assert_eq!(crosses.crosses[0].1.orders.len(), 1);
    assert_eq!(crosses.crosses[0].1.orders[0].2, 50); // Fill size should be 50
}

#[test]
fn dlob_find_crosses_for_auctions_oracle_orders() {
    let _ = env_logger::try_init();
    let dlob = DLOB::default();
    let market_index = 0;
    let market_type = MarketType::Perp;
    let slot = 100;
    let oracle_price = 1000;

    // Insert a resting limit bid at price 1000
    let limit_order = create_test_order(1, OrderType::Limit, Direction::Long, 1000, 100, slot);
    dlob.insert_order(&Pubkey::new_unique(), limit_order);

    // Insert an oracle ask that should cross (price will be 900)
    let oracle_order = create_test_order(
        2,
        OrderType::Oracle,
        Direction::Short,
        -100, // 100 below oracle price
        50,
        slot,
    );
    dlob.insert_order(&Pubkey::new_unique(), oracle_order);

    let crosses = dlob.find_crosses_for_auctions(
        market_index,
        market_type,
        slot,
        oracle_price,
        oracle_price,
        None,
    );
    assert_eq!(crosses.crosses.len(), 1);
    assert!(!crosses.crosses[0].1.is_empty());
    assert_eq!(crosses.crosses[0].1.orders.len(), 1);
    assert_eq!(crosses.crosses[0].1.orders[0].2, 50); // Fill size should be 50
}

#[test]
fn dlob_find_crosses_for_auctions_no_crosses() {
    let _ = env_logger::try_init();
    let dlob = DLOB::default();
    let market_index = 0;
    let market_type = MarketType::Perp;
    let slot = 100;
    let oracle_price = 1000;

    // Insert a resting limit ask at price 1000
    let limit_order = create_test_order(1, OrderType::Limit, Direction::Short, 1000, 100, slot);
    dlob.insert_order(&Pubkey::new_unique(), limit_order);

    // Insert a market bid that shouldn't cross (lower price)
    let market_order = create_test_order(
        2,
        OrderType::Market,
        Direction::Long, // This is correct as it's a bid
        900,             // Lower price than limit ask
        50,
        slot,
    );
    dlob.insert_order(&Pubkey::new_unique(), market_order);

    let crosses = dlob.find_crosses_for_auctions(
        market_index,
        market_type,
        slot,
        oracle_price,
        oracle_price,
        None,
    );
    assert!(crosses.crosses.is_empty());
}

#[test]
fn dlob_find_crosses_for_auctions_comprehensive() {
    let _ = env_logger::try_init();
    let dlob = DLOB::default();
    let market_index = 0;
    let market_type = MarketType::Perp;
    let slot = 100;
    let oracle_price = 1_000;

    dlob.markets.entry(MarketId::perp(0)).or_insert(Orderbook {
        market_index,
        market_tick_size: 5,
        ..Default::default()
    });

    // Insert resting limit orders
    let limit_ask_1 = create_test_order(1, OrderType::Limit, Direction::Short, 1000, 50, slot);
    let limit_ask_2 = create_test_order(2, OrderType::Limit, Direction::Short, 1100, 100, slot);
    let limit_bid_1 = create_test_order(3, OrderType::Limit, Direction::Long, 900, 75, slot);
    let limit_bid_2 = create_test_order(4, OrderType::Limit, Direction::Long, 800, 25, slot);

    dlob.insert_order(&Pubkey::new_unique(), limit_ask_1);
    dlob.insert_order(&Pubkey::new_unique(), limit_ask_2);
    dlob.insert_order(&Pubkey::new_unique(), limit_bid_1);
    dlob.insert_order(&Pubkey::new_unique(), limit_bid_2);

    // Add a large market bid that will cross both limit asks
    let mut market_bid_1 = create_test_order(5, OrderType::Market, Direction::Long, 0, 75, slot);
    market_bid_1.auction_duration = 10;
    market_bid_1.auction_start_price = 1200;
    market_bid_1.auction_end_price = 1300;

    let mut market_ask_1 = create_test_order(7, OrderType::Market, Direction::Short, 0, 20, slot); // Should cross with limit_bid_1
    market_ask_1.auction_duration = 10;
    market_ask_1.auction_start_price = 800;
    market_ask_1.auction_end_price = 700;

    dlob.insert_order(&Pubkey::new_unique(), market_bid_1);
    dlob.insert_order(&Pubkey::new_unique(), market_ask_1);

    // Insert oracle orders (some should cross, some shouldn't)
    let mut oracle_bid_1 = create_test_order(9, OrderType::Oracle, Direction::Long, 0, 35, slot); // Price 1100, should cross with limit_ask_1
    oracle_bid_1.auction_duration = 10;
    oracle_bid_1.auction_start_price = 1100;
    oracle_bid_1.auction_end_price = 1200;

    let mut oracle_bid_2 = create_test_order(10, OrderType::Oracle, Direction::Long, 0, 45, slot); // Price 800, shouldn't cross
    oracle_bid_2.auction_duration = 10;
    oracle_bid_2.auction_start_price = 800;
    oracle_bid_2.auction_end_price = 900;

    let mut oracle_ask_1 = create_test_order(11, OrderType::Oracle, Direction::Short, 0, 25, slot); // Price 850, should cross with limit_bid_1
    oracle_ask_1.auction_duration = 10;
    oracle_ask_1.auction_start_price = 850;
    oracle_ask_1.auction_end_price = 750;

    let mut oracle_ask_2 = create_test_order(12, OrderType::Oracle, Direction::Short, 0, 55, slot); // Price 1200, shouldn't cross
    oracle_ask_2.auction_duration = 10;
    oracle_ask_2.auction_start_price = 1200;
    oracle_ask_2.auction_end_price = 1100;

    dlob.insert_order(&Pubkey::new_unique(), oracle_bid_1);
    dlob.insert_order(&Pubkey::new_unique(), oracle_bid_2);
    dlob.insert_order(&Pubkey::new_unique(), oracle_ask_1);
    dlob.insert_order(&Pubkey::new_unique(), oracle_ask_2);

    let crosses = dlob.find_crosses_for_auctions(
        market_index,
        market_type,
        slot,
        oracle_price,
        oracle_price,
        None,
    );
    dbg!(&crosses);

    // Should find 4 crosses:
    // 1. oracle_bid_1 (35) crossing limit_ask_1 (35)
    // 2. oracle_bid_2 (45) crossing limit_ask_1 (15) and limit_ask_2 (30)
    // 3. market_bid_1 (75) crossing limit_ask_1 (15) and limit_ask_2 (60)
    // 4. market_ask_1 (20) crossing limit_bid_1 (20)

    let expected_crosses = vec![
        (9, vec![(1, 35)]),
        (10, vec![(1, 15), (2, 30)]),
        (5, vec![(1, 15), (2, 60)]),
        (7, vec![(3, 20)]),
    ];

    let actual_crosses: Vec<_> = crosses
        .crosses
        .iter()
        .map(|(meta, maker_crosses)| {
            (
                meta.order_id,
                maker_crosses
                    .orders
                    .iter()
                    .map(|(m, _, size)| (m.order_id, *size))
                    .collect::<Vec<_>>(),
            )
        })
        .collect();

    assert_eq!(expected_crosses.len(), actual_crosses.len());
    for expected in &expected_crosses {
        assert!(
            actual_crosses.contains(expected),
            "Missing cross: {:?}",
            expected
        );
    }
}

#[test]
fn dlob_trigger_order_transitions() {
    use crate::types::OrderTriggerCondition;
    let _ = env_logger::try_init();
    let dlob = DLOB::default();
    let user = Pubkey::new_unique();
    let slot = 100;

    // --- Insert TriggerMarket order (Above) ---
    let mut order = create_test_order(1, OrderType::TriggerMarket, Direction::Long, 0, 10, slot);
    order.trigger_price = 950;
    order.trigger_condition = OrderTriggerCondition::Above;
    dlob.insert_order(&user, order);
    {
        let book = dlob
            .markets
            .get(&MarketId::new(0, MarketType::Perp))
            .unwrap();
        assert_eq!(book.trigger_orders.bids.len(), 1);
        assert_eq!(book.market_orders.bids.len(), 0);
        assert_eq!(book.oracle_orders.bids.len(), 0);
    }

    // --- Update to triggered (should move to market_orders or oracle_orders) ---
    let mut triggered_order = order;
    triggered_order.trigger_condition = OrderTriggerCondition::TriggeredAbove;
    // Set oracle trigger flag for oracle-triggered market
    triggered_order.bit_flags |= Order::ORACLE_TRIGGER_MARKET_FLAG;
    dlob.update_order(&user, slot, triggered_order, order);
    {
        let book = dlob
            .markets
            .get(&MarketId::new(0, MarketType::Perp))
            .unwrap();
        // Should be removed from trigger_orders
        assert_eq!(book.trigger_orders.bids.len(), 0);
        // Should be in oracle_orders (oracle trigger flag set)
        assert_eq!(book.oracle_orders.bids.len(), 1);
        assert_eq!(book.market_orders.bids.len(), 0);
    }

    // --- Remove triggered order ---
    // Advance slot to ensure auction is completed
    let advanced_slot = slot + order.auction_duration as u64;
    dlob.remove_order(&user, advanced_slot, triggered_order);
    {
        let book = dlob
            .markets
            .get(&MarketId::new(0, MarketType::Perp))
            .unwrap();
        assert_eq!(book.oracle_orders.bids.len(), 0);
        assert_eq!(book.market_orders.bids.len(), 0);
        assert_eq!(book.trigger_orders.bids.len(), 0);
    }

    // --- Insert TriggerLimit order (Below) ---
    let mut order2 = create_test_order(2, OrderType::TriggerLimit, Direction::Short, 1049, 5, slot);
    order2.trigger_price = 1050;
    order2.trigger_condition = OrderTriggerCondition::Below;
    dlob.insert_order(&user, order2);
    {
        let book = dlob
            .markets
            .get(&MarketId::new(0, MarketType::Perp))
            .unwrap();
        assert_eq!(book.trigger_orders.asks.len(), 1);
        assert_eq!(book.market_orders.asks.len(), 0);
    }

    // --- Update to triggered (should move to market_orders) ---
    let mut triggered_order2 = order2;
    triggered_order2.trigger_condition = OrderTriggerCondition::TriggeredBelow;
    triggered_order2.auction_duration = 5;
    triggered_order2.auction_start_price = 1050;
    triggered_order2.auction_end_price = 1048;
    dlob.update_order(&user, slot + 1, triggered_order2, order2);
    {
        let book = dlob
            .markets
            .get(&MarketId::new(0, MarketType::Perp))
            .unwrap();
        assert_eq!(book.trigger_orders.asks.len(), 0);
        assert_eq!(book.market_orders.asks.len(), 1);
    }

    // --- Remove triggered limit order ---
    // Advance slot to ensure auction is completed
    let auction_complete_slot = slot + triggered_order2.auction_duration as u64 + 1;
    dlob.update_slot_and_oracle_price(0, MarketType::Perp, auction_complete_slot, 0);
    dlob.remove_order(&user, auction_complete_slot, triggered_order2);
    {
        let book = dlob
            .markets
            .get(&MarketId::new(0, MarketType::Perp))
            .unwrap();
        assert_eq!(book.market_orders.asks.len(), 0);
        assert_eq!(book.trigger_orders.asks.len(), 0);
    }
}

#[test]
fn dlob_metadata_consistency_after_auction_expiry_and_removal() {
    let _ = env_logger::try_init();
    let dlob = DLOB::default();
    let user = Pubkey::new_unique();
    let slot = 100;
    let order_size = 100;
    let oracle_price = 150_000;

    // bootstrap orderbook for market
    dlob.markets.entry(MarketId::perp(0)).or_insert(Orderbook {
        market_index: 0,
        market_tick_size: 10,
        ..Default::default()
    });

    // Create a limit order with auction that will expire
    let mut order = create_test_order(1, OrderType::Limit, Direction::Long, 0, order_size, slot);
    order.auction_start_price = 100_000;
    order.auction_end_price = 200_000;
    order.auction_duration = 5; // Will expire at slot 105
    order.post_only = false; // This makes it a LimitAuction

    // Insert the order
    dlob.insert_order(&user, order);

    // Verify initial state - order should be in market_orders with LimitAuction metadata
    let order_id = crate::dlob::util::order_hash(&user, 1);
    {
        let metadata = dlob.metadata.get(&order_id).unwrap();
        assert_eq!(metadata.kind, OrderKind::LimitAuction);
    } // Drop metadata reference before accessing orderbook

    {
        let book = dlob.markets.get(&MarketId::perp(0)).unwrap();
        assert_eq!(book.market_orders.bids.len(), 1);
        assert_eq!(book.resting_limit_orders.bids.len(), 0);
    } // Drop book reference

    // Advance slot to expire the auction (slot 105 > slot + duration)
    let expired_slot = slot + 6; // slot 106
    dlob.update_slot_and_oracle_price(0, MarketType::Perp, expired_slot, oracle_price);

    // Verify order moved to resting_limit_orders
    {
        let book = dlob.markets.get(&MarketId::perp(0)).unwrap();
        assert_eq!(book.market_orders.bids.len(), 0);
        assert_eq!(book.resting_limit_orders.bids.len(), 1);
    } // Drop book reference

    // CRITICAL: Now try to remove the order
    // Before the fix, this would fail because:
    // 1. remove_order() would remove metadata immediately
    // 2. Then try to remove from market_orders (fails - order not there)
    // 3. Then try to remove from resting_limit_orders (fails - metadata already gone)
    // 4. Result: order still in resting_limit_orders but metadata is gone
    dlob.remove_order(&user, expired_slot, order);

    // Verify order was actually removed from resting_limit_orders
    {
        let book = dlob.markets.get(&MarketId::perp(0)).unwrap();
        assert_eq!(
            book.resting_limit_orders.bids.len(),
            0,
            "Order should be removed from resting_limit_orders"
        );
    } // Drop book reference

    // Verify metadata was also removed
    assert!(
        dlob.metadata.get(&order_id).is_none(),
        "Metadata should be removed after successful order removal"
    );

    // Test that we can find crosses without "metadata missing" errors
    let taker_order = TakerOrder {
        price: 190_000,
        size: 50,
        direction: Direction::Short,
        market_index: 0,
        market_type: MarketType::Perp,
    };

    let crosses = dlob.find_crosses_for_taker_order(expired_slot, oracle_price, taker_order, None);

    // This should work without "metadata missing" errors
    // Before the fix, this would fail because the order would still be in resting_limit_orders
    // but without metadata, causing the "metadata missing" error
    assert_eq!(
        crosses.orders.len(),
        0,
        "Should find no crossing orders after removal"
    );
}

#[test]
fn dlob_metadata_consistency_limit_auction_expiry_and_removal() {
    let _ = env_logger::try_init();
    let dlob = DLOB::default();
    let user = Pubkey::new_unique();
    let slot = 100;
    let order_size = 100;
    let oracle_price = 150_000;

    // bootstrap orderbook for market
    dlob.markets.entry(MarketId::perp(0)).or_insert(Orderbook {
        market_index: 0,
        market_tick_size: 10,
        ..Default::default()
    });

    // Create a regular limit order with auction that will expire
    let mut order = create_test_order(1, OrderType::Limit, Direction::Short, 0, order_size, slot);
    order.auction_start_price = 100_000;
    order.auction_end_price = 200_000;
    order.auction_duration = 5; // Will expire at slot 105
                                // No oracle_price_offset - this makes it a regular limit order
    order.post_only = false; // This makes it a LimitAuction

    // Insert the order
    dlob.insert_order(&user, order);

    // Verify initial state - order should be in market_orders with LimitAuction metadata
    let order_id = crate::dlob::util::order_hash(&user, 1);
    {
        let metadata = dlob.metadata.get(&order_id).unwrap();
        assert_eq!(metadata.kind, OrderKind::LimitAuction);
    } // Drop metadata reference before accessing orderbook

    {
        let book = dlob.markets.get(&MarketId::perp(0)).unwrap();
        assert_eq!(book.market_orders.asks.len(), 1);
        assert_eq!(book.resting_limit_orders.asks.len(), 0);
    } // Drop book reference

    // Advance slot to expire the auction (slot 105 > slot + duration)
    let expired_slot = slot + 6; // slot 106
    dlob.update_slot_and_oracle_price(0, MarketType::Perp, expired_slot, oracle_price);

    // Verify order moved to resting_limit_orders
    {
        let book = dlob.markets.get(&MarketId::perp(0)).unwrap();
        assert_eq!(book.market_orders.asks.len(), 0);
        assert_eq!(book.resting_limit_orders.asks.len(), 1);
    } // Drop book reference

    // CRITICAL: Now try to remove the order
    dlob.remove_order(&user, expired_slot, order);

    // Verify order was actually removed from resting_limit_orders
    {
        let book = dlob.markets.get(&MarketId::perp(0)).unwrap();
        assert_eq!(
            book.resting_limit_orders.asks.len(),
            0,
            "Order should be removed from resting_limit_orders"
        );
    } // Drop book reference

    // Verify metadata was also removed
    assert!(
        dlob.metadata.get(&order_id).is_none(),
        "Metadata should be removed after successful order removal"
    );

    // Test that we can find crosses without "metadata missing" errors
    let taker_order = TakerOrder {
        price: 150_000,
        size: 50,
        direction: Direction::Long,
        market_index: 0,
        market_type: MarketType::Perp,
    };

    let crosses = dlob.find_crosses_for_taker_order(expired_slot, oracle_price, taker_order, None);
    // Should find no crosses since the order was removed
    // but without metadata, causing the "metadata missing" error
    assert_eq!(
        crosses.orders.len(),
        0,
        "Should find no crossing orders after removal"
    );
}

#[test]
fn dlob_metadata_consistency_floating_limit_auction_expiry_and_removal() {
    let _ = env_logger::try_init();
    let dlob = DLOB::default();
    let user = Pubkey::new_unique();
    let slot = 100;
    let order_size = 100;
    let oracle_price = 150_000;

    // bootstrap orderbook for market
    dlob.markets.entry(MarketId::perp(0)).or_insert(Orderbook {
        market_index: 0,
        market_tick_size: 10,
        ..Default::default()
    });

    // Create a floating limit order with auction that will expire
    let mut order = create_test_order(1, OrderType::Limit, Direction::Short, 0, order_size, slot);
    order.auction_start_price = 100_000;
    order.auction_end_price = 200_000;
    order.auction_duration = 5; // Will expire at slot 105
    order.oracle_price_offset = 1000; // This makes it a floating limit order
    order.post_only = false; // This makes it a FloatingLimitAuction

    // Insert the order
    dlob.insert_order(&user, order);

    // Verify initial state - order should be in oracle_orders with FloatingLimitAuction metadata
    let order_id = crate::dlob::util::order_hash(&user, 1);
    {
        let metadata = dlob.metadata.get(&order_id).unwrap();
        assert_eq!(metadata.kind, OrderKind::FloatingLimitAuction);
    } // Drop metadata reference before accessing orderbook

    {
        let book = dlob.markets.get(&MarketId::perp(0)).unwrap();
        assert_eq!(book.oracle_orders.asks.len(), 1);
        assert_eq!(book.floating_limit_orders.asks.len(), 0);
    } // Drop book reference

    // Advance slot to expire the auction (slot 105 > slot + duration)
    let expired_slot = slot + 6; // slot 106
    dlob.update_slot_and_oracle_price(0, MarketType::Perp, expired_slot, oracle_price);

    // Verify order moved to floating_limit_orders
    {
        let book = dlob.markets.get(&MarketId::perp(0)).unwrap();
        assert_eq!(book.oracle_orders.asks.len(), 0);
        assert_eq!(book.floating_limit_orders.asks.len(), 1);
    } // Drop book reference

    // CRITICAL: Now try to remove the order
    // Use a much later slot to ensure the auction logic works correctly
    dlob.remove_order(&user, expired_slot, order);

    // Verify order was actually removed from floating_limit_orders
    {
        let book = dlob.markets.get(&MarketId::perp(0)).unwrap();
        assert_eq!(
            book.floating_limit_orders.asks.len(),
            0,
            "Order should be removed from floating_limit_orders"
        );
    } // Drop book reference

    // Verify metadata was also removed
    assert!(
        dlob.metadata.get(&order_id).is_none(),
        "Metadata should be removed after successful order removal"
    );

    // Test that we can find crosses without "metadata missing" errors
    let taker_order = TakerOrder {
        price: 210_000, // Higher than the floating order price (oracle + offset)
        size: 50,
        direction: Direction::Long,
        market_index: 0,
        market_type: MarketType::Perp,
    };

    let crosses = dlob.find_crosses_for_taker_order(expired_slot, oracle_price, taker_order, None);

    // This should work without "metadata missing" errors
    // Before the fix, this would fail because the order would still be in floating_limit_orders
    // but without metadata, causing the "metadata missing" error
    assert_eq!(
        crosses.orders.len(),
        0,
        "Should find no crossing orders after removal"
    );
}

#[test]
fn dlob_trigger_order_transition_remove() {
    use crate::types::OrderTriggerCondition;
    let _ = env_logger::try_init();
    let dlob = DLOB::default();
    let user = Pubkey::new_unique();
    let slot = 100;

    // bootstrap orderbook for market
    dlob.markets.entry(MarketId::perp(0)).or_insert(Orderbook {
        market_index: 0,
        market_tick_size: 10,
        ..Default::default()
    });

    // Create a trigger market order
    let mut order = create_test_order(1, OrderType::TriggerMarket, Direction::Long, 0, 100, slot);
    order.trigger_price = 950;
    order.trigger_condition = OrderTriggerCondition::Above;
    order.price = 1000; // Set a price for the trigger order

    // Insert the order
    dlob.insert_order(&user, order);

    // Verify initial state - order should be in trigger_orders
    let order_id = crate::dlob::util::order_hash(&user, 1);
    {
        let metadata = dlob.metadata.get(&order_id).unwrap();
        assert_eq!(metadata.kind, OrderKind::TriggerMarket);
    }

    {
        let book = dlob.markets.get(&MarketId::perp(0)).unwrap();
        assert_eq!(book.trigger_orders.bids.len(), 1);
        assert_eq!(book.market_orders.bids.len(), 0);
    }

    // Now trigger the order - this should move it to market_orders
    let mut triggered_order = order;
    triggered_order.trigger_condition = OrderTriggerCondition::TriggeredAbove;
    triggered_order.slot = slot + 1; // Slot changes when triggered
    triggered_order.price = 1100; // Price might change when triggered

    // This update should work, but let's see if there are any issues
    dlob.update_order(&user, slot, triggered_order, order);

    // Verify the transition worked
    {
        let book = dlob.markets.get(&MarketId::perp(0)).unwrap();
        assert_eq!(book.trigger_orders.bids.len(), 0);
        assert_eq!(book.market_orders.bids.len(), 1);
    }

    // Now try to remove the triggered order
    // This might fail if the same key mismatch issue exists
    dlob.remove_order(&user, slot, triggered_order);

    // Verify the order was properly removed
    {
        let book = dlob.markets.get(&MarketId::perp(0)).unwrap();
        assert_eq!(book.market_orders.bids.len(), 0);
    }

    // Verify metadata was also removed
    assert!(
        dlob.metadata.get(&order_id).is_none(),
        "Metadata should be removed after successful order removal"
    );
}

#[test]
fn dlob_trigger_order_transition_update() {
    use crate::types::OrderTriggerCondition;
    let _ = env_logger::try_init();
    let dlob = DLOB::default();
    let user = Pubkey::new_unique();
    let slot = 100;

    // bootstrap orderbook for market
    dlob.markets.entry(MarketId::perp(0)).or_insert(Orderbook {
        market_index: 0,
        market_tick_size: 10,
        ..Default::default()
    });

    // Create a trigger market order
    let mut order = create_test_order(1, OrderType::TriggerMarket, Direction::Long, 0, 100, slot);
    order.trigger_price = 950;
    order.trigger_condition = OrderTriggerCondition::Above;
    order.price = 1000; // Set a price for the trigger order

    // Insert the order
    dlob.insert_order(&user, order);

    // Verify initial state - order should be in trigger_orders
    let order_id = crate::dlob::util::order_hash(&user, 1);
    {
        let metadata = dlob.metadata.get(&order_id).unwrap();
        assert_eq!(metadata.kind, OrderKind::TriggerMarket);
    }

    {
        let book = dlob.markets.get(&MarketId::perp(0)).unwrap();
        assert_eq!(book.trigger_orders.bids.len(), 1);
        assert_eq!(book.market_orders.bids.len(), 0);
    }

    // Now trigger the order - this should move it to market_orders
    let mut triggered_order = order;
    triggered_order.trigger_condition = OrderTriggerCondition::TriggeredAbove;
    triggered_order.slot = slot + 1; // Slot changes when triggered
    triggered_order.price = 1100; // Price changes when triggered

    // This update might fail silently
    dlob.update_order(&user, slot, triggered_order, order);

    // Check if the transition actually worked
    {
        let book = dlob.markets.get(&MarketId::perp(0)).unwrap();
        let metadata = dlob.metadata.get(&order_id).unwrap();

        // Metadata should be updated
        assert_eq!(metadata.kind, OrderKind::MarketTriggered);

        // But the order might not actually be in market_orders if the update failed
        if book.market_orders.bids.len() == 0 {
            panic!("BUG DEMONSTRATED: Metadata says MarketTriggered but order is not in market_orders - this could cause 'metadata missing' errors");
        }
    }

    // If we get here, the transition worked correctly
    {
        let book = dlob.markets.get(&MarketId::perp(0)).unwrap();
        assert_eq!(book.trigger_orders.bids.len(), 0);
        assert_eq!(book.market_orders.bids.len(), 1);
    }
}

// Test data structure for Snapshot testing
#[derive(Debug, Clone, Default, PartialEq)]
struct TestData {
    value: u64,
    counter: u32,
    data: Vec<u8>,
}

impl TestData {
    fn new(value: u64, counter: u32, data_size: usize) -> Self {
        Self {
            value,
            counter,
            data: vec![0u8; data_size],
        }
    }
}

#[test]
fn snapshot_basic_functionality() {
    let _ = env_logger::try_init();

    // Create initial data
    let initial_data = TestData::new(100, 0, 100);
    let snapshot = Snapshot::new(Arc::new(initial_data));

    // Test basic get
    let data = snapshot.get();
    assert_eq!(data.value, 100);
    assert_eq!(data.counter, 0);
    assert_eq!(data.data.len(), 100);

    // Test update
    let new_data = TestData::new(200, 1, 200);
    snapshot.update(Arc::new(new_data));

    let updated_data = snapshot.get();
    assert_eq!(updated_data.value, 200);
    assert_eq!(updated_data.counter, 1);
    assert_eq!(updated_data.data.len(), 200);

    // Test multiple updates
    for i in 2..10 {
        let new_data = TestData::new(200 + i as u64, i, 200 + i as usize);
        snapshot.update(Arc::new(new_data));

        let data = snapshot.get();
        assert_eq!(data.value, 200 + i as u64);
        assert_eq!(data.counter, i);
        assert_eq!(data.data.len(), 200 + i as usize);
    }
}

#[test]
fn snapshot_simple_multithreaded() {
    let _ = env_logger::try_init();

    // Create initial data
    let initial_data = TestData::new(100, 0, 100);
    let snapshot = Arc::new(Snapshot::new(Arc::new(initial_data)));

    const NUM_THREADS: usize = 2;
    const NUM_ITERATIONS: usize = 10;

    let barrier = Arc::new(Barrier::new(NUM_THREADS));
    let mut handles = Vec::new();

    // Spawn threads that both read and write
    for _thread_id in 0..NUM_THREADS {
        let snapshot = snapshot.clone();
        let barrier = barrier.clone();

        let handle = thread::spawn(move || {
            barrier.wait(); // Wait for all threads to start

            for i in 0..NUM_ITERATIONS {
                // Read current value
                let current = snapshot.get();
                let current_value = current.value;

                // Create new data with incremented value
                let new_data = TestData::new(current_value + 1, i as u32, 100);

                // Update snapshot
                snapshot.update(Arc::new(new_data));

                // Verify the update worked
                let updated = snapshot.get();
                assert!(updated.value >= current_value);
                assert_eq!(updated.data.len(), 100);
            }
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread should complete successfully");
    }

    // Verify final state
    let final_data = snapshot.get();
    assert!(final_data.value >= 100);
    assert_eq!(final_data.data.len(), 100);
}

#[test]
fn snapshot_multithreaded_readers_writers() {
    let _ = env_logger::try_init();

    // Create initial data
    let initial_data = TestData::new(100, 0, 1024);
    let snapshot = Arc::new(Snapshot::new(Arc::new(initial_data)));

    const NUM_READERS: usize = 5;
    const NUM_WRITERS: usize = 2;
    const NUM_ITERATIONS: usize = 100;

    let barrier = Arc::new(Barrier::new(NUM_READERS + NUM_WRITERS));
    let mut handles = Vec::new();

    // Spawn reader threads
    for reader_id in 0..NUM_READERS {
        let snapshot = snapshot.clone();
        let barrier = barrier.clone();

        let handle = thread::spawn(move || {
            barrier.wait(); // Wait for all threads to start

            for _ in 0..NUM_ITERATIONS {
                let data = snapshot.get();

                // Verify data integrity
                assert!(
                    data.value >= 100,
                    "Reader {}: value should be >= 100",
                    reader_id
                );
                assert!(
                    data.counter <= NUM_ITERATIONS as u32,
                    "Reader {}: counter too high",
                    reader_id
                );
                assert_eq!(
                    data.data.len(),
                    1024,
                    "Reader {}: data size mismatch",
                    reader_id
                );

                // Small delay to increase chance of race conditions
                if reader_id % 3 == 0 {
                    thread::sleep(Duration::from_micros(1));
                }
            }
        });

        handles.push(handle);
    }

    // Spawn writer threads
    for writer_id in 0..NUM_WRITERS {
        let snapshot = snapshot.clone();
        let barrier = barrier.clone();

        let handle = thread::spawn(move || {
            barrier.wait(); // Wait for all threads to start

            for iteration in 0..NUM_ITERATIONS {
                let new_data = TestData::new(100 + iteration as u64, iteration as u32, 1024);

                snapshot.update(Arc::new(new_data));

                // Small delay between updates
                if writer_id % 2 == 0 {
                    thread::sleep(Duration::from_micros(10));
                }
            }
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread should complete successfully");
    }

    // Verify final state
    let final_data = snapshot.get();
    assert!(final_data.value >= 100);
    assert!(final_data.counter <= NUM_ITERATIONS as u32);
    assert_eq!(final_data.data.len(), 1024);
}

#[test]
fn snapshot_safe_usage_pattern() {
    let _ = env_logger::try_init();

    // Create initial data
    let initial_data = TestData::new(100, 0, 100);
    let snapshot = Arc::new(Snapshot::new(Arc::new(initial_data)));
    let update_mutex = Arc::new(std::sync::Mutex::new(()));

    const NUM_READER_THREADS: usize = 4;
    const NUM_ITERATIONS: usize = 100;

    let barrier = Arc::new(Barrier::new(NUM_READER_THREADS + 1)); // +1 for writer
    let mut handles = Vec::new();

    // Spawn reader threads (many readers, lock-free)
    for _thread_id in 0..NUM_READER_THREADS {
        let snapshot = snapshot.clone();
        let barrier = barrier.clone();

        let handle = thread::spawn(move || {
            barrier.wait(); // Wait for all threads to start

            for _i in 0..NUM_ITERATIONS {
                // Read operation (lock-free)
                let data = snapshot.get();
                assert!(data.value >= 100);
                assert_eq!(data.data.len(), 100);
            }
        });

        handles.push(handle);
    }

    // Spawn single writer thread (synchronized)
    let snapshot_writer = snapshot.clone();
    let barrier = barrier.clone();
    let update_mutex = update_mutex.clone();

    let handle = thread::spawn(move || {
        barrier.wait(); // Wait for all threads to start

        for i in 0..NUM_ITERATIONS {
            // Write operation (synchronized)
            let _guard = update_mutex.lock().unwrap();
            let new_data = TestData::new(100 + i as u64, i as u32, 100);
            snapshot_writer.update(Arc::new(new_data));
            // Guard is dropped here, releasing the lock
        }
    });

    handles.push(handle);

    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread should complete successfully");
    }

    // Verify final state
    let final_data = snapshot.get();
    assert!(final_data.value >= 100);
    assert_eq!(final_data.data.len(), 100);
}

#[test]
fn snapshot_real_world_issue_reproduction() {
    let _ = env_logger::try_init();

    // Simulate real-world usage: set data, then read it back
    let initial_data = TestData::new(42, 1, 100);
    let snapshot = Snapshot::new(Arc::new(initial_data));

    // Immediately read back - should get the data we just set
    let data = snapshot.get();
    assert_eq!(data.value, 42, "Should get the value we just set");
    assert_eq!(data.counter, 1, "Should get the counter we just set");
    assert_eq!(data.data.len(), 100, "Should get the data size we just set");

    // Update with new data
    let new_data = TestData::new(100, 2, 200);
    snapshot.update(Arc::new(new_data));

    // Read back immediately - should get the new data
    let updated_data = snapshot.get();
    assert_eq!(updated_data.value, 100, "Should get the updated value");
    assert_eq!(updated_data.counter, 2, "Should get the updated counter");
    assert_eq!(
        updated_data.data.len(),
        200,
        "Should get the updated data size"
    );

    // Multiple rapid reads should be consistent
    for _ in 0..10 {
        let data = snapshot.get();
        assert_eq!(data.value, 100, "Rapid reads should be consistent");
        assert_eq!(data.counter, 2, "Rapid reads should be consistent");
        assert_eq!(data.data.len(), 200, "Rapid reads should be consistent");
    }
}

#[test]
fn snapshot_simple_race_test() {
    let _ = env_logger::try_init();

    // Simple test: set data, then read it back in another thread
    let initial_data = TestData::new(42, 1, 100);
    let snapshot = Arc::new(Snapshot::new(Arc::new(initial_data)));

    // Spawn a writer thread
    let snapshot_writer = snapshot.clone();
    let writer_handle = thread::spawn(move || {
        for i in 0..100 {
            let new_data = TestData::new(100 + i as u64, i as u32, 100);
            snapshot_writer.update(Arc::new(new_data));
            thread::sleep(Duration::from_micros(1));
        }
    });

    // Spawn a reader thread
    let snapshot_reader = snapshot.clone();
    let reader_handle = thread::spawn(move || {
        let mut default_count = 0;
        let mut max_value = 0;

        for _ in 0..100 {
            let data = snapshot_reader.get();

            if data.value == 0 && data.counter == 0 && data.data.len() == 0 {
                default_count += 1;
            }

            if data.value > max_value {
                max_value = data.value;
            }

            thread::sleep(Duration::from_micros(1));
        }

        (default_count, max_value)
    });

    // Wait for both threads
    writer_handle.join().expect("Writer should complete");
    let (default_count, max_value) = reader_handle.join().expect("Reader should complete");

    // We should have seen some updates
    assert!(
        max_value > 0,
        "Should have seen some non-default values, got max_value: {}",
        max_value
    );

    // Report if we got default values
    if default_count > 0 {
        eprintln!(
            "WARNING: Got {} default values out of 100 reads",
            default_count
        );
    }
}

#[test]
fn snapshot_default_behavior() {
    let _ = env_logger::try_init();

    // Test Default implementation
    let snapshot: Snapshot<TestData> = Snapshot::default();

    // Should be able to get default data
    let data = snapshot.get();
    assert_eq!(data.value, 0);
    assert_eq!(data.counter, 0);
    assert_eq!(data.data.len(), 0);

    // Should be able to update
    let new_data = TestData::new(123, 456, 789);
    snapshot.update(Arc::new(new_data));

    let updated_data = snapshot.get();
    assert_eq!(updated_data.value, 123);
    assert_eq!(updated_data.counter, 456);
    assert_eq!(updated_data.data.len(), 789);
}
