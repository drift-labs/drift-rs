use std::hash::{Hash, Hasher};

use crate::types::{accounts::User, Order, OrderStatus};
use ahash::AHasher;
use solana_sdk::pubkey::Pubkey;

/// change of order signal dlob
#[derive(Debug, PartialEq, Clone)]
pub enum OrderDelta {
    Create { user: Pubkey, order: Order },
    Update { user: Pubkey, order: Order },
    Remove { user: Pubkey, order: Order },
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

    // Find orders to remove (in existing but not in new)
    for existing in old.orders.iter().filter(|o| o.status == OrderStatus::Open) {
        // order is no longer open, remove it
        if !new
            .orders
            .iter()
            .any(|o| o.order_id == existing.order_id && o.status == OrderStatus::Open)
        {
            // Status::Open => !Status::Open = remove
            deltas.push(OrderDelta::Remove {
                user: pubkey,
                order: *existing,
            });
        }
    }

    // Find orders to create or update
    for new_order in new.orders.iter().filter(|o| o.status != OrderStatus::Init) {
        match old.orders.iter().find(|o| o.order_id == new_order.order_id) {
            Some(existing) => {
                if let (OrderStatus::Open, OrderStatus::Open) = (existing.status, new_order.status)
                {
                    // open still open, maybe updated
                    if new_order != existing {
                        deltas.push(OrderDelta::Update {
                            user: pubkey,
                            order: *new_order,
                        });
                    }
                }
            }
            None => {
                // new order
                if new_order.status == OrderStatus::Open {
                    deltas.push(OrderDelta::Create {
                        user: pubkey,
                        order: *new_order,
                    });
                } else {
                    deltas.push(OrderDelta::Remove {
                        user: pubkey,
                        order: *new_order,
                    });
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
            trigger_price: 0,
            auction_start_price: 0,
            auction_end_price: 0,
            max_ts: 0,
            oracle_price_offset: 0,
            order_id: id,
            market_index: 0,
            status,
            order_type: OrderType::Limit,
            market_type: MarketType::Perp,
            user_order_id: 0,
            existing_position_direction: PositionDirection::Long,
            direction: PositionDirection::Long,
            reduce_only: false,
            post_only: false,
            immediate_or_cancel: false,
            trigger_condition: crate::types::OrderTriggerCondition::Above,
            auction_duration: 0,
            posted_slot_tail: 0,
            bit_flags: 0,
            padding: [0; 1],
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
            OrderDelta::Update { user, order } => {
                assert_eq!(*user, pubkey);
                assert_eq!(order.order_id, 1);
                assert_eq!(order.price, 200);
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
}
