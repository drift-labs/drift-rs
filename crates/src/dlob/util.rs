use std::hash::{Hash, Hasher};

use crate::types::{accounts::User, Order, OrderStatus};
use ahash::AHasher;
use solana_sdk::pubkey::Pubkey;

/// change of order signal dlob
#[derive(Debug, PartialEq, Clone)]
pub enum OrderDelta {
    Create {
        order: Order,
        user: Pubkey,
    },
    Update {
        new_order: Order,
        old_order: Order,
        user: Pubkey,
    },
    Remove {
        order: Order,
        user: Pubkey,
    },
}

/// Helper function to generate unique order Id hash for internal DLOB use
pub fn order_hash(user: &Pubkey, order_id: u32) -> u64 {
    let mut hasher = AHasher::default();
    user.hash(&mut hasher);
    order_id.hash(&mut hasher);
    hasher.finish()
}

pub fn compare_user_orders(pubkey: Pubkey, old: &User, new: &User) -> Vec<OrderDelta> {
    let mut deltas = Vec::<OrderDelta>::with_capacity(16);
    // relies on the layout of orders and transitions made by the program
    // 1) orders transition from to open to closed/filled,
    // 2) not open orders can be replaced with new orders
    // 3) orders that remain open were possibly updated
    // 4) use order_id to determine if we're looking at the same order or different orders
    for (old_order, new_order) in old.orders.iter().zip(new.orders.iter()) {
        // Check if we're looking at the same order (same order_id) or different orders
        if old_order.order_id == new_order.order_id {
            // Same order - check for updates or status changes
            match (old_order.status, new_order.status) {
                (OrderStatus::Open, OrderStatus::Open) => {
                    // Same order, both open - check if it was updated
                    if new_order != old_order {
                        deltas.push(OrderDelta::Update {
                            user: pubkey,
                            new_order: *new_order,
                            old_order: *old_order,
                        });
                    }
                }
                (OrderStatus::Open, _) => {
                    // Same order, was open now filled/cancelled - remove it
                    deltas.push(OrderDelta::Remove {
                        user: pubkey,
                        order: *old_order, // Use old_order since it was the one that was removed
                    });
                }
                (_, OrderStatus::Open) => {
                    // invalid transition e.g. out of order update
                }
                _ => {
                    // Same order, both not open - no change needed
                }
            }
        } else {
            // Different orders - this means one was replaced by another
            match (old_order.status, new_order.status) {
                (OrderStatus::Open, OrderStatus::Open) => {
                    // Old order was open, new order is open - remove old, create new
                    deltas.push(OrderDelta::Remove {
                        user: pubkey,
                        order: *old_order,
                    });
                    deltas.push(OrderDelta::Create {
                        user: pubkey,
                        order: *new_order,
                    });
                }
                (OrderStatus::Open, _) => {
                    // Old order was open, new order is not open - remove old
                    deltas.push(OrderDelta::Remove {
                        user: pubkey,
                        order: *old_order,
                    });
                }
                (_, OrderStatus::Open) => {
                    // Old order was not open, new order is open - create new
                    deltas.push(OrderDelta::Create {
                        user: pubkey,
                        order: *new_order,
                    });
                }
                _ => {
                    // Both orders not open - no change needed
                }
            }
        }
    }

    deltas
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{MarketType, OrderType, PositionDirection};

    fn create_test_order(id: u32, status: OrderStatus) -> Order {
        Order {
            slot: 0,
            price: 100,
            base_asset_amount: 1000,
            base_asset_amount_filled: 0,
            quote_asset_amount_filled: 0,
            order_id: id,
            market_index: 0,
            status,
            order_type: OrderType::Limit,
            market_type: MarketType::Perp,
            existing_position_direction: PositionDirection::Long,
            direction: PositionDirection::Long,
            reduce_only: false,
            post_only: false,
            immediate_or_cancel: false,
            trigger_condition: crate::types::OrderTriggerCondition::Above,
            ..Default::default()
        }
    }

    fn create_test_user(orders: Vec<Order>) -> User {
        let mut user = User::default();
        for (i, order) in orders.into_iter().enumerate() {
            if i < 32 {
                user.orders[i] = order;
            }
        }
        user
    }

    #[test]
    fn dlob_util_test_empty_orders() {
        let pubkey = Pubkey::new_unique();
        let old = create_test_user(vec![]);
        let new = create_test_user(vec![]);

        let deltas = compare_user_orders(pubkey, &old, &new);
        assert!(deltas.is_empty());
    }

    #[test]
    fn dlob_util_test_create_order() {
        let pubkey = Pubkey::new_unique();
        let old = create_test_user(vec![]);
        let new = create_test_user(vec![create_test_order(1, OrderStatus::Open)]);

        let deltas = compare_user_orders(pubkey, &old, &new);
        assert_eq!(deltas.len(), 1);

        match &deltas[0] {
            OrderDelta::Create { user, order } => {
                assert_eq!(*user, pubkey);
                assert_eq!(order.order_id, 1);
            }
            _ => panic!("Expected Create delta"),
        }
    }

    #[test]
    fn dlob_util_test_remove_order() {
        let pubkey = Pubkey::new_unique();
        let old = create_test_user(vec![create_test_order(1, OrderStatus::Open)]);
        let new = create_test_user(vec![]);

        let deltas = compare_user_orders(pubkey, &old, &new);
        assert_eq!(deltas.len(), 1);

        match &deltas[0] {
            OrderDelta::Remove { user, order } => {
                assert_eq!(*user, pubkey);
                assert_eq!(order.order_id, 1);
            }
            _ => panic!("Expected Remove delta"),
        }
    }

    #[test]
    fn dlob_util_test_remove_order_via_cancel() {
        let pubkey = Pubkey::new_unique();
        let mut order = create_test_order(1, OrderStatus::Open);
        let old = create_test_user(vec![order.clone()]);
        order.status = OrderStatus::Canceled;
        let new = create_test_user(vec![order]);

        let deltas = compare_user_orders(pubkey, &old, &new);
        assert_eq!(deltas.len(), 1);

        match &deltas[0] {
            OrderDelta::Remove { user, order } => {
                assert_eq!(*user, pubkey);
                assert_eq!(order.order_id, 1);
            }
            _ => panic!("Expected Remove delta"),
        }
    }

    #[test]
    fn dlob_util_test_update_order() {
        let pubkey = Pubkey::new_unique();
        let old_order = create_test_order(1, OrderStatus::Open);
        let mut new_order = create_test_order(1, OrderStatus::Open);
        new_order.price = 200; // Change the price

        let old = create_test_user(vec![old_order]);
        let new = create_test_user(vec![new_order]);

        let deltas = compare_user_orders(pubkey, &old, &new);
        assert_eq!(deltas.len(), 1);

        match &deltas[0] {
            OrderDelta::Update {
                user,
                new_order,
                old_order,
            } => {
                assert_eq!(*user, pubkey);
                assert_eq!(new_order.order_id, 1);
                assert_eq!(new_order.price, 200);
                assert_eq!(old_order.order_id, 1);
                assert_eq!(old_order.price, 100);
            }
            _ => panic!("Expected Update delta"),
        }
    }

    #[test]
    fn dlob_util_test_multiple_operations() {
        let pubkey = Pubkey::new_unique();
        let old = create_test_user(vec![
            create_test_order(1, OrderStatus::Open),
            create_test_order(2, OrderStatus::Open),
        ]);

        let mut new_order = create_test_order(1, OrderStatus::Open);
        new_order.price = 200;

        let new = create_test_user(vec![new_order, create_test_order(3, OrderStatus::Open)]);

        let deltas = compare_user_orders(pubkey, &old, &new);
        assert_eq!(deltas.len(), 3);
    }

    // Helper function to assert order replacement deltas
    fn assert_order_replacement_deltas(
        deltas: &[OrderDelta],
        expected_remove_id: u32,
        expected_create_id: u32,
    ) {
        assert_eq!(deltas.len(), 2);

        let mut has_remove = false;
        let mut has_create = false;

        for delta in deltas {
            match delta {
                OrderDelta::Remove { order, .. } => {
                    assert_eq!(order.order_id, expected_remove_id);
                    has_remove = true;
                }
                OrderDelta::Create { order, .. } => {
                    assert_eq!(order.order_id, expected_create_id);
                    has_create = true;
                }
                _ => panic!("Unexpected delta type: {:?}", delta),
            }
        }

        assert!(
            has_remove,
            "Should have Remove delta for order {}",
            expected_remove_id
        );
        assert!(
            has_create,
            "Should have Create delta for order {}",
            expected_create_id
        );
    }

    #[test]
    fn dlob_util_test_order_replacement() {
        let pubkey = Pubkey::new_unique();

        // Test basic order replacement: Order 1 (Open) → Order 2 (Open) at same index
        let old = create_test_user(vec![create_test_order(1, OrderStatus::Open)]);
        let new = create_test_user(vec![create_test_order(2, OrderStatus::Open)]);

        let deltas = compare_user_orders(pubkey, &old, &new);
        assert_order_replacement_deltas(&deltas, 1, 2);
    }

    #[test]
    fn dlob_util_test_multiple_order_replacements() {
        let pubkey = Pubkey::new_unique();

        // Test multiple simultaneous replacements
        let old = create_test_user(vec![
            create_test_order(1, OrderStatus::Open),
            create_test_order(2, OrderStatus::Open),
            create_test_order(3, OrderStatus::Open),
        ]);

        let new = create_test_user(vec![
            create_test_order(4, OrderStatus::Open), // replaced order 1
            create_test_order(2, OrderStatus::Open), // unchanged
            create_test_order(5, OrderStatus::Open), // replaced order 3
        ]);

        let deltas = compare_user_orders(pubkey, &old, &new);

        // Should have 2 Remove and 2 Create deltas
        assert_eq!(deltas.len(), 4);

        let mut remove_count = 0;
        let mut create_count = 0;

        for delta in &deltas {
            match delta {
                OrderDelta::Remove { order, .. } => {
                    assert!(order.order_id == 1 || order.order_id == 3);
                    remove_count += 1;
                }
                OrderDelta::Create { order, .. } => {
                    assert!(order.order_id == 4 || order.order_id == 5);
                    create_count += 1;
                }
                _ => panic!("Unexpected delta type: {:?}", delta),
            }
        }

        assert_eq!(remove_count, 2, "Should have 2 Remove deltas");
        assert_eq!(create_count, 2, "Should have 2 Create deltas");
    }

    #[test]
    fn dlob_util_test_atomic_create_and_fill() {
        let pubkey = Pubkey::new_unique();

        // Test case: Order doesn't exist in old (default/empty order), but exists in new
        // as a filled order (created and filled atomically). Should NOT emit Create+Remove.
        let old = create_test_user(vec![]); // Empty user, all slots have default Order (order_id=0, status=Init)
        let new = create_test_user(vec![create_test_order(1, OrderStatus::Filled)]); // Order created and filled atomically

        let deltas = compare_user_orders(pubkey, &old, &new);

        // Should emit NO deltas - the order was created and filled atomically, so it never
        // existed in an open state that we need to track
        assert_eq!(
            deltas.len(),
            0,
            "Should not emit deltas for atomically created and filled orders"
        );
    }

    #[test]
    fn dlob_util_test_atomic_create_and_cancel() {
        let pubkey = Pubkey::new_unique();

        // Similar test but with Canceled status
        let old = create_test_user(vec![]);
        let new = create_test_user(vec![create_test_order(1, OrderStatus::Canceled)]);

        let deltas = compare_user_orders(pubkey, &old, &new);

        assert_eq!(
            deltas.len(),
            0,
            "Should not emit deltas for atomically created and canceled orders"
        );
    }

    #[test]
    fn dlob_util_test_atomic_create_and_fill_with_existing_orders() {
        let pubkey = Pubkey::new_unique();

        // Test with other orders present to ensure the logic works in context
        let old = create_test_user(vec![
            create_test_order(1, OrderStatus::Open),
            // Slot 1 is empty (default order)
        ]);

        let new = create_test_user(vec![
            create_test_order(1, OrderStatus::Open),   // Unchanged
            create_test_order(2, OrderStatus::Filled), // Created and filled atomically at slot 1
        ]);

        let deltas = compare_user_orders(pubkey, &old, &new);

        // Should only have 0 deltas - order 1 is unchanged, order 2 was created and filled atomically
        assert_eq!(
            deltas.len(),
            0,
            "Should not emit deltas for atomically created and filled orders"
        );
    }

    #[test]
    fn dlob_util_test_filled_order_replacement() {
        let pubkey = Pubkey::new_unique();

        // Test case: Old order is filled (not open), new order is also filled (different order_id)
        // This simulates: old filled order gets replaced by a new order that was created and filled atomically
        // Should NOT emit Remove+Create since neither order was ever open in the DLOB
        let old = create_test_user(vec![create_test_order(1, OrderStatus::Filled)]);
        let new = create_test_user(vec![create_test_order(2, OrderStatus::Filled)]);

        let deltas = compare_user_orders(pubkey, &old, &new);

        // Should emit NO deltas - both orders are filled, so neither was ever open in the DLOB
        assert_eq!(
            deltas.len(),
            0,
            "Should not emit Remove+Create for replacement of filled orders"
        );
    }

    #[test]
    fn dlob_util_test_canceled_order_replacement() {
        let pubkey = Pubkey::new_unique();

        // Similar test but with Canceled status
        let old = create_test_user(vec![create_test_order(1, OrderStatus::Canceled)]);
        let new = create_test_user(vec![create_test_order(2, OrderStatus::Filled)]);

        let deltas = compare_user_orders(pubkey, &old, &new);

        assert_eq!(
            deltas.len(),
            0,
            "Should not emit Remove+Create for replacement of canceled order with filled order"
        );
    }
}
