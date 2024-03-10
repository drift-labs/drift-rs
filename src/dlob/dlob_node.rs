use drift::state::{oracle::OraclePriceData, user::Order};
use num_bigint::BigInt;
use solana_sdk::pubkey::Pubkey;
use typed_arena::Arena;
use crate::{math::order::get_limit_price, SdkResult};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NodeType {
    TakingLimit,
    RestingLimit,
    FloatingLimit,
    Market,
    Trigger,
    VAMM,
}

#[derive(PartialEq, Eq, Clone)]
pub(crate) enum SortDirection {
    Ascending,
    Descending
}

#[derive(Copy, Clone, Debug)]
pub enum Node {
    OrderNode(OrderNode),
    VAMMNode(VAMMNode),
}

impl Node {
    pub fn new(node_type: NodeType, order: Order, user_account: Pubkey) -> Self {
        match node_type {
            NodeType::TakingLimit => Node::OrderNode(OrderNode::new(NodeType::TakingLimit, order, user_account)),
            NodeType::RestingLimit => Node::OrderNode(OrderNode::new(NodeType::RestingLimit, order, user_account)),
            NodeType::FloatingLimit => Node::OrderNode(OrderNode::new(NodeType::FloatingLimit, order, user_account)),
            NodeType::Market => Node::OrderNode(OrderNode::new(NodeType::Market, order, user_account)),
            NodeType::Trigger => Node::OrderNode(OrderNode::new(NodeType::Trigger, order, user_account)),
            NodeType::VAMM => Node::VAMMNode(VAMMNode::new(order, 0)),
        }
    }
}

impl DLOBNode for Node {
    fn get_price(&self, oracle_price_data: OraclePriceData, slot: u64) -> BigInt {
        match self {
            Node::OrderNode(order_node) => order_node.get_price(oracle_price_data, slot),
            Node::VAMMNode(vamm_node) => vamm_node.get_price(oracle_price_data, slot),
        }
    }

    fn is_vamm_node(&self) -> bool {
        match self {
            Node::OrderNode(_) => false,
            Node::VAMMNode(_) => true,
        }
    }

    fn is_base_filled(&self) -> bool {
        match self {
            Node::OrderNode(order_node) => order_node.is_base_filled(),
            Node::VAMMNode(vamm_node) => vamm_node.is_base_filled(),
        }
    }

    fn get_sort_value(&self, order: &Order) -> Option<i128> {
        match self {
            Node::OrderNode(order_node) => order_node.get_sort_value(order),
            Node::VAMMNode(vamm_node) => vamm_node.get_sort_value(order),
        }
    }

    fn get_order(&self) -> &Order {
        match self {
            Node::OrderNode(order_node) => order_node.get_order(),
            Node::VAMMNode(vamm_node) => vamm_node.get_order(),
        }
    }

    fn get_next_ptr(&self) -> Option<*mut Node> {
        match self {
            Node::OrderNode(order_node) => order_node.get_next_ptr(),
            Node::VAMMNode(vamm_node) => vamm_node.get_next_ptr(),
        }
    }

    fn get_prev_ptr(&self) -> Option<*mut Node> {
        match self {
            Node::OrderNode(order_node) => order_node.get_prev_ptr(),
            Node::VAMMNode(vamm_node) => vamm_node.get_prev_ptr(),
        }
    }

    fn get_user_account(&self) -> Pubkey {
        match self {
            Node::OrderNode(order_node) => order_node.get_user_account(),
            Node::VAMMNode(vamm_node) => vamm_node.get_user_account(),
        }
    }

    fn set_next(&mut self, next: Option<*mut Node>) {
        match self {
            Node::OrderNode(order_node) => order_node.set_next(next),
            Node::VAMMNode(vamm_node) => vamm_node.set_next(next),
        }
    }

    fn set_prev(&mut self, prev: Option<*mut Node>) {
        match self {
            Node::OrderNode(order_node) => order_node.set_prev(prev),
            Node::VAMMNode(vamm_node) => vamm_node.set_prev(prev),
        }
    }

    fn set_order(&mut self, order: Order) {
        match self {
            Node::OrderNode(order_node) => order_node.set_order(order),
            Node::VAMMNode(vamm_node) => vamm_node.set_order(order),
        }
    }

    fn get_node_type(&self) -> NodeType {
        match self {
            Node::OrderNode(order_node) => order_node.get_node_type(),
            Node::VAMMNode(_) => NodeType::VAMM,
        }
    }

    fn set_node_type(&mut self, node_type: NodeType) {
        match self {
            Node::OrderNode(order_node) => order_node.set_node_type(node_type),
            Node::VAMMNode(_) => unimplemented!(),
        }
    
    }
}

#[derive(Clone, Copy, Debug)]
pub struct OrderNode {
    pub order: Order,
    pub user_account: Pubkey,
    pub node_type: NodeType,
    pub next: Option<*mut OrderNode>,
    pub previous: Option<*mut OrderNode>,
}

impl OrderNode {
    pub fn new(node_type: NodeType, order: Order, user_account: Pubkey) -> Self {
        Self {
            order,
            user_account,
            node_type,
            next: None,
            previous: None,
        }
    }
}

impl DLOBNode for OrderNode {
    fn get_price(&self, oracle_price_data: OraclePriceData, slot: u64) -> BigInt  {
        get_limit_price(&self.order, &oracle_price_data, slot, None)
    }

    fn is_vamm_node(&self) -> bool {
        false
    }

    fn is_base_filled(&self) -> bool {
        self.order.base_asset_amount_filled == self.order.base_asset_amount
    }

    fn get_sort_value(&self, order: &Order) -> Option<i128> {
        match self.node_type {
            NodeType::TakingLimit => Some(order.slot.into()),
            NodeType::RestingLimit => Some(order.price.into()),
            NodeType::FloatingLimit => Some(order.oracle_price_offset.into()),
            NodeType::Market => Some(order.slot.into()),
            NodeType::Trigger => Some(order.trigger_price.into()),
            NodeType::VAMM => None,
        }
    }

    fn get_order(&self) -> &Order {
        &self.order
    }

    fn get_next_ptr(&self) -> Option<*mut Node> {
        self.next.map(|ptr| ptr as *mut Node)
    }

    fn get_prev_ptr(&self) -> Option<*mut Node> {
        self.previous.map(|ptr| ptr as *mut Node)
    }

    fn get_user_account(&self) -> Pubkey {
        self.user_account
    }

    fn set_next(&mut self, next: Option<*mut Node>) {
        self.next = next.map(|ptr| ptr as *mut OrderNode);
    }

    fn set_prev(&mut self, prev: Option<*mut Node>) {
        self.previous = prev.map(|ptr| ptr as *mut OrderNode);
    }

    fn set_order(&mut self, order: Order) {
        self.order = order;
    }

    fn get_node_type(&self) -> NodeType {
        self.node_type
    }

    fn set_node_type(&mut self, node_type: NodeType) {
        self.node_type = node_type;
    }
}

pub(crate) fn create_node<'a>(arena: &'a Arena<Node>, node_type: NodeType, order: Order, user_account: Pubkey) -> &'a mut Node {
    arena.alloc(Node::new(node_type, order, user_account))
}

pub(crate) fn get_order_signature(order_id: u32, user_account: Pubkey) -> String {
    format!("{}-{}", order_id, user_account.to_string())
}

pub(crate) trait DLOBNode {
    fn get_price(&self, oracle_price_data: OraclePriceData, slot: u64) -> BigInt;

    fn is_vamm_node(&self) -> bool;

    fn is_base_filled(&self) -> bool;

    fn get_sort_value(&self, order: &Order) -> Option<i128>;

    fn get_order(&self) -> &Order;

    fn get_next_ptr(&self) -> Option<*mut Node>;
    
    fn get_prev_ptr(&self) -> Option<*mut Node>;

    fn get_user_account(&self) -> Pubkey;

    fn set_next(&mut self, next: Option<*mut Node>);

    fn set_prev(&mut self, next: Option<*mut Node>);

    fn set_order(&mut self, order: Order);

    fn get_node_type(&self) -> NodeType;

    fn set_node_type(&mut self, node_type: NodeType);
}

pub(crate) trait DLOBNodePointerExt {
    unsafe fn to_node(&self) -> Option<&Node>;
}

impl DLOBNodePointerExt for Option<*mut Node> {
    unsafe fn to_node(&self) -> Option<&Node> {
        self.map(|ptr| &*ptr)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct VAMMNode {
    pub order: Order,
    pub price: u64,
}

impl DLOBNode for VAMMNode {
    fn get_price(&self, _oracle_price_data: OraclePriceData, _slot: u64) -> BigInt {
        BigInt::from(self.price)
    }

    fn is_vamm_node(&self) -> bool {
        true
    }

    fn is_base_filled(&self) -> bool {
        false
    }

    fn get_sort_value(&self, _order: &Order) -> Option<i128> {
        None
    }

    fn get_order(&self) -> &Order {
        &self.order
    }

    fn get_next_ptr(&self) -> Option<*mut Node> {
        None
    }

    fn get_prev_ptr(&self) -> Option<*mut Node> {
        None
    }

    fn set_next(&mut self, _next: Option<*mut Node>) {
        unimplemented!()
    }

    fn set_prev(&mut self, _prev: Option<*mut Node>) {
        unimplemented!()
    }

    fn get_user_account(&self) -> Pubkey {
        unimplemented!()
    }

    fn set_order(&mut self, _order: Order) {
        unimplemented!()
    }

    fn get_node_type(&self) -> NodeType {
        NodeType::VAMM
    }

    fn set_node_type(&mut self, _node_type: NodeType) {
        unimplemented!()
    }
}

impl VAMMNode {
    pub fn new(order: Order, price: u64) -> Self {
        Self {
            order,
            price,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_set_next_prev() {

        let arena = Arena::new();

        let mut order = Order::default();

        order.slot = 100;
        order.price = 1_000;
        order.trigger_price = 500;
        order.oracle_price_offset = 5_000;

        let user_account = Pubkey::new_unique();

        let taking_limit_order_node = create_node(&arena, NodeType::TakingLimit, order, user_account);
        let resting_limit_order_node = create_node(&arena, NodeType::RestingLimit, order, user_account);
        let floating_limit_order_node = create_node(&arena, NodeType::FloatingLimit, order, user_account);
        let market_order_node = create_node(&arena, NodeType::Market, order, user_account);
        let trigger_order_node = create_node(&arena, NodeType::Trigger, order, user_account);
        
        assert_eq!(taking_limit_order_node.get_sort_value(&order), Some(100));
        assert_eq!(resting_limit_order_node.get_sort_value(&order), Some(1_000));
        assert_eq!(market_order_node.get_sort_value(&order), Some(100));
        assert_eq!(trigger_order_node.get_sort_value(&order), Some(500));
        assert_eq!(floating_limit_order_node.get_sort_value(&order), Some(5_000));

        let mut order_2 = Order::default();

        order_2.slot = 200;
        order_2.price = 2_000;
        order_2.trigger_price = 600;
        order_2.oracle_price_offset = 6_000;

        let taking_limit_order_node_2 = create_node(&arena, NodeType::TakingLimit, order_2, user_account);
        let resting_limit_order_node_2 = create_node(&arena, NodeType::RestingLimit, order_2, user_account);
        let floating_limit_order_node_2 = create_node(&arena, NodeType::FloatingLimit, order_2, user_account);
        let market_order_node_2 = create_node(&arena, NodeType::Market, order_2, user_account);
        let trigger_order_node_2 = create_node(&arena, NodeType::Trigger, order_2, user_account);


        assert_eq!(taking_limit_order_node_2.get_sort_value(&order_2), Some(200));
        assert_eq!(resting_limit_order_node_2.get_sort_value(&order_2), Some(2_000));
        assert_eq!(market_order_node_2.get_sort_value(&order_2), Some(200));
        assert_eq!(trigger_order_node_2.get_sort_value(&order_2), Some(600));
        assert_eq!(floating_limit_order_node_2.get_sort_value(&order_2), Some(6_000));

        taking_limit_order_node.set_next(Some(&mut *taking_limit_order_node_2));
        resting_limit_order_node.set_next(Some(&mut *resting_limit_order_node_2));
        floating_limit_order_node.set_next(Some(&mut *floating_limit_order_node_2));
        market_order_node.set_next(Some(&mut *market_order_node_2));
        trigger_order_node.set_next(Some(&mut *trigger_order_node_2));

        assert_eq!(unsafe { taking_limit_order_node.get_next_ptr().to_node().unwrap().get_sort_value(&order_2) }, Some(200));
        assert_eq!(unsafe { resting_limit_order_node.get_next_ptr().to_node().unwrap().get_sort_value(&order_2) }, Some(2_000));
        assert_eq!(unsafe { market_order_node.get_next_ptr().to_node().unwrap().get_sort_value(&order_2) }, Some(200));
        assert_eq!(unsafe { trigger_order_node.get_next_ptr().to_node().unwrap().get_sort_value(&order_2) }, Some(600));
        assert_eq!(unsafe { floating_limit_order_node.get_next_ptr().to_node().unwrap().get_sort_value(&order_2) }, Some(6_000));

        let mut order_3 = Order::default();

        order_3.slot = 300;
        order_3.price = 3_000;
        order_3.trigger_price = 700;
        order_3.oracle_price_offset = 7_000;

        let taking_limit_order_node_3 = create_node(&arena, NodeType::TakingLimit, order_3, user_account);
        let resting_limit_order_node_3 = create_node(&arena, NodeType::RestingLimit, order_3, user_account);
        let floating_limit_order_node_3 = create_node(&arena, NodeType::FloatingLimit, order_3, user_account);
        let market_order_node_3 = create_node(&arena, NodeType::Market, order_3, user_account);
        let trigger_order_node_3 = create_node(&arena, NodeType::Trigger, order_3, user_account);

        assert_eq!(taking_limit_order_node_3.get_sort_value(&order_3), Some(300));
        assert_eq!(resting_limit_order_node_3.get_sort_value(&order_3), Some(3_000));
        assert_eq!(market_order_node_3.get_sort_value(&order_3), Some(300));
        assert_eq!(trigger_order_node_3.get_sort_value(&order_3), Some(700));
        assert_eq!(floating_limit_order_node_3.get_sort_value(&order_3), Some(7_000));

        taking_limit_order_node.set_prev(Some(&mut *taking_limit_order_node_3));
        resting_limit_order_node.set_prev(Some(&mut *resting_limit_order_node_3));
        floating_limit_order_node.set_prev(Some(&mut *floating_limit_order_node_3));
        market_order_node.set_prev(Some(&mut *market_order_node_3));
        trigger_order_node.set_prev(Some(&mut *trigger_order_node_3));

        assert_eq!(unsafe { taking_limit_order_node.get_next_ptr().to_node().unwrap().get_sort_value(&order_3) }, Some(300));
        assert_eq!(unsafe { resting_limit_order_node.get_next_ptr().to_node().unwrap().get_sort_value(&order_3) }, Some(3_000));
        assert_eq!(unsafe { market_order_node.get_next_ptr().to_node().unwrap().get_sort_value(&order_3) }, Some(300));
        assert_eq!(unsafe { trigger_order_node.get_next_ptr().to_node().unwrap().get_sort_value(&order_3) }, Some(700));
        assert_eq!(unsafe { floating_limit_order_node.get_next_ptr().to_node().unwrap().get_sort_value(&order_3) }, Some(7_000));
    }

    #[test]
    fn test_set_order() {
        let arena = Arena::new();

        let user_account = Pubkey::new_unique();

        let mut order = Order::default();

        order.slot = 100;
        order.price = 1_000;
        order.trigger_price = 500;
        order.oracle_price_offset = 5_000;

        let taking_limit_order_node = create_node(&arena, NodeType::TakingLimit, order, user_account);
        let resting_limit_order_node = create_node(&arena, NodeType::RestingLimit, order, user_account);
        let floating_limit_order_node = create_node(&arena, NodeType::FloatingLimit, order, user_account);
        let market_order_node = create_node(&arena, NodeType::Market, order, user_account);
        let trigger_order_node = create_node(&arena, NodeType::Trigger, order, user_account);

        let mut order_2 = Order::default();

        order_2.slot = 200;
        order_2.price = 2_000;
        order_2.trigger_price = 600;
        order_2.oracle_price_offset = 6_000;

        taking_limit_order_node.set_order(order_2);
        resting_limit_order_node.set_order(order_2);
        floating_limit_order_node.set_order(order_2);
        market_order_node.set_order(order_2);
        trigger_order_node.set_order(order_2);

        assert_eq!(taking_limit_order_node.get_order().slot, 200);
        assert_eq!(resting_limit_order_node.get_order().price, 2_000);
        assert_eq!(floating_limit_order_node.get_order().oracle_price_offset, 6_000);
        assert_eq!(market_order_node.get_order().slot, 200);
        assert_eq!(trigger_order_node.get_order().trigger_price, 600);
    }

    #[test]
    #[should_panic(expected = "not implemented")]
    fn test_vamm_node_set_next_panics() {
        let order = Order::default();
        let mut vamm_node = VAMMNode::new(order, 100);
        vamm_node.set_next(None);
    }

    #[test]
    #[should_panic(expected = "not implemented")]
    fn test_vamm_node_set_prev_panics() {
        let order = Order::default();
        let mut vamm_node = VAMMNode::new(order, 100);
        vamm_node.set_prev(None);
    }

    #[test]
    #[should_panic(expected = "not implemented")]
    fn test_vamm_node_get_user_account_panics() {
        let order = Order::default();
        let vamm_node = VAMMNode::new(order, 100);
        vamm_node.get_user_account();
    }

    #[test]
    #[should_panic(expected = "not implemented")]
    fn test_vamm_node_set_order_panics() {
        let order = Order::default();
        let mut vamm_node = VAMMNode::new(order, 100);
        vamm_node.set_order(order);
    }
}