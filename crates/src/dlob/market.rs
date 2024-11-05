use dashmap::{DashMap, DashSet};
use solana_sdk::pubkey::Pubkey;

use super::dlob_node::{
    node_types::{self, FloatingLimit, RestingLimit, TakingLimit, Trigger},
    Node, NodeType, OrderId,
};
use crate::{
    dlob::order_list::Orderlist,
    drift_idl::types::{MarketType, Order, OrderTriggerCondition, OrderType, PositionDirection},
    is_one_of_variant,
    math::order::is_resting_limit_order,
};

#[derive(Clone)]
pub(crate) struct Market {
    pub resting_limit_orders: Orderlist<true, false, RestingLimit>,
    pub floating_limit_orders: Orderlist<true, false, FloatingLimit>,
    pub taking_limit_orders: Orderlist<true, true, TakingLimit>,
    pub market_orders: Orderlist<true, true, node_types::Market>,
    pub trigger_orders: Orderlist<false, true, Trigger>,
}

#[derive(Copy, Clone, Debug)]
pub enum SubType {
    Bid,
    Ask,
    Above,
    Below,
}

impl Market {
    pub(crate) fn new() -> Market {
        Market {
            resting_limit_orders: Orderlist::<true, false, RestingLimit>::new(),
            floating_limit_orders: Orderlist::<true, false, FloatingLimit>::new(),
            taking_limit_orders: Orderlist::<true, true, TakingLimit>::new(),
            market_orders: Orderlist::<true, true, node_types::Market>::new(),
            trigger_orders: Orderlist::<false, true, Trigger>::new(),
        }
    }

    pub(crate) fn insert_bid_node(&mut self, node_type: NodeType, order: Order, pubkey: Pubkey) {
        match node_type {
            NodeType::RestingLimit => self
                .resting_limit_orders
                .insert_bid(Node::new(order, pubkey)),
            NodeType::FloatingLimit => self
                .floating_limit_orders
                .insert_bid(Node::new(order, pubkey)),
            NodeType::TakingLimit => self
                .taking_limit_orders
                .insert_bid(Node::new(order, pubkey)),
            NodeType::Market => self.market_orders.insert_bid(Node::new(order, pubkey)),
            NodeType::Trigger => self.trigger_orders.insert_bid(Node::new(order, pubkey)),
            NodeType::VAMM => panic!("VAMM order list not found"),
        }
    }

    pub(crate) fn insert_ask_node(&mut self, node_type: NodeType, order: Order, pubkey: Pubkey) {
        match node_type {
            NodeType::RestingLimit => self
                .resting_limit_orders
                .insert_ask(Node::new(order, pubkey)),
            NodeType::FloatingLimit => self
                .floating_limit_orders
                .insert_ask(Node::new(order, pubkey)),
            NodeType::TakingLimit => self
                .taking_limit_orders
                .insert_ask(Node::new(order, pubkey)),
            NodeType::Market => self.market_orders.insert_ask(Node::new(order, pubkey)),
            NodeType::Trigger => self.trigger_orders.insert_ask(Node::new(order, pubkey)),
            NodeType::VAMM => panic!("VAMM order list not found"),
        }
    }

    pub(crate) fn take_bids_by_node_type(&mut self, node_type: NodeType) -> Vec<(Order, Pubkey)> {
        match node_type {
            NodeType::RestingLimit => self
                .resting_limit_orders
                .get_best_bids()
                .drain(..)
                .map(Into::into)
                .collect(),
            NodeType::FloatingLimit => self
                .floating_limit_orders
                .get_best_bids()
                .drain(..)
                .map(Into::into)
                .collect(),
            NodeType::TakingLimit => self
                .taking_limit_orders
                .get_best_bids()
                .drain(..)
                .map(Into::into)
                .collect(),
            NodeType::Market => self
                .market_orders
                .get_best_bids()
                .drain(..)
                .map(Into::into)
                .collect(),
            NodeType::Trigger => self
                .trigger_orders
                .get_best_bids()
                .drain(..)
                .map(Into::into)
                .collect(),
            NodeType::VAMM => panic!("VAMM order list not found"),
        }
    }

    pub(crate) fn take_asks_by_node_type(&mut self, node_type: NodeType) -> Vec<(Order, Pubkey)> {
        match node_type {
            NodeType::RestingLimit => self
                .resting_limit_orders
                .get_best_asks()
                .drain(..)
                .map(Into::into)
                .collect(),
            NodeType::FloatingLimit => self
                .floating_limit_orders
                .get_best_asks()
                .drain(..)
                .map(Into::into)
                .collect(),
            NodeType::TakingLimit => self
                .taking_limit_orders
                .get_best_asks()
                .drain(..)
                .map(Into::into)
                .collect(),
            NodeType::Market => self
                .market_orders
                .get_best_asks()
                .drain(..)
                .map(Into::into)
                .collect(),
            NodeType::Trigger => self
                .trigger_orders
                .get_best_asks()
                .drain(..)
                .map(Into::into)
                .collect(),
            NodeType::VAMM => panic!("VAMM order list not found"),
        }
    }

    /// for debugging
    pub fn print_all_orders(&self) {
        self.resting_limit_orders.print();
        self.floating_limit_orders.print();
        self.taking_limit_orders.print();
        self.market_orders.print();
        self.trigger_orders.print();
    }
}

pub(crate) fn get_node_subtype_and_type(order: &Order, slot: u64) -> (SubType, NodeType) {
    // let is_inactive_trigger_order = order.must_be_triggered() && !order.triggered();
    let is_triggered = match order.trigger_condition {
        OrderTriggerCondition::TriggeredAbove | OrderTriggerCondition::TriggeredBelow => true,
        _ => false,
    };
    let must_be_triggered = match order.order_type {
        OrderType::TriggerMarket | OrderType::TriggerLimit => true,
        _ => false,
    };
    let is_inactive_trigger_order = must_be_triggered && !is_triggered;

    let node_type = if is_inactive_trigger_order {
        NodeType::Trigger
    } else if is_one_of_variant(
        &order.order_type,
        &[
            OrderType::Market,
            OrderType::TriggerMarket,
            OrderType::Oracle,
        ],
    ) {
        NodeType::Market
    } else if order.oracle_price_offset != 0 {
        NodeType::FloatingLimit
    } else if is_resting_limit_order(order, slot) {
        NodeType::RestingLimit
    } else {
        NodeType::TakingLimit
    };

    let sub_type = if is_inactive_trigger_order {
        if order.trigger_condition == OrderTriggerCondition::Above {
            SubType::Bid
        } else {
            SubType::Ask
        }
    } else {
        match order.direction {
            PositionDirection::Long => SubType::Bid,
            PositionDirection::Short => SubType::Ask,
        }
    };

    (sub_type, node_type)
}

#[derive(Clone)]
pub struct Exchange {
    pub perp: DashMap<u16, Market, ahash::RandomState>,
    pub spot: DashMap<u16, Market, ahash::RandomState>,
}

impl Exchange {
    pub fn new() -> Exchange {
        Exchange {
            perp: DashMap::<u16, Market, ahash::RandomState>::default(),
            spot: DashMap::<u16, Market, ahash::RandomState>::default(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &DashMap<u16, Market, ahash::RandomState>> {
        [&self.perp, &self.spot].into_iter()
    }

    pub fn clear(&self) {
        self.perp.clear();
        self.spot.clear();
    }

    pub fn add_market_indempotent(&self, market_type: MarketType, market_index: u16) {
        if !self.contains_market(market_type, market_index) {
            self.insert_market(market_type, market_index);
        }
    }

    fn contains_market(&self, market_type: MarketType, market_index: u16) -> bool {
        match market_type {
            MarketType::Perp => self.perp.contains_key(&market_index),
            MarketType::Spot => self.spot.contains_key(&market_index),
        }
    }

    fn insert_market(&self, market_type: MarketType, market_index: u16) {
        match market_type {
            MarketType::Perp => self.perp.insert(market_index, Market::new()),
            MarketType::Spot => self.spot.insert(market_index, Market::new()),
        };
    }

    pub fn find_order(&self, order: &OrderId) -> Option<Order> {
        for market_type_ref in self.iter() {
            for market_ref in market_type_ref.iter() {
                let market = market_ref.value();
                if let Some(node) = market.resting_limit_orders.get_node(order) {
                    return Some(node.order().clone());
                }
                if let Some(node) = market.floating_limit_orders.get_node(order) {
                    return Some(node.order().clone());
                }
                if let Some(node) = market.taking_limit_orders.get_node(order) {
                    return Some(node.order().clone());
                }
                if let Some(node) = market.market_orders.get_node(order) {
                    return Some(node.order().clone());
                }
                if let Some(node) = market.trigger_orders.get_node(order) {
                    return Some(node.order().clone());
                }
            }
        }

        return None;
    }

    /// for debugging
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        let mut size = 0;
        for market_type_ref in self.iter() {
            for market in market_type_ref.iter() {
                size += market.value().resting_limit_orders.size();
                size += market.value().floating_limit_orders.size();
                size += market.value().taking_limit_orders.size();
                size += market.value().market_orders.size();
                size += market.value().trigger_orders.size();
            }
        }
        size
    }

    /// for debugging
    pub fn perp_size(&self) -> usize {
        let mut size = 0;
        for market in self.perp.iter() {
            size += market.value().resting_limit_orders.size();
            size += market.value().floating_limit_orders.size();
            size += market.value().taking_limit_orders.size();
            size += market.value().market_orders.size();
            size += market.value().trigger_orders.size();
        }
        size
    }

    /// for debugging
    pub fn spot_size(&self) -> usize {
        let mut size = 0;
        for market in self.spot.iter() {
            size += market.value().resting_limit_orders.size();
            size += market.value().floating_limit_orders.size();
            size += market.value().taking_limit_orders.size();
            size += market.value().market_orders.size();
            size += market.value().trigger_orders.size();
        }
        size
    }
}

pub(crate) type OpenOrders = DashMap<String, DashSet<String>>;
