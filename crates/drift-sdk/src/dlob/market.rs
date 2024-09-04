use dashmap::{DashMap, DashSet};
use drift_abi::{
    controller::position::PositionDirection,
    state::user::{Order, OrderTriggerCondition, OrderType},
};

use crate::{
    dlob::{
        dlob_node::{NodeType, SortDirection},
        order_list::Orderlist,
    },
    is_one_of_variant,
    math::order::is_resting_limit_order,
};

#[derive(Clone)]
pub(crate) struct Market {
    pub resting_limit_orders: Orderlist,
    pub floating_limit_orders: Orderlist,
    pub taking_limit_orders: Orderlist,
    pub market_orders: Orderlist,
    pub trigger_orders: Orderlist,
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
            resting_limit_orders: Orderlist::new(
                SortDirection::Descending,
                SortDirection::Ascending,
            ),
            floating_limit_orders: Orderlist::new(
                SortDirection::Descending,
                SortDirection::Ascending,
            ),
            taking_limit_orders: Orderlist::new(SortDirection::Ascending, SortDirection::Ascending),
            market_orders: Orderlist::new(SortDirection::Ascending, SortDirection::Ascending),
            trigger_orders: Orderlist::new(SortDirection::Ascending, SortDirection::Descending),
        }
    }

    pub(crate) fn get_order_list_for_node_insert(&mut self, node_type: NodeType) -> &mut Orderlist {
        match node_type {
            NodeType::RestingLimit => &mut self.resting_limit_orders,
            NodeType::FloatingLimit => &mut self.floating_limit_orders,
            NodeType::TakingLimit => &mut self.taking_limit_orders,
            NodeType::Market => &mut self.market_orders,
            NodeType::Trigger => &mut self.trigger_orders,
            NodeType::VAMM => panic!("VAMM order list not found"),
        }
    }

    pub(crate) fn get_order_list_for_node_type(&self, node_type: NodeType) -> Orderlist {
        match node_type {
            NodeType::RestingLimit => &self.resting_limit_orders,
            NodeType::FloatingLimit => &self.floating_limit_orders,
            NodeType::TakingLimit => &self.taking_limit_orders,
            NodeType::Market => &self.market_orders,
            NodeType::Trigger => &self.trigger_orders,
            NodeType::VAMM => panic!("VAMM order list not found"),
        }
        .clone()
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
    let is_inactive_trigger_order = order.must_be_triggered() && !order.triggered();

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
    pub perp: DashMap<u16, Market>,
    pub spot: DashMap<u16, Market>,
}

impl Exchange {
    pub fn new() -> Exchange {
        Exchange {
            perp: DashMap::new(),
            spot: DashMap::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &DashMap<u16, Market>> {
        vec![&self.perp, &self.spot].into_iter()
    }

    pub fn clear(&self) {
        self.perp.clear();
        self.spot.clear();
    }

    pub fn add_market_indempotent(&self, market_type: &str, market_index: u16) {
        if !self.contains_market(market_type, market_index) {
            self.insert_market(market_type, market_index);
        }
    }

    fn contains_market(&self, market_type: &str, market_index: u16) -> bool {
        match market_type {
            "perp" => self.perp.contains_key(&market_index),
            "spot" => self.spot.contains_key(&market_index),
            _ => panic!("Invalid market type"),
        }
    }

    fn insert_market(&self, market_type: &str, market_index: u16) {
        match market_type {
            "perp" => self.perp.insert(market_index, Market::new()),
            "spot" => self.spot.insert(market_index, Market::new()),
            _ => panic!("Invalid market type"),
        };
    }

    pub fn get_order_lists(&self) -> Vec<Orderlist> {
        let mut order_lists = vec![];

        for market_type_ref in self.iter() {
            for market_ref in market_type_ref.iter() {
                let market = market_ref.value();
                order_lists.push(market.resting_limit_orders.clone());
                order_lists.push(market.floating_limit_orders.clone());
                order_lists.push(market.taking_limit_orders.clone());
                order_lists.push(market.market_orders.clone());
                order_lists.push(market.trigger_orders.clone());
            }
        }

        order_lists
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
