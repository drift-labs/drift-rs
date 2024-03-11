use crate::dlob::dlob_node::{DLOBNode, SortDirection, NodeType, get_order_signature, Node};

use dashmap::DashMap;
use drift::state::user::Order;
use typed_arena::Arena;
use std::sync::Arc;
use std::panic::Location;

#[derive(Clone)]
pub(crate) struct NodeList {
    length: usize,
    head: Option<*mut Node>,
    tail: Option<*mut Node>,
    pub(crate) node_type: NodeType,
    pub arena: Arc<Arena<Node>>,
    node_map: Arc<DashMap<String, *mut Node>>,
    sort_direction: SortDirection,
}

impl NodeList {
    pub(crate) fn new(node_type: NodeType, sort_direction: SortDirection) -> Self {
        Self {
            length: 0,
            head: None,
            tail: None,
            node_type,
            arena: Arc::new(Arena::new()),
            node_map: Arc::new(DashMap::new()),
            sort_direction,
        }
    }

    #[track_caller]
    pub(crate) fn insert(&mut self, node: Node) {
        if node.get_node_type() != self.node_type {
            let caller = Location::caller();
            panic!("{}", format!("Node type mismatch. Expected: {:?}, Got: {:?} at {:?}", self.node_type, node.get_node_type(), caller));
        }
        let sort_value = node.get_sort_value(node.get_order()).unwrap();
        let order_signature = get_order_signature(node.get_order().order_id, node.get_user_account());

        let node_ptr = Box::into_raw(Box::new(node));

        if let Some(mut current_ptr) = self.head {
            let mut prev_ptr: Option<*mut Node> = None;

            loop {
                let current_sort_value = unsafe { (&*current_ptr).get_sort_value((&*current_ptr).get_order()).unwrap() };

                let should_insert = match self.sort_direction {
                    SortDirection::Ascending => {
                        if sort_value == current_sort_value {
                            unsafe { (&*node_ptr).get_order().slot < (&*current_ptr).get_order().slot }
                        } else {
                            sort_value < current_sort_value
                        }
                    }
                    SortDirection::Descending => {
                        if sort_value == current_sort_value {
                            unsafe { (&*node_ptr).get_order().slot > (&*current_ptr).get_order().slot }
                        } else {
                            sort_value > current_sort_value
                        }
                    }
                };

                if should_insert {
                    unsafe {
                        (&mut *node_ptr).set_next(Some(current_ptr));
                        (&mut *node_ptr).set_prev(prev_ptr);
                
                        if let Some(prev_ptr) = prev_ptr {
                            (&mut *prev_ptr).set_next(Some(node_ptr));
                        } else {
                            self.head = Some(node_ptr);
                        }
                        
                        (&mut *current_ptr).set_prev(Some(node_ptr));
                    }
                
                    self.node_map.insert(order_signature, node_ptr);
                    self.length += 1;
                
                    break;
                }
                match unsafe { (&*current_ptr).get_next_ptr() } {
                    Some(next_ptr) => {
                        prev_ptr = Some(current_ptr);
                        current_ptr = next_ptr as *mut Node;
                    }
                    None => {
                        unsafe {
                            (&mut *current_ptr).set_next(Some(node_ptr));
                            (&mut *node_ptr).set_prev(Some(current_ptr));
                            self.tail = Some(node_ptr);
                        }
                        self.node_map.insert(order_signature, node_ptr);
                        self.length += 1;
                        break;
                    }
                }
            }
        } else {
            self.head = Some(node_ptr);
            self.tail = Some(node_ptr);
            self.node_map.insert(order_signature, node_ptr);
            self.length += 1;
        }
    }

    pub(crate) fn remove(&mut self, order_signature: &str) -> Option< Node> {
        if let Some(node_ptr) = self.node_map.remove(order_signature) {
            let node_ptr = node_ptr.1;
            unsafe {
                let prev_ptr = (&*node_ptr).get_prev_ptr();
                let next_ptr = (&*node_ptr).get_next_ptr();
    
                if let Some(prev_ptr) = prev_ptr {
                    (&mut *prev_ptr).set_next(next_ptr);
                } else {
                    self.head = next_ptr;
                }
    
                if let Some(next_ptr) = next_ptr {
                    (&mut *next_ptr).set_prev(prev_ptr);
                } else {
                    self.tail = prev_ptr;
                }
    
                let node = *Box::from_raw(node_ptr as *mut Node);
                self.length -= 1;
                Some(node)
            }
        } else {
            None
        }
    }

    pub(crate) fn update_order(&mut self, order_signature: &str, new_order: Order) -> bool {
        if let Some(node_ptr) = self.node_map.get(order_signature) {
            let node_ptr = *node_ptr;
            unsafe {
                (&mut *node_ptr).set_order(new_order);
            }
            true
        } else {
            false
        }
    }

    pub(crate) fn contains(&self, order_signature: &str) -> bool {
        self.node_map.contains_key(order_signature)
    }

    pub(crate) fn get_node(&self, order_signature: &str) -> Option<&Node> {
        self.node_map.get(order_signature).map(|node_ptr| unsafe { &**node_ptr })
    }

    pub(crate) fn len(&self) -> usize {
        self.length
    }

    pub(crate) fn iter(&self) -> Box<dyn Iterator<Item = Node>> {
        Box::new(NodeListIter {
            current: self.head,
        })
    }

    pub(crate) fn print(&self) {
        let mut iter = self.iter();
        while let Some(node) = iter.next() {
            println!("{:?}", node.get_order());
        }
    }

    pub(crate) fn clear(&mut self) {
        let mut iter = self.iter();
        while let Some(node) = iter.next() {
            let order_signature = get_order_signature(node.get_order().order_id, node.get_user_account());
            self.node_map.remove(&order_signature);
            self.length -= 1;
        }
    }

    pub(crate) fn head(&self) -> Option<Node> {
        match self.head {
            Some(head_ptr) => {
                unsafe {
                    let head = (*head_ptr).clone();
                    Some(head)
                }
            }
            None => None,
        }
    }

    pub(crate) fn tail(&self) -> Option<Node> {
        match self.tail {
            Some(tail_ptr) => {
                unsafe {
                    let tail = (*tail_ptr).clone();
                    Some(tail)
                }
            }
            None => None,
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.length == 0
    }
}

pub struct NodeListIter {
    pub(crate) current: Option<*mut Node>,
}

impl Iterator for NodeListIter {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(node_ptr) => {
                unsafe {
                    let node = (*node_ptr).clone();
                    self.current = (*node_ptr).get_next_ptr();
                    Some(node)
                }
            }
            None => None,
        }
    }

    fn collect<B: std::iter::FromIterator<Self::Item>>(mut self) -> B {
        let mut collected = vec![];
        let mut exhausted = false;

        while !exhausted {
            if let Some(node) = self.next() {
                collected.push(node);
            } else {
                exhausted = true;
            }
        }

        collected.into_iter().collect()
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::dlob::dlob_node::{NodeType, create_node};
    use typed_arena::Arena;

    #[test]
    fn test_insert_and_iter() {
        let arena = Arena::new();

        let mut node_list = NodeList::new(NodeType::TakingLimit, SortDirection::Ascending);

        let mut order1 = Order::default();
        order1.order_id = 1;
        order1.slot = 100;
        let node1 = create_node(&arena, NodeType::TakingLimit, order1, Default::default());

        let mut order2 = Order::default();
        order2.order_id = 2;
        order2.slot = 200;
        let node2 = create_node(&arena, NodeType::TakingLimit, order2, Default::default());

        let mut order3 = Order::default();
        order3.order_id = 3;
        order3.slot = 150;
        let node3 = create_node(&arena, NodeType::TakingLimit, order3, Default::default());

        node_list.insert(*node1);
        node_list.insert(*node2);
        node_list.insert(*node3);

        assert_eq!(node_list.len(), 3);
        assert_eq!(node_list.head().unwrap().get_order().order_id, 1);
        assert_eq!(node_list.tail().unwrap().get_order().order_id, 2);
        
        let mut iter = node_list.iter();
        assert_eq!(iter.next().unwrap().get_order().order_id, 1);
        assert_eq!(iter.next().unwrap().get_order().order_id, 3);
        assert_eq!(iter.next().unwrap().get_order().order_id, 2);
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_clear() {
        let arena = Arena::new();

        let mut node_list = NodeList::new(NodeType::TakingLimit, SortDirection::Ascending);

        let mut order1 = Order::default();
        order1.order_id = 1;
        order1.slot = 100;
        let node1 = create_node(&arena, NodeType::TakingLimit, order1, Default::default());

        let mut order2 = Order::default();
        order2.order_id = 2;
        order2.slot = 200;
        let node2 = create_node(&arena, NodeType::TakingLimit, order2, Default::default());

        let mut order3 = Order::default();
        order3.order_id = 3;
        order3.slot = 150;
        let node3 = create_node(&arena, NodeType::TakingLimit, order3, Default::default());

        node_list.insert(*node1);
        node_list.insert(*node2);
        node_list.insert(*node3);

        assert_eq!(node_list.len(), 3);

        node_list.clear();
        assert_eq!(node_list.len(), 0);
    }
    
    #[test]
    fn test_remove_head() {
        let arena = Arena::new();

        let mut node_list = NodeList::new(NodeType::TakingLimit, SortDirection::Ascending);

        let mut order1 = Order::default();
        order1.order_id = 1;
        order1.slot = 100;
        let node1 = create_node(&arena, NodeType::TakingLimit, order1.clone(), Default::default());

        let mut order2 = Order::default();
        order2.order_id = 2;
        order2.slot = 200;
        let node2 = create_node(&arena, NodeType::TakingLimit, order2.clone(), Default::default());

        let mut order3 = Order::default();
        order3.order_id = 3;
        order3.slot = 150;
        let node3 = create_node(&arena, NodeType::TakingLimit, order3.clone(), Default::default());

        node_list.insert(*node1);
        node_list.insert(*node2);
        node_list.insert(*node3);

        assert_eq!(node_list.len(), 3);

        let removed_node = node_list.remove(&get_order_signature(order1.order_id, Default::default())).unwrap();
        assert_eq!(removed_node.get_order().order_id, 1);
        assert_eq!(node_list.len(), 2);

        let mut iter = node_list.iter();

        assert_eq!(iter.next().unwrap().get_order().order_id, 3);
        assert_eq!(iter.next().unwrap().get_order().order_id, 2);
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_remove_middle() {
        let arena = Arena::new();

        let mut node_list = NodeList::new(NodeType::TakingLimit, SortDirection::Ascending);

        let mut order1 = Order::default();
        order1.order_id = 1;
        order1.slot = 100;
        let node1 = create_node(&arena, NodeType::TakingLimit, order1.clone(), Default::default());

        let mut order2 = Order::default();
        order2.order_id = 2;
        order2.slot = 200;
        let node2 = create_node(&arena, NodeType::TakingLimit, order2.clone(), Default::default());

        let mut order3 = Order::default();
        order3.order_id = 3;
        order3.slot = 150;
        let node3 = create_node(&arena, NodeType::TakingLimit, order3.clone(), Default::default());

        node_list.insert(*node1);
        node_list.insert(*node2);
        node_list.insert(*node3);

        assert_eq!(node_list.len(), 3);

        let removed_node = node_list.remove(&get_order_signature(order3.order_id, Default::default())).unwrap();
        assert_eq!(removed_node.get_order().order_id, 3);
        assert_eq!(node_list.len(), 2);

        let mut iter = node_list.iter();

        assert_eq!(iter.next().unwrap().get_order().order_id, 1);
        assert_eq!(iter.next().unwrap().get_order().order_id, 2);
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_remove_end() {
        let arena = Arena::new();

        let mut node_list = NodeList::new(NodeType::TakingLimit, SortDirection::Ascending);

        let mut order1 = Order::default();
        order1.order_id = 1;
        order1.slot = 100;
        let node1 = create_node(&arena, NodeType::TakingLimit, order1.clone(), Default::default());

        let mut order2 = Order::default();
        order2.order_id = 2;
        order2.slot = 200;
        let node2 = create_node(&arena, NodeType::TakingLimit, order2.clone(), Default::default());

        let mut order3 = Order::default();
        order3.order_id = 3;
        order3.slot = 150;
        let node3 = create_node(&arena, NodeType::TakingLimit, order3.clone(), Default::default());

        node_list.insert(*node1);
        node_list.insert(*node2);
        node_list.insert(*node3);

        assert_eq!(node_list.len(), 3);

        let removed_node = node_list.remove(&get_order_signature(order2.order_id, Default::default())).unwrap();
        assert_eq!(removed_node.get_order().order_id, 2);
        assert_eq!(node_list.len(), 2);

        let mut iter = node_list.iter();

        assert_eq!(iter.next().unwrap().get_order().order_id, 1);
        assert_eq!(iter.next().unwrap().get_order().order_id, 3);
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_update_order() {
        let arena = Arena::new();

        let mut node_list = NodeList::new(NodeType::TakingLimit, SortDirection::Ascending);

        let mut order1 = Order::default();
        order1.order_id = 1;
        order1.slot = 100;
        let node1 = create_node(&arena, NodeType::TakingLimit, order1.clone(), Default::default());

        node_list.insert(*node1);

        let mut updated_order = Order::default();
        updated_order.order_id = 1;
        updated_order.slot = 200;

        assert!(node_list.update_order(&get_order_signature(order1.order_id, Default::default()), updated_order.clone()));

        let node = node_list.get_node(&get_order_signature(updated_order.order_id, Default::default())).unwrap();
        assert_eq!(node.get_order().slot, 200);
    }

    #[test]
    fn test_contains() {
        let arena = Arena::new();
        let mut node_list = NodeList::new(NodeType::TakingLimit, SortDirection::Ascending);

        let mut order1 = Order::default();
        order1.order_id = 1;
        order1.slot = 100;
        let node1 = create_node(&arena, NodeType::TakingLimit, order1.clone(), Default::default());

        node_list.insert(*node1);

        assert!(node_list.contains(&get_order_signature(order1.order_id, Default::default())));
        assert!(!node_list.contains(&get_order_signature(2, Default::default())));
    }

    #[test]
    #[should_panic]
    fn test_different_node_types() {
        let arena = Arena::new();
        let mut node_list = NodeList::new(NodeType::TakingLimit, SortDirection::Ascending);

        let mut order1 = Order::default();
        order1.order_id = 1;
        order1.slot = 100;
        let node1 = create_node(&arena, NodeType::TakingLimit, order1.clone(), Default::default());

        let mut order2 = Order::default();
        order2.order_id = 2;
        order2.price = 200;
        let node2 = create_node(&arena, NodeType::RestingLimit, order2.clone(), Default::default());

        node_list.insert(*node1);
        node_list.insert(*node2);
    }

    #[test]
    fn test_sort_direction() {
        let arena = Arena::new();
        let mut node_list_asc = NodeList::new(NodeType::TakingLimit, SortDirection::Ascending);
        let mut node_list_desc = NodeList::new(NodeType::TakingLimit, SortDirection::Descending);

        let mut order1 = Order::default();
        order1.order_id = 1;
        order1.slot = 100;
        let node1 = create_node(&arena, NodeType::TakingLimit, order1.clone(), Default::default());
        let node1_clone = create_node(&arena, NodeType::TakingLimit, order1.clone(), Default::default());

        let mut order2 = Order::default();
        order2.order_id = 2;
        order2.slot = 200;
        let node2 = create_node(&arena, NodeType::TakingLimit, order2.clone(), Default::default());
        let node2_clone = create_node(&arena, NodeType::TakingLimit, order2.clone(), Default::default());

        node_list_asc.insert(*node1_clone);
        node_list_asc.insert(*node2_clone);

        node_list_desc.insert(*node1);
        node_list_desc.insert(*node2);

        let mut iter_asc = node_list_asc.iter();
        assert_eq!(iter_asc.next().unwrap().get_order().order_id, 1);
        assert_eq!(iter_asc.next().unwrap().get_order().order_id, 2);
        assert!(iter_asc.next().is_none());

        let mut iter_desc = node_list_desc.iter();
        assert_eq!(iter_desc.next().unwrap().get_order().order_id, 2);
        assert_eq!(iter_desc.next().unwrap().get_order().order_id, 1);
        assert!(iter_desc.next().is_none());
    }
}