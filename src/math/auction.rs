use num_bigint::BigInt;
use std::cmp::min;

use drift::{
    controller::position::PositionDirection,
    state::user::{Order, OrderType},
};

use crate::is_one_of_variant;

pub fn is_auction_complete(order: &Order, slot: u64) -> bool {
    if order.auction_duration == 0 {
        return true;
    }

    (order.slot + order.auction_duration as u64) < slot
}

#[track_caller]
pub fn get_auction_price(order: &Order, slot: u64, price: i64) -> BigInt {
    if is_one_of_variant(
        &order.order_type,
        &[
            OrderType::Market,
            OrderType::TriggerMarket,
            OrderType::Limit,
            OrderType::TriggerLimit,
        ],
    ) {
        get_auction_price_for_fixed_auction(order, slot)
    } else if order.order_type == OrderType::Oracle {
        get_auction_price_for_oracle_offset_auction(order, slot, price)
    } else {
        panic!("Invalid order type")
    }
}

fn get_auction_price_for_fixed_auction(order: &Order, slot: u64) -> BigInt {
    let slots_elapsed = slot - order.slot;

    let delta_denominator = BigInt::from(order.auction_duration);
    let delta_numerator = BigInt::from(min(slots_elapsed, order.auction_duration as u64));
    let auction_start_price = BigInt::from(order.auction_start_price);
    let auction_end_price = BigInt::from(order.auction_end_price);

    if delta_denominator == BigInt::from(0) {
        return auction_start_price;
    }

    match order.direction {
        PositionDirection::Long => {
            let price_delta = auction_end_price.clone()
                - auction_start_price.clone() * delta_numerator / delta_denominator;
            auction_start_price.clone() + price_delta
        }
        PositionDirection::Short => {
            let price_delta = auction_start_price.clone()
                - auction_end_price.clone() * delta_numerator / delta_denominator;
            auction_start_price.clone() - price_delta
        }
    }
}

fn get_auction_price_for_oracle_offset_auction(
    order: &Order,
    slot: u64,
    oracle_price: i64,
) -> BigInt {
    let slots_elapsed = slot - order.slot;

    let delta_denominator = BigInt::from(order.auction_duration);
    let delta_numerator = BigInt::from(min(slots_elapsed, order.auction_duration as u64));
    let auction_start_price = BigInt::from(order.auction_start_price);
    let auction_end_price = BigInt::from(order.auction_end_price);

    if delta_denominator == BigInt::from(0) {
        return auction_start_price;
    }

    let price_offset = match order.direction {
        PositionDirection::Long => {
            let price_delta = auction_end_price.clone()
                - auction_start_price.clone() * delta_numerator / delta_denominator;
            auction_start_price.clone() + price_delta
        }
        PositionDirection::Short => {
            let price_delta = auction_start_price.clone()
                - auction_end_price.clone() * delta_numerator / delta_denominator;
            auction_start_price.clone() - price_delta
        }
    };

    BigInt::from(oracle_price) + price_offset
}
