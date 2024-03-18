use dashmap::{DashMap, DashSet};
use drift::state::oracle::OraclePriceData;
use drift::state::user::{MarketType, Order};
use solana_sdk::pubkey::Pubkey;
use std::cell::Cell;
use std::collections::BinaryHeap;

use crate::dlob::dlob_node::{get_order_signature, DLOBNode, Node, NodeType};
use crate::dlob::market::{get_order_lists, Exchange, Market, OpenOrders, SubType};
use crate::math::order::is_resting_limit_order;
use crate::utils::market_type_to_string;

use super::dlob_node::DirectionalNode;

pub struct DLOB {
    exchange: Exchange,
    _open_orders: OpenOrders,
    _initialized: bool,
    _max_slot_for_resting_limit_orders: Cell<u64>,
}

impl DLOB {
    pub fn new() -> DLOB {
        let exchange = Exchange::new();
        exchange.insert("perp".to_string(), DashMap::new());
        exchange.insert("spot".to_string(), DashMap::new());

        let open_orders = OpenOrders::new();
        open_orders.insert("perp".to_string(), DashSet::new());
        open_orders.insert("spot".to_string(), DashSet::new());

        DLOB {
            exchange,
            _open_orders: open_orders,
            _initialized: true,
            _max_slot_for_resting_limit_orders: Cell::new(0),
        }
    }

    pub fn add_market(&self, market_type: &str, market_index: u16) {
        if !self
            .exchange
            .get(market_type)
            .unwrap()
            .contains_key(&market_index)
        {
            self.exchange
                .get(market_type)
                .unwrap()
                .insert(market_index, Market::new());
        }
    }

    pub fn insert_node(&self, node: &Node) {
        let market_type = market_type_to_string(&node.get_order().market_type);
        let market_index = node.get_order().market_index;

        if !self
            .exchange
            .get(&market_type)
            .unwrap()
            .contains_key(&market_index)
        {
            self.add_market(&market_type, market_index);
        }

        let markets_for_market_type = self
            .exchange
            .get(&market_type)
            .expect(format!("Market type {} not found", market_type).as_str());
        let mut market = markets_for_market_type
            .get_mut(&market_index)
            .expect(format!("Market index {} not found", market_index).as_str());

        let (order_list, subtype) =
            market.get_list_for_order(&node.get_order(), node.get_order().slot);

        if let Some(order_list) = order_list {
            match subtype {
                SubType::Bid => order_list.insert_bid(node.clone()),
                SubType::Ask => order_list.insert_ask(node.clone()),
                _ => {}
            }
        } else {
            panic!("Order list not found for order {:?}", node.get_order());
        }
    }

    pub fn get_order(&self, order_id: u32, user_account: Pubkey) -> Option<Order> {
        let order_signature = get_order_signature(order_id, user_account);
        for order_list in get_order_lists(&self.exchange) {
            if let Some(node) = order_list.get_node(&order_signature) {
                return Some(node.get_order().clone());
            }
        }

        None
    }

    fn update_resting_limit_orders_for_market_type(&mut self, slot: u64, market_type: String) {
        let mut new_taking_asks: BinaryHeap<DirectionalNode> = BinaryHeap::new();
        let mut new_taking_bids: BinaryHeap<DirectionalNode> = BinaryHeap::new();
        if let Some(market) = self.exchange.get_mut(&market_type) {
            for mut market_ref in market.iter_mut() {
                let market = market_ref.value_mut();

                for directional_node in market.taking_limit_orders.bids.iter() {
                    if is_resting_limit_order(directional_node.node.get_order(), slot) {
                        market
                            .resting_limit_orders
                            .insert_bid(directional_node.node)
                    } else {
                        new_taking_bids.push(*directional_node)
                    }
                }

                for directional_node in market.taking_limit_orders.asks.iter() {
                    if is_resting_limit_order(directional_node.node.get_order(), slot) {
                        market
                            .resting_limit_orders
                            .insert_ask(directional_node.node);
                    } else {
                        new_taking_asks.push(*directional_node);
                    }
                }

                market.taking_limit_orders.bids = new_taking_bids.clone();
                market.taking_limit_orders.asks = new_taking_asks.clone();
            }
        }
    }

    pub fn update_resting_limit_orders(&mut self, slot: u64) {
        if slot <= self._max_slot_for_resting_limit_orders.get() {
            return;
        }

        self._max_slot_for_resting_limit_orders.set(slot);

        self.update_resting_limit_orders_for_market_type(
            slot,
            market_type_to_string(&MarketType::Perp),
        );
        self.update_resting_limit_orders_for_market_type(
            slot,
            market_type_to_string(&MarketType::Spot),
        );
    }

    pub fn get_best_orders(
        &self,
        market_type: MarketType,
        sub_type: SubType,
        node_type: NodeType,
        market_index: u16,
    ) -> Vec<Node> {
        let market_type_str = market_type_to_string(&market_type);
        let markets_for_market_type = self
            .exchange
            .get(&market_type_str)
            .expect(format!("Market type {} not found", market_type_str).as_str());
        let market = markets_for_market_type
            .get(&market_index)
            .expect(format!("Market index {} not found", market_index).as_str());

        let mut order_list = market.get_order_list_for_node_type(node_type);

        let mut best_orders: Vec<Node> = vec![];

        match sub_type {
            SubType::Bid => {
                while !order_list.bids_empty() {
                    if let Some(node) = order_list.get_best_bid() {
                        best_orders.push(node);
                    }
                }
            }
            SubType::Ask => {
                while !order_list.asks_empty() {
                    if let Some(node) = order_list.get_best_ask() {
                        best_orders.push(node);
                    }
                }
            }
            _ => unimplemented!()
        }

        best_orders
    }

    pub fn get_resting_limit_asks(
        &mut self,
        slot: u64,
        market_type: MarketType,
        market_index: u16,
        oracle_price_data: OraclePriceData,
    ) -> Vec<Node> {
        self.update_resting_limit_orders(slot);

        let mut resting_limit_orders = self.get_best_orders(
            market_type,
            SubType::Ask,
            NodeType::RestingLimit,
            market_index,
        );
        let mut floating_limit_orders = self.get_best_orders(
            market_type,
            SubType::Ask,
            NodeType::FloatingLimit,
            market_index,
        );

        let comparative = Box::new(
            |node_a: &Node, node_b: &Node, slot: u64, oracle_price_data: OraclePriceData| {
                node_a.get_price(oracle_price_data, slot)
                    > node_b.get_price(oracle_price_data, slot)
            },
        );

        let mut all_orders = vec![];
        all_orders.append(&mut resting_limit_orders);
        all_orders.append(&mut floating_limit_orders);

        all_orders.sort_by(|a, b| {
            if comparative(a, b, slot, oracle_price_data) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        });

        all_orders
    }

    pub fn get_resting_limit_bids(
        &mut self,
        slot: u64,
        market_type: MarketType,
        market_index: u16,
        oracle_price_data: OraclePriceData,
    ) -> Vec<Node> {
        self.update_resting_limit_orders(slot);

        let mut resting_limit_orders = self.get_best_orders(
            market_type,
            SubType::Bid,
            NodeType::RestingLimit,
            market_index,
        );
        let mut floating_limit_orders = self.get_best_orders(
            market_type,
            SubType::Bid,
            NodeType::FloatingLimit,
            market_index,
        );

        let comparative = Box::new(
            |node_a: &Node, node_b: &Node, slot: u64, oracle_price_data: OraclePriceData| {
                node_a.get_price(oracle_price_data, slot)
                    < node_b.get_price(oracle_price_data, slot)
            },
        );

        let mut all_orders = vec![];
        all_orders.append(&mut resting_limit_orders);
        all_orders.append(&mut floating_limit_orders);

        all_orders.sort_by(|a, b| {
            if comparative(a, b, slot, oracle_price_data) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        });

        all_orders
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dlob::dlob_node::create_node;
    use drift::{
        math::constants::PRICE_PRECISION_U64,
        state::user::{Order, OrderType},
    };
    use solana_sdk::pubkey::Pubkey;

    #[test]
    fn test_dlob_insert() {
        let dlob = DLOB::new();
        let user_account = Pubkey::new_unique();
        let taking_limit_order = Order {
            order_id: 1,
            slot: 1,
            market_index: 0,
            market_type: MarketType::Perp,
            ..Order::default()
        };
        let floating_limit_order = Order {
            order_id: 2,
            oracle_price_offset: 1,
            market_index: 0,
            market_type: MarketType::Perp,
            ..Order::default()
        };
        let resting_limit_order = Order {
            order_id: 3,
            slot: 3,
            market_index: 0,
            market_type: MarketType::Perp,
            ..Order::default()
        };
        let market_order = Order {
            order_id: 4,
            slot: 4,
            market_index: 0,
            market_type: MarketType::Perp,
            ..Order::default()
        };
        let trigger_order = Order {
            order_id: 5,
            slot: 5,
            market_index: 0,
            market_type: MarketType::Perp,
            ..Order::default()
        };

        let taking_limit_node =
            create_node(NodeType::TakingLimit, taking_limit_order, user_account);
        let floating_limit_node =
            create_node(NodeType::FloatingLimit, floating_limit_order, user_account);
        let resting_limit_node =
            create_node(NodeType::RestingLimit, resting_limit_order, user_account);
        let market_node = create_node(NodeType::Market, market_order, user_account);
        let trigger_node = create_node(NodeType::Trigger, trigger_order, user_account);

        dlob.insert_node(&taking_limit_node);
        dlob.insert_node(&floating_limit_node);
        dlob.insert_node(&resting_limit_node);
        dlob.insert_node(&market_node);
        dlob.insert_node(&trigger_node);

        assert!(dlob.get_order(1, user_account).is_some());
        assert!(dlob.get_order(2, user_account).is_some());
        assert!(dlob.get_order(3, user_account).is_some());
        assert!(dlob.get_order(4, user_account).is_some());
        assert!(dlob.get_order(5, user_account).is_some());
    }

    #[test]
    fn test_dlob_ordering() {
        let dlob = DLOB::new();

        let user_account = Pubkey::new_unique();
        let order_1 = Order {
            order_id: 1,
            slot: 1,
            market_index: 0,
            direction: drift::controller::position::PositionDirection::Long,
            market_type: MarketType::Perp,
            auction_duration: 1,
            ..Order::default()
        };
        let order_2 = Order {
            order_id: 2,
            slot: 2,
            market_index: 0,
            direction: drift::controller::position::PositionDirection::Long,
            market_type: MarketType::Perp,
            auction_duration: 1,
            ..Order::default()
        };
        let order_3 = Order {
            order_id: 3,
            slot: 3,
            market_index: 0,
            direction: drift::controller::position::PositionDirection::Long,
            market_type: MarketType::Perp,
            auction_duration: 1,
            ..Order::default()
        };
        let order_4 = Order {
            order_id: 4,
            slot: 4,
            market_index: 0,
            direction: drift::controller::position::PositionDirection::Long,
            market_type: MarketType::Perp,
            auction_duration: 1,
            ..Order::default()
        };
        let order_5 = Order {
            order_id: 5,
            slot: 5,
            market_index: 0,
            direction: drift::controller::position::PositionDirection::Long,
            market_type: MarketType::Perp,
            auction_duration: 1,
            ..Order::default()
        };

        let node_1 = create_node(NodeType::TakingLimit, order_1, user_account);
        let node_2 = create_node(NodeType::TakingLimit, order_2, user_account);
        let node_3 = create_node(NodeType::TakingLimit, order_3, user_account);
        let node_4 = create_node(NodeType::TakingLimit, order_4, user_account);
        let node_5 = create_node(NodeType::TakingLimit, order_5, user_account);

        dlob.insert_node(&node_1);
        dlob.insert_node(&node_2);
        dlob.insert_node(&node_3);
        dlob.insert_node(&node_4);
        dlob.insert_node(&node_5);

        assert!(dlob.get_order(1, user_account).is_some());
        assert!(dlob.get_order(2, user_account).is_some());
        assert!(dlob.get_order(3, user_account).is_some());
        assert!(dlob.get_order(4, user_account).is_some());
        assert!(dlob.get_order(5, user_account).is_some());

        let best_orders =
            dlob.get_best_orders(MarketType::Perp, SubType::Bid, NodeType::TakingLimit, 0);

        assert_eq!(best_orders[0].get_order().slot, 1);
        assert_eq!(best_orders[1].get_order().slot, 2);
        assert_eq!(best_orders[2].get_order().slot, 3);
        assert_eq!(best_orders[3].get_order().slot, 4);
        assert_eq!(best_orders[4].get_order().slot, 5);
    }

    #[test]
    fn test_update_resting_limit_orders() {
        let mut dlob = DLOB::new();

        let user_account = Pubkey::new_unique();
        let order_1 = Order {
            order_id: 1,
            slot: 1,
            market_index: 0,
            direction: drift::controller::position::PositionDirection::Long,
            market_type: MarketType::Perp,
            auction_duration: 1,
            ..Order::default()
        };

        let node_1 = create_node(NodeType::TakingLimit, order_1, user_account);

        dlob.insert_node(&node_1);

        let markets_for_market_type = dlob.exchange.get("perp").unwrap();
        let market = markets_for_market_type.get(&0).unwrap();

        assert_eq!(market.taking_limit_orders.bids.len(), 1);

        let slot = 5;

        drop(market);
        drop(markets_for_market_type);

        dlob.update_resting_limit_orders(slot);

        let markets_for_market_type = dlob.exchange.get("perp").unwrap();
        let market = markets_for_market_type.get(&0).unwrap();

        assert_eq!(market.taking_limit_orders.bids.len(), 0);
        assert_eq!(market.resting_limit_orders.bids.len(), 1);
    }

    #[test]
    fn test_get_resting_limit_asks() {
        let mut dlob = DLOB::new();

        let v_ask = 15;
        let v_bid = 10;

        let oracle_price_data = OraclePriceData {
            price: (v_bid + v_ask) / 2,
            confidence: 1,
            delay: 0,
            has_sufficient_number_of_data_points: true,
        };

        let user_account = Pubkey::new_unique();
        let order_1 = Order {
            order_id: 1,
            slot: 1,
            market_index: 0,
            direction: drift::controller::position::PositionDirection::Short,
            market_type: MarketType::Perp,
            order_type: OrderType::Limit,
            auction_duration: 10,
            price: 11 * PRICE_PRECISION_U64,
            ..Order::default()
        };

        let order_2 = Order {
            order_id: 2,
            slot: 11,
            market_index: 0,
            direction: drift::controller::position::PositionDirection::Short,
            market_type: MarketType::Perp,
            order_type: OrderType::Limit,
            auction_duration: 10,
            price: 12 * PRICE_PRECISION_U64,
            ..Order::default()
        };

        let order_3 = Order {
            order_id: 3,
            slot: 21,
            market_index: 0,
            direction: drift::controller::position::PositionDirection::Short,
            market_type: MarketType::Perp,
            order_type: OrderType::Limit,
            auction_duration: 10,
            price: 13 * PRICE_PRECISION_U64,
            ..Order::default()
        };

        let node_1 = create_node(NodeType::TakingLimit, order_1, user_account);
        let node_2 = create_node(NodeType::TakingLimit, order_2, user_account);
        let node_3 = create_node(NodeType::TakingLimit, order_3, user_account);

        dlob.insert_node(&node_1);
        dlob.insert_node(&node_2);
        dlob.insert_node(&node_3);

        let mut slot = 1;

        dbg!("expecting 0");
        let resting_limit_asks =
            dlob.get_resting_limit_asks(slot, MarketType::Perp, 0, oracle_price_data);

        assert_eq!(resting_limit_asks.len(), 0);

        slot += 11;

        dbg!("expecting 1");
        let resting_limit_asks =
            dlob.get_resting_limit_asks(slot, MarketType::Perp, 0, oracle_price_data);

        assert_eq!(resting_limit_asks.len(), 1);
        assert_eq!(resting_limit_asks[0].get_order().order_id, 1);

        slot += 11;

        dbg!("expecting 2");
        let resting_limit_asks =
            dlob.get_resting_limit_asks(slot, MarketType::Perp, 0, oracle_price_data);

        assert_eq!(resting_limit_asks.len(), 2);
        assert_eq!(resting_limit_asks[0].get_order().order_id, 1);
        assert_eq!(resting_limit_asks[1].get_order().order_id, 2);

        slot += 11;

        dbg!("expecting 3");
        let resting_limit_asks =
            dlob.get_resting_limit_asks(slot, MarketType::Perp, 0, oracle_price_data);

        assert_eq!(resting_limit_asks.len(), 3);
        assert_eq!(resting_limit_asks[0].get_order().order_id, 1);
        assert_eq!(resting_limit_asks[1].get_order().order_id, 2);
        assert_eq!(resting_limit_asks[2].get_order().order_id, 3);
    }

    #[test]
    fn test_get_resting_limit_bids() {
        let mut dlob = DLOB::new();

        let v_ask = 15;
        let v_bid = 10;

        let oracle_price_data = OraclePriceData {
            price: (v_bid + v_ask) / 2,
            confidence: 1,
            delay: 0,
            has_sufficient_number_of_data_points: true,
        };

        let user_account = Pubkey::new_unique();
        let order_1 = Order {
            order_id: 1,
            slot: 1,
            market_index: 0,
            direction: drift::controller::position::PositionDirection::Long,
            market_type: MarketType::Perp,
            order_type: OrderType::Limit,
            auction_duration: 10,
            price: 11,
            ..Order::default()
        };

        let order_2 = Order {
            order_id: 2,
            slot: 11,
            market_index: 0,
            direction: drift::controller::position::PositionDirection::Long,
            market_type: MarketType::Perp,
            order_type: OrderType::Limit,
            auction_duration: 10,
            price: 12,
            ..Order::default()
        };

        let order_3 = Order {
            order_id: 3,
            slot: 21,
            market_index: 0,
            direction: drift::controller::position::PositionDirection::Long,
            market_type: MarketType::Perp,
            order_type: OrderType::Limit,
            auction_duration: 10,
            price: 13,
            ..Order::default()
        };

        let node_1 = create_node(NodeType::TakingLimit, order_1, user_account);
        let node_2 = create_node(NodeType::TakingLimit, order_2, user_account);
        let node_3 = create_node(NodeType::TakingLimit, order_3, user_account);

        dlob.insert_node(&node_1);
        dlob.insert_node(&node_2);
        dlob.insert_node(&node_3);

        let mut slot = 1;

        dbg!("expecting 0");
        let resting_limit_bids =
            dlob.get_resting_limit_bids(slot, MarketType::Perp, 0, oracle_price_data);

        assert_eq!(resting_limit_bids.len(), 0);

        slot += 11;

        dbg!("expecting 1");
        let resting_limit_bids =
            dlob.get_resting_limit_bids(slot, MarketType::Perp, 0, oracle_price_data);

        assert_eq!(resting_limit_bids.len(), 1);
        assert_eq!(resting_limit_bids[0].get_order().order_id, 1);

        slot += 11;

        dbg!("expecting 2");
        let resting_limit_bids =
            dlob.get_resting_limit_bids(slot, MarketType::Perp, 0, oracle_price_data);

        assert_eq!(resting_limit_bids.len(), 2);
        assert_eq!(resting_limit_bids[0].get_order().order_id, 2);
        assert_eq!(resting_limit_bids[1].get_order().order_id, 1);

        slot += 11;

        dbg!("expecting 3");
        let resting_limit_bids =
            dlob.get_resting_limit_bids(slot, MarketType::Perp, 0, oracle_price_data);

        assert_eq!(resting_limit_bids.len(), 3);
        assert_eq!(resting_limit_bids[0].get_order().order_id, 3);
        assert_eq!(resting_limit_bids[1].get_order().order_id, 2);
        assert_eq!(resting_limit_bids[2].get_order().order_id, 1);
    }
}
