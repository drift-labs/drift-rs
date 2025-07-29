use solana_sdk::pubkey::Pubkey;

use crate::{
    dlob::{types::DynamicPrice, Direction, OrderKind, OrderMetadata, Orderbook, TakerOrder, DLOB},
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
    let bid_prices = book.get_limit_bids(slot, 1_000_000);
    assert_eq!(
        bid_prices.iter().map(|o| o.price).collect::<Vec<u64>>(),
        vec![200_u64, 150, 100]
    );

    // Verify asks are sorted lowest to highest
    let ask_prices = book.get_limit_asks(slot, 1_000_000);
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
        .map(|v| v.offset_price)
        .collect();
    assert_eq!(bid_offsets, vec![30, 20, 10]);

    // Verify asks are sorted lowest to highest offset
    let ask_offsets: Vec<i32> = book
        .floating_limit_orders
        .asks
        .iter()
        .map(|v| v.offset_price)
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
    let l2book = dlob.get_l2_snapshot(0, MarketType::Perp);

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
    let l2book = dlob.get_l2_snapshot(0, MarketType::Perp);

    // Verify new order was added
    assert_eq!(l2book.bids.get(&1075), Some(&8));
    assert_eq!(l2book.bids.len(), 3);

    // Modify an existing order
    let old_order = create_test_order(1, OrderType::Limit, Direction::Long, 1100, 2, slot);
    let mut new_order = create_test_order(1, OrderType::Limit, Direction::Long, 1100, 4, slot); // Changed size from 2 to 4
    new_order.post_only = true;
    dlob.update_order(&user, new_order, old_order);

    // Get updated L2 snapshot
    dlob.update_slot_and_oracle_price(0, MarketType::Perp, slot, oracle_price);
    let l2book = dlob.get_l2_snapshot(0, MarketType::Perp);

    // Verify order was updated
    assert_eq!(l2book.bids.get(&1100), Some(&10)); // 4 (updated) + 6 (floating limit) = 10
    assert_eq!(l2book.bids.len(), 3);

    // Remove an order
    let old_order = create_test_order(3, OrderType::Market, Direction::Long, 0, 4, slot);
    let mut new_order = create_test_order(3, OrderType::Market, Direction::Long, 1050, 4, slot);
    new_order.base_asset_amount_filled = old_order.base_asset_amount; // Set filled amount equal to total amount
    dlob.update_order(&user, new_order, old_order);

    // Get updated L2 snapshot
    dlob.update_slot_and_oracle_price(0, MarketType::Perp, slot, oracle_price);
    let l2book = dlob.get_l2_snapshot(0, MarketType::Perp);

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
    dlob.update_order(&user, new_order, order);

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

    let crosses =
        dlob.find_crosses_for_auctions(market_index, market_type, slot, oracle_price, None);
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

    let crosses =
        dlob.find_crosses_for_auctions(market_index, market_type, slot, oracle_price, None);
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

    let crosses =
        dlob.find_crosses_for_auctions(market_index, market_type, slot, oracle_price, None);
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

    let crosses =
        dlob.find_crosses_for_auctions(market_index, market_type, slot, oracle_price, None);
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
    let oracle_price = 1000;

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
    dlob.update_order(&user, triggered_order, order);
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
    dlob.remove_order(&user, triggered_order);
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
    let mut order2 = create_test_order(2, OrderType::TriggerLimit, Direction::Short, 0, 5, slot);
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
    dlob.update_order(&user, triggered_order2, order2);
    {
        let book = dlob
            .markets
            .get(&MarketId::new(0, MarketType::Perp))
            .unwrap();
        assert_eq!(book.trigger_orders.asks.len(), 0);
        assert_eq!(book.market_orders.asks.len(), 1);
    }

    // --- Remove triggered limit order ---
    dlob.remove_order(&user, triggered_order2);
    {
        let book = dlob
            .markets
            .get(&MarketId::new(0, MarketType::Perp))
            .unwrap();
        assert_eq!(book.market_orders.asks.len(), 0);
        assert_eq!(book.trigger_orders.asks.len(), 0);
    }
}
