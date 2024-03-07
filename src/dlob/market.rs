use dashmap::{DashMap, DashSet};
use drift::controller::position::PositionDirection;
use drift::state::user::{Order, OrderTriggerCondition, OrderType};

use crate::dlob::node_list::NodeList;
use crate::dlob::dlob_node::{SortDirection, NodeType};
use crate::is_one_of_variant;


pub(crate) const SUPPORTED_ORDER_TYPES: [OrderType; 5] = [OrderType::Market, OrderType::TriggerMarket, OrderType::Limit, OrderType::TriggerLimit, OrderType::Oracle];

#[derive(Clone)]
pub(crate) struct RestingLimitOrders {
    pub resting_limit_bids: NodeList,
    pub resting_limit_asks: NodeList,
}

#[derive(Clone)]
pub(crate) struct FloatingLimitOrders {
    pub floating_limit_bids: NodeList,
    pub floating_limit_asks: NodeList,
}

#[derive(Clone)]
pub(crate) struct TakingLimitOrders {
    pub taking_limit_bids: NodeList,
    pub taking_limit_asks: NodeList,
}

#[derive(Clone)]
pub(crate) struct MarketOrders {
    pub market_bids: NodeList,
    pub market_asks: NodeList,
}

#[derive(Clone)]
pub(crate) struct TriggerOrders {
    pub trigger_above: NodeList,
    pub trigger_below: NodeList,
}

#[derive(Clone)]
pub(crate) struct Market {
    pub resting_limit_orders: RestingLimitOrders,
    pub floating_limit_orders: FloatingLimitOrders,
    pub taking_limit_orders: TakingLimitOrders,
    pub market_orders: MarketOrders,
    pub trigger_orders: TriggerOrders,
}

enum SubType {
    Bid,
    Ask,
    Above,
    Below
}

impl Market {
    pub(crate) fn new() -> Market {
        Market {
            resting_limit_orders: RestingLimitOrders {
                resting_limit_bids: NodeList::new(NodeType::RestingLimit, SortDirection::Descending),
                resting_limit_asks: NodeList::new(NodeType::RestingLimit, SortDirection::Ascending),
            },
            floating_limit_orders: FloatingLimitOrders {
                floating_limit_bids: NodeList::new(NodeType::FloatingLimit, SortDirection::Descending),
                floating_limit_asks: NodeList::new(NodeType::FloatingLimit, SortDirection::Ascending),
            },
            taking_limit_orders: TakingLimitOrders {
                taking_limit_bids: NodeList::new(NodeType::TakingLimit, SortDirection::Ascending),
                taking_limit_asks: NodeList::new(NodeType::TakingLimit, SortDirection::Ascending),
            },
            market_orders: MarketOrders {
                market_bids: NodeList::new(NodeType::Market, SortDirection::Ascending),
                market_asks: NodeList::new(NodeType::Market, SortDirection::Ascending),
            },
            trigger_orders: TriggerOrders {
                trigger_above: NodeList::new(NodeType::Trigger, SortDirection::Ascending),
                trigger_below: NodeList::new(NodeType::Trigger, SortDirection::Descending),
            },
        }
    }

    pub(crate) fn get_list_for_order(&mut self, order: &Order, slot: u64) -> Option<&mut NodeList> {
        let is_inactive_trigger_order = order.must_be_triggered() && !order.triggered();

        let node_type = if is_inactive_trigger_order {
            NodeType::Trigger
        } else if is_one_of_variant(&order.order_type, &[OrderType::Market, OrderType::TriggerMarket, OrderType::Oracle]) {
            NodeType::Market
        } else if order.oracle_price_offset != 0 {
            NodeType::FloatingLimit
        } else {
            if order.is_resting_limit_order(slot).unwrap() {
                NodeType::RestingLimit
            } else {
                NodeType::TakingLimit
            }
        };

        let sub_type = if is_inactive_trigger_order {
            if order.trigger_condition == OrderTriggerCondition::Above {
                SubType::Above
            } else {
                SubType::Below
            }
        } else {
            if order.direction == PositionDirection::Long {
                SubType::Bid
            } else {
                SubType::Ask
            }
        };

        match node_type {
            NodeType::RestingLimit => match sub_type {
                SubType::Bid => Some(&mut self.resting_limit_orders.resting_limit_bids),
                SubType::Ask => Some(&mut self.resting_limit_orders.resting_limit_asks),
                _ => None,
            },
            NodeType::FloatingLimit => match sub_type {
                SubType::Bid => Some(&mut self.floating_limit_orders.floating_limit_bids),
                SubType::Ask => Some(&mut self.floating_limit_orders.floating_limit_asks),
                _ => None,
            },
            NodeType::TakingLimit => match sub_type {
                SubType::Bid => Some(&mut self.taking_limit_orders.taking_limit_bids),
                SubType::Ask => Some(&mut self.taking_limit_orders.taking_limit_asks),
                _ => None,
            },
            NodeType::Market => match sub_type {
                SubType::Bid => Some(&mut self.market_orders.market_bids),
                SubType::Ask => Some(&mut self.market_orders.market_asks),
                _ => None,
            },
            NodeType::Trigger => match sub_type {
                SubType::Above => Some(&mut self.trigger_orders.trigger_above),
                SubType::Below => Some(&mut self.trigger_orders.trigger_below),
                _ => None,
            },
            NodeType::VAMM => None
        }
    }

}

pub(crate) type Exchange = DashMap<String, DashMap<u16, Market>>;

pub fn get_node_lists(exchange: &Exchange) -> Vec<NodeList> {
    exchange.iter().flat_map(|market_type_ref| {
        market_type_ref.value().iter().flat_map(move |market_ref| {
            vec![
                market_ref.value().resting_limit_orders.resting_limit_bids.clone(),
                market_ref.value().resting_limit_orders.resting_limit_asks.clone(),
                market_ref.value().taking_limit_orders.taking_limit_bids.clone(),
                market_ref.value().taking_limit_orders.taking_limit_asks.clone(),
                market_ref.value().market_orders.market_bids.clone(),
                market_ref.value().market_orders.market_asks.clone(),
                market_ref.value().floating_limit_orders.floating_limit_bids.clone(),
                market_ref.value().floating_limit_orders.floating_limit_asks.clone(),
                market_ref.value().trigger_orders.trigger_above.clone(),
                market_ref.value().trigger_orders.trigger_below.clone(),
            ]
        }).collect::<Vec<_>>()
    }).collect::<Vec<NodeList>>()
}

pub(crate) type OpenOrders = DashMap<String, DashSet<String>>;