#![allow(clippy::module_inception)]

use std::{collections::BinaryHeap, str::FromStr, sync::Arc};

use dashmap::DashSet;
use rayon::prelude::*;
use solana_sdk::pubkey::Pubkey;

use crate::{
    dlob::{
        dlob_node::{create_node, get_order_signature, DLOBNode, DirectionalNode, Node, NodeType},
        market::{get_node_subtype_and_type, Exchange, OpenOrders, SubType},
    },
    drift_idl::{
        ffi::OraclePriceData,
        types::{MarketType, Order, OrderStatus},
    },
    math::order::is_resting_limit_order,
    usermap::UserMap,
    utils::market_type_to_string,
};

#[derive(Clone)]
pub struct DLOB {
    exchange: Exchange,
    _open_orders: OpenOrders,
    _initialized: bool,
    _max_slot_for_resting_limit_orders: Arc<u64>,
}

impl DLOB {
    pub fn new() -> DLOB {
        let exchange = Exchange::new();

        let open_orders = OpenOrders::new();
        open_orders.insert("perp".to_string(), DashSet::new());
        open_orders.insert("spot".to_string(), DashSet::new());

        DLOB {
            exchange,
            _open_orders: open_orders,
            _initialized: true,
            _max_slot_for_resting_limit_orders: Arc::new(0),
        }
    }

    pub fn build_from_usermap(&mut self, usermap: &UserMap, slot: u64) {
        self.clear();
        usermap.usermap.iter().par_bridge().for_each(|user_ref| {
            let user = user_ref.value();
            let user_key = user_ref.key();
            let user_pubkey = Pubkey::from_str(user_key).expect("Valid pubkey");
            for order in user.orders.iter() {
                if order.status == OrderStatus::Init {
                    continue;
                }
                self.insert_order(order, user_pubkey, slot);
            }
        });
        self._initialized = true;
    }

    pub fn size(&self) -> (usize, usize) {
        (self.exchange.perp_size(), self.exchange.spot_size())
    }

    /// for debugging
    pub fn print_all_spot_orders(&self) {
        for market in self.exchange.spot.iter() {
            println!("market index: {}", market.key());
            market.value().print_all_orders();
        }
    }

    pub fn clear(&mut self) {
        self.exchange.clear();
        self._open_orders.clear();
        self._initialized = false;
        self._max_slot_for_resting_limit_orders = Arc::new(0);
    }

    pub fn insert_order(&self, order: &Order, user_account: Pubkey, slot: u64) {
        let market_type = market_type_to_string(&order.market_type);
        let market_index = order.market_index;

        let (subtype, node_type) = get_node_subtype_and_type(order, slot);
        let node = create_node(node_type, *order, user_account);

        self.exchange
            .add_market_indempotent(&market_type, market_index);

        let mut market = match order.market_type {
            MarketType::Perp => self.exchange.perp.get_mut(&market_index).expect("market"),
            MarketType::Spot => self.exchange.spot.get_mut(&market_index).expect("market"),
        };

        let order_list = market.get_order_list_for_node_insert(node_type);

        match subtype {
            SubType::Bid => order_list.insert_bid(node),
            SubType::Ask => order_list.insert_ask(node),
            _ => {}
        }
    }

    pub fn get_order(&self, order_id: u32, user_account: Pubkey) -> Option<Order> {
        let order_signature = get_order_signature(order_id, user_account);
        for order_list in self.exchange.get_order_lists() {
            if let Some(node) = order_list.get_node(&order_signature) {
                return Some(*node.get_order());
            }
        }

        None
    }

    fn update_resting_limit_orders_for_market_type(&mut self, slot: u64, market_type: MarketType) {
        let mut new_taking_asks: BinaryHeap<DirectionalNode> = BinaryHeap::new();
        let mut new_taking_bids: BinaryHeap<DirectionalNode> = BinaryHeap::new();

        let market = match market_type {
            MarketType::Perp => &self.exchange.perp,
            MarketType::Spot => &self.exchange.spot,
        };

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

    pub fn update_resting_limit_orders(&mut self, slot: u64) {
        if slot <= *self._max_slot_for_resting_limit_orders {
            return;
        }

        self._max_slot_for_resting_limit_orders = Arc::new(slot);

        self.update_resting_limit_orders_for_market_type(slot, MarketType::Perp);
        self.update_resting_limit_orders_for_market_type(slot, MarketType::Spot);
    }

    pub fn get_best_orders(
        &self,
        market_type: MarketType,
        sub_type: SubType,
        node_type: NodeType,
        market_index: u16,
    ) -> Vec<Node> {
        let market = match market_type {
            MarketType::Perp => self.exchange.perp.get_mut(&market_index).expect("market"),
            MarketType::Spot => self.exchange.spot.get_mut(&market_index).expect("market"),
        };
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
            _ => unimplemented!(),
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

impl Default for DLOB {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use drift_idl::{
        math::constants::PRICE_PRECISION_U64,
        state::user::{Order, OrderType},
    };
    use solana_sdk::pubkey::Pubkey;

    use super::*;

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

        dlob.insert_order(&taking_limit_order, user_account, 1);
        dlob.insert_order(&floating_limit_order, user_account, 0);
        dlob.insert_order(&resting_limit_order, user_account, 3);
        dlob.insert_order(&market_order, user_account, 4);
        dlob.insert_order(&trigger_order, user_account, 5);

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
            direction: drift_idl::controller::position::PositionDirection::Long,
            market_type: MarketType::Perp,
            auction_duration: 1,
            ..Order::default()
        };
        let order_2 = Order {
            order_id: 2,
            slot: 2,
            market_index: 0,
            direction: drift_idl::controller::position::PositionDirection::Long,
            market_type: MarketType::Perp,
            auction_duration: 1,
            ..Order::default()
        };
        let order_3 = Order {
            order_id: 3,
            slot: 3,
            market_index: 0,
            direction: drift_idl::controller::position::PositionDirection::Long,
            market_type: MarketType::Perp,
            auction_duration: 1,
            ..Order::default()
        };
        let order_4 = Order {
            order_id: 4,
            slot: 4,
            market_index: 0,
            direction: drift_idl::controller::position::PositionDirection::Long,
            market_type: MarketType::Perp,
            auction_duration: 1,
            ..Order::default()
        };
        let order_5 = Order {
            order_id: 5,
            slot: 5,
            market_index: 0,
            direction: drift_idl::controller::position::PositionDirection::Long,
            market_type: MarketType::Perp,
            auction_duration: 1,
            ..Order::default()
        };

        dlob.insert_order(&order_1, user_account, 1);
        dlob.insert_order(&order_2, user_account, 2);
        dlob.insert_order(&order_3, user_account, 3);
        dlob.insert_order(&order_4, user_account, 4);
        dlob.insert_order(&order_5, user_account, 5);

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
            direction: drift_idl::controller::position::PositionDirection::Long,
            market_type: MarketType::Perp,
            auction_duration: 1,
            ..Order::default()
        };

        dlob.insert_order(&order_1, user_account, 1);

        let markets_for_market_type = dlob.exchange.perp.clone();
        let market = markets_for_market_type.get(&0).unwrap();

        assert_eq!(market.taking_limit_orders.bids.len(), 1);

        let slot = 5;

        drop(market);
        drop(markets_for_market_type);

        dlob.update_resting_limit_orders(slot);

        let markets_for_market_type = dlob.exchange.perp.clone();
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
            direction: drift_idl::controller::position::PositionDirection::Short,
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
            direction: drift_idl::controller::position::PositionDirection::Short,
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
            direction: drift_idl::controller::position::PositionDirection::Short,
            market_type: MarketType::Perp,
            order_type: OrderType::Limit,
            auction_duration: 10,
            price: 13 * PRICE_PRECISION_U64,
            ..Order::default()
        };

        dlob.insert_order(&order_1, user_account, 1);
        dlob.insert_order(&order_2, user_account, 11);
        dlob.insert_order(&order_3, user_account, 21);

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
            direction: drift_idl::controller::position::PositionDirection::Long,
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
            direction: drift_idl::controller::position::PositionDirection::Long,
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
            direction: drift_idl::controller::position::PositionDirection::Long,
            market_type: MarketType::Perp,
            order_type: OrderType::Limit,
            auction_duration: 10,
            price: 13,
            ..Order::default()
        };

        dlob.insert_order(&order_1, user_account, 1);
        dlob.insert_order(&order_2, user_account, 11);
        dlob.insert_order(&order_3, user_account, 21);

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
