use dashmap::{DashMap, DashSet};
use drift::state::user::{Order, OrderStatus};
use solana_sdk::pubkey::Pubkey;

use crate::dlob::node_list::NodeList;
use crate::dlob::dlob_node::{DLOBNode, SortDirection, NodeType, get_order_signature, Node};
use crate::dlob::market::{Market, Exchange, OpenOrders, SUPPORTED_ORDER_TYPES, get_node_lists};
use crate::is_one_of_variant;
use crate::utils::market_type_to_string;


pub struct DLOB {
    exchange: Exchange,
    open_orders: OpenOrders,
    initialized: bool,
    max_slot_for_resting_limit_orders: u64
}

impl DLOB {
    pub fn new(max_slot_for_resting_limit_orders: u64) -> DLOB {

        let exchange = Exchange::new();
        exchange.insert("perp".to_string(), DashMap::new());
        exchange.insert("spot".to_string(), DashMap::new());

        let open_orders = OpenOrders::new();
        open_orders.insert("perp".to_string(), DashSet::new());
        open_orders.insert("spot".to_string(), DashSet::new());

        DLOB {
            exchange,
            open_orders,
            initialized: false,
            max_slot_for_resting_limit_orders
        }
    }

    pub fn add_market(&self, market_type: &str, market_index: u16) {
        if !self.exchange.contains_key(market_type) {
            self.exchange.insert(market_type.to_string(), DashMap::new());
        }

        if !self.exchange.get(market_type).unwrap().contains_key(&market_index) {
            self.exchange.get(market_type).unwrap().insert(market_index, Market::new());
        }
    }

    pub fn insert_order(&self, order: Order, user_account: Pubkey, slot: u64) {
        if order.status == OrderStatus::Init {
            return
        }

        if !is_one_of_variant(&order.order_type, &SUPPORTED_ORDER_TYPES) {
            return
        } 

        let market_type = market_type_to_string(&order.market_type);

        if !self.exchange.get(&market_type).unwrap().contains_key(&order.market_index) {
            self.add_market(&market_type, order.market_index);
        }

        if order.status == OrderStatus::Open {
            self.open_orders.get(&market_type).unwrap().insert(get_order_signature(order.order_id, user_account));
        }

        if let Some(mut market) = self.exchange.get_mut(&market_type).unwrap().get_mut(&order.market_index) {
            if let Some(node_list) = market.get_list_for_order(&order, slot) {
                let node = Node::new(node_list.node_type, order, user_account);
                node_list.insert(node);
            }
        }
    }

    pub fn get_order(&self, order_id: u32, user_account: Pubkey) -> Option<Order> {
        let order_signature = get_order_signature(order_id, user_account);
        for node_list in get_node_lists(&self.exchange) {
            if let Some(node) = node_list.get_node(&order_signature) {
                return Some(node.get_order().clone());
            }
        }

        None
    }
}


#[cfg(test)]
mod tests {
    use drift::state::user::{MarketType, OrderType};

    use super::*;
    
    #[test]
    fn test_insert_order() {
        let dlob = DLOB::new(0);
        let order = Order {
            order_id: 1,
            market_type: MarketType::Spot,
            market_index: 1,
            order_type: OrderType::Limit,
            status: OrderStatus::Open,
            ..Order::default()
        };
        let user_account = Pubkey::new_unique();
        let slot = 0;
        dlob.insert_order(order, user_account, slot);


        assert!(dlob.get_order(order.order_id, user_account).is_some());
    }
}