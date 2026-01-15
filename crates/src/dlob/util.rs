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
pub fn order_hash(user: &Pubkey, order_id: u32, market_index: u16) -> u64 {
    let mut hasher = AHasher::default();
    user.hash(&mut hasher);
    order_id.hash(&mut hasher);
    market_index.hash(&mut hasher);
    hasher.finish()
}

pub fn compare_user_orders(pubkey: Pubkey, old: &User, new: &User) -> Vec<OrderDelta> {
    let mut deltas = Vec::<OrderDelta>::with_capacity(16);

    for order in old.orders.iter().filter(|o| o.status == OrderStatus::Open) {
        deltas.push(OrderDelta::Remove {
            order: *order,
            user: pubkey,
        });
    }

    for order in new.orders.iter().filter(|o| o.status == OrderStatus::Open) {
        deltas.push(OrderDelta::Create {
            order: *order,
            user: pubkey,
        });
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
        assert_eq!(deltas.len(), 4);
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
