use crate::math::order::get_limit_price;
use drift::state::{oracle::OraclePriceData, user::Order};
use solana_sdk::pubkey::Pubkey;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NodeType {
    TakingLimit,
    RestingLimit,
    FloatingLimit,
    Market,
    Trigger,
    VAMM,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub(crate) enum SortDirection {
    Ascending,
    Descending,
}

pub trait DLOBNode {
    fn get_price(&self, oracle_price_data: OraclePriceData, slot: u64) -> u64;
    fn is_vamm_node(&self) -> bool;
    fn is_base_filled(&self) -> bool;
    fn get_sort_value(&self, order: &Order) -> Option<i128>;
    fn get_order(&self) -> &Order;
    fn get_user_account(&self) -> Pubkey;
    fn set_order(&mut self, order: Order);
    fn get_node_type(&self) -> NodeType;
    fn set_node_type(&mut self, node_type: NodeType);
}

#[derive(Copy, Clone, Debug)]
pub enum Node {
    OrderNode(OrderNode),
    VAMMNode(VAMMNode),
}

#[derive(Clone, Copy, Debug)]
pub struct DirectionalNode {
    pub node: Node,
    sort_direction: SortDirection,
}

impl DirectionalNode {
    pub fn new(node: Node, sort_direction: SortDirection) -> Self {
        Self {
            node,
            sort_direction,
        }
    }
}

impl PartialEq for DirectionalNode {
    fn eq(&self, other: &Self) -> bool {
        self.node.eq(&other.node)
    }
}

impl Eq for DirectionalNode {}

impl PartialOrd for DirectionalNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let mut cmp = self
            .node
            .get_sort_value(self.node.get_order())
            .partial_cmp(&other.node.get_sort_value(other.node.get_order()))
            .unwrap_or(std::cmp::Ordering::Equal);

        if cmp == std::cmp::Ordering::Equal {
            cmp = self.node.get_order().slot.cmp(&other.node.get_order().slot);
        }

        if self.sort_direction == SortDirection::Ascending {
            cmp = cmp.reverse();
        }

        Some(cmp)
    }
}

impl Ord for DirectionalNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.get_sort_value(self.get_order()) == other.get_sort_value(other.get_order())
    }
}

impl Eq for Node {}

impl Node {
    pub fn new(node_type: NodeType, order: Order, user_account: Pubkey) -> Self {
        match node_type {
            NodeType::TakingLimit => {
                Node::OrderNode(OrderNode::new(NodeType::TakingLimit, order, user_account))
            }
            NodeType::RestingLimit => {
                Node::OrderNode(OrderNode::new(NodeType::RestingLimit, order, user_account))
            }
            NodeType::FloatingLimit => {
                Node::OrderNode(OrderNode::new(NodeType::FloatingLimit, order, user_account))
            }
            NodeType::Market => {
                Node::OrderNode(OrderNode::new(NodeType::Market, order, user_account))
            }
            NodeType::Trigger => {
                Node::OrderNode(OrderNode::new(NodeType::Trigger, order, user_account))
            }
            NodeType::VAMM => Node::VAMMNode(VAMMNode::new(order, 0)),
        }
    }
}

impl DLOBNode for Node {
    fn get_price(&self, oracle_price_data: OraclePriceData, slot: u64) -> u64 {
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

    fn get_user_account(&self) -> Pubkey {
        match self {
            Node::OrderNode(order_node) => order_node.get_user_account(),
            Node::VAMMNode(vamm_node) => vamm_node.get_user_account(),
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
}

impl OrderNode {
    pub fn new(node_type: NodeType, order: Order, user_account: Pubkey) -> Self {
        Self {
            order,
            user_account,
            node_type,
        }
    }
}

impl DLOBNode for OrderNode {
    fn get_price(&self, oracle_price_data: OraclePriceData, slot: u64) -> u64 {
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

    fn get_user_account(&self) -> Pubkey {
        self.user_account
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

#[derive(Copy, Clone, Debug)]
pub struct VAMMNode {
    pub order: Order,
    pub price: u64,
}

impl VAMMNode {
    pub fn new(order: Order, price: u64) -> Self {
        Self { order, price }
    }
}

impl DLOBNode for VAMMNode {
    fn get_price(&self, _oracle_price_data: OraclePriceData, _slot: u64) -> u64 {
        self.price
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

pub(crate) fn create_node(node_type: NodeType, order: Order, user_account: Pubkey) -> Node {
    Node::new(node_type, order, user_account)
}

pub(crate) fn get_order_signature(order_id: u32, user_account: Pubkey) -> String {
    format!("{}-{}", order_id, user_account)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_set_next_prev() {
        let mut order = Order::default();

        order.slot = 100;
        order.price = 1_000;
        order.trigger_price = 500;
        order.oracle_price_offset = 5_000;

        let user_account = Pubkey::new_unique();

        let taking_limit_order_node = create_node(NodeType::TakingLimit, order, user_account);
        let resting_limit_order_node = create_node(NodeType::RestingLimit, order, user_account);
        let floating_limit_order_node = create_node(NodeType::FloatingLimit, order, user_account);
        let market_order_node = create_node(NodeType::Market, order, user_account);
        let trigger_order_node = create_node(NodeType::Trigger, order, user_account);

        assert_eq!(taking_limit_order_node.get_sort_value(&order), Some(100));
        assert_eq!(resting_limit_order_node.get_sort_value(&order), Some(1_000));
        assert_eq!(market_order_node.get_sort_value(&order), Some(100));
        assert_eq!(trigger_order_node.get_sort_value(&order), Some(500));
        assert_eq!(
            floating_limit_order_node.get_sort_value(&order),
            Some(5_000)
        );

        let mut order_2 = Order::default();

        order_2.slot = 200;
        order_2.price = 2_000;
        order_2.trigger_price = 600;
        order_2.oracle_price_offset = 6_000;

        let taking_limit_order_node_2 = create_node(NodeType::TakingLimit, order_2, user_account);
        let resting_limit_order_node_2 = create_node(NodeType::RestingLimit, order_2, user_account);
        let floating_limit_order_node_2 =
            create_node(NodeType::FloatingLimit, order_2, user_account);
        let market_order_node_2 = create_node(NodeType::Market, order_2, user_account);
        let trigger_order_node_2 = create_node(NodeType::Trigger, order_2, user_account);

        assert_eq!(
            taking_limit_order_node_2.get_sort_value(&order_2),
            Some(200)
        );
        assert_eq!(
            resting_limit_order_node_2.get_sort_value(&order_2),
            Some(2_000)
        );
        assert_eq!(market_order_node_2.get_sort_value(&order_2), Some(200));
        assert_eq!(trigger_order_node_2.get_sort_value(&order_2), Some(600));
        assert_eq!(
            floating_limit_order_node_2.get_sort_value(&order_2),
            Some(6_000)
        );

        let mut order_3 = Order::default();

        order_3.slot = 300;
        order_3.price = 3_000;
        order_3.trigger_price = 700;
        order_3.oracle_price_offset = 7_000;

        let taking_limit_order_node_3 = create_node(NodeType::TakingLimit, order_3, user_account);
        let resting_limit_order_node_3 = create_node(NodeType::RestingLimit, order_3, user_account);
        let floating_limit_order_node_3 =
            create_node(NodeType::FloatingLimit, order_3, user_account);
        let market_order_node_3 = create_node(NodeType::Market, order_3, user_account);
        let trigger_order_node_3 = create_node(NodeType::Trigger, order_3, user_account);

        assert_eq!(
            taking_limit_order_node_3.get_sort_value(&order_3),
            Some(300)
        );
        assert_eq!(
            resting_limit_order_node_3.get_sort_value(&order_3),
            Some(3_000)
        );
        assert_eq!(market_order_node_3.get_sort_value(&order_3), Some(300));
        assert_eq!(trigger_order_node_3.get_sort_value(&order_3), Some(700));
        assert_eq!(
            floating_limit_order_node_3.get_sort_value(&order_3),
            Some(7_000)
        );
    }

    #[test]
    fn test_set_order() {
        let user_account = Pubkey::new_unique();

        let mut order = Order::default();

        order.slot = 100;
        order.price = 1_000;
        order.trigger_price = 500;
        order.oracle_price_offset = 5_000;

        let mut taking_limit_order_node = create_node(NodeType::TakingLimit, order, user_account);
        let mut resting_limit_order_node = create_node(NodeType::RestingLimit, order, user_account);
        let mut floating_limit_order_node =
            create_node(NodeType::FloatingLimit, order, user_account);
        let mut market_order_node = create_node(NodeType::Market, order, user_account);
        let mut trigger_order_node = create_node(NodeType::Trigger, order, user_account);

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
        assert_eq!(
            floating_limit_order_node.get_order().oracle_price_offset,
            6_000
        );
        assert_eq!(market_order_node.get_order().slot, 200);
        assert_eq!(trigger_order_node.get_order().trigger_price, 600);
    }

    #[test]
    fn test_eq() {
        let user_account = Pubkey::new_unique();

        let mut order = Order::default();

        order.slot = 100;
        order.price = 1_000;
        order.trigger_price = 500;
        order.oracle_price_offset = 5_000;

        let taking_limit_order_node = create_node(NodeType::TakingLimit, order, user_account);
        let resting_limit_order_node = create_node(NodeType::RestingLimit, order, user_account);
        let floating_limit_order_node = create_node(NodeType::FloatingLimit, order, user_account);
        let market_order_node = create_node(NodeType::Market, order, user_account);
        let trigger_order_node = create_node(NodeType::Trigger, order, user_account);

        let mut order_2 = Order::default();

        order_2.slot = 200;
        order_2.price = 2_000;
        order_2.trigger_price = 600;
        order_2.oracle_price_offset = 6_000;

        let taking_limit_order_node_2 = create_node(NodeType::TakingLimit, order_2, user_account);
        let resting_limit_order_node_2 = create_node(NodeType::RestingLimit, order_2, user_account);
        let floating_limit_order_node_2 =
            create_node(NodeType::FloatingLimit, order_2, user_account);
        let market_order_node_2 = create_node(NodeType::Market, order_2, user_account);
        let trigger_order_node_2 = create_node(NodeType::Trigger, order_2, user_account);

        assert_eq!(taking_limit_order_node, taking_limit_order_node);
        assert_eq!(resting_limit_order_node, resting_limit_order_node);
        assert_eq!(floating_limit_order_node, floating_limit_order_node);
        assert_eq!(market_order_node, market_order_node);
        assert_eq!(trigger_order_node, trigger_order_node);

        assert_ne!(taking_limit_order_node, taking_limit_order_node_2);
        assert_ne!(resting_limit_order_node, resting_limit_order_node_2);
        assert_ne!(floating_limit_order_node, floating_limit_order_node_2);
        assert_ne!(market_order_node, market_order_node_2);
        assert_ne!(trigger_order_node, trigger_order_node_2);
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
