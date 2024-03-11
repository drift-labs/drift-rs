use dashmap::{DashMap, DashSet};
use drift::state::oracle::OraclePriceData;
use drift::state::user::{MarketType, Order, OrderStatus};
use solana_sdk::pubkey::Pubkey;
use std::cell::Cell;

use crate::dlob::node_list::NodeList;
use crate::dlob::dlob_node::{DLOBNode, create_node, NodeType, get_order_signature, Node};
use crate::dlob::market::{Market, Exchange, OpenOrders, SUPPORTED_ORDER_TYPES, get_node_lists};
use crate::is_one_of_variant;
use crate::math::order::is_resting_limit_order;
use crate::utils::market_type_to_string;

use super::node_list::NodeListIter;

type Comparative = dyn Fn(&Node, &Node, u64, OraclePriceData) -> bool;

pub struct DLOB {
    exchange: Exchange,
    open_orders: OpenOrders,
    initialized: bool,
    max_slot_for_resting_limit_orders: Cell<u64>
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
            max_slot_for_resting_limit_orders: Cell::new(max_slot_for_resting_limit_orders)
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
                let node = create_node(&node_list.arena, node_list.node_type, order, user_account);
                dbg!(node.get_order().order_id);
                node_list.insert(*node);
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


    pub fn delete_order(&self, order: Order, user_account: Pubkey, slot: u64) {
        if order.status == OrderStatus::Init {
            return
        }

        self.update_resting_limit_orders(slot);

        let order_signature = get_order_signature(order.order_id, user_account);
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
                    if !is_resting_limit_order(node.get_order(), slot) {
                        continue;
                    }
                    bid_order_sigs_to_update.push(get_order_signature(node.get_order().order_id, node.get_user_account()));
                }
                for node in market.taking_limit_orders.taking_limit_asks.iter() {
                    if !is_resting_limit_order(node.get_order(), slot) {
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


    pub fn update_resting_limit_orders(&self, slot: u64) {
        if slot <= self.max_slot_for_resting_limit_orders.get() {
            return
        }

        self.max_slot_for_resting_limit_orders.set(slot);

        self.update_resting_limit_orders_for_market_type(&MarketType::Perp, slot);
        self.update_resting_limit_orders_for_market_type(&MarketType::Spot, slot);

    }

    
    pub fn update_order(&self, order: Order, user_account: Pubkey, slot: u64, cumulative_base_asset_amount_filled: u64) {
        self.update_resting_limit_orders(slot);

        if order.base_asset_amount == cumulative_base_asset_amount_filled {
            self.delete_order(order, user_account, slot);
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


    pub fn clear(&self) {
        for market_type_ref in self.open_orders.iter() {
            market_type_ref.value().clear();
        }

        self.open_orders.clear();

        for market_type_ref in self.exchange.iter() {
            for mut market_ref in market_type_ref.value().iter_mut() {
                market_ref.value_mut().clear();
            }
        }
    }

    fn get_node_list(
        &self,
        mut node_lists: Vec<NodeList>,
        oracle_price_data: OraclePriceData,
        slot: u64,
        comparative: Box<Comparative>,
    ) -> Vec<Node> {
        let mut nodes = vec![];
    
        while node_lists.iter().any(|list| !list.is_empty()) {
            let mut best_node: Option<Node> = None;
            let mut best_node_list_index = 0;
    
            for (index, node_list) in node_lists.iter().enumerate() {
                if let Some(node) = node_list.head() { 
                    if let Some(best) = best_node {
                        if comparative(&node, &best, slot, oracle_price_data) {
                            best_node = Some(node);
                            best_node_list_index = index;
                        }
                    } else {
                        best_node = Some(node);
                        best_node_list_index = index;
                    }
                }
            }
    
            if let Some(node) = best_node {
                let order = node.get_order();
                dbg!(format!("Removing node {}", order.order_id));
                let order_sig = get_order_signature(order.order_id, node.get_user_account());
                dbg!(format!("Removing node {}", order_sig.clone()));
                node_lists[best_node_list_index].remove(&order_sig);
                nodes.push(node);
            }
        }
        nodes
    }


    pub fn get_resting_limit_bids(&self, market_index: u16, slot: u64, market_type: MarketType, oracle_price_data: OraclePriceData) -> Vec<Node> {
       self.update_resting_limit_orders(slot);

        let market_lists = self.exchange.get(&market_type_to_string(&market_type)).unwrap();
        let market = market_lists.get(&market_index).unwrap();
        for node in market.resting_limit_orders.resting_limit_bids.iter() {
            dbg!(node.get_order().order_id);
        }
        
        let node_lists: Vec<NodeList> = vec![
            market.resting_limit_orders.resting_limit_bids.clone(),
            market.floating_limit_orders.floating_limit_bids.clone(),
        ];  

        let comparative = Box::new(|node_a: &Node, node_b: &Node, slot: u64, oracle_price_data: OraclePriceData| {
            node_a.get_price(oracle_price_data, slot) > node_b.get_price(oracle_price_data, slot)
        });

        self.get_node_list(node_lists, oracle_price_data, slot, comparative)
    }

    
    // pub fn get_resting_limit_asks(&self, market_index: u16, slot: u64, market_type: MarketType, oracle_price_data: OraclePriceData, filter: Option<Box<Filter>>) -> impl Iterator<Item = Node> {
    //     self.update_resting_limit_orders(slot);

    //     let market_lists = self.exchange.get(&market_type_to_string(&market_type)).unwrap();
    //     let market = market_lists.get(&market_index).unwrap();
    //     let node_lists: Vec<Box<dyn Iterator<Item = Node>>> = vec![
    //         market.resting_limit_orders.resting_limit_asks.iter(),
    //         market.floating_limit_orders.floating_limit_asks.iter(),
    //     ];  

    //     let comparative = Box::new(|node_a: &Node, node_b: &Node, slot: u64, oracle_price_data: OraclePriceData| {
    //         node_a.get_price(oracle_price_data, slot) > node_b.get_price(oracle_price_data, slot)
    //     });

    //     self.get_best_node(node_lists, oracle_price_data, slot, comparative, filter)
    // }
}


#[cfg(test)]
mod tests {
    use drift::{controller::position::PositionDirection, math::constants::BASE_PRECISION_U64, state::user::{MarketType, OrderType}};

    use super::*;

    fn insert_order_to_dlob(
        dlob: &DLOB,
        user_account: Pubkey,
        order_type: OrderType,
        market_type: MarketType,
        order_id: u32,
        market_index: u16,
        price: u64,
        base_asset_amount: u64,
        direction: PositionDirection, 
        auction_start_price: i64,
        auction_end_price: i64,
        slot: Option<u64>,
        max_ts: i64,
        oracle_price_offset: i32,
        post_only: bool,
        auction_duration: u8
    ) 
    {
        let slot = slot.unwrap_or(1);

        let order = Order {
            order_id,
            market_type,
            market_index,
            order_type,
            price,
            base_asset_amount,
            direction,
            auction_start_price,
            auction_end_price,
            slot,
            max_ts,
            oracle_price_offset,
            post_only,
            auction_duration,
            status: OrderStatus::Open,
            ..Order::default()
        };

        dlob.insert_order(order, user_account, slot);
    }
    

    // #[test]
    // fn test_insert_order() {
    //     let dlob = DLOB::new(0);
    //     let order = Order {
    //         order_id: 1,
    //         market_type: MarketType::Spot,
    //         market_index: 1,
    //         order_type: OrderType::Limit,
    //         status: OrderStatus::Open,
    //         ..Order::default()
    //     };
    //     let order_2 = Order {
    //         order_id: 2,
    //         market_type: MarketType::Perp,
    //         market_index: 1,
    //         order_type: OrderType::Limit,
    //         status: OrderStatus::Open,
    //         ..Order::default()
    //     };
    //     let user_account = Pubkey::new_unique();
    //     let slot = 0;
    //     dlob.insert_order(order, user_account, slot);
    //     dlob.insert_order(order_2, user_account, slot);

    //     for market in dlob.exchange.iter() {
    //         for market_ref in market.value().iter() {
    //             let market = market_ref.value();
    //             for node in market.resting_limit_orders.resting_limit_bids.iter() {
    //                 dbg!(node.get_order().order_id);
    //             }
    //         }
    //     }

    //     assert!(dlob.get_order(order.order_id, user_account).is_some());
    //     assert!(dlob.get_order(2, user_account).is_none());
    // }

    // #[test]
    // fn test_clear() {
    //     let dlob = DLOB::new(0);
    //     let order = Order {
    //         order_id: 1,
    //         market_type: MarketType::Spot,
    //         market_index: 1,
    //         order_type: OrderType::Limit,
    //         status: OrderStatus::Open,
    //         ..Order::default()
    //     };
    //     let order_2 = Order {
    //         order_id: 1,
    //         market_type: MarketType::Perp,
    //         market_index: 1,
    //         order_type: OrderType::Limit,
    //         status: OrderStatus::Open,
    //         ..Order::default()
    //     };
    //     let order_3 = Order {
    //         order_id: 1,
    //         market_type: MarketType::Spot,
    //         market_index: 1,
    //         order_type: OrderType::Market,
    //         status: OrderStatus::Open,
    //         ..Order::default()
    //     };
    //     let order_4 = Order {
    //         order_id: 1,
    //         market_type: MarketType::Perp,
    //         market_index: 1,
    //         order_type: OrderType::Market,
    //         status: OrderStatus::Open,
    //         ..Order::default()
    //     };
    //     let user_account = Pubkey::new_unique();
    //     let slot = 0;
    //     dlob.insert_order(order, user_account, slot);
    //     dlob.insert_order(order_2, user_account, slot);
    //     dlob.insert_order(order_3, user_account, slot);
    //     dlob.insert_order(order_4, user_account, slot);

    //     assert!(dlob.get_order(order.order_id, user_account).is_some());
    //     assert!(dlob.get_order(order_2.order_id, user_account).is_some());
    //     assert!(dlob.get_order(order_3.order_id, user_account).is_some());
    //     assert!(dlob.get_order(order_4.order_id, user_account).is_some());

    //     dlob.clear();

    //     assert!(dlob.get_order(order.order_id, user_account).is_none());
    //     assert!(dlob.get_order(order_2.order_id, user_account).is_none());
    //     assert!(dlob.get_order(order_3.order_id, user_account).is_none());
    //     assert!(dlob.get_order(order_4.order_id, user_account).is_none());

    // }

    // #[test]
    // fn test_delete_order() {
    //     let dlob = DLOB::new(0);
    //     let order = Order {
    //         order_id: 1,
    //         market_type: MarketType::Spot,
    //         market_index: 1,
    //         order_type: OrderType::Limit,
    //         status: OrderStatus::Open,
    //         ..Order::default()
    //     };
    //     let user_account = Pubkey::new_unique();
    //     let slot = 0;
    //     dlob.insert_order(order, user_account, slot);

    //     assert!(dlob.get_order(order.order_id, user_account).is_some());

    //     dlob.delete_order(order, user_account, slot);

    //     assert!(dlob.get_order(order.order_id, user_account).is_none());
    // }

    // #[test]
    // fn test_update_order_cumulative_baa_filled_below_order_baa_filled() {
        
    //     let mut dlob = DLOB::new(0);
    //     let order = Order {
    //         order_id: 1,
    //         market_type: MarketType::Spot,
    //         market_index: 1,
    //         order_type: OrderType::Limit,
    //         status: OrderStatus::Open,
    //         slot: 0,
    //         ..Order::default()
    //     };
    //     let user_account = Pubkey::new_unique();
    //     let slot = 0;
    //     dlob.insert_order(order, user_account, slot);

    //     assert!(dlob.get_order(order.order_id, user_account).unwrap().slot == 0);

    //     let updated_order = Order {
    //         order_id: 1,
    //         market_type: MarketType::Spot,
    //         market_index: 1,
    //         order_type: OrderType::Limit,
    //         status: OrderStatus::Open,
    //         slot: 1,
    //         base_asset_amount_filled: 1_000_000,
    //         ..Order::default()
    //     };
    //     dlob.update_order(updated_order, user_account, slot, 500_000);

    //     assert!(dlob.get_order(order.order_id, user_account).unwrap().slot == 1);
    // }

    // #[test]
    // fn test_update_order_cumulative_baa_equal_order_baa_filled() {    
    //     let dlob = DLOB::new(0);
    //     let order = Order {
    //         order_id: 1,
    //         market_type: MarketType::Spot,
    //         market_index: 1,
    //         order_type: OrderType::Limit,
    //         status: OrderStatus::Open,
    //         slot: 100,
    //         ..Order::default()
    //     };
    //     let user_account = Pubkey::new_unique();
    //     let slot = 0;
    //     dlob.insert_order(order, user_account, slot);

    //     assert!(dlob.get_order(order.order_id, user_account).unwrap().slot == 100);

    //     let updated_order = Order {
    //         order_id: 1,
    //         market_type: MarketType::Spot,
    //         market_index: 1,
    //         order_type: OrderType::Limit,
    //         status: OrderStatus::Open,
    //         slot: 1,
    //         base_asset_amount: 1_000_000,
    //         ..Order::default()
    //     };
    //     dlob.update_order(updated_order, user_account, slot, 1_000_000);

    //     assert!(dlob.get_order(order.order_id, user_account).is_none());
    // }

    // #[test]
    // fn test_update_resting_limit_nodes() {
    //     let dlob = DLOB::new(0);
    //     let order = Order {
    //         order_id: 1,
    //         market_type: MarketType::Spot,
    //         market_index: 1,
    //         order_type: OrderType::Limit,
    //         status: OrderStatus::Open,
    //         slot: 0,
    //         base_asset_amount: 1_000_000,
    //         ..Order::default()
    //     };
    //     let order_2 = Order {
    //         order_id: 2,
    //         market_type: MarketType::Spot,
    //         market_index: 1,
    //         order_type: OrderType::Limit,
    //         status: OrderStatus::Open,
    //         slot: 0,
    //         auction_duration: 100,
    //         base_asset_amount: 1_000_000,
    //         ..Order::default()
    //     };
    //     let user_account = Pubkey::new_unique();
    //     let slot = 1;
    //     dlob.insert_order(order, user_account, slot);
    //     dlob.insert_order(order_2, user_account, slot);

    //     for market in dlob.exchange.iter() {
    //         for market_ref in market.value().iter() {
    //             let market = market_ref.value();
    //             for node in market.resting_limit_orders.resting_limit_bids.iter() {
    //                 dbg!(node.get_order().order_id);
    //             }
    //         }
    //     }
        
    //     let updated_order = Order {
    //         order_id: 1,
    //         market_type: MarketType::Spot,
    //         market_index: 1,
    //         order_type: OrderType::Limit,
    //         status: OrderStatus::Open,
    //         slot: 1,
    //         base_asset_amount: 1_000_000,
    //         base_asset_amount_filled: 500_000,
    //         direction: PositionDirection::Long,
    //         ..Order::default()
    //     };

    //     let update_slot = 105;

    //     dlob.update_order(updated_order, user_account, update_slot, 1_000_000);

    //     assert!(dlob.get_order(order.order_id, user_account).is_none());

    //     let side = &dlob.exchange.get("spot").unwrap();
    //     let node_list = &side.get(&1).unwrap().resting_limit_orders.resting_limit_bids;
    //     let order_sig = get_order_signature(order_2.order_id, user_account);

    //     assert!(node_list.contains(&order_sig));
    // }

    #[test]
    fn test_resting_limit_bids() {
        let dlob = DLOB::new(0);

        let v_ask = 15;
        let v_bid = 10;

        let mut slot = 1;

        let oracle_price_data = OraclePriceData {
            price: (v_bid + v_ask) / 2,
            confidence: 1,
            delay: 0,
            has_sufficient_number_of_data_points: true,
        };

        let user_account_1 = Pubkey::new_unique();
        let user_account_2 = Pubkey::new_unique();
        let user_account_3 = Pubkey::new_unique();

        let market_index = 0;
        let market_type = MarketType::Perp;

        insert_order_to_dlob(
            &dlob,
            user_account_1,
            OrderType::Limit,
            market_type,
            1,
            market_index,
            11,
            BASE_PRECISION_U64,
            PositionDirection::Long,
            v_bid,
            v_ask,
            Some(1),
            0,
            0,
            false,
            10
        );

        insert_order_to_dlob(
            &dlob,
            user_account_2,
            OrderType::Limit,
            market_type,
            2,
            market_index,
            12,
            BASE_PRECISION_U64,
            PositionDirection::Long,
            v_bid,
            v_ask,
            Some(11),
            0,
            0,
            false,
            10
        );

        insert_order_to_dlob(
            &dlob,
            user_account_3,
            OrderType::Limit,
            market_type,
            3,
            market_index,
            13,
            BASE_PRECISION_U64,
            PositionDirection::Long,
            v_bid,
            v_ask,
            Some(21),
            0,
            0,
            false,
            10
        );



        let resting_bids = dlob.get_resting_limit_bids(market_index, slot, market_type, oracle_price_data);
        dbg!("inside first test");
        assert!(resting_bids.len() == 0);

        slot += 11;

        let resting_bids = dlob.get_resting_limit_bids(market_index, slot, market_type, oracle_price_data);
        dbg!("inside second test");
        for bid in resting_bids.iter() {
            dbg!(bid);
        }
        assert!(resting_bids.len() == 1);
        dbg!(resting_bids[0].get_order().order_id);
        assert!(resting_bids[0].get_order().order_id == 1);


        slot += 11;

        let resting_bids = dlob.get_resting_limit_bids(market_index, slot, market_type, oracle_price_data);
        dbg!("inside third test");
        for bid in resting_bids.iter() {
            dbg!(bid);
        }
        assert!(resting_bids.len() == 2);
        assert!(resting_bids[0].get_order().order_id == 2);
        assert!(resting_bids[0].get_order().order_id == 1);
        

        slot += 11;

        let resting_bids = dlob.get_resting_limit_bids(market_index, slot, market_type, oracle_price_data);
        dbg!("inside fourth test");
        for bid in resting_bids.iter() {
            dbg!(bid);
        }
        assert!(resting_bids.len() == 3);
        assert!(resting_bids[0].get_order().order_id == 3);
        assert!(resting_bids[0].get_order().order_id == 2);
        assert!(resting_bids[0].get_order().order_id == 1);
        
    }
}