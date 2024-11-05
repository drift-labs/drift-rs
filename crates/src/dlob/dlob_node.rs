use node_types::{FloatingLimit, Market, RestingLimit, TakingLimit, Trigger, VAMM};
use solana_sdk::pubkey::Pubkey;

use crate::{drift_idl::types::Order, ffi::OraclePriceData, math::order::get_limit_price};

pub trait NodeKind: Eq + Ord + PartialOrd + Copy + Clone + std::fmt::Debug {
    fn sort_value(order: &Order) -> Option<i128>;
    fn is_base_filled(order: &Order) -> bool {
        order.base_asset_amount_filled == order.base_asset_amount
    }
    fn get_price(order: &Order, oracle_price_data: OraclePriceData, slot: u64) -> u64 {
        get_limit_price(order, &oracle_price_data, slot, None)
    }
}

pub enum NodeType {
    RestingLimit,
    FloatingLimit,
    TakingLimit,
    Market,
    Trigger,
    VAMM,
}

pub mod node_types {
    use super::*;
    #[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
    pub struct TakingLimit;
    impl NodeKind for TakingLimit {
        fn sort_value(order: &Order) -> Option<i128> {
            Some(order.slot.into())
        }
    }
    #[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
    pub struct RestingLimit;
    impl NodeKind for RestingLimit {
        fn sort_value(order: &Order) -> Option<i128> {
            Some(order.price.into())
        }
    }
    #[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
    pub struct FloatingLimit;
    impl NodeKind for FloatingLimit {
        fn sort_value(order: &Order) -> Option<i128> {
            Some(order.oracle_price_offset.into())
        }
    }
    #[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
    pub struct Market;
    impl NodeKind for Market {
        fn sort_value(order: &Order) -> Option<i128> {
            Some(order.slot.into())
        }
    }
    #[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
    pub struct Trigger;
    impl NodeKind for Trigger {
        fn sort_value(order: &Order) -> Option<i128> {
            Some(order.trigger_price.into())
        }
    }
    #[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
    pub struct VAMM;
    impl NodeKind for VAMM {
        fn sort_value(_order: &Order) -> Option<i128> {
            None
        }
        fn is_base_filled(_order: &Order) -> bool {
            false
        }
        fn get_price(order: &Order, _oracle_price_data: OraclePriceData, _slot: u64) -> u64 {
            order.price
        }
    }
}

/// Strongly typed order entry the dlob
#[derive(Copy, Clone, Debug)]
pub struct Node<T: NodeKind> {
    order: Order,
    user_account: Pubkey,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: NodeKind> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        T::sort_value(&self.order) == T::sort_value(&other.order)
    }
}

impl<T: NodeKind> Eq for Node<T> {}

impl<T: NodeKind> From<Node<T>> for (Order, Pubkey) {
    fn from(value: Node<T>) -> Self {
        (value.order, value.user_account)
    }
}

impl<T: NodeKind> Node<T> {
    pub fn price(&self, oracle_price_data: OraclePriceData, slot: u64) -> u64 {
        T::get_price(&self.order, oracle_price_data, slot)
    }
    pub fn is_base_filled(&self) -> bool {
        T::is_base_filled(&self.order)
    }
    pub fn order(&self) -> &Order {
        &self.order
    }
    pub fn is_vamm(&self) -> bool {
        self.user_account == Pubkey::default()
    }
    pub fn set_order(&mut self, order: Order) {
        self.order = order;
    }
    pub fn user_account(&self) -> Pubkey {
        self.user_account
    }
    /// Get the sorting value for this node
    pub fn sort_value(&self) -> Option<i128> {
        T::sort_value(&self.order)
    }
    /// Make a new order node of type `T`
    pub fn new(order: Order, user_account: Pubkey) -> Node<T> {
        Node::<T> {
            order,
            user_account,
            _phantom: Default::default(),
        }
    }
    /// Create a new trigger order node
    pub fn trigger(order: Order, user_account: Pubkey) -> Node<Trigger> {
        Node::<Trigger>::new(order, user_account)
    }
    /// Create a new resting limit order node
    pub fn resting_limit(order: Order, user_account: Pubkey) -> Node<RestingLimit> {
        Node::<RestingLimit>::new(order, user_account)
    }
    /// Create a new market order node
    pub fn market(order: Order, user_account: Pubkey) -> Node<Market> {
        Node::<Market>::new(order, user_account)
    }
    /// Create a new floating limit order node
    pub fn floating_limit(order: Order, user_account: Pubkey) -> Node<FloatingLimit> {
        Node::<FloatingLimit>::new(order, user_account)
    }
    /// Create a new taker limit order node
    pub fn taking_limit(order: Order, user_account: Pubkey) -> Node<TakingLimit> {
        Node::<TakingLimit>::new(order, user_account)
    }
    /// Create a new VAMM order node
    pub fn vamm(order: Order) -> Node<VAMM> {
        Node::<VAMM>::new(order, Pubkey::default())
    }
    /// Transform self into a different order node
    pub fn transform<U: NodeKind>(self) -> Node<U> {
        Node::<U>::new(self.order, self.user_account)
    }
}

/// Order wrapped for sorting in high-level data structure
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DirectionalNode<const ASC: bool, T: NodeKind> {
    pub node: Node<T>,
}

impl<const ASC: bool, T: NodeKind> DirectionalNode<ASC, T> {
    /// Wrap `node` for ASCended sorting
    pub fn asc(node: Node<T>) -> DirectionalNode<true, T> {
        DirectionalNode::<true, T> { node }
    }
    /// Wrap `node` for Descended sorting
    pub fn desc(node: Node<T>) -> DirectionalNode<false, T> {
        DirectionalNode::<false, T> { node }
    }
    pub fn new(node: Node<T>) -> DirectionalNode<ASC, T> {
        Self { node }
    }
}

impl<const ASC: bool, T: NodeKind> Ord for DirectionalNode<ASC, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let mut cmp = T::sort_value(&self.node.order)
            .partial_cmp(&T::sort_value(&other.node.order))
            .unwrap_or(std::cmp::Ordering::Equal);

        if cmp == std::cmp::Ordering::Equal {
            cmp = self.node.order().slot.cmp(&other.node.order().slot);
        }

        if ASC == true {
            cmp.reverse()
        } else {
            cmp
        }
    }
}

impl<const ASC: bool, T: NodeKind> PartialOrd for DirectionalNode<ASC, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub type OrderId = [u8; 32 + 4];
#[inline]
pub(crate) fn get_order_signature(order_id: u32, user_account: Pubkey) -> OrderId {
    let mut id_buf = [0u8; 36];
    id_buf[..32].copy_from_slice(&user_account.to_bytes());
    id_buf[32..36].copy_from_slice(&order_id.to_le_bytes());
    id_buf
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

        let taking_limit_order_node = Node::<TakingLimit>::new(order, user_account);
        let resting_limit_order_node = Node::<RestingLimit>::new(order, user_account);
        let floating_limit_order_node = Node::<FloatingLimit>::new(order, user_account);
        let market_order_node = Node::<Market>::new(order, user_account);
        let trigger_order_node = Node::<Trigger>::new(order, user_account);

        assert_eq!(taking_limit_order_node.sort_value(), Some(100));
        assert_eq!(resting_limit_order_node.sort_value(), Some(1_000));
        assert_eq!(market_order_node.sort_value(), Some(100));
        assert_eq!(trigger_order_node.sort_value(), Some(500));
        assert_eq!(floating_limit_order_node.sort_value(), Some(5_000));

        let mut order_2 = Order::default();

        order_2.slot = 200;
        order_2.price = 2_000;
        order_2.trigger_price = 600;
        order_2.oracle_price_offset = 6_000;

        let taking_limit_order_node_2 = Node::<TakingLimit>::new(order_2, user_account);
        let resting_limit_order_node_2 = Node::<RestingLimit>::new(order_2, user_account);
        let floating_limit_order_node_2 = Node::<FloatingLimit>::new(order_2, user_account);
        let market_order_node_2 = Node::<Market>::new(order_2, user_account);
        let trigger_order_node_2 = Node::<Trigger>::new(order_2, user_account);

        assert_eq!(taking_limit_order_node_2.sort_value(), Some(200));
        assert_eq!(resting_limit_order_node_2.sort_value(), Some(2_000));
        assert_eq!(market_order_node_2.sort_value(), Some(200));
        assert_eq!(trigger_order_node_2.sort_value(), Some(600));
        assert_eq!(floating_limit_order_node_2.sort_value(), Some(6_000));

        let mut order_3 = Order::default();

        order_3.slot = 300;
        order_3.price = 3_000;
        order_3.trigger_price = 700;
        order_3.oracle_price_offset = 7_000;

        let taking_limit_order_node_3 = Node::<TakingLimit>::new(order_3, user_account);
        let resting_limit_order_node_3 = Node::<RestingLimit>::new(order_3, user_account);
        let floating_limit_order_node_3 = Node::<FloatingLimit>::new(order_3, user_account);
        let market_order_node_3 = Node::<Market>::new(order_3, user_account);
        let trigger_order_node_3 = Node::<Trigger>::new(order_3, user_account);

        assert_eq!(taking_limit_order_node_3.sort_value(), Some(300));
        assert_eq!(resting_limit_order_node_3.sort_value(), Some(3_000));
        assert_eq!(market_order_node_3.sort_value(), Some(300));
        assert_eq!(trigger_order_node_3.sort_value(), Some(700));
        assert_eq!(floating_limit_order_node_3.sort_value(), Some(7_000));
    }

    #[test]
    fn test_set_order() {
        let user_account = Pubkey::new_unique();

        let mut order = Order::default();

        order.slot = 100;
        order.price = 1_000;
        order.trigger_price = 500;
        order.oracle_price_offset = 5_000;

        let mut taking_limit_order_node = Node::<TakingLimit>::new(order, user_account);
        let mut resting_limit_order_node = Node::<RestingLimit>::new(order, user_account);
        let mut floating_limit_order_node = Node::<FloatingLimit>::new(order, user_account);
        let mut market_order_node = Node::<Market>::new(order, user_account);
        let mut trigger_order_node = Node::<Trigger>::new(order, user_account);

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

        assert_eq!(taking_limit_order_node.order().slot, 200);
        assert_eq!(resting_limit_order_node.order().price, 2_000);
        assert_eq!(floating_limit_order_node.order().oracle_price_offset, 6_000);
        assert_eq!(market_order_node.order().slot, 200);
        assert_eq!(trigger_order_node.order().trigger_price, 600);
    }

    #[test]
    fn test_eq() {
        let user_account = Pubkey::new_unique();

        let mut order = Order::default();

        order.slot = 100;
        order.price = 1_000;
        order.trigger_price = 500;
        order.oracle_price_offset = 5_000;

        let taking_limit_order_node = Node::<TakingLimit>::new(order, user_account);
        let resting_limit_order_node = Node::<RestingLimit>::new(order, user_account);
        let floating_limit_order_node = Node::<FloatingLimit>::new(order, user_account);
        let market_order_node = Node::<Market>::new(order, user_account);
        let trigger_order_node = Node::<Trigger>::new(order, user_account);

        let mut order_2 = Order::default();

        order_2.slot = 200;
        order_2.price = 2_000;
        order_2.trigger_price = 600;
        order_2.oracle_price_offset = 6_000;

        let taking_limit_order_node_2 = Node::<TakingLimit>::new(order_2, user_account);
        let resting_limit_order_node_2 = Node::<RestingLimit>::new(order_2, user_account);
        let floating_limit_order_node_2 = Node::<FloatingLimit>::new(order_2, user_account);
        let market_order_node_2 = Node::<Market>::new(order_2, user_account);
        let trigger_order_node_2 = Node::<Trigger>::new(order_2, user_account);

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
}
