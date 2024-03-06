use drift::state::{oracle::OraclePriceData, user::Order};
use solana_sdk::pubkey::Pubkey;

macro_rules! impl_order_node {
    ($($node_type:ident),+) => {
        $(
            pub struct $node_type {
                pub order: Order,
                pub user_account: Pubkey,
                pub next: Option<Box<$node_type>>,
                pub previous: Option<Box<$node_type>>,
            }

            impl $node_type {
                fn new(order: Order, user_account: Pubkey) -> Self {
                    Self {
                        order,
                        user_account,
                        next: None,
                        previous: None,
                    }
                }
            }

            impl DLOBNode for $node_type {
                fn get_sort_value(&self, order: &Order) -> Option<i128> {
                    match stringify!($node_type) {
                        "TakingLimitOrderNode" => Some(order.slot.into()),
                        "RestingLimitOrderNode" => Some(order.price.into()),
                        "FloatingLimitOrderNode" => Some(order.oracle_price_offset.into()),
                        "MarketOrderNode" => Some(order.slot.into()),
                        "TriggerOrderNode" => Some(order.trigger_price.into()),
                        _ => panic!("Unknown node type"),
                    }
                }

                fn get_price(&self, oracle_price_data: OraclePriceData, slot: u64, tick_size: u64) -> Option<u64> {
                    match self.order.get_limit_price(Some(oracle_price_data.price), None, slot, tick_size) {
                        Ok(price) => price,
                        Err(_) => None,
                    }
                }

                fn is_base_filled(&self) -> bool {
                    self.order.base_asset_amount_filled == self.order.base_asset_amount
                }

                fn is_vamm_node(&self) -> bool {
                    false
                }
            }
        )+
    };
}


pub trait DLOBNode {
    fn get_price(&self, oracle_price_data: OraclePriceData, slot: u64, tick_size: u64) -> Option<u64>;

    fn is_vamm_node(&self) -> bool;

    fn is_base_filled(&self) -> bool;

    fn get_sort_value(&self, order: &Order) -> Option<i128>;
}

pub struct VAMMNode {
    pub order: Order,
    pub price: u64,
}

impl DLOBNode for VAMMNode {
    fn get_price(&self, _oracle_price_data: OraclePriceData, _slot: u64, _tick_size: u64) -> Option<u64> {
        Some(self.price)
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
}

impl VAMMNode {
    pub fn new(order: Order, price: u64) -> Self {
        Self {
            order,
            price,
        }
    }
}

impl_order_node!(TakingLimitOrderNode, RestingLimitOrderNode, FloatingLimitOrderNode, MarketOrderNode, TriggerOrderNode);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_macro() {
        impl_order_node!(TakingLimitOrderNode, RestingLimitOrderNode, FloatingLimitOrderNode, MarketOrderNode, TriggerOrderNode);

        let mut order = Order::default();

        order.slot = 100;
        order.price = 1_000;
        order.trigger_price = 500;
        order.oracle_price_offset = 5_000;

        let user_account = Pubkey::new_unique();

        let taking_limit_order_node = TakingLimitOrderNode::new(order, user_account);
        let resting_limit_order_node = RestingLimitOrderNode::new(order, user_account);
        let floating_limit_order_node = FloatingLimitOrderNode::new(order, user_account);
        let market_order_node = MarketOrderNode::new(order, user_account);
        let trigger_order_node = TriggerOrderNode::new(order, user_account);
        let vamm_node = VAMMNode::new(order, 100);
        
        assert_eq!(taking_limit_order_node.get_sort_value(&order), Some(100));
        assert_eq!(resting_limit_order_node.get_sort_value(&order), Some(1_000));
        assert_eq!(market_order_node.get_sort_value(&order), Some(100));
        assert_eq!(trigger_order_node.get_sort_value(&order), Some(500));
        assert_eq!(floating_limit_order_node.get_sort_value(&order), Some(5_000));
        assert_eq!(vamm_node.get_sort_value(&order), None);
        
    }
}