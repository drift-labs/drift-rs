use anchor_lang::Discriminator;
use solana_client::rpc_filter::{Memcmp, RpcFilterType};

use crate::{drift_idl::types::MarketType, types::User, PerpMarket, SpotMarket};

pub fn get_user_filter() -> RpcFilterType {
    RpcFilterType::Memcmp(Memcmp::new_raw_bytes(0, User::discriminator().into()))
}

pub fn get_non_idle_user_filter() -> RpcFilterType {
    RpcFilterType::Memcmp(Memcmp::new_raw_bytes(4_350, vec![0]))
}

pub fn get_user_with_auction_filter() -> RpcFilterType {
    RpcFilterType::Memcmp(Memcmp::new_raw_bytes(4_354, vec![1]))
}

pub fn get_user_with_order_filter() -> RpcFilterType {
    RpcFilterType::Memcmp(Memcmp::new_raw_bytes(4_352, vec![1]))
}

pub fn get_market_filter(market_type: MarketType) -> RpcFilterType {
    match market_type {
        MarketType::Spot => RpcFilterType::Memcmp(Memcmp::new_raw_bytes(
            0,
            SpotMarket::discriminator().to_vec(),
        )),
        MarketType::Perp => RpcFilterType::Memcmp(Memcmp::new_raw_bytes(
            0,
            PerpMarket::discriminator().to_vec(),
        )),
    }
}
