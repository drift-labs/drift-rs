use std::collections::BinaryHeap;

use dashmap::DashMap;

use crate::dlob::dlob_node::{get_order_signature, DLOBNode, DirectionalNode, Node, SortDirection};

#[derive(Clone, Debug)]
pub struct Orderlist {
    pub bids: BinaryHeap<DirectionalNode>,
    pub asks: BinaryHeap<DirectionalNode>,
    pub order_sigs: DashMap<String, Node>,
    bid_sort_direction: SortDirection,
    ask_sort_direction: SortDirection,
}

impl Orderlist {
    pub fn new(bid_sort_direction: SortDirection, ask_sort_direction: SortDirection) -> Self {
        Orderlist {
            bids: BinaryHeap::new(),
            asks: BinaryHeap::new(),
            order_sigs: DashMap::new(),
            bid_sort_direction,
            ask_sort_direction,
        }
    }

    /// for debugging
    pub fn print(&self) {
        println!("Bids: {:?}", self.bids);
        println!("Asks: {:?}", self.asks);
    }

    pub fn insert_bid(&mut self, node: Node) {
        let order_sig = get_order_signature(node.get_order().order_id, node.get_user_account());
        self.order_sigs.insert(order_sig.clone(), node);
        let directional = DirectionalNode::new(node, self.bid_sort_direction);
        self.bids.push(directional);
    }

    pub fn insert_ask(&mut self, node: Node) {
        let order_sig = get_order_signature(node.get_order().order_id, node.get_user_account());
        self.order_sigs.insert(order_sig.clone(), node);
        let directional = DirectionalNode::new(node, self.ask_sort_direction);
        self.asks.push(directional);
    }

    pub fn get_best_bid(&mut self) -> Option<Node> {
        if let Some(node) = self.bids.pop().map(|node| node.node) {
            let order_sig = get_order_signature(node.get_order().order_id, node.get_user_account());
            if self.order_sigs.contains_key(&order_sig) {
                self.order_sigs.remove(&order_sig);
                return Some(node);
            }
        }
        None
    }

    pub fn get_best_ask(&mut self) -> Option<Node> {
        if let Some(node) = self.asks.pop().map(|node| node.node) {
            let order_sig = get_order_signature(node.get_order().order_id, node.get_user_account());
            if self.order_sigs.contains_key(&order_sig) {
                self.order_sigs.remove(&order_sig);
                return Some(node);
            }
        }
        None
    }

    pub fn get_node(&self, order_sig: &String) -> Option<Node> {
        self.order_sigs.get(order_sig).map(|node| *node)
    }

    pub fn bids_empty(&self) -> bool {
        self.bids.is_empty()
    }

    pub fn asks_empty(&self) -> bool {
        self.asks.is_empty()
    }

    pub fn size(&self) -> usize {
        self.bids.len() + self.asks.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::dlob::dlob_node::{create_node, NodeType};

    use super::*;
    use drift::state::user::Order;
    use solana_sdk::pubkey::Pubkey;

    #[test]
    fn test_insertion_and_ordering() {
        let mut orderlist = Orderlist::new(SortDirection::Ascending, SortDirection::Ascending);
        let user_account = Pubkey::new_unique();
        let order_1 = Order {
            order_id: 1,
            slot: 1,
            ..Order::default()
        };
        let order_2 = Order {
            order_id: 2,
            slot: 2,
            ..Order::default()
        };
        let order_3 = Order {
            order_id: 3,
            slot: 3,
            ..Order::default()
        };
        let order_4 = Order {
            order_id: 4,
            slot: 4,
            ..Order::default()
        };
        let order_5 = Order {
            order_id: 5,
            slot: 5,
            ..Order::default()
        };
        let order_6 = Order {
            order_id: 6,
            slot: 1,
            ..Order::default()
        };
        let order_7 = Order {
            order_id: 7,
            slot: 2,
            ..Order::default()
        };
        let order_8 = Order {
            order_id: 8,
            slot: 3,
            ..Order::default()
        };
        let order_9 = Order {
            order_id: 9,
            slot: 4,
            ..Order::default()
        };
        let order_10 = Order {
            order_id: 10,
            slot: 5,
            ..Order::default()
        };

        let node_1 = create_node(NodeType::TakingLimit, order_1, user_account);
        let node_2 = create_node(NodeType::TakingLimit, order_2, user_account);
        let node_3 = create_node(NodeType::TakingLimit, order_3, user_account);
        let node_4 = create_node(NodeType::TakingLimit, order_4, user_account);
        let node_5 = create_node(NodeType::TakingLimit, order_5, user_account);

        let node_6 = create_node(NodeType::TakingLimit, order_6, user_account);
        let node_7 = create_node(NodeType::TakingLimit, order_7, user_account);
        let node_8 = create_node(NodeType::TakingLimit, order_8, user_account);
        let node_9 = create_node(NodeType::TakingLimit, order_9, user_account);
        let node_10 = create_node(NodeType::TakingLimit, order_10, user_account);

        orderlist.insert_bid(node_1);
        orderlist.insert_bid(node_2);
        orderlist.insert_bid(node_3);
        orderlist.insert_bid(node_4);
        orderlist.insert_bid(node_5);

        orderlist.insert_ask(node_6);
        orderlist.insert_ask(node_7);
        orderlist.insert_ask(node_8);
        orderlist.insert_ask(node_9);
        orderlist.insert_ask(node_10);

        assert_eq!(orderlist.get_best_bid().unwrap().get_order().slot, 1);
        assert_eq!(orderlist.get_best_bid().unwrap().get_order().slot, 2);
        assert_eq!(orderlist.get_best_bid().unwrap().get_order().slot, 3);
        assert_eq!(orderlist.get_best_bid().unwrap().get_order().slot, 4);
        assert_eq!(orderlist.get_best_bid().unwrap().get_order().slot, 5);

        assert_eq!(orderlist.get_best_ask().unwrap().get_order().slot, 1);
        assert_eq!(orderlist.get_best_ask().unwrap().get_order().slot, 2);
        assert_eq!(orderlist.get_best_ask().unwrap().get_order().slot, 3);
        assert_eq!(orderlist.get_best_ask().unwrap().get_order().slot, 4);
        assert_eq!(orderlist.get_best_ask().unwrap().get_order().slot, 5);
    }
}
