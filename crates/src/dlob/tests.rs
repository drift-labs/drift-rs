use solana_sdk::pubkey::Pubkey;

use crate::{
    dlob::{
        types::DynamicPrice, util::order_hash, Direction, OrderKind, OrderMetadata, TakerOrder,
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
    let dlob = DLOB::default();
    let user = Pubkey::new_unique();
    let slot = 100;

    // Insert bids in random order
    let mut order = create_test_order(1, OrderType::Market, Direction::Long, 100, 1, slot);
    order.auction_duration = 10;
    dlob.insert_order(&user, order);
    let mut order = create_test_order(2, OrderType::Market, Direction::Long, 200, 1, slot);
    order.auction_duration = 10;
    dlob.insert_order(&user, order);
    let mut order = create_test_order(3, OrderType::Market, Direction::Long, 150, 1, slot);
    order.auction_duration = 10;
    dlob.insert_order(&user, order);

    // Insert asks in random order
    let mut order = create_test_order(4, OrderType::Market, Direction::Short, 300, 1, slot);
    order.auction_duration = 10;
    dlob.insert_order(&user, order);
    let mut order = create_test_order(5, OrderType::Market, Direction::Short, 250, 1, slot);
    order.auction_duration = 10;
    dlob.insert_order(&user, order);
    let mut order = create_test_order(6, OrderType::Market, Direction::Short, 350, 1, slot);
    order.auction_duration = 10;
    dlob.insert_order(&user, order);

    let oracle_price = 100_000;
    dlob.update_slot_and_oracle_price(0, MarketType::Perp, slot, oracle_price);
    let book = dlob
        .markets
        .get(&MarketId::new(0, MarketType::Perp))
        .unwrap();

    // Verify bids are sorted highest to lowest
    let market_tick_size = 1;
    assert!(book
        .market_orders
        .bids
        .iter()
        .map(|x| x.get_price(slot, oracle_price, market_tick_size))
        .eq([200, 150, 100]));
    // Verify asks are sorted lowest to highest
    assert!(book
        .market_orders
        .asks
        .iter()
        .map(|x| x.get_price(slot, oracle_price, market_tick_size))
        .eq([250, 300, 350]));
}

#[test]
fn dlob_limit_order_sorting() {
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
    let bid_prices: Vec<u64> = book
        .resting_limit_orders
        .bids
        .iter()
        .map(|(_, v)| v.get_price())
        .collect();
    assert_eq!(bid_prices, vec![200, 150, 100]);

    // Verify asks are sorted lowest to highest
    let ask_prices: Vec<u64> = book
        .resting_limit_orders
        .asks
        .iter()
        .map(|(_, v)| v.get_price())
        .collect();
    assert_eq!(ask_prices, vec![250, 300, 350]);
}

#[test]
fn dlob_floating_limit_order_sorting() {
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
    let dlob = DLOB::default();
    let user = Pubkey::new_unique();
    let slot = 100;

    // Insert bids in random order
    let mut order = create_test_order(1, OrderType::Oracle, Direction::Long, 100, 1, slot);
    order.oracle_price_offset = 10;
    order.auction_start_price = 10;
    order.auction_end_price = 20;
    order.auction_duration = 10;
    dlob.insert_order(&user, order);

    let mut order = create_test_order(2, OrderType::Oracle, Direction::Long, 200, 1, slot);
    order.oracle_price_offset = 30;
    order.auction_start_price = 30;
    order.auction_end_price = 40;
    order.auction_duration = 10;
    dlob.insert_order(&user, order);

    let mut order = create_test_order(3, OrderType::Oracle, Direction::Long, 150, 1, slot);
    order.oracle_price_offset = 20;
    order.auction_start_price = 20;
    order.auction_end_price = 30;
    order.auction_duration = 10;
    dlob.insert_order(&user, order);

    // Insert asks in random order
    let mut order = create_test_order(4, OrderType::Oracle, Direction::Short, 300, 1, slot);
    order.oracle_price_offset = -30;
    order.auction_start_price = -30;
    order.auction_end_price = -20;
    order.auction_duration = 10;
    dlob.insert_order(&user, order);

    let mut order = create_test_order(5, OrderType::Oracle, Direction::Short, 250, 1, slot);
    order.oracle_price_offset = -20;
    order.auction_start_price = -20;
    order.auction_end_price = -10;
    order.auction_duration = 10;
    dlob.insert_order(&user, order);

    let mut order = create_test_order(6, OrderType::Oracle, Direction::Short, 350, 1, slot);
    order.oracle_price_offset = -10;
    order.auction_start_price = -10;
    order.auction_end_price = 0;
    order.auction_duration = 10;
    dlob.insert_order(&user, order);

    dlob.update_slot_and_oracle_price(0, MarketType::Perp, slot, 1);
    let book = dlob
        .markets
        .get(&MarketId::new(0, MarketType::Perp))
        .unwrap();

    // Verify bids are sorted highest to lowest start price offset
    let bid_offsets: Vec<i64> = book
        .oracle_orders
        .bids
        .iter()
        .map(|v| v.start_price_offset)
        .collect();
    assert_eq!(bid_offsets, vec![30, 20, 10]);

    // Verify asks are sorted lowest to highest start price offset
    let ask_offsets: Vec<i64> = book
        .oracle_orders
        .asks
        .iter()
        .map(|v| v.start_price_offset)
        .collect();
    assert_eq!(ask_offsets, vec![-30, -20, -10]);
}

#[test]
fn dlob_same_order_different_users() {
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
    let dlob = DLOB::default();
    let user = Pubkey::new_unique();
    let slot = 100;
    let oracle_price = 1000;

    // Insert resting limit orders
    let mut order = create_test_order(1, OrderType::Limit, Direction::Long, 1100, 2, slot);
    order.post_only = true;
    dlob.insert_order(&user, order);

    let mut order = create_test_order(2, OrderType::Limit, Direction::Short, 900, 3, slot);
    order.post_only = true;
    dlob.insert_order(&user, order);

    // Insert market orders (dynamic price)
    let mut order = create_test_order(3, OrderType::Market, Direction::Long, 1050, 4, slot);
    order.auction_duration = 10;
    dlob.insert_order(&user, order);

    let mut order = create_test_order(4, OrderType::Market, Direction::Short, 950, 5, slot);
    order.auction_duration = 10;
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
    assert_eq!(l2book.bids.get(&1100), Some(&8));
    // At 1050: 4 (market)
    assert_eq!(l2book.bids.get(&1050), Some(&4));

    // Verify ask prices and sizes
    // At 900: 3 (resting limit) + 7 (floating limit) = 10
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
    let mut order = create_test_order(1, OrderType::Limit, Direction::Long, 1100, 4, slot); // Changed size from 2 to 4
    order.post_only = true;
    dlob.update_order(&user, order);

    // Get updated L2 snapshot
    dlob.update_slot_and_oracle_price(0, MarketType::Perp, slot, oracle_price);
    let l2book = dlob.get_l2_snapshot(0, MarketType::Perp);

    // Verify order was updated
    assert_eq!(l2book.bids.get(&1100), Some(&10)); // 4 (updated) + 6 (floating limit) = 10
    assert_eq!(l2book.bids.len(), 3);

    // Remove an order
    let mut order = create_test_order(3, OrderType::Market, Direction::Long, 1050, 4, slot);
    order.base_asset_amount_filled = order.base_asset_amount; // Set filled amount equal to total amount
    dlob.update_order(&user, order);

    // Get updated L2 snapshot
    dlob.update_slot_and_oracle_price(0, MarketType::Perp, slot, oracle_price);
    let l2book = dlob.get_l2_snapshot(0, MarketType::Perp);

    // Verify order was removed
    assert_eq!(l2book.bids.get(&1050), None);
    assert_eq!(l2book.bids.len(), 2);
}

#[test]
fn dlob_find_crosses_for_taker_order_full_fill() {
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

    let result = dlob.find_crosses_for_taker_order(slot, oracle_price, taker_order);

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
            order_hash(&user, 1),
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
            order_hash(&user, 2),
            2
        )
    );
    assert!(!result.is_partial);
}

#[test]
fn dlob_find_crosses_for_taker_order_partial_fill() {
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

    let result = dlob.find_crosses_for_taker_order(slot, oracle_price, taker_order);

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
            order_hash(&user, 1),
            3
        )
    );
    assert!(result.is_partial);
}

#[test]
fn dlob_find_crosses_for_taker_order_no_cross() {
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

    let result = dlob.find_crosses_for_taker_order(slot, oracle_price, taker_order);

    // Should not fill any orders
    assert_eq!(result.orders.len(), 0);
    assert!(result.is_partial);
}

#[test]
fn dlob_find_crosses_for_taker_order_floating_limit() {
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

    let result = dlob.find_crosses_for_taker_order(slot, oracle_price, taker_order);

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
            order_hash(&user, 1),
            5
        )
    );
    assert!(!result.is_partial);
}

#[test]
fn dlob_find_crosses_for_taker_order_price_priority() {
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

    let result = dlob.find_crosses_for_taker_order(slot, oracle_price, taker_order);

    // Should fill the better price first (900)
    assert_eq!(result.orders.len(), 2);
    assert_eq!(
        result.orders[0],
        (
            OrderMetadata {
                order_id: 1,
                user,
                kind: OrderKind::Limit
            },
            order_hash(&user, 2),
            3
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
            order_hash(&user, 1),
            2
        )
    );
    assert!(!result.is_partial);
}

#[test]
fn dlob_auction_expiry_market_orders() {
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
    assert_eq!(book.resting_limit_orders.bids.len(), 1);
    assert_eq!(book.resting_limit_orders.asks.len(), 0);

    // Oracle orders should be moved to floating limit or removed
    assert_eq!(book.oracle_orders.bids.len(), 0);
    assert_eq!(book.oracle_orders.asks.len(), 0);
    assert_eq!(book.floating_limit_orders.bids.len(), 0);
    assert_eq!(book.floating_limit_orders.asks.len(), 1);
}

#[test]
fn dlob_zero_size_order_handling() {
    let dlob = DLOB::default();
    let user = Pubkey::new_unique();
    let slot = 100;
    let oracle_price = 1000;

    // Test 1: Insert fully filled order (should be skipped)
    let mut order = create_test_order(1, OrderType::Limit, Direction::Long, 1000, 10, slot);
    order.base_asset_amount_filled = 10; // Fully filled
    dlob.insert_order(&user, order);

    // Verify no orders in book
    let book = dlob
        .markets
        .get(&MarketId::new(0, MarketType::Perp))
        .unwrap();
    assert_eq!(book.resting_limit_orders.bids.len(), 0);

    // Test 2: Update order to be fully filled (should be removed)
    let mut order = create_test_order(2, OrderType::Limit, Direction::Long, 1000, 10, slot);
    dlob.insert_order(&user, order);
    order.base_asset_amount_filled = 10; // Fully fill it
    dlob.update_order(&user, order);

    // Verify order was removed
    let book = dlob
        .markets
        .get(&MarketId::new(0, MarketType::Perp))
        .unwrap();
    assert_eq!(book.resting_limit_orders.bids.len(), 0);

    // Test 3: Auction order expiring with zero size (should be removed)
    let mut order = create_test_order(3, OrderType::Limit, Direction::Long, 1000, 10, slot);
    order.auction_duration = 5;
    order.base_asset_amount_filled = 10; // Fully filled
    dlob.insert_order(&user, order);

    // Update to slot after auction end
    dlob.update_slot_and_oracle_price(0, MarketType::Perp, slot + 6, oracle_price);

    // Verify no orders in book
    let book = dlob
        .markets
        .get(&MarketId::new(0, MarketType::Perp))
        .unwrap();
    assert_eq!(book.resting_limit_orders.bids.len(), 0);
    assert_eq!(book.market_orders.bids.len(), 0);

    // Test 4: Verify L2Book doesn't show zero-sized orders
    let mut order = create_test_order(4, OrderType::Limit, Direction::Long, 1000, 10, slot);
    dlob.insert_order(&user, order);
    order.base_asset_amount_filled = 10; // Fully fill it
    dlob.update_order(&user, order);

    // Add another order to ensure book is not empty
    let order = create_test_order(5, OrderType::Limit, Direction::Long, 1000, 10, slot);
    dlob.insert_order(&user, order);

    // Get L2 snapshot
    let l2book = dlob.get_l2_snapshot(0, MarketType::Perp);

    // Verify only non-zero size orders are in L2 book
    assert_eq!(l2book.bids.get(&1000), Some(&10));
    assert_eq!(l2book.bids.len(), 1);
}

#[test]
fn dlob_zero_size_auction_orders() {
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

    // Verify no orders in book
    let book = dlob
        .markets
        .get(&MarketId::new(0, MarketType::Perp))
        .unwrap();
    assert_eq!(book.resting_limit_orders.bids.len(), 0);
    assert_eq!(book.market_orders.bids.len(), 0);

    // Test 2: Oracle order auction expiring with zero size
    let mut order = create_test_order(2, OrderType::Limit, Direction::Long, 0, 10, slot);
    order.auction_duration = 5;
    order.oracle_price_offset = 100;
    order.base_asset_amount_filled = 10; // Fully filled
    dlob.insert_order(&user, order);

    // Update to slot after auction end
    dlob.update_slot_and_oracle_price(0, MarketType::Perp, slot + 6, oracle_price);

    // Verify no orders in book
    let book = dlob
        .markets
        .get(&MarketId::new(0, MarketType::Perp))
        .unwrap();
    assert_eq!(book.floating_limit_orders.bids.len(), 0);
    assert_eq!(book.oracle_orders.bids.len(), 0);

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
    let dlob = DLOB::default();
    let market_index = 0;
    let market_type = MarketType::Perp;
    let slot = 100;
    let oracle_price = 1000;

    // Insert a resting limit ask at price 1000
    let limit_order = create_test_order(1, OrderType::Limit, Direction::Short, 1000, 100, slot);
    dlob.insert_order(&Pubkey::new_unique(), limit_order);

    // Insert a market bid that should cross
    let market_order = create_test_order(
        2,
        OrderType::Market,
        Direction::Long, // This is correct as it's a bid
        1100,            // Higher price than limit ask
        50,
        slot,
    );
    dlob.insert_order(&Pubkey::new_unique(), market_order);

    let crosses = dlob.find_crosses_for_auctions(market_index, market_type, slot, oracle_price);
    assert_eq!(crosses.len(), 1);
    assert!(!crosses[0].is_empty());
    assert_eq!(crosses[0].orders.len(), 1);
    assert_eq!(crosses[0].orders[0].2, 50); // Fill size should be 50
}

#[test]
fn dlob_find_crosses_for_auctions_oracle_orders() {
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

    let crosses = dlob.find_crosses_for_auctions(market_index, market_type, slot, oracle_price);
    assert_eq!(crosses.len(), 1);
    assert!(!crosses[0].is_empty());
    assert_eq!(crosses[0].orders.len(), 1);
    assert_eq!(crosses[0].orders[0].2, 50); // Fill size should be 50
}

#[test]
fn dlob_find_crosses_for_auctions_no_crosses() {
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

    let crosses = dlob.find_crosses_for_auctions(market_index, market_type, slot, oracle_price);
    assert!(crosses.is_empty());
}

#[test]
fn dlob_find_crosses_for_auctions_comprehensive() {
    let dlob = DLOB::default();
    let market_index = 0;
    let market_type = MarketType::Perp;
    let slot = 100;
    let oracle_price = 1000;

    // Insert resting limit orders
    let limit_ask_1 = create_test_order(1, OrderType::Limit, Direction::Short, 1000, 100, slot);
    let limit_ask_2 = create_test_order(2, OrderType::Limit, Direction::Short, 1100, 50, slot);
    let limit_bid_1 = create_test_order(3, OrderType::Limit, Direction::Long, 900, 75, slot);
    let limit_bid_2 = create_test_order(4, OrderType::Limit, Direction::Long, 800, 25, slot);

    dlob.insert_order(&Pubkey::new_unique(), limit_ask_1);
    dlob.insert_order(&Pubkey::new_unique(), limit_ask_2);
    dlob.insert_order(&Pubkey::new_unique(), limit_bid_1);
    dlob.insert_order(&Pubkey::new_unique(), limit_bid_2);

    // Insert market orders (some should cross, some shouldn't)
    let market_bid_1 = create_test_order(5, OrderType::Market, Direction::Long, 1100, 30, slot); // Should cross with limit_ask_1
    let market_bid_2 = create_test_order(6, OrderType::Market, Direction::Long, 900, 40, slot); // Shouldn't cross
    let market_ask_1 = create_test_order(7, OrderType::Market, Direction::Short, 800, 20, slot); // Should cross with limit_bid_1
    let market_ask_2 = create_test_order(8, OrderType::Market, Direction::Short, 1200, 60, slot); // Shouldn't cross

    dlob.insert_order(&Pubkey::new_unique(), market_bid_1);
    dlob.insert_order(&Pubkey::new_unique(), market_bid_2);
    dlob.insert_order(&Pubkey::new_unique(), market_ask_1);
    dlob.insert_order(&Pubkey::new_unique(), market_ask_2);

    // Insert oracle orders (some should cross, some shouldn't)
    let oracle_bid_1 = create_test_order(9, OrderType::Oracle, Direction::Long, 100, 35, slot); // Price 1100, should cross with limit_ask_1
    let oracle_bid_2 = create_test_order(10, OrderType::Oracle, Direction::Long, -200, 45, slot); // Price 800, shouldn't cross
    let oracle_ask_1 = create_test_order(11, OrderType::Oracle, Direction::Short, -150, 25, slot); // Price 850, should cross with limit_bid_1
    let oracle_ask_2 = create_test_order(12, OrderType::Oracle, Direction::Short, 200, 55, slot); // Price 1200, shouldn't cross

    dlob.insert_order(&Pubkey::new_unique(), oracle_bid_1);
    dlob.insert_order(&Pubkey::new_unique(), oracle_bid_2);
    dlob.insert_order(&Pubkey::new_unique(), oracle_ask_1);
    dlob.insert_order(&Pubkey::new_unique(), oracle_ask_2);

    let crosses = dlob.find_crosses_for_auctions(market_index, market_type, slot, oracle_price);

    // Should find 4 crosses:
    // 1. market_bid_1 (30) + oracle_bid_1 (35) crossing with limit_ask_1 (100)
    // 2. market_ask_1 (20) + oracle_ask_1 (25) crossing with limit_bid_1 (75)
    assert_eq!(crosses.len(), 2);

    // Verify the first cross (against limit_ask_1)
    let first_cross = &crosses[0];
    assert!(!first_cross.is_empty());
    assert_eq!(first_cross.orders.len(), 2);
    assert_eq!(first_cross.orders[0].2, 30); // market_bid_1 size
    assert_eq!(first_cross.orders[1].2, 35); // oracle_bid_1 size

    // Verify the second cross (against limit_bid_1)
    let second_cross = &crosses[1];
    assert!(!second_cross.is_empty());
    assert_eq!(second_cross.orders.len(), 2);
    assert_eq!(second_cross.orders[0].2, 20); // market_ask_1 size
    assert_eq!(second_cross.orders[1].2, 25); // oracle_ask_1 size
}
