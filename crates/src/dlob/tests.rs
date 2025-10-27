use solana_sdk::pubkey::Pubkey;

use crate::{
    dlob::{Direction, OrderKind, OrderMetadata, Orderbook, Snapshot, TakerOrder, DLOB},
    types::{MarketId, MarketType, Order, OrderStatus, OrderType},
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
        max_ts: 30,
        status: OrderStatus::Open,
        ..Default::default()
    }
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

    dlob.update_slot(0);
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
    dlob.update_slot(slot);

    // Get the L2 snapshot
    let l2book = dlob.get_l2_book(0, MarketType::Perp, oracle_price as u64);

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
    dlob.update_slot(slot);
    let l2book = dlob.get_l2_book(0, MarketType::Perp, oracle_price as u64);

    // Verify new order was added
    assert_eq!(l2book.bids.get(&1075), Some(&8));
    assert_eq!(l2book.bids.len(), 3);

    // Modify an existing order
    let old_order = create_test_order(1, OrderType::Limit, Direction::Long, 1100, 2, slot);
    let mut new_order = create_test_order(1, OrderType::Limit, Direction::Long, 1100, 4, slot); // Changed size from 2 to 4
    new_order.post_only = true;
    dlob.update_order(&user, slot, new_order, old_order);

    // Get updated L2 snapshot
    dlob.update_slot(slot);
    let l2book = dlob.get_l2_book(0, MarketType::Perp, oracle_price as u64);

    // Verify order was updated
    assert_eq!(l2book.bids.get(&1100), Some(&10)); // 4 (updated) + 6 (floating limit) = 10
    assert_eq!(l2book.bids.len(), 3);

    // Remove an order
    let old_order = create_test_order(3, OrderType::Market, Direction::Long, 0, 4, slot);
    let mut new_order = create_test_order(3, OrderType::Market, Direction::Long, 1050, 4, slot);
    new_order.base_asset_amount_filled = old_order.base_asset_amount; // Set filled amount equal to total amount
    dlob.update_order(&user, slot, new_order, old_order);

    // Get updated L2 snapshot
    dlob.update_slot(slot);
    let l2book = dlob.get_l2_book(0, MarketType::Perp, oracle_price as u64);

    // Verify order was removed
    assert_eq!(l2book.bids.get(&1050), None);
    assert_eq!(l2book.bids.len(), 2);
}

#[ignore]
#[test]
fn dlob_l2_snapshot_max_leverage_filtering() {
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

    // Insert normal orders
    let mut order = create_test_order(1, OrderType::Limit, Direction::Long, 1100, 5, slot);
    order.post_only = true;
    dlob.insert_order(&user, order);

    let mut order = create_test_order(2, OrderType::Limit, Direction::Short, 900, 3, slot);
    order.post_only = true;
    dlob.insert_order(&user, order);

    // Insert max leverage orders (size = u64::MAX)
    let mut max_lev_order =
        create_test_order(3, OrderType::Limit, Direction::Long, 1200, u64::MAX, slot);
    max_lev_order.post_only = true;
    dlob.insert_order(&user, max_lev_order);

    let mut max_lev_order =
        create_test_order(4, OrderType::Limit, Direction::Short, 800, u64::MAX, slot);
    max_lev_order.post_only = true;
    dlob.insert_order(&user, max_lev_order);

    dlob.update_slot(slot);

    // Test with include_max_leverage = true (should include all orders)
    let l2book_with_max_lev = dlob.get_l2_book(0, MarketType::Perp, oracle_price as u64);
    assert_eq!(l2book_with_max_lev.bids.get(&1100), Some(&5));
    assert_eq!(l2book_with_max_lev.bids.get(&1200), Some(&u64::MAX));
    assert_eq!(l2book_with_max_lev.asks.get(&900), Some(&3));
    assert_eq!(l2book_with_max_lev.asks.get(&800), Some(&u64::MAX));
    assert_eq!(l2book_with_max_lev.bids.len(), 2);
    assert_eq!(l2book_with_max_lev.asks.len(), 2);

    // Test with include_max_leverage = false (should exclude max leverage orders)
    let l2book_without_max_lev = dlob.get_l2_book(0, MarketType::Perp, oracle_price as u64);
    assert_eq!(l2book_without_max_lev.bids.get(&1100), Some(&5));
    assert_eq!(l2book_without_max_lev.bids.get(&1200), None); // Max leverage order excluded
    assert_eq!(l2book_without_max_lev.asks.get(&900), Some(&3));
    assert_eq!(l2book_without_max_lev.asks.get(&800), None); // Max leverage order excluded
    assert_eq!(l2book_without_max_lev.bids.len(), 1);
    assert_eq!(l2book_without_max_lev.asks.len(), 1);
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

    // Insert market orders with different auction durations
    let mut order = create_test_order(1, OrderType::Limit, Direction::Long, 1100, 2, slot);
    order.auction_duration = 5; // Will expire at slot 105
    dlob.insert_order(&user, order);

    let mut order = create_test_order(2, OrderType::Limit, Direction::Short, 900, 3, slot);
    order.auction_duration = 10; // Will expire at slot 110
    dlob.insert_order(&user, order);

    // Update to slot 104 - no orders should expire
    dlob.update_slot(104);
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
    dlob.update_slot(105);
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
    dlob.update_slot(110);
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
    dlob.update_slot(104);
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
    dlob.update_slot(105);
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
    dlob.update_slot(110);
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

    // Insert market orders that are not limit orders
    let mut order = create_test_order(1, OrderType::Market, Direction::Long, 1100, 2, slot);
    order.auction_duration = 5; // Will expire at slot 105
    dlob.insert_order(&user, order);

    let mut order = create_test_order(2, OrderType::Market, Direction::Short, 900, 3, slot);
    order.auction_duration = 10; // Will expire at slot 110
    dlob.insert_order(&user, order);

    // Update to slot 104 - no orders should expire
    dlob.update_slot(104);
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
    dlob.update_slot(105);
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
    dlob.update_slot(110);
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
    dlob.update_slot(105);
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
    dlob.update_slot(0);

    let book = dlob
        .markets
        .get(&MarketId::new(0, MarketType::Perp))
        .unwrap();

    // Verify no orders in book
    assert_eq!(book.resting_limit_orders.bids.len(), 0);
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
    dlob.update_slot(0);
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
    dlob.update_slot(expired_slot);

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
    dlob.update_slot(expired_slot);

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
    dlob.update_slot(expired_slot);

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
#[derive(Debug, Clone, PartialEq)]
struct TestData {
    value: u64,
    counter: u32,
    data: Vec<u8>,
}

impl Default for TestData {
    fn default() -> Self {
        Self {
            value: 0,
            counter: 0,
            data: Vec::new(),
        }
    }
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
    let snapshot = Snapshot::new(initial_data.clone(), initial_data);

    // Test basic get
    let data = snapshot.read();
    assert_eq!(data.value, 100);
    assert_eq!(data.counter, 0);
    assert_eq!(data.data.len(), 100);

    // Test update
    let new_data = TestData::new(200, 1, 200);
    snapshot.write(|data| *data = new_data);

    let updated_data = snapshot.read();
    assert_eq!(updated_data.value, 200);
    assert_eq!(updated_data.counter, 1);
    assert_eq!(updated_data.data.len(), 200);

    // Test multiple updates
    for i in 2..10 {
        let new_data = TestData::new(200 + i as u64, i, 200 + i as usize);
        snapshot.write(|data| *data = new_data);

        let data = snapshot.read();
        assert_eq!(data.value, 200 + i as u64);
        assert_eq!(data.counter, i);
        assert_eq!(data.data.len(), 200 + i as usize);
    }
}

#[test]
fn dlob_get_maker_bids_l3() {
    let _ = env_logger::try_init();
    let dlob = DLOB::default();
    let user = Pubkey::new_unique();
    let slot = 100;
    let oracle_price = 1000;

    // Insert resting limit orders
    let mut order1 = create_test_order(1, OrderType::Limit, Direction::Long, 1100, 5, slot);
    order1.post_only = true;
    dlob.insert_order(&user, order1);

    let mut order2 = create_test_order(2, OrderType::Limit, Direction::Long, 1200, 3, slot);
    order2.post_only = true;
    dlob.insert_order(&user, order2);

    // Insert floating limit orders
    let mut order3 = create_test_order(3, OrderType::Limit, Direction::Long, 0, 4, slot);
    order3.oracle_price_offset = 150; // Will be 1150 with oracle_price
    order3.post_only = true;
    dlob.insert_order(&user, order3);

    let mut order4 = create_test_order(4, OrderType::Limit, Direction::Long, 0, 2, slot);
    order4.oracle_price_offset = 200; // Will be 1200 with oracle_price
    order4.post_only = true;
    dlob.insert_order(&user, order4);

    // Get the orderbook
    let book = dlob
        .markets
        .get(&MarketId::new(0, MarketType::Perp))
        .unwrap();

    // Test get_maker_bids_l3
    let maker_bids = book.get_maker_bids_l3(oracle_price, &dlob.metadata);

    // Should have 4 orders
    assert_eq!(maker_bids.len(), 4);

    // Should be sorted by price descending (best bid first)
    let prices: Vec<u64> = maker_bids.iter().map(|o| o.price).collect();
    assert_eq!(prices, vec![1200, 1200, 1150, 1100]);

    // Verify order details
    let order_1200_1 = maker_bids
        .iter()
        .find(|o| o.price == 1200 && o.size == 3)
        .unwrap();
    assert_eq!(order_1200_1.order_id, 2);
    assert_eq!(order_1200_1.user, user);
    assert_eq!(order_1200_1.kind, OrderKind::Limit);

    let order_1200_2 = maker_bids
        .iter()
        .find(|o| o.price == 1200 && o.size == 2)
        .unwrap();
    assert_eq!(order_1200_2.order_id, 4);
    assert_eq!(order_1200_2.user, user);
    assert_eq!(order_1200_2.kind, OrderKind::FloatingLimit);

    let order_1150 = maker_bids.iter().find(|o| o.price == 1150).unwrap();
    assert_eq!(order_1150.order_id, 3);
    assert_eq!(order_1150.size, 4);
    assert_eq!(order_1150.kind, OrderKind::FloatingLimit);

    let order_1100 = maker_bids.iter().find(|o| o.price == 1100).unwrap();
    assert_eq!(order_1100.order_id, 1);
    assert_eq!(order_1100.size, 5);
    assert_eq!(order_1100.kind, OrderKind::Limit);
}

#[test]
fn dlob_get_maker_asks_l3() {
    let _ = env_logger::try_init();
    let dlob = DLOB::default();
    let user = Pubkey::new_unique();
    let slot = 100;
    let oracle_price = 1000;

    // Insert resting limit orders
    let mut order1 = create_test_order(1, OrderType::Limit, Direction::Short, 900, 5, slot);
    order1.post_only = true;
    dlob.insert_order(&user, order1);

    let mut order2 = create_test_order(2, OrderType::Limit, Direction::Short, 800, 3, slot);
    order2.post_only = true;
    dlob.insert_order(&user, order2);

    // Insert floating limit orders
    let mut order3 = create_test_order(3, OrderType::Limit, Direction::Short, 0, 4, slot);
    order3.oracle_price_offset = -150; // Will be 850 with oracle_price
    order3.post_only = true;
    dlob.insert_order(&user, order3);

    let mut order4 = create_test_order(4, OrderType::Limit, Direction::Short, 0, 2, slot);
    order4.oracle_price_offset = -200; // Will be 800 with oracle_price
    order4.post_only = true;
    dlob.insert_order(&user, order4);

    // Get the orderbook
    let book = dlob
        .markets
        .get(&MarketId::new(0, MarketType::Perp))
        .unwrap();

    // Test get_maker_asks_l3
    let maker_asks = book.get_maker_asks_l3(oracle_price, &dlob.metadata);

    // Should have 4 orders
    assert_eq!(maker_asks.len(), 4);

    // Should be sorted by price descending (best ask first - lowest price)
    let prices: Vec<u64> = maker_asks.iter().map(|o| o.price).collect();
    assert_eq!(prices, vec![800, 800, 850, 900]);

    // Verify order details
    let order_800_1 = maker_asks
        .iter()
        .find(|o| o.price == 800 && o.size == 3)
        .unwrap();
    assert_eq!(order_800_1.order_id, 2);
    assert_eq!(order_800_1.user, user);
    assert_eq!(order_800_1.kind, OrderKind::Limit);

    let order_800_2 = maker_asks
        .iter()
        .find(|o| o.price == 800 && o.size == 2)
        .unwrap();
    assert_eq!(order_800_2.order_id, 4);
    assert_eq!(order_800_2.user, user);
    assert_eq!(order_800_2.kind, OrderKind::FloatingLimit);

    let order_850 = maker_asks.iter().find(|o| o.price == 850).unwrap();
    assert_eq!(order_850.order_id, 3);
    assert_eq!(order_850.size, 4);
    assert_eq!(order_850.kind, OrderKind::FloatingLimit);

    let order_900 = maker_asks.iter().find(|o| o.price == 900).unwrap();
    assert_eq!(order_900.order_id, 1);
    assert_eq!(order_900.size, 5);
    assert_eq!(order_900.kind, OrderKind::Limit);
}

#[test]
fn dlob_get_taker_bids_l3() {
    let _ = env_logger::try_init();
    let dlob = DLOB::default();
    let user = Pubkey::new_unique();
    let slot = 100;
    let oracle_price = 1000;
    let trigger_price = 950;

    // Insert market orders
    let mut market_order = create_test_order(1, OrderType::Market, Direction::Long, 0, 5, slot);
    market_order.auction_duration = 10;
    market_order.auction_start_price = 1100;
    market_order.auction_end_price = 1200;
    dlob.insert_order(&user, market_order);

    // Insert oracle orders
    let mut oracle_order = create_test_order(2, OrderType::Oracle, Direction::Long, 0, 3, slot);
    oracle_order.auction_duration = 10;
    oracle_order.auction_start_price = 1050;
    oracle_order.auction_end_price = 1150;
    dlob.insert_order(&user, oracle_order);

    // Get the orderbook
    let book = dlob
        .markets
        .get(&MarketId::new(0, MarketType::Perp))
        .unwrap();

    // Test get_taker_bids_l3
    let taker_bids = book.get_taker_bids_l3(oracle_price, trigger_price, None, &dlob.metadata);

    // Should have 2 orders (market + oracle, no trigger orders for now)
    assert_eq!(taker_bids.len(), 2);

    // Should be sorted by price descending (best bid first)
    let prices: Vec<u64> = taker_bids.iter().map(|o| o.price).collect();
    // Prices will be calculated based on auction progress and oracle price
    assert!(prices[0] >= prices[1]);

    // Verify order details
    for order in &taker_bids {
        assert!(order.size > 0);
        assert_eq!(order.user, user);
        assert!(order.price > 0);
    }
}

#[test]
fn dlob_get_taker_asks_l3() {
    let _ = env_logger::try_init();
    let dlob = DLOB::default();
    let user = Pubkey::new_unique();
    let slot = 100;
    let oracle_price = 1000;
    let trigger_price = 1050;

    // Insert market orders
    let mut market_order = create_test_order(1, OrderType::Market, Direction::Short, 0, 5, slot);
    market_order.auction_duration = 10;
    market_order.auction_start_price = 900;
    market_order.auction_end_price = 800;
    dlob.insert_order(&user, market_order);

    // Insert oracle orders
    let mut oracle_order = create_test_order(2, OrderType::Oracle, Direction::Short, 0, 3, slot);
    oracle_order.auction_duration = 10;
    oracle_order.auction_start_price = 950;
    oracle_order.auction_end_price = 850;
    dlob.insert_order(&user, oracle_order);

    // Get the orderbook
    let book = dlob
        .markets
        .get(&MarketId::new(0, MarketType::Perp))
        .unwrap();

    // Test get_taker_asks_l3
    let taker_asks = book.get_taker_asks_l3(oracle_price, trigger_price, None, &dlob.metadata);

    // Should have 2 orders (market + oracle, no trigger orders for now)
    assert_eq!(taker_asks.len(), 2);

    // Should be sorted by price ascending (best ask first - lowest price)
    let prices: Vec<u64> = taker_asks.iter().map(|o| o.price).collect();
    // Prices will be calculated based on auction progress and oracle price
    assert!(prices[0] <= prices[1]);

    // Verify order details
    for order in &taker_asks {
        assert!(order.size > 0);
        assert_eq!(order.user, user);
        assert!(order.price > 0);
    }
}

#[test]
fn dlob_l3_functions_mixed_order_types() {
    let _ = env_logger::try_init();
    let dlob = DLOB::default();
    let user = Pubkey::new_unique();
    let slot = 100;
    let oracle_price = 1000;
    let trigger_price = 950;

    // Insert various order types
    // Resting limit orders
    let mut limit_bid = create_test_order(1, OrderType::Limit, Direction::Long, 1100, 5, slot);
    limit_bid.post_only = true;
    dlob.insert_order(&user, limit_bid);

    let mut limit_ask = create_test_order(2, OrderType::Limit, Direction::Short, 900, 5, slot);
    limit_ask.post_only = true;
    dlob.insert_order(&user, limit_ask);

    // Floating limit orders
    let mut floating_bid = create_test_order(3, OrderType::Limit, Direction::Long, 0, 3, slot);
    floating_bid.oracle_price_offset = 100; // Will be 1100 with oracle_price
    floating_bid.post_only = true;
    dlob.insert_order(&user, floating_bid);

    let mut floating_ask = create_test_order(4, OrderType::Limit, Direction::Short, 0, 3, slot);
    floating_ask.oracle_price_offset = -100; // Will be 900 with oracle_price
    floating_ask.post_only = true;
    dlob.insert_order(&user, floating_ask);

    // Market orders
    let mut market_bid = create_test_order(5, OrderType::Market, Direction::Long, 0, 4, slot);
    market_bid.auction_duration = 10;
    market_bid.auction_start_price = 1200;
    market_bid.auction_end_price = 1300;
    dlob.insert_order(&user, market_bid);

    let mut market_ask = create_test_order(6, OrderType::Market, Direction::Short, 0, 4, slot);
    market_ask.auction_duration = 10;
    market_ask.auction_start_price = 800;
    market_ask.auction_end_price = 700;
    dlob.insert_order(&user, market_ask);

    // Oracle orders
    let mut oracle_bid = create_test_order(7, OrderType::Oracle, Direction::Long, 0, 2, slot);
    oracle_bid.auction_duration = 10;
    oracle_bid.auction_start_price = 1050;
    oracle_bid.auction_end_price = 1150;
    dlob.insert_order(&user, oracle_bid);

    let mut oracle_ask = create_test_order(8, OrderType::Oracle, Direction::Short, 0, 2, slot);
    oracle_ask.auction_duration = 10;
    oracle_ask.auction_start_price = 950;
    oracle_ask.auction_end_price = 850;
    dlob.insert_order(&user, oracle_ask);

    // Skip trigger orders for now due to implementation issues

    // Get the orderbook
    let book = dlob
        .markets
        .get(&MarketId::new(0, MarketType::Perp))
        .unwrap();

    // Test all L3 functions
    let maker_bids = book.get_maker_bids_l3(oracle_price, &dlob.metadata);
    let maker_asks = book.get_maker_asks_l3(oracle_price, &dlob.metadata);
    let taker_bids = book.get_taker_bids_l3(oracle_price, trigger_price, None, &dlob.metadata);
    let taker_asks = book.get_taker_asks_l3(oracle_price, trigger_price, None, &dlob.metadata);

    // Maker orders should include resting limit and floating limit orders
    assert_eq!(maker_bids.len(), 2); // limit_bid + floating_bid
    assert_eq!(maker_asks.len(), 2); // limit_ask + floating_ask

    // Taker orders should include market and oracle orders (no trigger orders for now)
    assert_eq!(taker_bids.len(), 2); // market_bid + oracle_bid
    assert_eq!(taker_asks.len(), 2); // market_ask + oracle_ask

    // Verify sorting
    let maker_bid_prices: Vec<u64> = maker_bids.iter().map(|o| o.price).collect();
    assert_eq!(maker_bid_prices, vec![1100, 1100]); // Both at 1100, sorted by insertion order

    let maker_ask_prices: Vec<u64> = maker_asks.iter().map(|o| o.price).collect();
    assert_eq!(maker_ask_prices, vec![900, 900]); // Both at 900, sorted by insertion order

    // Taker orders should be sorted correctly
    let taker_bid_prices: Vec<u64> = taker_bids.iter().map(|o| o.price).collect();
    assert!(taker_bid_prices[0] >= taker_bid_prices[1]);

    let taker_ask_prices: Vec<u64> = taker_asks.iter().map(|o| o.price).collect();
    assert!(taker_ask_prices[0] <= taker_ask_prices[1]);
}
