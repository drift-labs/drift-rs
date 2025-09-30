//! Example demonstrating the simplified margin calculation FFI API
//! 
//! This example shows how to use the ergonomic FfiMarketStateWrapper
//! to calculate simplified margin requirements.

use drift_rs::ffi::FfiMarketStateWrapper;
use drift_rs::drift_idl::{
    accounts::{SpotMarket, PerpMarket, User, OraclePriceData},
    types::MarginRequirementType,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Drift Simplified Margin Calculation Example");
    
    // Create a new market state
    let market_state = FfiMarketStateWrapper::new();
    
    // Example: Add a spot market (you would populate this with real data)
    let spot_market = SpotMarket {
        market_index: 0,
        // ... other fields would be populated with real data
        ..Default::default()
    };
    
    // Example: Add a perp market
    let perp_market = PerpMarket {
        market_index: 0,
        // ... other fields would be populated with real data
        ..Default::default()
    };
    
    // Example: Add oracle price data
    let oracle_price = OraclePriceData {
        price: 100_000_000, // $100 in price precision
        // ... other fields would be populated with real data
        ..Default::default()
    };
    
    // Add markets and oracle data to the state
    market_state.add_spot_market(&spot_market)?;
    market_state.add_perp_market(&perp_market)?;
    market_state.add_oracle_price(0, &oracle_price)?;
    
    // Example: Create a user (you would populate this with real data)
    let user = User {
        // ... fields would be populated with real user data
        ..Default::default()
    };
    
    // Calculate simplified margin requirement
    let margin_calc = market_state.calculate_simplified_margin_requirement(
        &user,
        MarginRequirementType::Initial,
    )?;
    
    println!("Margin Calculation Results:");
    println!("  Total Collateral: {}", margin_calc.total_collateral.0);
    println!("  Margin Requirement: {}", margin_calc.margin_requirement.0);
    println!("  Free Collateral: {}", margin_calc.free_collateral.0);
    println!("  Spot Asset Value: {}", margin_calc.spot_asset_value.0);
    println!("  Spot Liability Value: {}", margin_calc.spot_liability_value.0);
    println!("  Perp PnL: {}", margin_calc.perp_pnl.0);
    println!("  Perp Liability Value: {}", margin_calc.perp_liability_value.0);
    
    Ok(())
}
