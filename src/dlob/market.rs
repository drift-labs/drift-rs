use dashmap::{DashMap, DashSet};
use drift::controller::position::PositionDirection;
use drift::state::user::{Order, OrderTriggerCondition, OrderType};

use crate::dlob::dlob_node::{Node, NodeType, SortDirection};
use crate::dlob::order_list::Orderlist;
use crate::is_one_of_variant;
use crate::math::order::is_resting_limit_order;

#[derive(Clone)]
pub(crate) struct Market {
    pub resting_limit_orders: Orderlist,
    pub floating_limit_orders: Orderlist,
    pub taking_limit_orders: Orderlist,
    pub market_orders: Orderlist,
    pub trigger_orders: Orderlist,
}

#[derive(Copy, Clone)]
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

    pub(crate) fn get_list_for_order(
        &mut self,
        order: &Order,
        slot: u64,
    ) -> (Option<&mut Orderlist>, SubType) {
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
        } else {
            if is_resting_limit_order(order, slot) {
                NodeType::RestingLimit
            } else {
                NodeType::TakingLimit
            }
        };

        let order_list = match node_type {
            NodeType::RestingLimit => &mut self.resting_limit_orders,
            NodeType::FloatingLimit => &mut self.floating_limit_orders,
            NodeType::TakingLimit => &mut self.taking_limit_orders,
            NodeType::Market => &mut self.market_orders,
            NodeType::Trigger => &mut self.trigger_orders,
            NodeType::VAMM => return (None, SubType::Bid),
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

        (Some(order_list), sub_type)
    }

    pub(crate) fn get_best_order(
        &self,
        order_list: &mut Orderlist,
        sub_type: SubType,
    ) -> Option<Node> {
        match sub_type {
            SubType::Bid => order_list.get_best_bid(),
            SubType::Ask => order_list.get_best_ask(),
            _ => unimplemented!(),
        };

        None
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
}

pub(crate) type Exchange = DashMap<String, DashMap<u16, Market>>;

pub fn get_order_lists(exchange: &Exchange) -> Vec<Orderlist> {
    let mut order_lists = vec![];

    for market_type_ref in exchange.iter() {
        for market_ref in market_type_ref.iter() {
            order_lists.push(market_ref.value().resting_limit_orders.clone());
            order_lists.push(market_ref.value().floating_limit_orders.clone());
            order_lists.push(market_ref.value().taking_limit_orders.clone());
            order_lists.push(market_ref.value().market_orders.clone());
            order_lists.push(market_ref.value().trigger_orders.clone());
        }
    }

    order_lists
}

pub(crate) type OpenOrders = DashMap<String, DashSet<String>>;
