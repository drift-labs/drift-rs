use dashmap::{DashMap, DashSet};
use drift::state::user::{MarketType, Order, OrderStatus};
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
            initialized: true,
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

    pub fn delete_order(&self, order_id: u32, user_account: Pubkey) {
        let order_signature = get_order_signature(order_id, user_account);
        for mut node_list in get_node_lists(&self.exchange) {
            if let Some(_) = node_list.get_node(&order_signature) {
                node_list.remove(&order_signature);
            }
        }
    }

    fn update_resting_limit_orders_for_market_type(&self, market_type: &MarketType, slot: u64) {
        let mut ask_order_sigs_to_update: Vec<String> = vec![];
        let mut bid_order_sigs_to_update: Vec<String> = vec![];
        let market_type = market_type_to_string(market_type);
        if let Some(market) = self.exchange.get_mut(&market_type) {
            for mut market_ref in market.iter_mut() {
                let market = market_ref.value_mut();
                for node in market.taking_limit_orders.taking_limit_bids.iter() {
                    if !node.get_order().is_resting_limit_order(slot).unwrap() {
                        continue;
                    }
                    bid_order_sigs_to_update.push(get_order_signature(node.get_order().order_id, node.get_user_account()));
                }
                for node in market.taking_limit_orders.taking_limit_asks.iter() {
                    if !node.get_order().is_resting_limit_order(slot).unwrap() {
                        continue;
                    }
                    ask_order_sigs_to_update.push(get_order_signature(node.get_order().order_id, node.get_user_account()));
                }
    
                for order_sig in ask_order_sigs_to_update.iter() {
                    if let Some(mut node) = market.taking_limit_orders.taking_limit_asks.remove(order_sig) {
                        node.set_node_type(NodeType::RestingLimit);
                        market.resting_limit_orders.resting_limit_asks.insert(node);
                    }
                }
    
                for order_sig in bid_order_sigs_to_update.iter() {
                    if let Some(mut node) = market.taking_limit_orders.taking_limit_bids.remove(order_sig) {
                        node.set_node_type(NodeType::RestingLimit);
                        market.resting_limit_orders.resting_limit_bids.insert(node);
                    }
                }
            }
        }
    }
    pub fn update_resting_limit_orders(&mut self, slot: u64) {
        if slot <= self.max_slot_for_resting_limit_orders {
            return
        }

        self.max_slot_for_resting_limit_orders = slot;

        self.update_resting_limit_orders_for_market_type(&MarketType::Perp, slot);
        self.update_resting_limit_orders_for_market_type(&MarketType::Spot, slot);

    }

    pub fn update_order(&mut self, order: Order, user_account: Pubkey, slot: u64, cumulative_base_asset_amount_filled: u64) {
        self.update_resting_limit_orders(slot);

        if order.base_asset_amount == cumulative_base_asset_amount_filled {
            self.delete_order(order.order_id, user_account);
            return
        }

        if order.base_asset_amount_filled == cumulative_base_asset_amount_filled {
            return
        }

        let mut new_order = order.clone();

        new_order.base_asset_amount_filled = cumulative_base_asset_amount_filled;

        if let Some(mut market) = self.exchange.get_mut(&market_type_to_string(&order.market_type)).unwrap().get_mut(&order.market_index) {
            if let Some(node_list) = market.get_list_for_order(&order, slot) {
                node_list.update_order(&get_order_signature(order.order_id, user_account), new_order);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use drift::{controller::position::PositionDirection, state::user::{MarketType, OrderType}};

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
        assert!(dlob.get_order(2, user_account).is_none());
    }

    #[test]
    fn test_delete_order() {
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

        dlob.delete_order(order.order_id, user_account);

        assert!(dlob.get_order(order.order_id, user_account).is_none());
    }

    #[test]
    fn test_update_order_cumulative_baa_filled_below_order_baa_filled() {
        
        let mut dlob = DLOB::new(0);
        let order = Order {
            order_id: 1,
            market_type: MarketType::Spot,
            market_index: 1,
            order_type: OrderType::Limit,
            status: OrderStatus::Open,
            slot: 0,
            ..Order::default()
        };
        let user_account = Pubkey::new_unique();
        let slot = 0;
        dlob.insert_order(order, user_account, slot);

        assert!(dlob.get_order(order.order_id, user_account).unwrap().slot == 0);

        let updated_order = Order {
            order_id: 1,
            market_type: MarketType::Spot,
            market_index: 1,
            order_type: OrderType::Limit,
            status: OrderStatus::Open,
            slot: 1,
            base_asset_amount_filled: 1_000_000,
            ..Order::default()
        };
        dlob.update_order(updated_order, user_account, slot, 500_000);

        assert!(dlob.get_order(order.order_id, user_account).unwrap().slot == 1);
    }

    #[test]
    fn test_update_order_cumulative_baa_equal_order_baa_filled() {    
        let mut dlob = DLOB::new(0);
        let order = Order {
            order_id: 1,
            market_type: MarketType::Spot,
            market_index: 1,
            order_type: OrderType::Limit,
            status: OrderStatus::Open,
            slot: 0,
            ..Order::default()
        };
        let user_account = Pubkey::new_unique();
        let slot = 0;
        dlob.insert_order(order, user_account, slot);

        assert!(dlob.get_order(order.order_id, user_account).unwrap().slot == 0);

        let updated_order = Order {
            order_id: 1,
            market_type: MarketType::Spot,
            market_index: 1,
            order_type: OrderType::Limit,
            status: OrderStatus::Open,
            slot: 1,
            base_asset_amount: 1_000_000,
            ..Order::default()
        };
        dlob.update_order(updated_order, user_account, slot, 1_000_000);

        assert!(dlob.get_order(order.order_id, user_account).is_none());
    }

    #[test]
    fn test_update_resting_limit_nodes() {
        let mut dlob = DLOB::new(0);
        let order = Order {
            order_id: 1,
            market_type: MarketType::Spot,
            market_index: 1,
            order_type: OrderType::Limit,
            status: OrderStatus::Open,
            slot: 0,
            ..Order::default()
        };
        let order_2 = Order {
            order_id: 2,
            market_type: MarketType::Spot,
            market_index: 1,
            order_type: OrderType::Limit,
            status: OrderStatus::Open,
            slot: 0,
            auction_duration: 100,
            ..Order::default()
        };
        let user_account = Pubkey::new_unique();
        let slot = 1;
        dlob.insert_order(order, user_account, slot);
        dlob.insert_order(order_2, user_account, slot);

        let updated_order = Order {
            order_id: 1,
            market_type: MarketType::Spot,
            market_index: 1,
            order_type: OrderType::Limit,
            status: OrderStatus::Open,
            slot: 1,
            base_asset_amount: 1_000_000,
            direction: PositionDirection::Long,
            ..Order::default()
        };

        let update_slot = 105;
        dlob.update_order(updated_order, user_account, update_slot, 1_000_000);

        assert!(dlob.get_order(order.order_id, user_account).is_none());

        let side = &dlob.exchange.get("spot").unwrap();
        let node_list = &side.get(&1).unwrap().resting_limit_orders.resting_limit_bids;
        let order_sig = get_order_signature(order_2.order_id, user_account);

        assert!(node_list.contains(&order_sig));
    }
}