#![allow(unused_imports)]
#![doc = r" auto-generated IDL types"]
use self::traits::ToAccountMetas;
use anchor_lang::prelude::{
    account,
    borsh::{self},
    error_code, event, msg, AnchorDeserialize, AnchorSerialize, InitSpace,
};
use solana_sdk::{instruction::AccountMeta, pubkey::Pubkey};
pub mod traits {
    use solana_sdk::instruction::AccountMeta;
    pub trait ToAccountMetas {
        fn to_account_metas(&self) -> Vec<AccountMeta>;
    }
}
pub mod instructions {
    use super::{types::*, *};
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializeUser {
        pub sub_account_id: u16,
        pub name: [u8; 32],
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeUser {
        const DISCRIMINATOR: [u8; 8] = [63, 179, 92, 174, 117, 148, 14, 158];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeUser {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializeUserStats {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeUserStats {
        const DISCRIMINATOR: [u8; 8] = [42, 162, 158, 251, 16, 218, 250, 34];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeUserStats {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializeReferrerName {
        pub name: [u8; 32],
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeReferrerName {
        const DISCRIMINATOR: [u8; 8] = [226, 55, 170, 111, 73, 222, 208, 47];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeReferrerName {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct Deposit {
        pub market_index: u16,
        pub amount: u64,
        pub reduce_only: bool,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for Deposit {
        const DISCRIMINATOR: [u8; 8] = [116, 132, 8, 124, 181, 89, 27, 31];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for Deposit {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct Withdraw {
        pub market_index: u16,
        pub amount: u64,
        pub reduce_only: bool,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for Withdraw {
        const DISCRIMINATOR: [u8; 8] = [166, 1, 40, 81, 114, 237, 30, 216];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for Withdraw {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct TransferDeposit {
        pub market_index: u16,
        pub amount: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for TransferDeposit {
        const DISCRIMINATOR: [u8; 8] = [57, 92, 137, 33, 20, 94, 92, 113];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for TransferDeposit {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PlacePerpOrder {
        pub params: OrderParams,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PlacePerpOrder {
        const DISCRIMINATOR: [u8; 8] = [65, 198, 17, 51, 166, 232, 211, 108];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for PlacePerpOrder {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct CancelOrder {
        pub order_id: Option<u32>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for CancelOrder {
        const DISCRIMINATOR: [u8; 8] = [70, 239, 67, 26, 46, 53, 86, 170];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for CancelOrder {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct CancelOrderByUserId {
        pub user_order_id: u8,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for CancelOrderByUserId {
        const DISCRIMINATOR: [u8; 8] = [193, 218, 13, 96, 118, 209, 255, 14];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for CancelOrderByUserId {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct CancelOrders {
        pub market_type: Option<MarketType>,
        pub market_index: Option<u16>,
        pub direction: Option<PositionDirection>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for CancelOrders {
        const DISCRIMINATOR: [u8; 8] = [38, 153, 24, 37, 93, 160, 11, 163];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for CancelOrders {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct CancelOrdersByIds {
        pub order_ids: Vec<u32>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for CancelOrdersByIds {
        const DISCRIMINATOR: [u8; 8] = [73, 187, 0, 241, 187, 37, 147, 58];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for CancelOrdersByIds {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct ModifyOrder {
        pub order_id: Option<u32>,
        pub modify_order_params: ModifyOrderParams,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for ModifyOrder {
        const DISCRIMINATOR: [u8; 8] = [185, 9, 195, 149, 139, 116, 233, 179];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for ModifyOrder {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct ModifyOrderByUserId {
        pub user_order_id: u8,
        pub modify_order_params: ModifyOrderParams,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for ModifyOrderByUserId {
        const DISCRIMINATOR: [u8; 8] = [34, 53, 182, 51, 193, 217, 168, 224];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for ModifyOrderByUserId {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PlaceAndTakePerpOrder {
        pub params: OrderParams,
        pub maker_order_id: Option<u32>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PlaceAndTakePerpOrder {
        const DISCRIMINATOR: [u8; 8] = [12, 118, 99, 30, 178, 38, 172, 160];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for PlaceAndTakePerpOrder {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PlaceAndMakePerpOrder {
        pub params: OrderParams,
        pub taker_order_id: u32,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PlaceAndMakePerpOrder {
        const DISCRIMINATOR: [u8; 8] = [102, 213, 65, 151, 9, 5, 23, 68];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for PlaceAndMakePerpOrder {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PlaceSpotOrder {
        pub params: OrderParams,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PlaceSpotOrder {
        const DISCRIMINATOR: [u8; 8] = [87, 170, 187, 84, 74, 6, 69, 59];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for PlaceSpotOrder {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PlaceAndTakeSpotOrder {
        pub params: OrderParams,
        pub fulfillment_type: Option<SpotFulfillmentType>,
        pub maker_order_id: Option<u32>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PlaceAndTakeSpotOrder {
        const DISCRIMINATOR: [u8; 8] = [127, 146, 141, 213, 87, 133, 28, 132];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for PlaceAndTakeSpotOrder {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PlaceAndMakeSpotOrder {
        pub params: OrderParams,
        pub taker_order_id: u32,
        pub fulfillment_type: Option<SpotFulfillmentType>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PlaceAndMakeSpotOrder {
        const DISCRIMINATOR: [u8; 8] = [23, 205, 70, 209, 173, 16, 152, 39];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for PlaceAndMakeSpotOrder {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PlaceOrders {
        pub params: Vec<OrderParams>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PlaceOrders {
        const DISCRIMINATOR: [u8; 8] = [85, 180, 149, 187, 120, 72, 75, 119];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for PlaceOrders {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct BeginSwap {
        pub in_market_index: u16,
        pub out_market_index: u16,
        pub amount_in: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for BeginSwap {
        const DISCRIMINATOR: [u8; 8] = [220, 99, 13, 13, 243, 180, 243, 141];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for BeginSwap {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct EndSwap {
        pub in_market_index: u16,
        pub out_market_index: u16,
        pub limit_price: Option<u64>,
        pub reduce_only: Option<SwapReduceOnly>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for EndSwap {
        const DISCRIMINATOR: [u8; 8] = [124, 214, 248, 3, 3, 72, 24, 83];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for EndSwap {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct AddPerpLpShares {
        pub n_shares: u64,
        pub market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for AddPerpLpShares {
        const DISCRIMINATOR: [u8; 8] = [124, 140, 160, 178, 204, 196, 4, 116];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for AddPerpLpShares {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct RemovePerpLpShares {
        pub shares_to_burn: u64,
        pub market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for RemovePerpLpShares {
        const DISCRIMINATOR: [u8; 8] = [138, 225, 117, 101, 38, 177, 186, 120];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for RemovePerpLpShares {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct RemovePerpLpSharesInExpiringMarket {
        pub shares_to_burn: u64,
        pub market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for RemovePerpLpSharesInExpiringMarket {
        const DISCRIMINATOR: [u8; 8] = [207, 165, 252, 24, 216, 58, 106, 96];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for RemovePerpLpSharesInExpiringMarket {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateUserName {
        pub sub_account_id: u16,
        pub name: [u8; 32],
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserName {
        const DISCRIMINATOR: [u8; 8] = [34, 26, 115, 56, 245, 65, 228, 226];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserName {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateUserCustomMarginRatio {
        pub sub_account_id: u16,
        pub margin_ratio: u32,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserCustomMarginRatio {
        const DISCRIMINATOR: [u8; 8] = [117, 131, 37, 122, 147, 96, 51, 130];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserCustomMarginRatio {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateUserMarginTradingEnabled {
        pub sub_account_id: u16,
        pub margin_trading_enabled: bool,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserMarginTradingEnabled {
        const DISCRIMINATOR: [u8; 8] = [81, 26, 211, 16, 113, 206, 252, 138];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserMarginTradingEnabled {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateUserDelegate {
        pub sub_account_id: u16,
        pub delegate: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserDelegate {
        const DISCRIMINATOR: [u8; 8] = [252, 43, 31, 74, 67, 206, 58, 11];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserDelegate {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateUserReduceOnly {
        pub sub_account_id: u16,
        pub reduce_only: bool,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserReduceOnly {
        const DISCRIMINATOR: [u8; 8] = [120, 139, 39, 205, 61, 84, 3, 40];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserReduceOnly {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateUserAdvancedLp {
        pub sub_account_id: u16,
        pub advanced_lp: bool,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserAdvancedLp {
        const DISCRIMINATOR: [u8; 8] = [120, 158, 31, 147, 149, 172, 80, 217];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserAdvancedLp {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct DeleteUser {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for DeleteUser {
        const DISCRIMINATOR: [u8; 8] = [0, 54, 43, 157, 215, 226, 246, 215];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for DeleteUser {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct ReclaimRent {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for ReclaimRent {
        const DISCRIMINATOR: [u8; 8] = [88, 186, 211, 54, 211, 120, 233, 100];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for ReclaimRent {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct FillPerpOrder {
        pub order_id: Option<u32>,
        pub maker_order_id: Option<u32>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for FillPerpOrder {
        const DISCRIMINATOR: [u8; 8] = [28, 39, 164, 182, 17, 217, 102, 208];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for FillPerpOrder {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct RevertFill {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for RevertFill {
        const DISCRIMINATOR: [u8; 8] = [205, 144, 164, 11, 245, 222, 160, 237];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for RevertFill {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct FillSpotOrder {
        pub order_id: Option<u32>,
        pub fulfillment_type: Option<SpotFulfillmentType>,
        pub maker_order_id: Option<u32>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for FillSpotOrder {
        const DISCRIMINATOR: [u8; 8] = [225, 59, 76, 202, 156, 251, 157, 125];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for FillSpotOrder {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct TriggerOrder {
        pub order_id: u32,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for TriggerOrder {
        const DISCRIMINATOR: [u8; 8] = [216, 16, 11, 11, 200, 169, 22, 105];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for TriggerOrder {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct ForceCancelOrders {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for ForceCancelOrders {
        const DISCRIMINATOR: [u8; 8] = [137, 44, 247, 229, 189, 149, 207, 196];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for ForceCancelOrders {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateUserIdle {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserIdle {
        const DISCRIMINATOR: [u8; 8] = [136, 55, 35, 5, 193, 231, 176, 80];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserIdle {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateUserOpenOrdersCount {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserOpenOrdersCount {
        const DISCRIMINATOR: [u8; 8] = [24, 44, 200, 19, 62, 140, 141, 243];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserOpenOrdersCount {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct AdminDisableUpdatePerpBidAskTwap {
        pub disable: bool,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for AdminDisableUpdatePerpBidAskTwap {
        const DISCRIMINATOR: [u8; 8] = [186, 73, 240, 220, 31, 60, 222, 204];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for AdminDisableUpdatePerpBidAskTwap {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct SettlePnl {
        pub market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for SettlePnl {
        const DISCRIMINATOR: [u8; 8] = [170, 168, 54, 193, 72, 148, 148, 31];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for SettlePnl {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct SettleMultiplePnls {
        pub market_indexes: Vec<u16>,
        pub mode: SettlePnlMode,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for SettleMultiplePnls {
        const DISCRIMINATOR: [u8; 8] = [238, 159, 173, 81, 218, 202, 146, 40];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for SettleMultiplePnls {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct SettleFundingPayment {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for SettleFundingPayment {
        const DISCRIMINATOR: [u8; 8] = [173, 160, 67, 115, 3, 166, 99, 246];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for SettleFundingPayment {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct SettleLp {
        pub market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for SettleLp {
        const DISCRIMINATOR: [u8; 8] = [99, 187, 12, 58, 172, 0, 227, 73];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for SettleLp {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct SettleExpiredMarket {
        pub market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for SettleExpiredMarket {
        const DISCRIMINATOR: [u8; 8] = [175, 147, 197, 154, 90, 106, 204, 200];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for SettleExpiredMarket {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct LiquidatePerp {
        pub market_index: u16,
        pub liquidator_max_base_asset_amount: u64,
        pub limit_price: Option<u64>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for LiquidatePerp {
        const DISCRIMINATOR: [u8; 8] = [194, 41, 226, 31, 35, 136, 229, 227];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for LiquidatePerp {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct LiquidatePerpWithFill {
        pub market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for LiquidatePerpWithFill {
        const DISCRIMINATOR: [u8; 8] = [235, 215, 214, 205, 219, 57, 66, 77];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for LiquidatePerpWithFill {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct LiquidateSpot {
        pub asset_market_index: u16,
        pub liability_market_index: u16,
        pub liquidator_max_liability_transfer: u128,
        pub limit_price: Option<u64>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for LiquidateSpot {
        const DISCRIMINATOR: [u8; 8] = [198, 120, 41, 145, 135, 51, 95, 71];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for LiquidateSpot {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct LiquidateBorrowForPerpPnl {
        pub perp_market_index: u16,
        pub spot_market_index: u16,
        pub liquidator_max_liability_transfer: u128,
        pub limit_price: Option<u64>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for LiquidateBorrowForPerpPnl {
        const DISCRIMINATOR: [u8; 8] = [179, 28, 197, 233, 33, 61, 163, 199];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for LiquidateBorrowForPerpPnl {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct LiquidatePerpPnlForDeposit {
        pub perp_market_index: u16,
        pub spot_market_index: u16,
        pub liquidator_max_pnl_transfer: u128,
        pub limit_price: Option<u64>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for LiquidatePerpPnlForDeposit {
        const DISCRIMINATOR: [u8; 8] = [151, 53, 184, 162, 233, 15, 254, 192];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for LiquidatePerpPnlForDeposit {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct SetUserStatusToBeingLiquidated {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for SetUserStatusToBeingLiquidated {
        const DISCRIMINATOR: [u8; 8] = [168, 116, 73, 0, 74, 191, 35, 182];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for SetUserStatusToBeingLiquidated {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct ResolvePerpPnlDeficit {
        pub spot_market_index: u16,
        pub perp_market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for ResolvePerpPnlDeficit {
        const DISCRIMINATOR: [u8; 8] = [111, 56, 61, 226, 198, 72, 52, 59];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for ResolvePerpPnlDeficit {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct ResolvePerpBankruptcy {
        pub quote_spot_market_index: u16,
        pub market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for ResolvePerpBankruptcy {
        const DISCRIMINATOR: [u8; 8] = [189, 197, 58, 240, 98, 71, 91, 229];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for ResolvePerpBankruptcy {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct ResolveSpotBankruptcy {
        pub market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for ResolveSpotBankruptcy {
        const DISCRIMINATOR: [u8; 8] = [175, 135, 10, 46, 114, 236, 26, 204];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for ResolveSpotBankruptcy {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct SettleRevenueToInsuranceFund {
        pub spot_market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for SettleRevenueToInsuranceFund {
        const DISCRIMINATOR: [u8; 8] = [48, 41, 224, 183, 4, 21, 196, 122];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for SettleRevenueToInsuranceFund {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateFundingRate {
        pub market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateFundingRate {
        const DISCRIMINATOR: [u8; 8] = [175, 131, 101, 247, 81, 3, 88, 142];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateFundingRate {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePrelaunchOracle {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePrelaunchOracle {
        const DISCRIMINATOR: [u8; 8] = [32, 32, 109, 28, 58, 162, 4, 146];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePrelaunchOracle {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpBidAskTwap {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpBidAskTwap {
        const DISCRIMINATOR: [u8; 8] = [254, 122, 166, 101, 79, 146, 255, 191];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpBidAskTwap {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketCumulativeInterest {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketCumulativeInterest {
        const DISCRIMINATOR: [u8; 8] = [100, 56, 246, 179, 65, 238, 131, 45];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketCumulativeInterest {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateAmms {
        pub market_indexes: [u16; 5],
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateAmms {
        const DISCRIMINATOR: [u8; 8] = [36, 198, 87, 44, 180, 153, 186, 135];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateAmms {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketExpiry {
        pub expiry_ts: i64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketExpiry {
        const DISCRIMINATOR: [u8; 8] = [214, 167, 34, 236, 103, 45, 136, 60];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketExpiry {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateUserQuoteAssetInsuranceStake {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserQuoteAssetInsuranceStake {
        const DISCRIMINATOR: [u8; 8] = [8, 77, 199, 237, 124, 105, 186, 64];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserQuoteAssetInsuranceStake {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateUserGovTokenInsuranceStake {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserGovTokenInsuranceStake {
        const DISCRIMINATOR: [u8; 8] = [138, 247, 235, 140, 209, 246, 2, 68];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserGovTokenInsuranceStake {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializeInsuranceFundStake {
        pub market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeInsuranceFundStake {
        const DISCRIMINATOR: [u8; 8] = [181, 229, 1, 88, 0, 17, 36, 164];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeInsuranceFundStake {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct AddInsuranceFundStake {
        pub market_index: u16,
        pub amount: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for AddInsuranceFundStake {
        const DISCRIMINATOR: [u8; 8] = [105, 247, 44, 164, 216, 40, 236, 81];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for AddInsuranceFundStake {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct RequestRemoveInsuranceFundStake {
        pub market_index: u16,
        pub amount: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for RequestRemoveInsuranceFundStake {
        const DISCRIMINATOR: [u8; 8] = [21, 28, 139, 183, 227, 150, 133, 125];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for RequestRemoveInsuranceFundStake {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct CancelRequestRemoveInsuranceFundStake {
        pub market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for CancelRequestRemoveInsuranceFundStake {
        const DISCRIMINATOR: [u8; 8] = [232, 20, 161, 220, 201, 149, 130, 96];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for CancelRequestRemoveInsuranceFundStake {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct RemoveInsuranceFundStake {
        pub market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for RemoveInsuranceFundStake {
        const DISCRIMINATOR: [u8; 8] = [96, 220, 195, 168, 254, 234, 79, 160];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for RemoveInsuranceFundStake {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct TransferProtocolIfShares {
        pub market_index: u16,
        pub shares: u128,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for TransferProtocolIfShares {
        const DISCRIMINATOR: [u8; 8] = [117, 155, 59, 159, 25, 202, 36, 129];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for TransferProtocolIfShares {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePythPullOracle {
        pub feed_id: [u8; 32],
        pub params: Vec<u8>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePythPullOracle {
        const DISCRIMINATOR: [u8; 8] = [165, 134, 90, 23, 126, 250, 154, 87];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePythPullOracle {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PostPythPullOracleUpdateAtomic {
        pub feed_id: [u8; 32],
        pub params: Vec<u8>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PostPythPullOracleUpdateAtomic {
        const DISCRIMINATOR: [u8; 8] = [231, 234, 147, 130, 95, 134, 65, 227];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for PostPythPullOracleUpdateAtomic {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PostMultiPythPullOracleUpdatesAtomic {
        pub params: Vec<u8>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PostMultiPythPullOracleUpdatesAtomic {
        const DISCRIMINATOR: [u8; 8] = [240, 120, 103, 185, 106, 15, 244, 18];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for PostMultiPythPullOracleUpdatesAtomic {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct Initialize {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for Initialize {
        const DISCRIMINATOR: [u8; 8] = [9, 223, 177, 72, 156, 50, 68, 214];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for Initialize {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializeSpotMarket {
        pub optimal_utilization: u32,
        pub optimal_borrow_rate: u32,
        pub max_borrow_rate: u32,
        pub oracle_source: OracleSource,
        pub initial_asset_weight: u32,
        pub maintenance_asset_weight: u32,
        pub initial_liability_weight: u32,
        pub maintenance_liability_weight: u32,
        pub imf_factor: u32,
        pub liquidator_fee: u32,
        pub if_liquidation_fee: u32,
        pub active_status: bool,
        pub asset_tier: AssetTier,
        pub scale_initial_asset_weight_start: u64,
        pub withdraw_guard_threshold: u64,
        pub order_tick_size: u64,
        pub order_step_size: u64,
        pub if_total_factor: u32,
        pub name: [u8; 32],
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeSpotMarket {
        const DISCRIMINATOR: [u8; 8] = [18, 231, 214, 171, 221, 205, 177, 255];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeSpotMarket {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct DeleteInitializedSpotMarket {
        pub market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for DeleteInitializedSpotMarket {
        const DISCRIMINATOR: [u8; 8] = [105, 141, 24, 186, 50, 155, 239, 41];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for DeleteInitializedSpotMarket {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializeSerumFulfillmentConfig {
        pub market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeSerumFulfillmentConfig {
        const DISCRIMINATOR: [u8; 8] = [173, 132, 99, 233, 86, 161, 27, 146];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeSerumFulfillmentConfig {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSerumFulfillmentConfigStatus {
        pub status: SpotFulfillmentConfigStatus,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSerumFulfillmentConfigStatus {
        const DISCRIMINATOR: [u8; 8] = [165, 122, 28, 215, 183, 101, 100, 79];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSerumFulfillmentConfigStatus {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializeOpenbookV2FulfillmentConfig {
        pub market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeOpenbookV2FulfillmentConfig {
        const DISCRIMINATOR: [u8; 8] = [210, 94, 130, 130, 159, 35, 199, 66];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeOpenbookV2FulfillmentConfig {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct OpenbookV2FulfillmentConfigStatus {
        pub status: SpotFulfillmentConfigStatus,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for OpenbookV2FulfillmentConfigStatus {
        const DISCRIMINATOR: [u8; 8] = [79, 178, 121, 29, 196, 209, 208, 117];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for OpenbookV2FulfillmentConfigStatus {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializePhoenixFulfillmentConfig {
        pub market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializePhoenixFulfillmentConfig {
        const DISCRIMINATOR: [u8; 8] = [5, 114, 80, 37, 88, 34, 202, 132];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializePhoenixFulfillmentConfig {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PhoenixFulfillmentConfigStatus {
        pub status: SpotFulfillmentConfigStatus,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PhoenixFulfillmentConfigStatus {
        const DISCRIMINATOR: [u8; 8] = [178, 146, 140, 76, 221, 85, 109, 16];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for PhoenixFulfillmentConfigStatus {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSerumVault {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSerumVault {
        const DISCRIMINATOR: [u8; 8] = [22, 174, 84, 170, 101, 185, 37, 50];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSerumVault {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializePerpMarket {
        pub market_index: u16,
        pub amm_base_asset_reserve: u128,
        pub amm_quote_asset_reserve: u128,
        pub amm_periodicity: i64,
        pub amm_peg_multiplier: u128,
        pub oracle_source: OracleSource,
        pub contract_tier: ContractTier,
        pub margin_ratio_initial: u32,
        pub margin_ratio_maintenance: u32,
        pub liquidator_fee: u32,
        pub if_liquidation_fee: u32,
        pub imf_factor: u32,
        pub active_status: bool,
        pub base_spread: u32,
        pub max_spread: u32,
        pub max_open_interest: u128,
        pub max_revenue_withdraw_per_period: u64,
        pub quote_max_insurance: u64,
        pub order_step_size: u64,
        pub order_tick_size: u64,
        pub min_order_size: u64,
        pub concentration_coef_scale: u128,
        pub curve_update_intensity: u8,
        pub amm_jit_intensity: u8,
        pub name: [u8; 32],
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializePerpMarket {
        const DISCRIMINATOR: [u8; 8] = [148, 199, 202, 230, 252, 178, 42, 29];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializePerpMarket {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializePredictionMarket {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializePredictionMarket {
        const DISCRIMINATOR: [u8; 8] = [46, 43, 153, 168, 142, 105, 114, 238];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializePredictionMarket {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct DeleteInitializedPerpMarket {
        pub market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for DeleteInitializedPerpMarket {
        const DISCRIMINATOR: [u8; 8] = [219, 14, 114, 73, 19, 99, 209, 53];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for DeleteInitializedPerpMarket {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct MoveAmmPrice {
        pub base_asset_reserve: u128,
        pub quote_asset_reserve: u128,
        pub sqrt_k: u128,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for MoveAmmPrice {
        const DISCRIMINATOR: [u8; 8] = [216, 116, 106, 26, 199, 17, 224, 135];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for MoveAmmPrice {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct RecenterPerpMarketAmm {
        pub peg_multiplier: u128,
        pub sqrt_k: u128,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for RecenterPerpMarketAmm {
        const DISCRIMINATOR: [u8; 8] = [50, 137, 234, 114, 92, 166, 222, 247];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for RecenterPerpMarketAmm {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketAmmSummaryStats {
        pub params: UpdatePerpMarketSummaryStatsParams,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketAmmSummaryStats {
        const DISCRIMINATOR: [u8; 8] = [229, 126, 9, 160, 79, 24, 21, 203];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketAmmSummaryStats {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketExpiry {
        pub expiry_ts: i64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketExpiry {
        const DISCRIMINATOR: [u8; 8] = [12, 47, 172, 38, 63, 18, 92, 27];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketExpiry {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct SettleExpiredMarketPoolsToRevenuePool {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for SettleExpiredMarketPoolsToRevenuePool {
        const DISCRIMINATOR: [u8; 8] = [81, 218, 130, 92, 190, 241, 74, 69];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for SettleExpiredMarketPoolsToRevenuePool {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct DepositIntoPerpMarketFeePool {
        pub amount: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for DepositIntoPerpMarketFeePool {
        const DISCRIMINATOR: [u8; 8] = [187, 80, 24, 206, 47, 105, 49, 19];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for DepositIntoPerpMarketFeePool {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct DepositIntoSpotMarketVault {
        pub amount: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for DepositIntoSpotMarketVault {
        const DISCRIMINATOR: [u8; 8] = [157, 66, 91, 176, 199, 240, 252, 170];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for DepositIntoSpotMarketVault {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct DepositIntoSpotMarketRevenuePool {
        pub amount: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for DepositIntoSpotMarketRevenuePool {
        const DISCRIMINATOR: [u8; 8] = [18, 155, 189, 164, 77, 101, 238, 195];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for DepositIntoSpotMarketRevenuePool {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct RepegAmmCurve {
        pub new_peg_candidate: u128,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for RepegAmmCurve {
        const DISCRIMINATOR: [u8; 8] = [126, 239, 157, 139, 125, 252, 30, 74];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for RepegAmmCurve {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketAmmOracleTwap {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketAmmOracleTwap {
        const DISCRIMINATOR: [u8; 8] = [204, 65, 76, 179, 189, 156, 80, 213];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketAmmOracleTwap {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct ResetPerpMarketAmmOracleTwap {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for ResetPerpMarketAmmOracleTwap {
        const DISCRIMINATOR: [u8; 8] = [213, 100, 210, 106, 236, 11, 211, 179];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for ResetPerpMarketAmmOracleTwap {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateK {
        pub sqrt_k: u128,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateK {
        const DISCRIMINATOR: [u8; 8] = [61, 15, 174, 144, 200, 219, 117, 123];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateK {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketMarginRatio {
        pub margin_ratio_initial: u32,
        pub margin_ratio_maintenance: u32,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketMarginRatio {
        const DISCRIMINATOR: [u8; 8] = [185, 46, 222, 233, 220, 50, 235, 54];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketMarginRatio {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketFundingPeriod {
        pub funding_period: i64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketFundingPeriod {
        const DISCRIMINATOR: [u8; 8] = [99, 156, 7, 103, 23, 36, 78, 150];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketFundingPeriod {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketMaxImbalances {
        pub unrealized_max_imbalance: u64,
        pub max_revenue_withdraw_per_period: u64,
        pub quote_max_insurance: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketMaxImbalances {
        const DISCRIMINATOR: [u8; 8] = [12, 45, 185, 141, 145, 147, 38, 111];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketMaxImbalances {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketLiquidationFee {
        pub liquidator_fee: u32,
        pub if_liquidation_fee: u32,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketLiquidationFee {
        const DISCRIMINATOR: [u8; 8] = [175, 141, 225, 193, 209, 44, 143, 175];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketLiquidationFee {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateInsuranceFundUnstakingPeriod {
        pub insurance_fund_unstaking_period: i64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateInsuranceFundUnstakingPeriod {
        const DISCRIMINATOR: [u8; 8] = [163, 190, 11, 188, 2, 113, 126, 122];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateInsuranceFundUnstakingPeriod {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketLiquidationFee {
        pub liquidator_fee: u32,
        pub if_liquidation_fee: u32,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketLiquidationFee {
        const DISCRIMINATOR: [u8; 8] = [75, 65, 223, 238, 181, 232, 79, 150];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketLiquidationFee {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateWithdrawGuardThreshold {
        pub withdraw_guard_threshold: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateWithdrawGuardThreshold {
        const DISCRIMINATOR: [u8; 8] = [3, 206, 206, 1, 159, 28, 61, 170];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateWithdrawGuardThreshold {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketIfFactor {
        pub spot_market_index: u16,
        pub user_if_factor: u32,
        pub total_if_factor: u32,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketIfFactor {
        const DISCRIMINATOR: [u8; 8] = [223, 23, 229, 208, 9, 149, 146, 182];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketIfFactor {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketRevenueSettlePeriod {
        pub revenue_settle_period: i64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketRevenueSettlePeriod {
        const DISCRIMINATOR: [u8; 8] = [7, 206, 24, 176, 80, 174, 165, 147];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketRevenueSettlePeriod {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketStatus {
        pub status: MarketStatus,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketStatus {
        const DISCRIMINATOR: [u8; 8] = [41, 33, 10, 82, 199, 186, 77, 213];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketStatus {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketPausedOperations {
        pub paused_operations: u8,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketPausedOperations {
        const DISCRIMINATOR: [u8; 8] = [146, 111, 120, 66, 119, 180, 51, 37];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketPausedOperations {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketAssetTier {
        pub asset_tier: AssetTier,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketAssetTier {
        const DISCRIMINATOR: [u8; 8] = [238, 129, 100, 227, 241, 67, 51, 35];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketAssetTier {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketMarginWeights {
        pub initial_asset_weight: u32,
        pub maintenance_asset_weight: u32,
        pub initial_liability_weight: u32,
        pub maintenance_liability_weight: u32,
        pub imf_factor: u32,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketMarginWeights {
        const DISCRIMINATOR: [u8; 8] = [102, 43, 99, 124, 9, 38, 180, 78];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketMarginWeights {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketBorrowRate {
        pub optimal_utilization: u32,
        pub optimal_borrow_rate: u32,
        pub max_borrow_rate: u32,
        pub min_borrow_rate: Option<u8>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketBorrowRate {
        const DISCRIMINATOR: [u8; 8] = [106, 191, 198, 78, 220, 77, 58, 164];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketBorrowRate {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketMaxTokenDeposits {
        pub max_token_deposits: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketMaxTokenDeposits {
        const DISCRIMINATOR: [u8; 8] = [113, 121, 179, 156, 99, 80, 241, 62];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketMaxTokenDeposits {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketMaxTokenBorrows {
        pub max_token_borrows_fraction: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketMaxTokenBorrows {
        const DISCRIMINATOR: [u8; 8] = [88, 104, 192, 1, 83, 108, 136, 230];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketMaxTokenBorrows {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketScaleInitialAssetWeightStart {
        pub scale_initial_asset_weight_start: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketScaleInitialAssetWeightStart {
        const DISCRIMINATOR: [u8; 8] = [29, 62, 180, 45, 168, 144, 167, 96];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketScaleInitialAssetWeightStart {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketOracle {
        pub oracle: Pubkey,
        pub oracle_source: OracleSource,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketOracle {
        const DISCRIMINATOR: [u8; 8] = [193, 241, 249, 110, 237, 142, 222, 213];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketOracle {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketStepSizeAndTickSize {
        pub step_size: u64,
        pub tick_size: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketStepSizeAndTickSize {
        const DISCRIMINATOR: [u8; 8] = [227, 237, 17, 146, 157, 75, 167, 199];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketStepSizeAndTickSize {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketMinOrderSize {
        pub order_size: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketMinOrderSize {
        const DISCRIMINATOR: [u8; 8] = [186, 178, 157, 122, 54, 67, 251, 94];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketMinOrderSize {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketOrdersEnabled {
        pub orders_enabled: bool,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketOrdersEnabled {
        const DISCRIMINATOR: [u8; 8] = [172, 56, 247, 26, 147, 17, 142, 147];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketOrdersEnabled {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketIfPausedOperations {
        pub paused_operations: u8,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketIfPausedOperations {
        const DISCRIMINATOR: [u8; 8] = [245, 108, 77, 187, 26, 214, 158, 68];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketIfPausedOperations {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketName {
        pub name: [u8; 32],
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketName {
        const DISCRIMINATOR: [u8; 8] = [63, 67, 93, 235, 188, 98, 129, 113];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketName {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketStatus {
        pub status: MarketStatus,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketStatus {
        const DISCRIMINATOR: [u8; 8] = [104, 3, 229, 52, 193, 38, 170, 151];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketStatus {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketPausedOperations {
        pub paused_operations: u8,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketPausedOperations {
        const DISCRIMINATOR: [u8; 8] = [250, 113, 186, 164, 94, 11, 121, 131];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketPausedOperations {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketContractTier {
        pub contract_tier: ContractTier,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketContractTier {
        const DISCRIMINATOR: [u8; 8] = [158, 229, 240, 106, 180, 32, 83, 88];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketContractTier {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketImfFactor {
        pub imf_factor: u32,
        pub unrealized_pnl_imf_factor: u32,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketImfFactor {
        const DISCRIMINATOR: [u8; 8] = [35, 43, 93, 148, 156, 171, 142, 168];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketImfFactor {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketUnrealizedAssetWeight {
        pub unrealized_initial_asset_weight: u32,
        pub unrealized_maintenance_asset_weight: u32,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketUnrealizedAssetWeight {
        const DISCRIMINATOR: [u8; 8] = [91, 132, 97, 182, 54, 158, 132, 42];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketUnrealizedAssetWeight {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketConcentrationCoef {
        pub concentration_scale: u128,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketConcentrationCoef {
        const DISCRIMINATOR: [u8; 8] = [174, 15, 49, 90, 177, 215, 16, 138];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketConcentrationCoef {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketCurveUpdateIntensity {
        pub curve_update_intensity: u8,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketCurveUpdateIntensity {
        const DISCRIMINATOR: [u8; 8] = [243, 251, 191, 36, 139, 6, 209, 225];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketCurveUpdateIntensity {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketTargetBaseAssetAmountPerLp {
        pub target_base_asset_amount_per_lp: i32,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketTargetBaseAssetAmountPerLp {
        const DISCRIMINATOR: [u8; 8] = [87, 115, 195, 194, 220, 71, 141, 145];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketTargetBaseAssetAmountPerLp {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketPerLpBase {
        pub per_lp_base: i8,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketPerLpBase {
        const DISCRIMINATOR: [u8; 8] = [239, 62, 4, 91, 188, 28, 164, 24];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketPerLpBase {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateLpCooldownTime {
        pub lp_cooldown_time: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateLpCooldownTime {
        const DISCRIMINATOR: [u8; 8] = [90, 61, 204, 203, 28, 171, 28, 115];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateLpCooldownTime {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpFeeStructure {
        pub fee_structure: FeeStructure,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpFeeStructure {
        const DISCRIMINATOR: [u8; 8] = [4, 7, 150, 19, 255, 86, 74, 198];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpFeeStructure {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotFeeStructure {
        pub fee_structure: FeeStructure,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotFeeStructure {
        const DISCRIMINATOR: [u8; 8] = [236, 254, 252, 54, 93, 197, 4, 202];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotFeeStructure {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateInitialPctToLiquidate {
        pub initial_pct_to_liquidate: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateInitialPctToLiquidate {
        const DISCRIMINATOR: [u8; 8] = [245, 87, 236, 224, 186, 202, 205, 56];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateInitialPctToLiquidate {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateLiquidationDuration {
        pub liquidation_duration: u8,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateLiquidationDuration {
        const DISCRIMINATOR: [u8; 8] = [251, 210, 14, 72, 227, 11, 213, 213];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateLiquidationDuration {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateLiquidationMarginBufferRatio {
        pub liquidation_margin_buffer_ratio: u32,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateLiquidationMarginBufferRatio {
        const DISCRIMINATOR: [u8; 8] = [149, 239, 31, 46, 10, 188, 189, 205];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateLiquidationMarginBufferRatio {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateOracleGuardRails {
        pub oracle_guard_rails: OracleGuardRails,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateOracleGuardRails {
        const DISCRIMINATOR: [u8; 8] = [234, 20, 73, 125, 231, 195, 199, 69];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateOracleGuardRails {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateStateSettlementDuration {
        pub settlement_duration: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateStateSettlementDuration {
        const DISCRIMINATOR: [u8; 8] = [172, 42, 114, 114, 90, 254, 77, 140];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateStateSettlementDuration {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateStateMaxNumberOfSubAccounts {
        pub max_number_of_sub_accounts: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateStateMaxNumberOfSubAccounts {
        const DISCRIMINATOR: [u8; 8] = [201, 108, 95, 206, 242, 192, 202, 17];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateStateMaxNumberOfSubAccounts {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateStateMaxInitializeUserFee {
        pub max_initialize_user_fee: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateStateMaxInitializeUserFee {
        const DISCRIMINATOR: [u8; 8] = [191, 215, 98, 78, 139, 144, 214, 6];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateStateMaxInitializeUserFee {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketOracle {
        pub oracle: Pubkey,
        pub oracle_source: OracleSource,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketOracle {
        const DISCRIMINATOR: [u8; 8] = [244, 195, 212, 72, 201, 214, 79, 167];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketOracle {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketBaseSpread {
        pub base_spread: u32,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketBaseSpread {
        const DISCRIMINATOR: [u8; 8] = [161, 37, 147, 21, 122, 0, 48, 163];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketBaseSpread {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateAmmJitIntensity {
        pub amm_jit_intensity: u8,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateAmmJitIntensity {
        const DISCRIMINATOR: [u8; 8] = [199, 233, 116, 181, 227, 225, 160, 146];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateAmmJitIntensity {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketMaxSpread {
        pub max_spread: u32,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketMaxSpread {
        const DISCRIMINATOR: [u8; 8] = [139, 87, 9, 143, 244, 66, 107, 102];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketMaxSpread {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketStepSizeAndTickSize {
        pub step_size: u64,
        pub tick_size: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketStepSizeAndTickSize {
        const DISCRIMINATOR: [u8; 8] = [19, 115, 209, 182, 45, 32, 168, 214];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketStepSizeAndTickSize {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketName {
        pub name: [u8; 32],
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketName {
        const DISCRIMINATOR: [u8; 8] = [189, 217, 32, 200, 38, 59, 196, 107];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketName {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketMinOrderSize {
        pub order_size: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketMinOrderSize {
        const DISCRIMINATOR: [u8; 8] = [34, 179, 219, 246, 104, 199, 77, 245];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketMinOrderSize {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketMaxSlippageRatio {
        pub max_slippage_ratio: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketMaxSlippageRatio {
        const DISCRIMINATOR: [u8; 8] = [27, 252, 55, 248, 153, 108, 123, 210];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketMaxSlippageRatio {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketMaxFillReserveFraction {
        pub max_fill_reserve_fraction: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketMaxFillReserveFraction {
        const DISCRIMINATOR: [u8; 8] = [17, 108, 221, 67, 113, 123, 177, 84];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketMaxFillReserveFraction {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketMaxOpenInterest {
        pub max_open_interest: u128,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketMaxOpenInterest {
        const DISCRIMINATOR: [u8; 8] = [185, 21, 26, 206, 192, 123, 35, 83];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketMaxOpenInterest {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketNumberOfUsers {
        pub number_of_users: Option<u32>,
        pub number_of_users_with_base: Option<u32>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketNumberOfUsers {
        const DISCRIMINATOR: [u8; 8] = [72, 93, 65, 177, 28, 142, 86, 178];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketNumberOfUsers {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketFeeAdjustment {
        pub fee_adjustment: i16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketFeeAdjustment {
        const DISCRIMINATOR: [u8; 8] = [55, 42, 108, 236, 230, 252, 96, 101];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketFeeAdjustment {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketFeeAdjustment {
        pub fee_adjustment: i16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketFeeAdjustment {
        const DISCRIMINATOR: [u8; 8] = [181, 200, 92, 60, 229, 232, 154, 146];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketFeeAdjustment {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketFuel {
        pub fuel_boost_taker: Option<u8>,
        pub fuel_boost_maker: Option<u8>,
        pub fuel_boost_position: Option<u8>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketFuel {
        const DISCRIMINATOR: [u8; 8] = [142, 54, 34, 143, 167, 83, 175, 188];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketFuel {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketFuel {
        pub fuel_boost_deposits: Option<u8>,
        pub fuel_boost_borrows: Option<u8>,
        pub fuel_boost_taker: Option<u8>,
        pub fuel_boost_maker: Option<u8>,
        pub fuel_boost_insurance: Option<u8>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketFuel {
        const DISCRIMINATOR: [u8; 8] = [118, 160, 123, 104, 92, 187, 164, 191];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketFuel {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitUserFuel {
        pub fuel_boost_deposits: Option<u32>,
        pub fuel_boost_borrows: Option<u32>,
        pub fuel_boost_taker: Option<u32>,
        pub fuel_boost_maker: Option<u32>,
        pub fuel_boost_insurance: Option<u32>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitUserFuel {
        const DISCRIMINATOR: [u8; 8] = [220, 14, 11, 84, 194, 75, 19, 232];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitUserFuel {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateAdmin {
        pub admin: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateAdmin {
        const DISCRIMINATOR: [u8; 8] = [113, 14, 125, 75, 57, 252, 185, 92];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateAdmin {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateWhitelistMint {
        pub whitelist_mint: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateWhitelistMint {
        const DISCRIMINATOR: [u8; 8] = [153, 129, 154, 4, 241, 27, 64, 173];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateWhitelistMint {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateDiscountMint {
        pub discount_mint: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateDiscountMint {
        const DISCRIMINATOR: [u8; 8] = [112, 225, 35, 186, 137, 34, 220, 102];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateDiscountMint {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateExchangeStatus {
        pub exchange_status: u8,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateExchangeStatus {
        const DISCRIMINATOR: [u8; 8] = [173, 244, 148, 56, 62, 192, 219, 188];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateExchangeStatus {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpAuctionDuration {
        pub min_perp_auction_duration: u8,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpAuctionDuration {
        const DISCRIMINATOR: [u8; 8] = [181, 65, 59, 2, 163, 164, 227, 20];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpAuctionDuration {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotAuctionDuration {
        pub default_spot_auction_duration: u8,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotAuctionDuration {
        const DISCRIMINATOR: [u8; 8] = [16, 251, 20, 38, 178, 127, 160, 45];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotAuctionDuration {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializeProtocolIfSharesTransferConfig {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeProtocolIfSharesTransferConfig {
        const DISCRIMINATOR: [u8; 8] = [127, 78, 21, 186, 205, 166, 113, 50];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeProtocolIfSharesTransferConfig {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateProtocolIfSharesTransferConfig {
        pub whitelisted_signers: Option<[Pubkey; 4]>,
        pub max_transfer_per_epoch: Option<u128>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateProtocolIfSharesTransferConfig {
        const DISCRIMINATOR: [u8; 8] = [234, 170, 4, 197, 185, 34, 103, 50];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateProtocolIfSharesTransferConfig {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializePrelaunchOracle {
        pub params: PrelaunchOracleParams,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializePrelaunchOracle {
        const DISCRIMINATOR: [u8; 8] = [50, 65, 1, 179, 91, 203, 144, 43];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializePrelaunchOracle {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePrelaunchOracleParams {
        pub params: PrelaunchOracleParams,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePrelaunchOracleParams {
        const DISCRIMINATOR: [u8; 8] = [35, 251, 112, 141, 9, 117, 184, 231];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePrelaunchOracleParams {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct DeletePrelaunchOracle {
        pub perp_market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for DeletePrelaunchOracle {
        const DISCRIMINATOR: [u8; 8] = [171, 55, 241, 236, 17, 127, 99, 190];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for DeletePrelaunchOracle {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializePythPullOracle {
        pub feed_id: [u8; 32],
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializePythPullOracle {
        const DISCRIMINATOR: [u8; 8] = [235, 115, 210, 83, 67, 24, 85, 181];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializePythPullOracle {}
}
pub mod types {
    use super::*;
    #[doc = r" wrapper around fixed array types used for padding with `Default` implementation"]
    #[repr(transparent)]
    #[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone)]
    pub struct Padding<const N: usize>([u8; N]);
    impl<const N: usize> Default for Padding<N> {
        fn default() -> Self {
            Self([0u8; N])
        }
    }
    impl<const N: usize> std::fmt::Debug for Padding<N> {
        fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            Ok(())
        }
    }
    impl<const N: usize> anchor_lang::Space for Padding<N> {
        const INIT_SPACE: usize = 8 * N;
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub struct UpdatePerpMarketSummaryStatsParams {
        pub quote_asset_amount_with_unsettled_lp: Option<i64>,
        pub net_unsettled_funding_pnl: Option<i64>,
        pub update_amm_summary_stats: Option<bool>,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub struct LiquidatePerpRecord {
        pub market_index: u16,
        pub oracle_price: i64,
        pub base_asset_amount: i64,
        pub quote_asset_amount: i64,
        pub lp_shares: u64,
        pub fill_record_id: u64,
        pub user_order_id: u32,
        pub liquidator_order_id: u32,
        pub liquidator_fee: u64,
        pub if_fee: u64,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub struct LiquidateSpotRecord {
        pub asset_market_index: u16,
        pub asset_price: i64,
        pub asset_transfer: u128,
        pub liability_market_index: u16,
        pub liability_price: i64,
        pub liability_transfer: u128,
        pub if_fee: u64,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub struct LiquidateBorrowForPerpPnlRecord {
        pub perp_market_index: u16,
        pub market_oracle_price: i64,
        pub pnl_transfer: u128,
        pub liability_market_index: u16,
        pub liability_price: i64,
        pub liability_transfer: u128,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub struct LiquidatePerpPnlForDepositRecord {
        pub perp_market_index: u16,
        pub market_oracle_price: i64,
        pub pnl_transfer: u128,
        pub asset_market_index: u16,
        pub asset_price: i64,
        pub asset_transfer: u128,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub struct PerpBankruptcyRecord {
        pub market_index: u16,
        pub pnl: i128,
        pub if_payment: u128,
        pub clawback_user: Option<Pubkey>,
        pub clawback_user_payment: Option<u128>,
        pub cumulative_funding_rate_delta: i128,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub struct SpotBankruptcyRecord {
        pub market_index: u16,
        pub borrow_amount: u128,
        pub if_payment: u128,
        pub cumulative_deposit_interest_delta: u128,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub struct MarketIdentifier {
        pub market_type: MarketType,
        pub market_index: u16,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub struct HistoricalOracleData {
        pub last_oracle_price: i64,
        pub last_oracle_conf: u64,
        pub last_oracle_delay: i64,
        pub last_oracle_price_twap: i64,
        pub last_oracle_price_twap5min: i64,
        pub last_oracle_price_twap_ts: i64,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub struct HistoricalIndexData {
        pub last_index_bid_price: u64,
        pub last_index_ask_price: u64,
        pub last_index_price_twap: u64,
        pub last_index_price_twap5min: u64,
        pub last_index_price_twap_ts: i64,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub struct PrelaunchOracleParams {
        pub perp_market_index: u16,
        pub price: Option<i64>,
        pub max_price: Option<i64>,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub struct OrderParams {
        pub order_type: OrderType,
        pub market_type: MarketType,
        pub direction: PositionDirection,
        pub user_order_id: u8,
        pub base_asset_amount: u64,
        pub price: u64,
        pub market_index: u16,
        pub reduce_only: bool,
        pub post_only: PostOnlyParam,
        pub immediate_or_cancel: bool,
        pub max_ts: Option<i64>,
        pub trigger_price: Option<u64>,
        pub trigger_condition: OrderTriggerCondition,
        pub oracle_price_offset: Option<i32>,
        pub auction_duration: Option<u8>,
        pub auction_start_price: Option<i64>,
        pub auction_end_price: Option<i64>,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub struct ModifyOrderParams {
        pub direction: Option<PositionDirection>,
        pub base_asset_amount: Option<u64>,
        pub price: Option<u64>,
        pub reduce_only: Option<bool>,
        pub post_only: Option<PostOnlyParam>,
        pub immediate_or_cancel: Option<bool>,
        pub max_ts: Option<i64>,
        pub trigger_price: Option<u64>,
        pub trigger_condition: Option<OrderTriggerCondition>,
        pub oracle_price_offset: Option<i32>,
        pub auction_duration: Option<u8>,
        pub auction_start_price: Option<i64>,
        pub auction_end_price: Option<i64>,
        pub policy: Option<ModifyOrderPolicy>,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub struct InsuranceClaim {
        pub revenue_withdraw_since_last_settle: i64,
        pub max_revenue_withdraw_per_period: u64,
        pub quote_max_insurance: u64,
        pub quote_settled_insurance: u64,
        pub last_revenue_withdraw_ts: i64,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub struct PoolBalance {
        pub scaled_balance: u128,
        pub market_index: u16,
        pub padding: [u8; 6],
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub struct AMM {
        pub oracle: Pubkey,
        pub historical_oracle_data: HistoricalOracleData,
        pub base_asset_amount_per_lp: i128,
        pub quote_asset_amount_per_lp: i128,
        pub fee_pool: PoolBalance,
        pub base_asset_reserve: u128,
        pub quote_asset_reserve: u128,
        pub concentration_coef: u128,
        pub min_base_asset_reserve: u128,
        pub max_base_asset_reserve: u128,
        pub sqrt_k: u128,
        pub peg_multiplier: u128,
        pub terminal_quote_asset_reserve: u128,
        pub base_asset_amount_long: i128,
        pub base_asset_amount_short: i128,
        pub base_asset_amount_with_amm: i128,
        pub base_asset_amount_with_unsettled_lp: i128,
        pub max_open_interest: u128,
        pub quote_asset_amount: i128,
        pub quote_entry_amount_long: i128,
        pub quote_entry_amount_short: i128,
        pub quote_break_even_amount_long: i128,
        pub quote_break_even_amount_short: i128,
        pub user_lp_shares: u128,
        pub last_funding_rate: i64,
        pub last_funding_rate_long: i64,
        pub last_funding_rate_short: i64,
        pub last24h_avg_funding_rate: i64,
        pub total_fee: i128,
        pub total_mm_fee: i128,
        pub total_exchange_fee: u128,
        pub total_fee_minus_distributions: i128,
        pub total_fee_withdrawn: u128,
        pub total_liquidation_fee: u128,
        pub cumulative_funding_rate_long: i128,
        pub cumulative_funding_rate_short: i128,
        pub total_social_loss: u128,
        pub ask_base_asset_reserve: u128,
        pub ask_quote_asset_reserve: u128,
        pub bid_base_asset_reserve: u128,
        pub bid_quote_asset_reserve: u128,
        pub last_oracle_normalised_price: i64,
        pub last_oracle_reserve_price_spread_pct: i64,
        pub last_bid_price_twap: u64,
        pub last_ask_price_twap: u64,
        pub last_mark_price_twap: u64,
        pub last_mark_price_twap5min: u64,
        pub last_update_slot: u64,
        pub last_oracle_conf_pct: u64,
        pub net_revenue_since_last_funding: i64,
        pub last_funding_rate_ts: i64,
        pub funding_period: i64,
        pub order_step_size: u64,
        pub order_tick_size: u64,
        pub min_order_size: u64,
        pub max_position_size: u64,
        pub volume24h: u64,
        pub long_intensity_volume: u64,
        pub short_intensity_volume: u64,
        pub last_trade_ts: i64,
        pub mark_std: u64,
        pub oracle_std: u64,
        pub last_mark_price_twap_ts: i64,
        pub base_spread: u32,
        pub max_spread: u32,
        pub long_spread: u32,
        pub short_spread: u32,
        pub long_intensity_count: u32,
        pub short_intensity_count: u32,
        pub max_fill_reserve_fraction: u16,
        pub max_slippage_ratio: u16,
        pub curve_update_intensity: u8,
        pub amm_jit_intensity: u8,
        pub oracle_source: OracleSource,
        pub last_oracle_valid: bool,
        pub target_base_asset_amount_per_lp: i32,
        pub per_lp_base: i8,
        pub padding1: u8,
        pub padding2: u16,
        pub total_fee_earned_per_lp: u64,
        pub net_unsettled_funding_pnl: i64,
        pub quote_asset_amount_with_unsettled_lp: i64,
        pub reference_price_offset: i32,
        pub padding: [u8; 12],
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub struct InsuranceFund {
        pub vault: Pubkey,
        pub total_shares: u128,
        pub user_shares: u128,
        pub shares_base: u128,
        pub unstaking_period: i64,
        pub last_revenue_settle_ts: i64,
        pub revenue_settle_period: i64,
        pub total_factor: u32,
        pub user_factor: u32,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub struct OracleGuardRails {
        pub price_divergence: PriceDivergenceGuardRails,
        pub validity: ValidityGuardRails,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub struct PriceDivergenceGuardRails {
        pub mark_oracle_percent_divergence: u64,
        pub oracle_twap5min_percent_divergence: u64,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub struct ValidityGuardRails {
        pub slots_before_stale_for_amm: i64,
        pub slots_before_stale_for_margin: i64,
        pub confidence_interval_max_size: u64,
        pub too_volatile_ratio: i64,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub struct FeeStructure {
        pub fee_tiers: [FeeTier; 10],
        pub filler_reward_structure: OrderFillerRewardStructure,
        pub referrer_reward_epoch_upper_bound: u64,
        pub flat_filler_fee: u64,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub struct FeeTier {
        pub fee_numerator: u32,
        pub fee_denominator: u32,
        pub maker_rebate_numerator: u32,
        pub maker_rebate_denominator: u32,
        pub referrer_reward_numerator: u32,
        pub referrer_reward_denominator: u32,
        pub referee_fee_numerator: u32,
        pub referee_fee_denominator: u32,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub struct OrderFillerRewardStructure {
        pub reward_numerator: u32,
        pub reward_denominator: u32,
        pub time_based_reward_lower_bound: u128,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub struct UserFees {
        pub total_fee_paid: u64,
        pub total_fee_rebate: u64,
        pub total_token_discount: u64,
        pub total_referee_discount: u64,
        pub total_referrer_reward: u64,
        pub current_epoch_referrer_reward: u64,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub struct SpotPosition {
        pub scaled_balance: u64,
        pub open_bids: i64,
        pub open_asks: i64,
        pub cumulative_deposits: i64,
        pub market_index: u16,
        pub balance_type: SpotBalanceType,
        pub open_orders: u8,
        pub padding: [u8; 4],
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub struct PerpPosition {
        pub last_cumulative_funding_rate: i64,
        pub base_asset_amount: i64,
        pub quote_asset_amount: i64,
        pub quote_break_even_amount: i64,
        pub quote_entry_amount: i64,
        pub open_bids: i64,
        pub open_asks: i64,
        pub settled_pnl: i64,
        pub lp_shares: u64,
        pub last_base_asset_amount_per_lp: i64,
        pub last_quote_asset_amount_per_lp: i64,
        pub remainder_base_asset_amount: i32,
        pub market_index: u16,
        pub open_orders: u8,
        pub per_lp_base: i8,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub struct Order {
        pub slot: u64,
        pub price: u64,
        pub base_asset_amount: u64,
        pub base_asset_amount_filled: u64,
        pub quote_asset_amount_filled: u64,
        pub trigger_price: u64,
        pub auction_start_price: i64,
        pub auction_end_price: i64,
        pub max_ts: i64,
        pub oracle_price_offset: i32,
        pub order_id: u32,
        pub market_index: u16,
        pub status: OrderStatus,
        pub order_type: OrderType,
        pub market_type: MarketType,
        pub user_order_id: u8,
        pub existing_position_direction: PositionDirection,
        pub direction: PositionDirection,
        pub reduce_only: bool,
        pub post_only: bool,
        pub immediate_or_cancel: bool,
        pub trigger_condition: OrderTriggerCondition,
        pub auction_duration: u8,
        pub padding: [u8; 3],
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum SwapDirection {
        #[default]
        Add,
        Remove,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum ModifyOrderId {
        #[default]
        UserOrderId,
        OrderId,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum PositionDirection {
        #[default]
        Long,
        Short,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum SpotFulfillmentType {
        #[default]
        SerumV3,
        Match,
        PhoenixV1,
        OpenbookV2,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum SwapReduceOnly {
        #[default]
        In,
        Out,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum TwapPeriod {
        #[default]
        FundingPeriod,
        FiveMin,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum LiquidationMultiplierType {
        #[default]
        Discount,
        Premium,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum MarginRequirementType {
        #[default]
        Initial,
        Fill,
        Maintenance,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum OracleValidity {
        #[default]
        NonPositive,
        TooVolatile,
        TooUncertain,
        StaleForMargin,
        InsufficientDataPoints,
        StaleForAMM,
        Valid,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum DriftAction {
        #[default]
        UpdateFunding,
        SettlePnl,
        TriggerOrder,
        FillOrderMatch,
        FillOrderAmm,
        Liquidate,
        MarginCalc,
        UpdateTwap,
        UpdateAMMCurve,
        OracleOrderPrice,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum PositionUpdateType {
        #[default]
        Open,
        Increase,
        Reduce,
        Close,
        Flip,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum DepositExplanation {
        #[default]
        None,
        Transfer,
        Borrow,
        RepayBorrow,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum DepositDirection {
        #[default]
        Deposit,
        Withdraw,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum OrderAction {
        #[default]
        Place,
        Cancel,
        Fill,
        Trigger,
        Expire,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum OrderActionExplanation {
        #[default]
        None,
        InsufficientFreeCollateral,
        OraclePriceBreachedLimitPrice,
        MarketOrderFilledToLimitPrice,
        OrderExpired,
        Liquidation,
        OrderFilledWithAMM,
        OrderFilledWithAMMJit,
        OrderFilledWithMatch,
        OrderFilledWithMatchJit,
        MarketExpired,
        RiskingIncreasingOrder,
        ReduceOnlyOrderIncreasedPosition,
        OrderFillWithSerum,
        NoBorrowLiquidity,
        OrderFillWithPhoenix,
        OrderFilledWithAMMJitLPSplit,
        OrderFilledWithLPJit,
        DeriskLp,
        OrderFilledWithOpenbookV2,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum LPAction {
        #[default]
        AddLiquidity,
        RemoveLiquidity,
        SettleLiquidity,
        RemoveLiquidityDerisk,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum LiquidationType {
        #[default]
        LiquidatePerp,
        LiquidateSpot,
        LiquidateBorrowForPerpPnl,
        LiquidatePerpPnlForDeposit,
        PerpBankruptcy,
        SpotBankruptcy,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum SettlePnlExplanation {
        #[default]
        None,
        ExpiredPosition,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum StakeAction {
        #[default]
        Stake,
        UnstakeRequest,
        UnstakeCancelRequest,
        Unstake,
        UnstakeTransfer,
        StakeTransfer,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum FillMode {
        #[default]
        Fill,
        PlaceAndMake,
        PlaceAndTake,
        Liquidation,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum PerpFulfillmentMethod {
        #[default]
        AMM,
        Match,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum SpotFulfillmentMethod {
        #[default]
        ExternalMarket,
        Match,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum MarginCalculationMode {
        Standard {
            track_open_orders_fraction: bool,
        },
        Liquidation {
            market_to_track_margin_requirement: Option<MarketIdentifier>,
        },
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum OracleSource {
        #[default]
        Pyth,
        Switchboard,
        QuoteAsset,
        Pyth1K,
        Pyth1M,
        PythStableCoin,
        Prelaunch,
        PythPull,
        Pyth1KPull,
        Pyth1MPull,
        PythStableCoinPull,
        SwitchboardOnDemand,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum PostOnlyParam {
        #[default]
        None,
        MustPostOnly,
        TryPostOnly,
        Slide,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum ModifyOrderPolicy {
        #[default]
        TryModify,
        MustModify,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum PerpOperation {
        #[default]
        UpdateFunding,
        AmmFill,
        Fill,
        SettlePnl,
        SettlePnlWithPosition,
        Liquidation,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum SpotOperation {
        #[default]
        UpdateCumulativeInterest,
        Fill,
        Deposit,
        Withdraw,
        Liquidation,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum InsuranceFundOperation {
        #[default]
        Init,
        Add,
        RequestRemove,
        Remove,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum MarketStatus {
        #[default]
        Initialized,
        Active,
        FundingPaused,
        AmmPaused,
        FillPaused,
        WithdrawPaused,
        ReduceOnly,
        Settlement,
        Delisted,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum ContractType {
        #[default]
        Perpetual,
        Future,
        Prediction,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum ContractTier {
        #[default]
        A,
        B,
        C,
        Speculative,
        HighlySpeculative,
        Isolated,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum AMMLiquiditySplit {
        #[default]
        ProtocolOwned,
        LPOwned,
        Shared,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum SettlePnlMode {
        #[default]
        MustSettle,
        TrySettle,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum SpotBalanceType {
        #[default]
        Deposit,
        Borrow,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum SpotFulfillmentConfigStatus {
        #[default]
        Enabled,
        Disabled,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum AssetTier {
        #[default]
        Collateral,
        Protected,
        Cross,
        Isolated,
        Unlisted,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum ExchangeStatus {
        #[default]
        DepositPaused,
        WithdrawPaused,
        AmmPaused,
        FillPaused,
        LiqPaused,
        FundingPaused,
        SettlePnlPaused,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum UserStatus {
        #[default]
        BeingLiquidated,
        Bankrupt,
        ReduceOnly,
        AdvancedLp,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum AssetType {
        #[default]
        Base,
        Quote,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum OrderStatus {
        #[default]
        Init,
        Open,
        Filled,
        Canceled,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum OrderType {
        #[default]
        Market,
        Limit,
        TriggerMarket,
        TriggerLimit,
        Oracle,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum OrderTriggerCondition {
        #[default]
        Above,
        Below,
        TriggeredAbove,
        TriggeredBelow,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug, PartialEq,
    )]
    pub enum MarketType {
        #[default]
        Spot,
        Perp,
    }
}
pub mod accounts {
    use super::{types::*, *};
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug)]
    pub struct OpenbookV2FulfillmentConfig {
        pub pubkey: Pubkey,
        pub openbook_v2_program_id: Pubkey,
        pub openbook_v2_market: Pubkey,
        pub openbook_v2_market_authority: Pubkey,
        pub openbook_v2_event_heap: Pubkey,
        pub openbook_v2_bids: Pubkey,
        pub openbook_v2_asks: Pubkey,
        pub openbook_v2_base_vault: Pubkey,
        pub openbook_v2_quote_vault: Pubkey,
        pub market_index: u16,
        pub fulfillment_type: SpotFulfillmentType,
        pub status: SpotFulfillmentConfigStatus,
        pub padding: Padding<4>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for OpenbookV2FulfillmentConfig {
        const DISCRIMINATOR: [u8; 8] = [120, 189, 192, 26, 229, 159, 5, 168];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for OpenbookV2FulfillmentConfig {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for OpenbookV2FulfillmentConfig {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for OpenbookV2FulfillmentConfig {}
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug)]
    pub struct PhoenixV1FulfillmentConfig {
        pub pubkey: Pubkey,
        pub phoenix_program_id: Pubkey,
        pub phoenix_log_authority: Pubkey,
        pub phoenix_market: Pubkey,
        pub phoenix_base_vault: Pubkey,
        pub phoenix_quote_vault: Pubkey,
        pub market_index: u16,
        pub fulfillment_type: SpotFulfillmentType,
        pub status: SpotFulfillmentConfigStatus,
        pub padding: Padding<4>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PhoenixV1FulfillmentConfig {
        const DISCRIMINATOR: [u8; 8] = [7, 70, 203, 65, 207, 202, 90, 195];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for PhoenixV1FulfillmentConfig {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for PhoenixV1FulfillmentConfig {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for PhoenixV1FulfillmentConfig {}
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug)]
    pub struct SerumV3FulfillmentConfig {
        pub pubkey: Pubkey,
        pub serum_program_id: Pubkey,
        pub serum_market: Pubkey,
        pub serum_request_queue: Pubkey,
        pub serum_event_queue: Pubkey,
        pub serum_bids: Pubkey,
        pub serum_asks: Pubkey,
        pub serum_base_vault: Pubkey,
        pub serum_quote_vault: Pubkey,
        pub serum_open_orders: Pubkey,
        pub serum_signer_nonce: u64,
        pub market_index: u16,
        pub fulfillment_type: SpotFulfillmentType,
        pub status: SpotFulfillmentConfigStatus,
        pub padding: Padding<4>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for SerumV3FulfillmentConfig {
        const DISCRIMINATOR: [u8; 8] = [232, 118, 179, 182, 74, 163, 49, 180];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for SerumV3FulfillmentConfig {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for SerumV3FulfillmentConfig {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for SerumV3FulfillmentConfig {}
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug)]
    pub struct InsuranceFundStake {
        pub authority: Pubkey,
        pub if_shares: u128,
        pub last_withdraw_request_shares: u128,
        pub if_base: u128,
        pub last_valid_ts: i64,
        pub last_withdraw_request_value: u64,
        pub last_withdraw_request_ts: i64,
        pub cost_basis: i64,
        pub market_index: u16,
        pub padding: Padding<14>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InsuranceFundStake {
        const DISCRIMINATOR: [u8; 8] = [55, 89, 55, 54, 11, 250, 81, 215];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for InsuranceFundStake {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for InsuranceFundStake {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for InsuranceFundStake {}
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug)]
    pub struct ProtocolIfSharesTransferConfig {
        pub whitelisted_signers: [Pubkey; 4],
        pub max_transfer_per_epoch: u128,
        pub current_epoch_transfer: u128,
        pub next_epoch_ts: i64,
        pub padding: Padding<8>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for ProtocolIfSharesTransferConfig {
        const DISCRIMINATOR: [u8; 8] = [94, 56, 42, 33, 1, 163, 243, 35];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for ProtocolIfSharesTransferConfig {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for ProtocolIfSharesTransferConfig {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for ProtocolIfSharesTransferConfig {}
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug)]
    pub struct PrelaunchOracle {
        pub price: i64,
        pub max_price: i64,
        pub confidence: u64,
        pub last_update_slot: u64,
        pub amm_last_update_slot: u64,
        pub perp_market_index: u16,
        pub padding: Padding<70>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PrelaunchOracle {
        const DISCRIMINATOR: [u8; 8] = [106, 202, 203, 225, 214, 163, 132, 51];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for PrelaunchOracle {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for PrelaunchOracle {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for PrelaunchOracle {}
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug)]
    pub struct PerpMarket {
        pub pubkey: Pubkey,
        pub amm: AMM,
        pub pnl_pool: PoolBalance,
        pub name: [u8; 32],
        pub insurance_claim: InsuranceClaim,
        pub unrealized_pnl_max_imbalance: u64,
        pub expiry_ts: i64,
        pub expiry_price: i64,
        pub next_fill_record_id: u64,
        pub next_funding_rate_record_id: u64,
        pub next_curve_record_id: u64,
        pub imf_factor: u32,
        pub unrealized_pnl_imf_factor: u32,
        pub liquidator_fee: u32,
        pub if_liquidation_fee: u32,
        pub margin_ratio_initial: u32,
        pub margin_ratio_maintenance: u32,
        pub unrealized_pnl_initial_asset_weight: u32,
        pub unrealized_pnl_maintenance_asset_weight: u32,
        pub number_of_users_with_base: u32,
        pub number_of_users: u32,
        pub market_index: u16,
        pub status: MarketStatus,
        pub contract_type: ContractType,
        pub contract_tier: ContractTier,
        pub paused_operations: u8,
        pub quote_spot_market_index: u16,
        pub fee_adjustment: i16,
        pub fuel_boost_position: u8,
        pub fuel_boost_taker: u8,
        pub fuel_boost_maker: u8,
        pub padding: Padding<43>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PerpMarket {
        const DISCRIMINATOR: [u8; 8] = [192, 136, 167, 241, 208, 109, 144, 134];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for PerpMarket {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for PerpMarket {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for PerpMarket {}
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug)]
    pub struct SpotMarket {
        pub pubkey: Pubkey,
        pub oracle: Pubkey,
        pub mint: Pubkey,
        pub vault: Pubkey,
        pub name: [u8; 32],
        pub historical_oracle_data: HistoricalOracleData,
        pub historical_index_data: HistoricalIndexData,
        pub revenue_pool: PoolBalance,
        pub spot_fee_pool: PoolBalance,
        pub insurance_fund: InsuranceFund,
        pub total_spot_fee: u128,
        pub deposit_balance: u128,
        pub borrow_balance: u128,
        pub cumulative_deposit_interest: u128,
        pub cumulative_borrow_interest: u128,
        pub total_social_loss: u128,
        pub total_quote_social_loss: u128,
        pub withdraw_guard_threshold: u64,
        pub max_token_deposits: u64,
        pub deposit_token_twap: u64,
        pub borrow_token_twap: u64,
        pub utilization_twap: u64,
        pub last_interest_ts: u64,
        pub last_twap_ts: u64,
        pub expiry_ts: i64,
        pub order_step_size: u64,
        pub order_tick_size: u64,
        pub min_order_size: u64,
        pub max_position_size: u64,
        pub next_fill_record_id: u64,
        pub next_deposit_record_id: u64,
        pub initial_asset_weight: u32,
        pub maintenance_asset_weight: u32,
        pub initial_liability_weight: u32,
        pub maintenance_liability_weight: u32,
        pub imf_factor: u32,
        pub liquidator_fee: u32,
        pub if_liquidation_fee: u32,
        pub optimal_utilization: u32,
        pub optimal_borrow_rate: u32,
        pub max_borrow_rate: u32,
        pub decimals: u32,
        pub market_index: u16,
        pub orders_enabled: bool,
        pub oracle_source: OracleSource,
        pub status: MarketStatus,
        pub asset_tier: AssetTier,
        pub paused_operations: u8,
        pub if_paused_operations: u8,
        pub fee_adjustment: i16,
        pub max_token_borrows_fraction: u16,
        pub flash_loan_amount: u64,
        pub flash_loan_initial_token_amount: u64,
        pub total_swap_fee: u64,
        pub scale_initial_asset_weight_start: u64,
        pub min_borrow_rate: u8,
        pub fuel_boost_deposits: u8,
        pub fuel_boost_borrows: u8,
        pub fuel_boost_taker: u8,
        pub fuel_boost_maker: u8,
        pub fuel_boost_insurance: u8,
        pub token_program: u8,
        pub padding: Padding<41>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for SpotMarket {
        const DISCRIMINATOR: [u8; 8] = [236, 213, 124, 65, 171, 179, 140, 142];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for SpotMarket {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for SpotMarket {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for SpotMarket {}
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug)]
    pub struct State {
        pub admin: Pubkey,
        pub whitelist_mint: Pubkey,
        pub discount_mint: Pubkey,
        pub signer: Pubkey,
        pub srm_vault: Pubkey,
        pub perp_fee_structure: FeeStructure,
        pub spot_fee_structure: FeeStructure,
        pub oracle_guard_rails: OracleGuardRails,
        pub number_of_authorities: u64,
        pub number_of_sub_accounts: u64,
        pub lp_cooldown_time: u64,
        pub liquidation_margin_buffer_ratio: u32,
        pub settlement_duration: u16,
        pub number_of_markets: u16,
        pub number_of_spot_markets: u16,
        pub signer_nonce: u8,
        pub min_perp_auction_duration: u8,
        pub default_market_order_time_in_force: u8,
        pub default_spot_auction_duration: u8,
        pub exchange_status: u8,
        pub liquidation_duration: u8,
        pub initial_pct_to_liquidate: u16,
        pub max_number_of_sub_accounts: u16,
        pub max_initialize_user_fee: u16,
        pub padding: Padding<10>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for State {
        const DISCRIMINATOR: [u8; 8] = [112, 31, 91, 238, 109, 114, 9, 165];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for State {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for State {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for State {}
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug)]
    pub struct User {
        pub authority: Pubkey,
        pub delegate: Pubkey,
        pub name: [u8; 32],
        pub spot_positions: [SpotPosition; 8],
        pub perp_positions: [PerpPosition; 8],
        pub orders: [Order; 32],
        pub last_add_perp_lp_shares_ts: i64,
        pub total_deposits: u64,
        pub total_withdraws: u64,
        pub total_social_loss: u64,
        pub settled_perp_pnl: i64,
        pub cumulative_spot_fees: i64,
        pub cumulative_perp_funding: i64,
        pub liquidation_margin_freed: u64,
        pub last_active_slot: u64,
        pub next_order_id: u32,
        pub max_margin_ratio: u32,
        pub next_liquidation_id: u16,
        pub sub_account_id: u16,
        pub status: u8,
        pub is_margin_trading_enabled: bool,
        pub idle: bool,
        pub open_orders: u8,
        pub has_open_order: bool,
        pub open_auctions: u8,
        pub has_open_auction: bool,
        pub padding1: [u8; 5],
        pub last_fuel_bonus_update_ts: u32,
        pub padding: Padding<12>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for User {
        const DISCRIMINATOR: [u8; 8] = [54, 15, 240, 217, 8, 203, 70, 190];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for User {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for User {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for User {}
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug)]
    pub struct UserStats {
        pub authority: Pubkey,
        pub referrer: Pubkey,
        pub fees: UserFees,
        pub next_epoch_ts: i64,
        pub maker_volume30d: u64,
        pub taker_volume30d: u64,
        pub filler_volume30d: u64,
        pub last_maker_volume30d_ts: i64,
        pub last_taker_volume30d_ts: i64,
        pub last_filler_volume30d_ts: i64,
        pub if_staked_quote_asset_amount: u64,
        pub number_of_sub_accounts: u16,
        pub number_of_sub_accounts_created: u16,
        pub is_referrer: bool,
        pub disable_update_perp_bid_ask_twap: bool,
        pub padding1: [u8; 2],
        pub fuel_insurance: u32,
        pub fuel_deposits: u32,
        pub fuel_borrows: u32,
        pub fuel_positions: u32,
        pub fuel_taker: u32,
        pub fuel_maker: u32,
        pub if_staked_gov_token_amount: u64,
        pub last_fuel_if_bonus_update_ts: u32,
        pub padding: Padding<12>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UserStats {
        const DISCRIMINATOR: [u8; 8] = [107, 198, 255, 85, 150, 98, 174, 178];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UserStats {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UserStats {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UserStats {}
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, InitSpace, Copy, Clone, Default, Debug)]
    pub struct ReferrerName {
        pub authority: Pubkey,
        pub user: Pubkey,
        pub user_stats: Pubkey,
        pub name: [u8; 32],
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for ReferrerName {
        const DISCRIMINATOR: [u8; 8] = [41, 43, 132, 170, 29, 222, 109, 8];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for ReferrerName {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for ReferrerName {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for ReferrerName {}
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeUser {
        pub user: Pubkey,
        pub user_stats: Pubkey,
        pub state: Pubkey,
        pub authority: Pubkey,
        pub payer: Pubkey,
        pub rent: Pubkey,
        pub system_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeUser {
        const DISCRIMINATOR: [u8; 8] = [63, 179, 92, 174, 117, 148, 14, 158];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeUser {}
    #[automatically_derived]
    impl ToAccountMetas for InitializeUser {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.payer,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.rent,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.system_program,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeUserStats {
        pub user_stats: Pubkey,
        pub state: Pubkey,
        pub authority: Pubkey,
        pub payer: Pubkey,
        pub rent: Pubkey,
        pub system_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeUserStats {
        const DISCRIMINATOR: [u8; 8] = [42, 162, 158, 251, 16, 218, 250, 34];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeUserStats {}
    #[automatically_derived]
    impl ToAccountMetas for InitializeUserStats {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.payer,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.rent,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.system_program,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeReferrerName {
        pub referrer_name: Pubkey,
        pub user: Pubkey,
        pub user_stats: Pubkey,
        pub authority: Pubkey,
        pub payer: Pubkey,
        pub rent: Pubkey,
        pub system_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeReferrerName {
        const DISCRIMINATOR: [u8; 8] = [226, 55, 170, 111, 73, 222, 208, 47];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeReferrerName {}
    #[automatically_derived]
    impl ToAccountMetas for InitializeReferrerName {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.referrer_name,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.payer,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.rent,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.system_program,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct Deposit {
        pub state: Pubkey,
        pub user: Pubkey,
        pub user_stats: Pubkey,
        pub authority: Pubkey,
        pub spot_market_vault: Pubkey,
        pub user_token_account: Pubkey,
        pub token_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for Deposit {
        const DISCRIMINATOR: [u8; 8] = [116, 132, 8, 124, 181, 89, 27, 31];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for Deposit {}
    #[automatically_derived]
    impl ToAccountMetas for Deposit {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_token_account,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.token_program,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct Withdraw {
        pub state: Pubkey,
        pub user: Pubkey,
        pub user_stats: Pubkey,
        pub authority: Pubkey,
        pub spot_market_vault: Pubkey,
        pub drift_signer: Pubkey,
        pub user_token_account: Pubkey,
        pub token_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for Withdraw {
        const DISCRIMINATOR: [u8; 8] = [166, 1, 40, 81, 114, 237, 30, 216];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for Withdraw {}
    #[automatically_derived]
    impl ToAccountMetas for Withdraw {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.drift_signer,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.user_token_account,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.token_program,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct TransferDeposit {
        pub from_user: Pubkey,
        pub to_user: Pubkey,
        pub user_stats: Pubkey,
        pub authority: Pubkey,
        pub state: Pubkey,
        pub spot_market_vault: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for TransferDeposit {
        const DISCRIMINATOR: [u8; 8] = [57, 92, 137, 33, 20, 94, 92, 113];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for TransferDeposit {}
    #[automatically_derived]
    impl ToAccountMetas for TransferDeposit {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.from_user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.to_user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market_vault,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct PlacePerpOrder {
        pub state: Pubkey,
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PlacePerpOrder {
        const DISCRIMINATOR: [u8; 8] = [65, 198, 17, 51, 166, 232, 211, 108];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for PlacePerpOrder {}
    #[automatically_derived]
    impl ToAccountMetas for PlacePerpOrder {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct CancelOrder {
        pub state: Pubkey,
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for CancelOrder {
        const DISCRIMINATOR: [u8; 8] = [70, 239, 67, 26, 46, 53, 86, 170];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for CancelOrder {}
    #[automatically_derived]
    impl ToAccountMetas for CancelOrder {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct CancelOrderByUserId {
        pub state: Pubkey,
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for CancelOrderByUserId {
        const DISCRIMINATOR: [u8; 8] = [193, 218, 13, 96, 118, 209, 255, 14];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for CancelOrderByUserId {}
    #[automatically_derived]
    impl ToAccountMetas for CancelOrderByUserId {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct CancelOrders {
        pub state: Pubkey,
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for CancelOrders {
        const DISCRIMINATOR: [u8; 8] = [38, 153, 24, 37, 93, 160, 11, 163];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for CancelOrders {}
    #[automatically_derived]
    impl ToAccountMetas for CancelOrders {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct CancelOrdersByIds {
        pub state: Pubkey,
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for CancelOrdersByIds {
        const DISCRIMINATOR: [u8; 8] = [73, 187, 0, 241, 187, 37, 147, 58];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for CancelOrdersByIds {}
    #[automatically_derived]
    impl ToAccountMetas for CancelOrdersByIds {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct ModifyOrder {
        pub state: Pubkey,
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for ModifyOrder {
        const DISCRIMINATOR: [u8; 8] = [185, 9, 195, 149, 139, 116, 233, 179];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for ModifyOrder {}
    #[automatically_derived]
    impl ToAccountMetas for ModifyOrder {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct ModifyOrderByUserId {
        pub state: Pubkey,
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for ModifyOrderByUserId {
        const DISCRIMINATOR: [u8; 8] = [34, 53, 182, 51, 193, 217, 168, 224];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for ModifyOrderByUserId {}
    #[automatically_derived]
    impl ToAccountMetas for ModifyOrderByUserId {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct PlaceAndTakePerpOrder {
        pub state: Pubkey,
        pub user: Pubkey,
        pub user_stats: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PlaceAndTakePerpOrder {
        const DISCRIMINATOR: [u8; 8] = [12, 118, 99, 30, 178, 38, 172, 160];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for PlaceAndTakePerpOrder {}
    #[automatically_derived]
    impl ToAccountMetas for PlaceAndTakePerpOrder {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct PlaceAndMakePerpOrder {
        pub state: Pubkey,
        pub user: Pubkey,
        pub user_stats: Pubkey,
        pub taker: Pubkey,
        pub taker_stats: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PlaceAndMakePerpOrder {
        const DISCRIMINATOR: [u8; 8] = [102, 213, 65, 151, 9, 5, 23, 68];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for PlaceAndMakePerpOrder {}
    #[automatically_derived]
    impl ToAccountMetas for PlaceAndMakePerpOrder {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.taker,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.taker_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct PlaceSpotOrder {
        pub state: Pubkey,
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PlaceSpotOrder {
        const DISCRIMINATOR: [u8; 8] = [87, 170, 187, 84, 74, 6, 69, 59];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for PlaceSpotOrder {}
    #[automatically_derived]
    impl ToAccountMetas for PlaceSpotOrder {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct PlaceAndTakeSpotOrder {
        pub state: Pubkey,
        pub user: Pubkey,
        pub user_stats: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PlaceAndTakeSpotOrder {
        const DISCRIMINATOR: [u8; 8] = [127, 146, 141, 213, 87, 133, 28, 132];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for PlaceAndTakeSpotOrder {}
    #[automatically_derived]
    impl ToAccountMetas for PlaceAndTakeSpotOrder {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct PlaceAndMakeSpotOrder {
        pub state: Pubkey,
        pub user: Pubkey,
        pub user_stats: Pubkey,
        pub taker: Pubkey,
        pub taker_stats: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PlaceAndMakeSpotOrder {
        const DISCRIMINATOR: [u8; 8] = [23, 205, 70, 209, 173, 16, 152, 39];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for PlaceAndMakeSpotOrder {}
    #[automatically_derived]
    impl ToAccountMetas for PlaceAndMakeSpotOrder {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.taker,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.taker_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct PlaceOrders {
        pub state: Pubkey,
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PlaceOrders {
        const DISCRIMINATOR: [u8; 8] = [85, 180, 149, 187, 120, 72, 75, 119];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for PlaceOrders {}
    #[automatically_derived]
    impl ToAccountMetas for PlaceOrders {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct BeginSwap {
        pub state: Pubkey,
        pub user: Pubkey,
        pub user_stats: Pubkey,
        pub authority: Pubkey,
        pub out_spot_market_vault: Pubkey,
        pub in_spot_market_vault: Pubkey,
        pub out_token_account: Pubkey,
        pub in_token_account: Pubkey,
        pub token_program: Pubkey,
        pub drift_signer: Pubkey,
        pub instructions: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for BeginSwap {
        const DISCRIMINATOR: [u8; 8] = [220, 99, 13, 13, 243, 180, 243, 141];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for BeginSwap {}
    #[automatically_derived]
    impl ToAccountMetas for BeginSwap {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.out_spot_market_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.in_spot_market_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.out_token_account,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.in_token_account,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.token_program,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.drift_signer,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.instructions,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct EndSwap {
        pub state: Pubkey,
        pub user: Pubkey,
        pub user_stats: Pubkey,
        pub authority: Pubkey,
        pub out_spot_market_vault: Pubkey,
        pub in_spot_market_vault: Pubkey,
        pub out_token_account: Pubkey,
        pub in_token_account: Pubkey,
        pub token_program: Pubkey,
        pub drift_signer: Pubkey,
        pub instructions: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for EndSwap {
        const DISCRIMINATOR: [u8; 8] = [124, 214, 248, 3, 3, 72, 24, 83];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for EndSwap {}
    #[automatically_derived]
    impl ToAccountMetas for EndSwap {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.out_spot_market_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.in_spot_market_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.out_token_account,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.in_token_account,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.token_program,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.drift_signer,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.instructions,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct AddPerpLpShares {
        pub state: Pubkey,
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for AddPerpLpShares {
        const DISCRIMINATOR: [u8; 8] = [124, 140, 160, 178, 204, 196, 4, 116];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for AddPerpLpShares {}
    #[automatically_derived]
    impl ToAccountMetas for AddPerpLpShares {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct RemovePerpLpShares {
        pub state: Pubkey,
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for RemovePerpLpShares {
        const DISCRIMINATOR: [u8; 8] = [138, 225, 117, 101, 38, 177, 186, 120];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for RemovePerpLpShares {}
    #[automatically_derived]
    impl ToAccountMetas for RemovePerpLpShares {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct RemovePerpLpSharesInExpiringMarket {
        pub state: Pubkey,
        pub user: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for RemovePerpLpSharesInExpiringMarket {
        const DISCRIMINATOR: [u8; 8] = [207, 165, 252, 24, 216, 58, 106, 96];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for RemovePerpLpSharesInExpiringMarket {}
    #[automatically_derived]
    impl ToAccountMetas for RemovePerpLpSharesInExpiringMarket {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateUserName {
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserName {
        const DISCRIMINATOR: [u8; 8] = [34, 26, 115, 56, 245, 65, 228, 226];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserName {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateUserName {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateUserCustomMarginRatio {
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserCustomMarginRatio {
        const DISCRIMINATOR: [u8; 8] = [117, 131, 37, 122, 147, 96, 51, 130];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserCustomMarginRatio {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateUserCustomMarginRatio {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateUserMarginTradingEnabled {
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserMarginTradingEnabled {
        const DISCRIMINATOR: [u8; 8] = [81, 26, 211, 16, 113, 206, 252, 138];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserMarginTradingEnabled {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateUserMarginTradingEnabled {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateUserDelegate {
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserDelegate {
        const DISCRIMINATOR: [u8; 8] = [252, 43, 31, 74, 67, 206, 58, 11];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserDelegate {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateUserDelegate {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateUserReduceOnly {
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserReduceOnly {
        const DISCRIMINATOR: [u8; 8] = [120, 139, 39, 205, 61, 84, 3, 40];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserReduceOnly {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateUserReduceOnly {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateUserAdvancedLp {
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserAdvancedLp {
        const DISCRIMINATOR: [u8; 8] = [120, 158, 31, 147, 149, 172, 80, 217];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserAdvancedLp {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateUserAdvancedLp {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct DeleteUser {
        pub user: Pubkey,
        pub user_stats: Pubkey,
        pub state: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for DeleteUser {
        const DISCRIMINATOR: [u8; 8] = [0, 54, 43, 157, 215, 226, 246, 215];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for DeleteUser {}
    #[automatically_derived]
    impl ToAccountMetas for DeleteUser {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct ReclaimRent {
        pub user: Pubkey,
        pub user_stats: Pubkey,
        pub state: Pubkey,
        pub authority: Pubkey,
        pub rent: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for ReclaimRent {
        const DISCRIMINATOR: [u8; 8] = [88, 186, 211, 54, 211, 120, 233, 100];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for ReclaimRent {}
    #[automatically_derived]
    impl ToAccountMetas for ReclaimRent {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.rent,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct FillPerpOrder {
        pub state: Pubkey,
        pub authority: Pubkey,
        pub filler: Pubkey,
        pub filler_stats: Pubkey,
        pub user: Pubkey,
        pub user_stats: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for FillPerpOrder {
        const DISCRIMINATOR: [u8; 8] = [28, 39, 164, 182, 17, 217, 102, 208];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for FillPerpOrder {}
    #[automatically_derived]
    impl ToAccountMetas for FillPerpOrder {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.filler,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.filler_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct RevertFill {
        pub state: Pubkey,
        pub authority: Pubkey,
        pub filler: Pubkey,
        pub filler_stats: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for RevertFill {
        const DISCRIMINATOR: [u8; 8] = [205, 144, 164, 11, 245, 222, 160, 237];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for RevertFill {}
    #[automatically_derived]
    impl ToAccountMetas for RevertFill {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.filler,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.filler_stats,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct FillSpotOrder {
        pub state: Pubkey,
        pub authority: Pubkey,
        pub filler: Pubkey,
        pub filler_stats: Pubkey,
        pub user: Pubkey,
        pub user_stats: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for FillSpotOrder {
        const DISCRIMINATOR: [u8; 8] = [225, 59, 76, 202, 156, 251, 157, 125];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for FillSpotOrder {}
    #[automatically_derived]
    impl ToAccountMetas for FillSpotOrder {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.filler,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.filler_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct TriggerOrder {
        pub state: Pubkey,
        pub authority: Pubkey,
        pub filler: Pubkey,
        pub user: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for TriggerOrder {
        const DISCRIMINATOR: [u8; 8] = [216, 16, 11, 11, 200, 169, 22, 105];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for TriggerOrder {}
    #[automatically_derived]
    impl ToAccountMetas for TriggerOrder {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.filler,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct ForceCancelOrders {
        pub state: Pubkey,
        pub authority: Pubkey,
        pub filler: Pubkey,
        pub user: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for ForceCancelOrders {
        const DISCRIMINATOR: [u8; 8] = [137, 44, 247, 229, 189, 149, 207, 196];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for ForceCancelOrders {}
    #[automatically_derived]
    impl ToAccountMetas for ForceCancelOrders {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.filler,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateUserIdle {
        pub state: Pubkey,
        pub authority: Pubkey,
        pub filler: Pubkey,
        pub user: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserIdle {
        const DISCRIMINATOR: [u8; 8] = [136, 55, 35, 5, 193, 231, 176, 80];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserIdle {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateUserIdle {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.filler,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateUserOpenOrdersCount {
        pub state: Pubkey,
        pub authority: Pubkey,
        pub filler: Pubkey,
        pub user: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserOpenOrdersCount {
        const DISCRIMINATOR: [u8; 8] = [24, 44, 200, 19, 62, 140, 141, 243];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserOpenOrdersCount {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateUserOpenOrdersCount {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.filler,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct AdminDisableUpdatePerpBidAskTwap {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub user_stats: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for AdminDisableUpdatePerpBidAskTwap {
        const DISCRIMINATOR: [u8; 8] = [186, 73, 240, 220, 31, 60, 222, 204];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for AdminDisableUpdatePerpBidAskTwap {}
    #[automatically_derived]
    impl ToAccountMetas for AdminDisableUpdatePerpBidAskTwap {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct SettlePnl {
        pub state: Pubkey,
        pub user: Pubkey,
        pub authority: Pubkey,
        pub spot_market_vault: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for SettlePnl {
        const DISCRIMINATOR: [u8; 8] = [170, 168, 54, 193, 72, 148, 148, 31];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for SettlePnl {}
    #[automatically_derived]
    impl ToAccountMetas for SettlePnl {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market_vault,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct SettleMultiplePnls {
        pub state: Pubkey,
        pub user: Pubkey,
        pub authority: Pubkey,
        pub spot_market_vault: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for SettleMultiplePnls {
        const DISCRIMINATOR: [u8; 8] = [238, 159, 173, 81, 218, 202, 146, 40];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for SettleMultiplePnls {}
    #[automatically_derived]
    impl ToAccountMetas for SettleMultiplePnls {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market_vault,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct SettleFundingPayment {
        pub state: Pubkey,
        pub user: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for SettleFundingPayment {
        const DISCRIMINATOR: [u8; 8] = [173, 160, 67, 115, 3, 166, 99, 246];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for SettleFundingPayment {}
    #[automatically_derived]
    impl ToAccountMetas for SettleFundingPayment {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct SettleLp {
        pub state: Pubkey,
        pub user: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for SettleLp {
        const DISCRIMINATOR: [u8; 8] = [99, 187, 12, 58, 172, 0, 227, 73];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for SettleLp {}
    #[automatically_derived]
    impl ToAccountMetas for SettleLp {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct SettleExpiredMarket {
        pub state: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for SettleExpiredMarket {
        const DISCRIMINATOR: [u8; 8] = [175, 147, 197, 154, 90, 106, 204, 200];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for SettleExpiredMarket {}
    #[automatically_derived]
    impl ToAccountMetas for SettleExpiredMarket {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct LiquidatePerp {
        pub state: Pubkey,
        pub authority: Pubkey,
        pub liquidator: Pubkey,
        pub liquidator_stats: Pubkey,
        pub user: Pubkey,
        pub user_stats: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for LiquidatePerp {
        const DISCRIMINATOR: [u8; 8] = [194, 41, 226, 31, 35, 136, 229, 227];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for LiquidatePerp {}
    #[automatically_derived]
    impl ToAccountMetas for LiquidatePerp {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.liquidator,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.liquidator_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct LiquidatePerpWithFill {
        pub state: Pubkey,
        pub authority: Pubkey,
        pub liquidator: Pubkey,
        pub liquidator_stats: Pubkey,
        pub user: Pubkey,
        pub user_stats: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for LiquidatePerpWithFill {
        const DISCRIMINATOR: [u8; 8] = [235, 215, 214, 205, 219, 57, 66, 77];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for LiquidatePerpWithFill {}
    #[automatically_derived]
    impl ToAccountMetas for LiquidatePerpWithFill {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.liquidator,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.liquidator_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct LiquidateSpot {
        pub state: Pubkey,
        pub authority: Pubkey,
        pub liquidator: Pubkey,
        pub liquidator_stats: Pubkey,
        pub user: Pubkey,
        pub user_stats: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for LiquidateSpot {
        const DISCRIMINATOR: [u8; 8] = [198, 120, 41, 145, 135, 51, 95, 71];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for LiquidateSpot {}
    #[automatically_derived]
    impl ToAccountMetas for LiquidateSpot {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.liquidator,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.liquidator_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct LiquidateBorrowForPerpPnl {
        pub state: Pubkey,
        pub authority: Pubkey,
        pub liquidator: Pubkey,
        pub liquidator_stats: Pubkey,
        pub user: Pubkey,
        pub user_stats: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for LiquidateBorrowForPerpPnl {
        const DISCRIMINATOR: [u8; 8] = [179, 28, 197, 233, 33, 61, 163, 199];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for LiquidateBorrowForPerpPnl {}
    #[automatically_derived]
    impl ToAccountMetas for LiquidateBorrowForPerpPnl {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.liquidator,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.liquidator_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct LiquidatePerpPnlForDeposit {
        pub state: Pubkey,
        pub authority: Pubkey,
        pub liquidator: Pubkey,
        pub liquidator_stats: Pubkey,
        pub user: Pubkey,
        pub user_stats: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for LiquidatePerpPnlForDeposit {
        const DISCRIMINATOR: [u8; 8] = [151, 53, 184, 162, 233, 15, 254, 192];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for LiquidatePerpPnlForDeposit {}
    #[automatically_derived]
    impl ToAccountMetas for LiquidatePerpPnlForDeposit {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.liquidator,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.liquidator_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct SetUserStatusToBeingLiquidated {
        pub state: Pubkey,
        pub user: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for SetUserStatusToBeingLiquidated {
        const DISCRIMINATOR: [u8; 8] = [168, 116, 73, 0, 74, 191, 35, 182];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for SetUserStatusToBeingLiquidated {}
    #[automatically_derived]
    impl ToAccountMetas for SetUserStatusToBeingLiquidated {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct ResolvePerpPnlDeficit {
        pub state: Pubkey,
        pub authority: Pubkey,
        pub spot_market_vault: Pubkey,
        pub insurance_fund_vault: Pubkey,
        pub drift_signer: Pubkey,
        pub token_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for ResolvePerpPnlDeficit {
        const DISCRIMINATOR: [u8; 8] = [111, 56, 61, 226, 198, 72, 52, 59];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for ResolvePerpPnlDeficit {}
    #[automatically_derived]
    impl ToAccountMetas for ResolvePerpPnlDeficit {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.insurance_fund_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.drift_signer,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.token_program,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct ResolvePerpBankruptcy {
        pub state: Pubkey,
        pub authority: Pubkey,
        pub liquidator: Pubkey,
        pub liquidator_stats: Pubkey,
        pub user: Pubkey,
        pub user_stats: Pubkey,
        pub spot_market_vault: Pubkey,
        pub insurance_fund_vault: Pubkey,
        pub drift_signer: Pubkey,
        pub token_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for ResolvePerpBankruptcy {
        const DISCRIMINATOR: [u8; 8] = [189, 197, 58, 240, 98, 71, 91, 229];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for ResolvePerpBankruptcy {}
    #[automatically_derived]
    impl ToAccountMetas for ResolvePerpBankruptcy {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.liquidator,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.liquidator_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.spot_market_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.insurance_fund_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.drift_signer,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.token_program,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct ResolveSpotBankruptcy {
        pub state: Pubkey,
        pub authority: Pubkey,
        pub liquidator: Pubkey,
        pub liquidator_stats: Pubkey,
        pub user: Pubkey,
        pub user_stats: Pubkey,
        pub spot_market_vault: Pubkey,
        pub insurance_fund_vault: Pubkey,
        pub drift_signer: Pubkey,
        pub token_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for ResolveSpotBankruptcy {
        const DISCRIMINATOR: [u8; 8] = [175, 135, 10, 46, 114, 236, 26, 204];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for ResolveSpotBankruptcy {}
    #[automatically_derived]
    impl ToAccountMetas for ResolveSpotBankruptcy {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.liquidator,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.liquidator_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.spot_market_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.insurance_fund_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.drift_signer,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.token_program,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct SettleRevenueToInsuranceFund {
        pub state: Pubkey,
        pub spot_market: Pubkey,
        pub spot_market_vault: Pubkey,
        pub drift_signer: Pubkey,
        pub insurance_fund_vault: Pubkey,
        pub token_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for SettleRevenueToInsuranceFund {
        const DISCRIMINATOR: [u8; 8] = [48, 41, 224, 183, 4, 21, 196, 122];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for SettleRevenueToInsuranceFund {}
    #[automatically_derived]
    impl ToAccountMetas for SettleRevenueToInsuranceFund {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.spot_market_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.drift_signer,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.insurance_fund_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.token_program,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateFundingRate {
        pub state: Pubkey,
        pub perp_market: Pubkey,
        pub oracle: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateFundingRate {
        const DISCRIMINATOR: [u8; 8] = [175, 131, 101, 247, 81, 3, 88, 142];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateFundingRate {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateFundingRate {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.oracle,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePrelaunchOracle {
        pub state: Pubkey,
        pub perp_market: Pubkey,
        pub oracle: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePrelaunchOracle {
        const DISCRIMINATOR: [u8; 8] = [32, 32, 109, 28, 58, 162, 4, 146];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePrelaunchOracle {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePrelaunchOracle {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.oracle,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePerpBidAskTwap {
        pub state: Pubkey,
        pub perp_market: Pubkey,
        pub oracle: Pubkey,
        pub keeper_stats: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpBidAskTwap {
        const DISCRIMINATOR: [u8; 8] = [254, 122, 166, 101, 79, 146, 255, 191];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpBidAskTwap {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpBidAskTwap {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.oracle,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.keeper_stats,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateSpotMarketCumulativeInterest {
        pub state: Pubkey,
        pub spot_market: Pubkey,
        pub oracle: Pubkey,
        pub spot_market_vault: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketCumulativeInterest {
        const DISCRIMINATOR: [u8; 8] = [100, 56, 246, 179, 65, 238, 131, 45];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketCumulativeInterest {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateSpotMarketCumulativeInterest {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.oracle,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market_vault,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateAmms {
        pub state: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateAmms {
        const DISCRIMINATOR: [u8; 8] = [36, 198, 87, 44, 180, 153, 186, 135];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateAmms {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateAmms {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateSpotMarketExpiry {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketExpiry {
        const DISCRIMINATOR: [u8; 8] = [214, 167, 34, 236, 103, 45, 136, 60];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketExpiry {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateSpotMarketExpiry {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateUserQuoteAssetInsuranceStake {
        pub state: Pubkey,
        pub spot_market: Pubkey,
        pub insurance_fund_stake: Pubkey,
        pub user_stats: Pubkey,
        pub signer: Pubkey,
        pub insurance_fund_vault: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserQuoteAssetInsuranceStake {
        const DISCRIMINATOR: [u8; 8] = [8, 77, 199, 237, 124, 105, 186, 64];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserQuoteAssetInsuranceStake {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateUserQuoteAssetInsuranceStake {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.insurance_fund_stake,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.signer,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.insurance_fund_vault,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateUserGovTokenInsuranceStake {
        pub state: Pubkey,
        pub spot_market: Pubkey,
        pub insurance_fund_stake: Pubkey,
        pub user_stats: Pubkey,
        pub signer: Pubkey,
        pub insurance_fund_vault: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserGovTokenInsuranceStake {
        const DISCRIMINATOR: [u8; 8] = [138, 247, 235, 140, 209, 246, 2, 68];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserGovTokenInsuranceStake {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateUserGovTokenInsuranceStake {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.insurance_fund_stake,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.signer,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.insurance_fund_vault,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeInsuranceFundStake {
        pub spot_market: Pubkey,
        pub insurance_fund_stake: Pubkey,
        pub user_stats: Pubkey,
        pub state: Pubkey,
        pub authority: Pubkey,
        pub payer: Pubkey,
        pub rent: Pubkey,
        pub system_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeInsuranceFundStake {
        const DISCRIMINATOR: [u8; 8] = [181, 229, 1, 88, 0, 17, 36, 164];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeInsuranceFundStake {}
    #[automatically_derived]
    impl ToAccountMetas for InitializeInsuranceFundStake {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.insurance_fund_stake,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.payer,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.rent,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.system_program,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct AddInsuranceFundStake {
        pub state: Pubkey,
        pub spot_market: Pubkey,
        pub insurance_fund_stake: Pubkey,
        pub user_stats: Pubkey,
        pub authority: Pubkey,
        pub spot_market_vault: Pubkey,
        pub insurance_fund_vault: Pubkey,
        pub drift_signer: Pubkey,
        pub user_token_account: Pubkey,
        pub token_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for AddInsuranceFundStake {
        const DISCRIMINATOR: [u8; 8] = [105, 247, 44, 164, 216, 40, 236, 81];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for AddInsuranceFundStake {}
    #[automatically_derived]
    impl ToAccountMetas for AddInsuranceFundStake {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.insurance_fund_stake,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.insurance_fund_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.drift_signer,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.user_token_account,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.token_program,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct RequestRemoveInsuranceFundStake {
        pub spot_market: Pubkey,
        pub insurance_fund_stake: Pubkey,
        pub user_stats: Pubkey,
        pub authority: Pubkey,
        pub insurance_fund_vault: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for RequestRemoveInsuranceFundStake {
        const DISCRIMINATOR: [u8; 8] = [21, 28, 139, 183, 227, 150, 133, 125];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for RequestRemoveInsuranceFundStake {}
    #[automatically_derived]
    impl ToAccountMetas for RequestRemoveInsuranceFundStake {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.insurance_fund_stake,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.insurance_fund_vault,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct CancelRequestRemoveInsuranceFundStake {
        pub spot_market: Pubkey,
        pub insurance_fund_stake: Pubkey,
        pub user_stats: Pubkey,
        pub authority: Pubkey,
        pub insurance_fund_vault: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for CancelRequestRemoveInsuranceFundStake {
        const DISCRIMINATOR: [u8; 8] = [232, 20, 161, 220, 201, 149, 130, 96];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for CancelRequestRemoveInsuranceFundStake {}
    #[automatically_derived]
    impl ToAccountMetas for CancelRequestRemoveInsuranceFundStake {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.insurance_fund_stake,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.insurance_fund_vault,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct RemoveInsuranceFundStake {
        pub state: Pubkey,
        pub spot_market: Pubkey,
        pub insurance_fund_stake: Pubkey,
        pub user_stats: Pubkey,
        pub authority: Pubkey,
        pub insurance_fund_vault: Pubkey,
        pub drift_signer: Pubkey,
        pub user_token_account: Pubkey,
        pub token_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for RemoveInsuranceFundStake {
        const DISCRIMINATOR: [u8; 8] = [96, 220, 195, 168, 254, 234, 79, 160];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for RemoveInsuranceFundStake {}
    #[automatically_derived]
    impl ToAccountMetas for RemoveInsuranceFundStake {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.insurance_fund_stake,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.insurance_fund_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.drift_signer,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.user_token_account,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.token_program,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct TransferProtocolIfShares {
        pub signer: Pubkey,
        pub transfer_config: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
        pub insurance_fund_stake: Pubkey,
        pub user_stats: Pubkey,
        pub authority: Pubkey,
        pub insurance_fund_vault: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for TransferProtocolIfShares {
        const DISCRIMINATOR: [u8; 8] = [117, 155, 59, 159, 25, 202, 36, 129];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for TransferProtocolIfShares {}
    #[automatically_derived]
    impl ToAccountMetas for TransferProtocolIfShares {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.signer,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.transfer_config,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.insurance_fund_stake,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.insurance_fund_vault,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePythPullOracle {
        pub keeper: Pubkey,
        pub pyth_solana_receiver: Pubkey,
        pub encoded_vaa: Pubkey,
        pub price_feed: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePythPullOracle {
        const DISCRIMINATOR: [u8; 8] = [165, 134, 90, 23, 126, 250, 154, 87];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePythPullOracle {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePythPullOracle {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.keeper,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.pyth_solana_receiver,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.encoded_vaa,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.price_feed,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct PostPythPullOracleUpdateAtomic {
        pub keeper: Pubkey,
        pub pyth_solana_receiver: Pubkey,
        pub guardian_set: Pubkey,
        pub price_feed: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PostPythPullOracleUpdateAtomic {
        const DISCRIMINATOR: [u8; 8] = [231, 234, 147, 130, 95, 134, 65, 227];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for PostPythPullOracleUpdateAtomic {}
    #[automatically_derived]
    impl ToAccountMetas for PostPythPullOracleUpdateAtomic {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.keeper,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.pyth_solana_receiver,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.guardian_set,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.price_feed,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct PostMultiPythPullOracleUpdatesAtomic {
        pub keeper: Pubkey,
        pub pyth_solana_receiver: Pubkey,
        pub guardian_set: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PostMultiPythPullOracleUpdatesAtomic {
        const DISCRIMINATOR: [u8; 8] = [240, 120, 103, 185, 106, 15, 244, 18];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for PostMultiPythPullOracleUpdatesAtomic {}
    #[automatically_derived]
    impl ToAccountMetas for PostMultiPythPullOracleUpdatesAtomic {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.keeper,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.pyth_solana_receiver,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.guardian_set,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct Initialize {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub quote_asset_mint: Pubkey,
        pub drift_signer: Pubkey,
        pub rent: Pubkey,
        pub system_program: Pubkey,
        pub token_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for Initialize {
        const DISCRIMINATOR: [u8; 8] = [9, 223, 177, 72, 156, 50, 68, 214];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for Initialize {}
    #[automatically_derived]
    impl ToAccountMetas for Initialize {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.quote_asset_mint,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.drift_signer,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.rent,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.system_program,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.token_program,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeSpotMarket {
        pub spot_market: Pubkey,
        pub spot_market_mint: Pubkey,
        pub spot_market_vault: Pubkey,
        pub insurance_fund_vault: Pubkey,
        pub drift_signer: Pubkey,
        pub state: Pubkey,
        pub oracle: Pubkey,
        pub admin: Pubkey,
        pub rent: Pubkey,
        pub system_program: Pubkey,
        pub token_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeSpotMarket {
        const DISCRIMINATOR: [u8; 8] = [18, 231, 214, 171, 221, 205, 177, 255];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeSpotMarket {}
    #[automatically_derived]
    impl ToAccountMetas for InitializeSpotMarket {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.spot_market_mint,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.insurance_fund_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.drift_signer,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.oracle,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.rent,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.system_program,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.token_program,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct DeleteInitializedSpotMarket {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
        pub spot_market_vault: Pubkey,
        pub insurance_fund_vault: Pubkey,
        pub drift_signer: Pubkey,
        pub token_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for DeleteInitializedSpotMarket {
        const DISCRIMINATOR: [u8; 8] = [105, 141, 24, 186, 50, 155, 239, 41];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for DeleteInitializedSpotMarket {}
    #[automatically_derived]
    impl ToAccountMetas for DeleteInitializedSpotMarket {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.spot_market_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.insurance_fund_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.drift_signer,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.token_program,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeSerumFulfillmentConfig {
        pub base_spot_market: Pubkey,
        pub quote_spot_market: Pubkey,
        pub state: Pubkey,
        pub serum_program: Pubkey,
        pub serum_market: Pubkey,
        pub serum_open_orders: Pubkey,
        pub drift_signer: Pubkey,
        pub serum_fulfillment_config: Pubkey,
        pub admin: Pubkey,
        pub rent: Pubkey,
        pub system_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeSerumFulfillmentConfig {
        const DISCRIMINATOR: [u8; 8] = [173, 132, 99, 233, 86, 161, 27, 146];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeSerumFulfillmentConfig {}
    #[automatically_derived]
    impl ToAccountMetas for InitializeSerumFulfillmentConfig {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.base_spot_market,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.quote_spot_market,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.serum_program,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.serum_market,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.serum_open_orders,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.drift_signer,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.serum_fulfillment_config,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.rent,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.system_program,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateSerumFulfillmentConfigStatus {
        pub state: Pubkey,
        pub serum_fulfillment_config: Pubkey,
        pub admin: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSerumFulfillmentConfigStatus {
        const DISCRIMINATOR: [u8; 8] = [165, 122, 28, 215, 183, 101, 100, 79];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSerumFulfillmentConfigStatus {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateSerumFulfillmentConfigStatus {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.serum_fulfillment_config,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeOpenbookV2FulfillmentConfig {
        pub base_spot_market: Pubkey,
        pub quote_spot_market: Pubkey,
        pub state: Pubkey,
        pub openbook_v2_program: Pubkey,
        pub openbook_v2_market: Pubkey,
        pub drift_signer: Pubkey,
        pub openbook_v2_fulfillment_config: Pubkey,
        pub admin: Pubkey,
        pub rent: Pubkey,
        pub system_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeOpenbookV2FulfillmentConfig {
        const DISCRIMINATOR: [u8; 8] = [210, 94, 130, 130, 159, 35, 199, 66];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeOpenbookV2FulfillmentConfig {}
    #[automatically_derived]
    impl ToAccountMetas for InitializeOpenbookV2FulfillmentConfig {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.base_spot_market,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.quote_spot_market,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.openbook_v2_program,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.openbook_v2_market,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.drift_signer,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.openbook_v2_fulfillment_config,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.rent,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.system_program,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct OpenbookV2FulfillmentConfigStatus {
        pub state: Pubkey,
        pub openbook_v2_fulfillment_config: Pubkey,
        pub admin: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for OpenbookV2FulfillmentConfigStatus {
        const DISCRIMINATOR: [u8; 8] = [79, 178, 121, 29, 196, 209, 208, 117];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for OpenbookV2FulfillmentConfigStatus {}
    #[automatically_derived]
    impl ToAccountMetas for OpenbookV2FulfillmentConfigStatus {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.openbook_v2_fulfillment_config,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct InitializePhoenixFulfillmentConfig {
        pub base_spot_market: Pubkey,
        pub quote_spot_market: Pubkey,
        pub state: Pubkey,
        pub phoenix_program: Pubkey,
        pub phoenix_market: Pubkey,
        pub drift_signer: Pubkey,
        pub phoenix_fulfillment_config: Pubkey,
        pub admin: Pubkey,
        pub rent: Pubkey,
        pub system_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializePhoenixFulfillmentConfig {
        const DISCRIMINATOR: [u8; 8] = [5, 114, 80, 37, 88, 34, 202, 132];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializePhoenixFulfillmentConfig {}
    #[automatically_derived]
    impl ToAccountMetas for InitializePhoenixFulfillmentConfig {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.base_spot_market,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.quote_spot_market,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.phoenix_program,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.phoenix_market,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.drift_signer,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.phoenix_fulfillment_config,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.rent,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.system_program,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct PhoenixFulfillmentConfigStatus {
        pub state: Pubkey,
        pub phoenix_fulfillment_config: Pubkey,
        pub admin: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PhoenixFulfillmentConfigStatus {
        const DISCRIMINATOR: [u8; 8] = [178, 146, 140, 76, 221, 85, 109, 16];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for PhoenixFulfillmentConfigStatus {}
    #[automatically_derived]
    impl ToAccountMetas for PhoenixFulfillmentConfigStatus {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.phoenix_fulfillment_config,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateSerumVault {
        pub state: Pubkey,
        pub admin: Pubkey,
        pub srm_vault: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSerumVault {
        const DISCRIMINATOR: [u8; 8] = [22, 174, 84, 170, 101, 185, 37, 50];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSerumVault {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateSerumVault {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.srm_vault,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct InitializePerpMarket {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
        pub oracle: Pubkey,
        pub rent: Pubkey,
        pub system_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializePerpMarket {
        const DISCRIMINATOR: [u8; 8] = [148, 199, 202, 230, 252, 178, 42, 29];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializePerpMarket {}
    #[automatically_derived]
    impl ToAccountMetas for InitializePerpMarket {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.oracle,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.rent,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.system_program,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct InitializePredictionMarket {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializePredictionMarket {
        const DISCRIMINATOR: [u8; 8] = [46, 43, 153, 168, 142, 105, 114, 238];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializePredictionMarket {}
    #[automatically_derived]
    impl ToAccountMetas for InitializePredictionMarket {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct DeleteInitializedPerpMarket {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for DeleteInitializedPerpMarket {
        const DISCRIMINATOR: [u8; 8] = [219, 14, 114, 73, 19, 99, 209, 53];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for DeleteInitializedPerpMarket {}
    #[automatically_derived]
    impl ToAccountMetas for DeleteInitializedPerpMarket {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct MoveAmmPrice {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for MoveAmmPrice {
        const DISCRIMINATOR: [u8; 8] = [216, 116, 106, 26, 199, 17, 224, 135];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for MoveAmmPrice {}
    #[automatically_derived]
    impl ToAccountMetas for MoveAmmPrice {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct RecenterPerpMarketAmm {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for RecenterPerpMarketAmm {
        const DISCRIMINATOR: [u8; 8] = [50, 137, 234, 114, 92, 166, 222, 247];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for RecenterPerpMarketAmm {}
    #[automatically_derived]
    impl ToAccountMetas for RecenterPerpMarketAmm {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePerpMarketAmmSummaryStats {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
        pub spot_market: Pubkey,
        pub oracle: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketAmmSummaryStats {
        const DISCRIMINATOR: [u8; 8] = [229, 126, 9, 160, 79, 24, 21, 203];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketAmmSummaryStats {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketAmmSummaryStats {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.oracle,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePerpMarketExpiry {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketExpiry {
        const DISCRIMINATOR: [u8; 8] = [12, 47, 172, 38, 63, 18, 92, 27];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketExpiry {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketExpiry {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct SettleExpiredMarketPoolsToRevenuePool {
        pub state: Pubkey,
        pub admin: Pubkey,
        pub spot_market: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for SettleExpiredMarketPoolsToRevenuePool {
        const DISCRIMINATOR: [u8; 8] = [81, 218, 130, 92, 190, 241, 74, 69];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for SettleExpiredMarketPoolsToRevenuePool {}
    #[automatically_derived]
    impl ToAccountMetas for SettleExpiredMarketPoolsToRevenuePool {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct DepositIntoPerpMarketFeePool {
        pub state: Pubkey,
        pub perp_market: Pubkey,
        pub admin: Pubkey,
        pub source_vault: Pubkey,
        pub drift_signer: Pubkey,
        pub quote_spot_market: Pubkey,
        pub spot_market_vault: Pubkey,
        pub token_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for DepositIntoPerpMarketFeePool {
        const DISCRIMINATOR: [u8; 8] = [187, 80, 24, 206, 47, 105, 49, 19];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for DepositIntoPerpMarketFeePool {}
    #[automatically_derived]
    impl ToAccountMetas for DepositIntoPerpMarketFeePool {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.source_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.drift_signer,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.quote_spot_market,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.spot_market_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.token_program,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct DepositIntoSpotMarketVault {
        pub state: Pubkey,
        pub spot_market: Pubkey,
        pub admin: Pubkey,
        pub source_vault: Pubkey,
        pub spot_market_vault: Pubkey,
        pub token_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for DepositIntoSpotMarketVault {
        const DISCRIMINATOR: [u8; 8] = [157, 66, 91, 176, 199, 240, 252, 170];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for DepositIntoSpotMarketVault {}
    #[automatically_derived]
    impl ToAccountMetas for DepositIntoSpotMarketVault {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.source_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.spot_market_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.token_program,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct DepositIntoSpotMarketRevenuePool {
        pub state: Pubkey,
        pub spot_market: Pubkey,
        pub authority: Pubkey,
        pub spot_market_vault: Pubkey,
        pub user_token_account: Pubkey,
        pub token_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for DepositIntoSpotMarketRevenuePool {
        const DISCRIMINATOR: [u8; 8] = [18, 155, 189, 164, 77, 101, 238, 195];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for DepositIntoSpotMarketRevenuePool {}
    #[automatically_derived]
    impl ToAccountMetas for DepositIntoSpotMarketRevenuePool {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.spot_market_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_token_account,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.token_program,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct RepegAmmCurve {
        pub state: Pubkey,
        pub perp_market: Pubkey,
        pub oracle: Pubkey,
        pub admin: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for RepegAmmCurve {
        const DISCRIMINATOR: [u8; 8] = [126, 239, 157, 139, 125, 252, 30, 74];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for RepegAmmCurve {}
    #[automatically_derived]
    impl ToAccountMetas for RepegAmmCurve {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.oracle,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePerpMarketAmmOracleTwap {
        pub state: Pubkey,
        pub perp_market: Pubkey,
        pub oracle: Pubkey,
        pub admin: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketAmmOracleTwap {
        const DISCRIMINATOR: [u8; 8] = [204, 65, 76, 179, 189, 156, 80, 213];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketAmmOracleTwap {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketAmmOracleTwap {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.oracle,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct ResetPerpMarketAmmOracleTwap {
        pub state: Pubkey,
        pub perp_market: Pubkey,
        pub oracle: Pubkey,
        pub admin: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for ResetPerpMarketAmmOracleTwap {
        const DISCRIMINATOR: [u8; 8] = [213, 100, 210, 106, 236, 11, 211, 179];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for ResetPerpMarketAmmOracleTwap {}
    #[automatically_derived]
    impl ToAccountMetas for ResetPerpMarketAmmOracleTwap {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.oracle,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateK {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
        pub oracle: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateK {
        const DISCRIMINATOR: [u8; 8] = [61, 15, 174, 144, 200, 219, 117, 123];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateK {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateK {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.oracle,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePerpMarketMarginRatio {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketMarginRatio {
        const DISCRIMINATOR: [u8; 8] = [185, 46, 222, 233, 220, 50, 235, 54];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketMarginRatio {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketMarginRatio {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePerpMarketFundingPeriod {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketFundingPeriod {
        const DISCRIMINATOR: [u8; 8] = [99, 156, 7, 103, 23, 36, 78, 150];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketFundingPeriod {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketFundingPeriod {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePerpMarketMaxImbalances {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketMaxImbalances {
        const DISCRIMINATOR: [u8; 8] = [12, 45, 185, 141, 145, 147, 38, 111];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketMaxImbalances {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketMaxImbalances {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePerpMarketLiquidationFee {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketLiquidationFee {
        const DISCRIMINATOR: [u8; 8] = [175, 141, 225, 193, 209, 44, 143, 175];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketLiquidationFee {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketLiquidationFee {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateInsuranceFundUnstakingPeriod {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateInsuranceFundUnstakingPeriod {
        const DISCRIMINATOR: [u8; 8] = [163, 190, 11, 188, 2, 113, 126, 122];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateInsuranceFundUnstakingPeriod {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateInsuranceFundUnstakingPeriod {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateSpotMarketLiquidationFee {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketLiquidationFee {
        const DISCRIMINATOR: [u8; 8] = [75, 65, 223, 238, 181, 232, 79, 150];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketLiquidationFee {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateSpotMarketLiquidationFee {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateWithdrawGuardThreshold {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateWithdrawGuardThreshold {
        const DISCRIMINATOR: [u8; 8] = [3, 206, 206, 1, 159, 28, 61, 170];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateWithdrawGuardThreshold {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateWithdrawGuardThreshold {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateSpotMarketIfFactor {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketIfFactor {
        const DISCRIMINATOR: [u8; 8] = [223, 23, 229, 208, 9, 149, 146, 182];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketIfFactor {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateSpotMarketIfFactor {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateSpotMarketRevenueSettlePeriod {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketRevenueSettlePeriod {
        const DISCRIMINATOR: [u8; 8] = [7, 206, 24, 176, 80, 174, 165, 147];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketRevenueSettlePeriod {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateSpotMarketRevenueSettlePeriod {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateSpotMarketStatus {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketStatus {
        const DISCRIMINATOR: [u8; 8] = [41, 33, 10, 82, 199, 186, 77, 213];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketStatus {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateSpotMarketStatus {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateSpotMarketPausedOperations {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketPausedOperations {
        const DISCRIMINATOR: [u8; 8] = [146, 111, 120, 66, 119, 180, 51, 37];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketPausedOperations {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateSpotMarketPausedOperations {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateSpotMarketAssetTier {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketAssetTier {
        const DISCRIMINATOR: [u8; 8] = [238, 129, 100, 227, 241, 67, 51, 35];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketAssetTier {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateSpotMarketAssetTier {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateSpotMarketMarginWeights {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketMarginWeights {
        const DISCRIMINATOR: [u8; 8] = [102, 43, 99, 124, 9, 38, 180, 78];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketMarginWeights {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateSpotMarketMarginWeights {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateSpotMarketBorrowRate {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketBorrowRate {
        const DISCRIMINATOR: [u8; 8] = [106, 191, 198, 78, 220, 77, 58, 164];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketBorrowRate {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateSpotMarketBorrowRate {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateSpotMarketMaxTokenDeposits {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketMaxTokenDeposits {
        const DISCRIMINATOR: [u8; 8] = [113, 121, 179, 156, 99, 80, 241, 62];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketMaxTokenDeposits {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateSpotMarketMaxTokenDeposits {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateSpotMarketMaxTokenBorrows {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketMaxTokenBorrows {
        const DISCRIMINATOR: [u8; 8] = [88, 104, 192, 1, 83, 108, 136, 230];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketMaxTokenBorrows {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateSpotMarketMaxTokenBorrows {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateSpotMarketScaleInitialAssetWeightStart {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketScaleInitialAssetWeightStart {
        const DISCRIMINATOR: [u8; 8] = [29, 62, 180, 45, 168, 144, 167, 96];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketScaleInitialAssetWeightStart {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateSpotMarketScaleInitialAssetWeightStart {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateSpotMarketOracle {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
        pub oracle: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketOracle {
        const DISCRIMINATOR: [u8; 8] = [193, 241, 249, 110, 237, 142, 222, 213];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketOracle {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateSpotMarketOracle {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.oracle,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateSpotMarketStepSizeAndTickSize {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketStepSizeAndTickSize {
        const DISCRIMINATOR: [u8; 8] = [227, 237, 17, 146, 157, 75, 167, 199];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketStepSizeAndTickSize {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateSpotMarketStepSizeAndTickSize {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateSpotMarketMinOrderSize {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketMinOrderSize {
        const DISCRIMINATOR: [u8; 8] = [186, 178, 157, 122, 54, 67, 251, 94];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketMinOrderSize {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateSpotMarketMinOrderSize {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateSpotMarketOrdersEnabled {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketOrdersEnabled {
        const DISCRIMINATOR: [u8; 8] = [172, 56, 247, 26, 147, 17, 142, 147];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketOrdersEnabled {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateSpotMarketOrdersEnabled {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateSpotMarketIfPausedOperations {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketIfPausedOperations {
        const DISCRIMINATOR: [u8; 8] = [245, 108, 77, 187, 26, 214, 158, 68];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketIfPausedOperations {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateSpotMarketIfPausedOperations {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateSpotMarketName {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketName {
        const DISCRIMINATOR: [u8; 8] = [63, 67, 93, 235, 188, 98, 129, 113];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketName {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateSpotMarketName {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePerpMarketStatus {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketStatus {
        const DISCRIMINATOR: [u8; 8] = [104, 3, 229, 52, 193, 38, 170, 151];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketStatus {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketStatus {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePerpMarketPausedOperations {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketPausedOperations {
        const DISCRIMINATOR: [u8; 8] = [250, 113, 186, 164, 94, 11, 121, 131];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketPausedOperations {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketPausedOperations {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePerpMarketContractTier {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketContractTier {
        const DISCRIMINATOR: [u8; 8] = [158, 229, 240, 106, 180, 32, 83, 88];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketContractTier {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketContractTier {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePerpMarketImfFactor {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketImfFactor {
        const DISCRIMINATOR: [u8; 8] = [35, 43, 93, 148, 156, 171, 142, 168];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketImfFactor {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketImfFactor {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePerpMarketUnrealizedAssetWeight {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketUnrealizedAssetWeight {
        const DISCRIMINATOR: [u8; 8] = [91, 132, 97, 182, 54, 158, 132, 42];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketUnrealizedAssetWeight {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketUnrealizedAssetWeight {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePerpMarketConcentrationCoef {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketConcentrationCoef {
        const DISCRIMINATOR: [u8; 8] = [174, 15, 49, 90, 177, 215, 16, 138];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketConcentrationCoef {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketConcentrationCoef {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePerpMarketCurveUpdateIntensity {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketCurveUpdateIntensity {
        const DISCRIMINATOR: [u8; 8] = [243, 251, 191, 36, 139, 6, 209, 225];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketCurveUpdateIntensity {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketCurveUpdateIntensity {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePerpMarketTargetBaseAssetAmountPerLp {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketTargetBaseAssetAmountPerLp {
        const DISCRIMINATOR: [u8; 8] = [87, 115, 195, 194, 220, 71, 141, 145];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketTargetBaseAssetAmountPerLp {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketTargetBaseAssetAmountPerLp {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePerpMarketPerLpBase {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketPerLpBase {
        const DISCRIMINATOR: [u8; 8] = [239, 62, 4, 91, 188, 28, 164, 24];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketPerLpBase {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketPerLpBase {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateLpCooldownTime {
        pub admin: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateLpCooldownTime {
        const DISCRIMINATOR: [u8; 8] = [90, 61, 204, 203, 28, 171, 28, 115];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateLpCooldownTime {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateLpCooldownTime {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePerpFeeStructure {
        pub admin: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpFeeStructure {
        const DISCRIMINATOR: [u8; 8] = [4, 7, 150, 19, 255, 86, 74, 198];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpFeeStructure {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpFeeStructure {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateSpotFeeStructure {
        pub admin: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotFeeStructure {
        const DISCRIMINATOR: [u8; 8] = [236, 254, 252, 54, 93, 197, 4, 202];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotFeeStructure {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateSpotFeeStructure {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateInitialPctToLiquidate {
        pub admin: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateInitialPctToLiquidate {
        const DISCRIMINATOR: [u8; 8] = [245, 87, 236, 224, 186, 202, 205, 56];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateInitialPctToLiquidate {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateInitialPctToLiquidate {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateLiquidationDuration {
        pub admin: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateLiquidationDuration {
        const DISCRIMINATOR: [u8; 8] = [251, 210, 14, 72, 227, 11, 213, 213];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateLiquidationDuration {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateLiquidationDuration {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateLiquidationMarginBufferRatio {
        pub admin: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateLiquidationMarginBufferRatio {
        const DISCRIMINATOR: [u8; 8] = [149, 239, 31, 46, 10, 188, 189, 205];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateLiquidationMarginBufferRatio {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateLiquidationMarginBufferRatio {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateOracleGuardRails {
        pub admin: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateOracleGuardRails {
        const DISCRIMINATOR: [u8; 8] = [234, 20, 73, 125, 231, 195, 199, 69];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateOracleGuardRails {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateOracleGuardRails {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateStateSettlementDuration {
        pub admin: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateStateSettlementDuration {
        const DISCRIMINATOR: [u8; 8] = [172, 42, 114, 114, 90, 254, 77, 140];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateStateSettlementDuration {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateStateSettlementDuration {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateStateMaxNumberOfSubAccounts {
        pub admin: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateStateMaxNumberOfSubAccounts {
        const DISCRIMINATOR: [u8; 8] = [201, 108, 95, 206, 242, 192, 202, 17];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateStateMaxNumberOfSubAccounts {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateStateMaxNumberOfSubAccounts {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateStateMaxInitializeUserFee {
        pub admin: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateStateMaxInitializeUserFee {
        const DISCRIMINATOR: [u8; 8] = [191, 215, 98, 78, 139, 144, 214, 6];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateStateMaxInitializeUserFee {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateStateMaxInitializeUserFee {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePerpMarketOracle {
        pub state: Pubkey,
        pub perp_market: Pubkey,
        pub oracle: Pubkey,
        pub admin: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketOracle {
        const DISCRIMINATOR: [u8; 8] = [244, 195, 212, 72, 201, 214, 79, 167];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketOracle {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketOracle {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.oracle,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePerpMarketBaseSpread {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketBaseSpread {
        const DISCRIMINATOR: [u8; 8] = [161, 37, 147, 21, 122, 0, 48, 163];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketBaseSpread {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketBaseSpread {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateAmmJitIntensity {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateAmmJitIntensity {
        const DISCRIMINATOR: [u8; 8] = [199, 233, 116, 181, 227, 225, 160, 146];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateAmmJitIntensity {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateAmmJitIntensity {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePerpMarketMaxSpread {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketMaxSpread {
        const DISCRIMINATOR: [u8; 8] = [139, 87, 9, 143, 244, 66, 107, 102];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketMaxSpread {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketMaxSpread {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePerpMarketStepSizeAndTickSize {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketStepSizeAndTickSize {
        const DISCRIMINATOR: [u8; 8] = [19, 115, 209, 182, 45, 32, 168, 214];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketStepSizeAndTickSize {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketStepSizeAndTickSize {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePerpMarketName {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketName {
        const DISCRIMINATOR: [u8; 8] = [189, 217, 32, 200, 38, 59, 196, 107];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketName {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketName {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePerpMarketMinOrderSize {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketMinOrderSize {
        const DISCRIMINATOR: [u8; 8] = [34, 179, 219, 246, 104, 199, 77, 245];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketMinOrderSize {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketMinOrderSize {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePerpMarketMaxSlippageRatio {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketMaxSlippageRatio {
        const DISCRIMINATOR: [u8; 8] = [27, 252, 55, 248, 153, 108, 123, 210];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketMaxSlippageRatio {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketMaxSlippageRatio {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePerpMarketMaxFillReserveFraction {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketMaxFillReserveFraction {
        const DISCRIMINATOR: [u8; 8] = [17, 108, 221, 67, 113, 123, 177, 84];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketMaxFillReserveFraction {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketMaxFillReserveFraction {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePerpMarketMaxOpenInterest {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketMaxOpenInterest {
        const DISCRIMINATOR: [u8; 8] = [185, 21, 26, 206, 192, 123, 35, 83];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketMaxOpenInterest {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketMaxOpenInterest {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePerpMarketNumberOfUsers {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketNumberOfUsers {
        const DISCRIMINATOR: [u8; 8] = [72, 93, 65, 177, 28, 142, 86, 178];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketNumberOfUsers {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketNumberOfUsers {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePerpMarketFeeAdjustment {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketFeeAdjustment {
        const DISCRIMINATOR: [u8; 8] = [55, 42, 108, 236, 230, 252, 96, 101];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketFeeAdjustment {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketFeeAdjustment {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateSpotMarketFeeAdjustment {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketFeeAdjustment {
        const DISCRIMINATOR: [u8; 8] = [181, 200, 92, 60, 229, 232, 154, 146];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketFeeAdjustment {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateSpotMarketFeeAdjustment {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePerpMarketFuel {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketFuel {
        const DISCRIMINATOR: [u8; 8] = [142, 54, 34, 143, 167, 83, 175, 188];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketFuel {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketFuel {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateSpotMarketFuel {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketFuel {
        const DISCRIMINATOR: [u8; 8] = [118, 160, 123, 104, 92, 187, 164, 191];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketFuel {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateSpotMarketFuel {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.spot_market,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct InitUserFuel {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub user: Pubkey,
        pub user_stats: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitUserFuel {
        const DISCRIMINATOR: [u8; 8] = [220, 14, 11, 84, 194, 75, 19, 232];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitUserFuel {}
    #[automatically_derived]
    impl ToAccountMetas for InitUserFuel {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateAdmin {
        pub admin: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateAdmin {
        const DISCRIMINATOR: [u8; 8] = [113, 14, 125, 75, 57, 252, 185, 92];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateAdmin {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateAdmin {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateWhitelistMint {
        pub admin: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateWhitelistMint {
        const DISCRIMINATOR: [u8; 8] = [153, 129, 154, 4, 241, 27, 64, 173];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateWhitelistMint {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateWhitelistMint {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateDiscountMint {
        pub admin: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateDiscountMint {
        const DISCRIMINATOR: [u8; 8] = [112, 225, 35, 186, 137, 34, 220, 102];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateDiscountMint {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateDiscountMint {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateExchangeStatus {
        pub admin: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateExchangeStatus {
        const DISCRIMINATOR: [u8; 8] = [173, 244, 148, 56, 62, 192, 219, 188];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateExchangeStatus {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateExchangeStatus {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePerpAuctionDuration {
        pub admin: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpAuctionDuration {
        const DISCRIMINATOR: [u8; 8] = [181, 65, 59, 2, 163, 164, 227, 20];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpAuctionDuration {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpAuctionDuration {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateSpotAuctionDuration {
        pub admin: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotAuctionDuration {
        const DISCRIMINATOR: [u8; 8] = [16, 251, 20, 38, 178, 127, 160, 45];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotAuctionDuration {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateSpotAuctionDuration {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct InitializeProtocolIfSharesTransferConfig {
        pub admin: Pubkey,
        pub protocol_if_shares_transfer_config: Pubkey,
        pub state: Pubkey,
        pub rent: Pubkey,
        pub system_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeProtocolIfSharesTransferConfig {
        const DISCRIMINATOR: [u8; 8] = [127, 78, 21, 186, 205, 166, 113, 50];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeProtocolIfSharesTransferConfig {}
    #[automatically_derived]
    impl ToAccountMetas for InitializeProtocolIfSharesTransferConfig {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.protocol_if_shares_transfer_config,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.rent,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.system_program,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdateProtocolIfSharesTransferConfig {
        pub admin: Pubkey,
        pub protocol_if_shares_transfer_config: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateProtocolIfSharesTransferConfig {
        const DISCRIMINATOR: [u8; 8] = [234, 170, 4, 197, 185, 34, 103, 50];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateProtocolIfSharesTransferConfig {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateProtocolIfSharesTransferConfig {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.protocol_if_shares_transfer_config,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct InitializePrelaunchOracle {
        pub admin: Pubkey,
        pub prelaunch_oracle: Pubkey,
        pub state: Pubkey,
        pub rent: Pubkey,
        pub system_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializePrelaunchOracle {
        const DISCRIMINATOR: [u8; 8] = [50, 65, 1, 179, 91, 203, 144, 43];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializePrelaunchOracle {}
    #[automatically_derived]
    impl ToAccountMetas for InitializePrelaunchOracle {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.prelaunch_oracle,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.rent,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.system_program,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct UpdatePrelaunchOracleParams {
        pub admin: Pubkey,
        pub prelaunch_oracle: Pubkey,
        pub perp_market: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePrelaunchOracleParams {
        const DISCRIMINATOR: [u8; 8] = [35, 251, 112, 141, 9, 117, 184, 231];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePrelaunchOracleParams {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePrelaunchOracleParams {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.prelaunch_oracle,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct DeletePrelaunchOracle {
        pub admin: Pubkey,
        pub prelaunch_oracle: Pubkey,
        pub perp_market: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for DeletePrelaunchOracle {
        const DISCRIMINATOR: [u8; 8] = [171, 55, 241, 236, 17, 127, 99, 190];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for DeletePrelaunchOracle {}
    #[automatically_derived]
    impl ToAccountMetas for DeletePrelaunchOracle {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.prelaunch_oracle,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.perp_market,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize)]
    pub struct InitializePythPullOracle {
        pub admin: Pubkey,
        pub pyth_solana_receiver: Pubkey,
        pub price_feed: Pubkey,
        pub system_program: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializePythPullOracle {
        const DISCRIMINATOR: [u8; 8] = [235, 115, 210, 83, 67, 24, 85, 181];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializePythPullOracle {}
    #[automatically_derived]
    impl ToAccountMetas for InitializePythPullOracle {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.pyth_solana_receiver,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.price_feed,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.system_program,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
}
pub mod errors {
    use super::{types::*, *};
    #[derive(PartialEq)]
    #[error_code]
    pub enum ErrorCode {
        #[msg("Invalid Spot Market Authority")]
        InvalidSpotMarketAuthority = 6000u32,
        #[msg("Clearing house not insurance fund authority")]
        InvalidInsuranceFundAuthority = 6001u32,
        #[msg("Insufficient deposit")]
        InsufficientDeposit = 6002u32,
        #[msg("Insufficient collateral")]
        InsufficientCollateral = 6003u32,
        #[msg("Sufficient collateral")]
        SufficientCollateral = 6004u32,
        #[msg("Max number of positions taken")]
        MaxNumberOfPositions = 6005u32,
        #[msg("Admin Controls Prices Disabled")]
        AdminControlsPricesDisabled = 6006u32,
        #[msg("Market Delisted")]
        MarketDelisted = 6007u32,
        #[msg("Market Index Already Initialized")]
        MarketIndexAlreadyInitialized = 6008u32,
        #[msg("User Account And User Positions Account Mismatch")]
        UserAccountAndUserPositionsAccountMismatch = 6009u32,
        #[msg("User Has No Position In Market")]
        UserHasNoPositionInMarket = 6010u32,
        #[msg("Invalid Initial Peg")]
        InvalidInitialPeg = 6011u32,
        #[msg("AMM repeg already configured with amt given")]
        InvalidRepegRedundant = 6012u32,
        #[msg("AMM repeg incorrect repeg direction")]
        InvalidRepegDirection = 6013u32,
        #[msg("AMM repeg out of bounds pnl")]
        InvalidRepegProfitability = 6014u32,
        #[msg("Slippage Outside Limit Price")]
        SlippageOutsideLimit = 6015u32,
        #[msg("Order Size Too Small")]
        OrderSizeTooSmall = 6016u32,
        #[msg("Price change too large when updating K")]
        InvalidUpdateK = 6017u32,
        #[msg("Admin tried to withdraw amount larger than fees collected")]
        AdminWithdrawTooLarge = 6018u32,
        #[msg("Math Error")]
        MathError = 6019u32,
        #[msg("Conversion to u128/u64 failed with an overflow or underflow")]
        BnConversionError = 6020u32,
        #[msg("Clock unavailable")]
        ClockUnavailable = 6021u32,
        #[msg("Unable To Load Oracles")]
        UnableToLoadOracle = 6022u32,
        #[msg("Price Bands Breached")]
        PriceBandsBreached = 6023u32,
        #[msg("Exchange is paused")]
        ExchangePaused = 6024u32,
        #[msg("Invalid whitelist token")]
        InvalidWhitelistToken = 6025u32,
        #[msg("Whitelist token not found")]
        WhitelistTokenNotFound = 6026u32,
        #[msg("Invalid discount token")]
        InvalidDiscountToken = 6027u32,
        #[msg("Discount token not found")]
        DiscountTokenNotFound = 6028u32,
        #[msg("Referrer not found")]
        ReferrerNotFound = 6029u32,
        #[msg("ReferrerNotFound")]
        ReferrerStatsNotFound = 6030u32,
        #[msg("ReferrerMustBeWritable")]
        ReferrerMustBeWritable = 6031u32,
        #[msg("ReferrerMustBeWritable")]
        ReferrerStatsMustBeWritable = 6032u32,
        #[msg("ReferrerAndReferrerStatsAuthorityUnequal")]
        ReferrerAndReferrerStatsAuthorityUnequal = 6033u32,
        #[msg("InvalidReferrer")]
        InvalidReferrer = 6034u32,
        #[msg("InvalidOracle")]
        InvalidOracle = 6035u32,
        #[msg("OracleNotFound")]
        OracleNotFound = 6036u32,
        #[msg("Liquidations Blocked By Oracle")]
        LiquidationsBlockedByOracle = 6037u32,
        #[msg("Can not deposit more than max deposit")]
        MaxDeposit = 6038u32,
        #[msg("Can not delete user that still has collateral")]
        CantDeleteUserWithCollateral = 6039u32,
        #[msg("AMM funding out of bounds pnl")]
        InvalidFundingProfitability = 6040u32,
        #[msg("Casting Failure")]
        CastingFailure = 6041u32,
        #[msg("InvalidOrder")]
        InvalidOrder = 6042u32,
        #[msg("InvalidOrderMaxTs")]
        InvalidOrderMaxTs = 6043u32,
        #[msg("InvalidOrderMarketType")]
        InvalidOrderMarketType = 6044u32,
        #[msg("InvalidOrderForInitialMarginReq")]
        InvalidOrderForInitialMarginReq = 6045u32,
        #[msg("InvalidOrderNotRiskReducing")]
        InvalidOrderNotRiskReducing = 6046u32,
        #[msg("InvalidOrderSizeTooSmall")]
        InvalidOrderSizeTooSmall = 6047u32,
        #[msg("InvalidOrderNotStepSizeMultiple")]
        InvalidOrderNotStepSizeMultiple = 6048u32,
        #[msg("InvalidOrderBaseQuoteAsset")]
        InvalidOrderBaseQuoteAsset = 6049u32,
        #[msg("InvalidOrderIOC")]
        InvalidOrderIOC = 6050u32,
        #[msg("InvalidOrderPostOnly")]
        InvalidOrderPostOnly = 6051u32,
        #[msg("InvalidOrderIOCPostOnly")]
        InvalidOrderIOCPostOnly = 6052u32,
        #[msg("InvalidOrderTrigger")]
        InvalidOrderTrigger = 6053u32,
        #[msg("InvalidOrderAuction")]
        InvalidOrderAuction = 6054u32,
        #[msg("InvalidOrderOracleOffset")]
        InvalidOrderOracleOffset = 6055u32,
        #[msg("InvalidOrderMinOrderSize")]
        InvalidOrderMinOrderSize = 6056u32,
        #[msg("Failed to Place Post-Only Limit Order")]
        PlacePostOnlyLimitFailure = 6057u32,
        #[msg("User has no order")]
        UserHasNoOrder = 6058u32,
        #[msg("Order Amount Too Small")]
        OrderAmountTooSmall = 6059u32,
        #[msg("Max number of orders taken")]
        MaxNumberOfOrders = 6060u32,
        #[msg("Order does not exist")]
        OrderDoesNotExist = 6061u32,
        #[msg("Order not open")]
        OrderNotOpen = 6062u32,
        #[msg("FillOrderDidNotUpdateState")]
        FillOrderDidNotUpdateState = 6063u32,
        #[msg("Reduce only order increased risk")]
        ReduceOnlyOrderIncreasedRisk = 6064u32,
        #[msg("Unable to load AccountLoader")]
        UnableToLoadAccountLoader = 6065u32,
        #[msg("Trade Size Too Large")]
        TradeSizeTooLarge = 6066u32,
        #[msg("User cant refer themselves")]
        UserCantReferThemselves = 6067u32,
        #[msg("Did not receive expected referrer")]
        DidNotReceiveExpectedReferrer = 6068u32,
        #[msg("Could not deserialize referrer")]
        CouldNotDeserializeReferrer = 6069u32,
        #[msg("Could not deserialize referrer stats")]
        CouldNotDeserializeReferrerStats = 6070u32,
        #[msg("User Order Id Already In Use")]
        UserOrderIdAlreadyInUse = 6071u32,
        #[msg("No positions liquidatable")]
        NoPositionsLiquidatable = 6072u32,
        #[msg("Invalid Margin Ratio")]
        InvalidMarginRatio = 6073u32,
        #[msg("Cant Cancel Post Only Order")]
        CantCancelPostOnlyOrder = 6074u32,
        #[msg("InvalidOracleOffset")]
        InvalidOracleOffset = 6075u32,
        #[msg("CantExpireOrders")]
        CantExpireOrders = 6076u32,
        #[msg("CouldNotLoadMarketData")]
        CouldNotLoadMarketData = 6077u32,
        #[msg("PerpMarketNotFound")]
        PerpMarketNotFound = 6078u32,
        #[msg("InvalidMarketAccount")]
        InvalidMarketAccount = 6079u32,
        #[msg("UnableToLoadMarketAccount")]
        UnableToLoadPerpMarketAccount = 6080u32,
        #[msg("MarketWrongMutability")]
        MarketWrongMutability = 6081u32,
        #[msg("UnableToCastUnixTime")]
        UnableToCastUnixTime = 6082u32,
        #[msg("CouldNotFindSpotPosition")]
        CouldNotFindSpotPosition = 6083u32,
        #[msg("NoSpotPositionAvailable")]
        NoSpotPositionAvailable = 6084u32,
        #[msg("InvalidSpotMarketInitialization")]
        InvalidSpotMarketInitialization = 6085u32,
        #[msg("CouldNotLoadSpotMarketData")]
        CouldNotLoadSpotMarketData = 6086u32,
        #[msg("SpotMarketNotFound")]
        SpotMarketNotFound = 6087u32,
        #[msg("InvalidSpotMarketAccount")]
        InvalidSpotMarketAccount = 6088u32,
        #[msg("UnableToLoadSpotMarketAccount")]
        UnableToLoadSpotMarketAccount = 6089u32,
        #[msg("SpotMarketWrongMutability")]
        SpotMarketWrongMutability = 6090u32,
        #[msg("SpotInterestNotUpToDate")]
        SpotMarketInterestNotUpToDate = 6091u32,
        #[msg("SpotMarketInsufficientDeposits")]
        SpotMarketInsufficientDeposits = 6092u32,
        #[msg("UserMustSettleTheirOwnPositiveUnsettledPNL")]
        UserMustSettleTheirOwnPositiveUnsettledPNL = 6093u32,
        #[msg("CantUpdatePoolBalanceType")]
        CantUpdatePoolBalanceType = 6094u32,
        #[msg("InsufficientCollateralForSettlingPNL")]
        InsufficientCollateralForSettlingPNL = 6095u32,
        #[msg("AMMNotUpdatedInSameSlot")]
        AMMNotUpdatedInSameSlot = 6096u32,
        #[msg("AuctionNotComplete")]
        AuctionNotComplete = 6097u32,
        #[msg("MakerNotFound")]
        MakerNotFound = 6098u32,
        #[msg("MakerNotFound")]
        MakerStatsNotFound = 6099u32,
        #[msg("MakerMustBeWritable")]
        MakerMustBeWritable = 6100u32,
        #[msg("MakerMustBeWritable")]
        MakerStatsMustBeWritable = 6101u32,
        #[msg("MakerOrderNotFound")]
        MakerOrderNotFound = 6102u32,
        #[msg("CouldNotDeserializeMaker")]
        CouldNotDeserializeMaker = 6103u32,
        #[msg("CouldNotDeserializeMaker")]
        CouldNotDeserializeMakerStats = 6104u32,
        #[msg("AuctionPriceDoesNotSatisfyMaker")]
        AuctionPriceDoesNotSatisfyMaker = 6105u32,
        #[msg("MakerCantFulfillOwnOrder")]
        MakerCantFulfillOwnOrder = 6106u32,
        #[msg("MakerOrderMustBePostOnly")]
        MakerOrderMustBePostOnly = 6107u32,
        #[msg("CantMatchTwoPostOnlys")]
        CantMatchTwoPostOnlys = 6108u32,
        #[msg("OrderBreachesOraclePriceLimits")]
        OrderBreachesOraclePriceLimits = 6109u32,
        #[msg("OrderMustBeTriggeredFirst")]
        OrderMustBeTriggeredFirst = 6110u32,
        #[msg("OrderNotTriggerable")]
        OrderNotTriggerable = 6111u32,
        #[msg("OrderDidNotSatisfyTriggerCondition")]
        OrderDidNotSatisfyTriggerCondition = 6112u32,
        #[msg("PositionAlreadyBeingLiquidated")]
        PositionAlreadyBeingLiquidated = 6113u32,
        #[msg("PositionDoesntHaveOpenPositionOrOrders")]
        PositionDoesntHaveOpenPositionOrOrders = 6114u32,
        #[msg("AllOrdersAreAlreadyLiquidations")]
        AllOrdersAreAlreadyLiquidations = 6115u32,
        #[msg("CantCancelLiquidationOrder")]
        CantCancelLiquidationOrder = 6116u32,
        #[msg("UserIsBeingLiquidated")]
        UserIsBeingLiquidated = 6117u32,
        #[msg("LiquidationsOngoing")]
        LiquidationsOngoing = 6118u32,
        #[msg("WrongSpotBalanceType")]
        WrongSpotBalanceType = 6119u32,
        #[msg("UserCantLiquidateThemself")]
        UserCantLiquidateThemself = 6120u32,
        #[msg("InvalidPerpPositionToLiquidate")]
        InvalidPerpPositionToLiquidate = 6121u32,
        #[msg("InvalidBaseAssetAmountForLiquidatePerp")]
        InvalidBaseAssetAmountForLiquidatePerp = 6122u32,
        #[msg("InvalidPositionLastFundingRate")]
        InvalidPositionLastFundingRate = 6123u32,
        #[msg("InvalidPositionDelta")]
        InvalidPositionDelta = 6124u32,
        #[msg("UserBankrupt")]
        UserBankrupt = 6125u32,
        #[msg("UserNotBankrupt")]
        UserNotBankrupt = 6126u32,
        #[msg("UserHasInvalidBorrow")]
        UserHasInvalidBorrow = 6127u32,
        #[msg("DailyWithdrawLimit")]
        DailyWithdrawLimit = 6128u32,
        #[msg("DefaultError")]
        DefaultError = 6129u32,
        #[msg("Insufficient LP tokens")]
        InsufficientLPTokens = 6130u32,
        #[msg("Cant LP with a market position")]
        CantLPWithPerpPosition = 6131u32,
        #[msg("Unable to burn LP tokens")]
        UnableToBurnLPTokens = 6132u32,
        #[msg("Trying to remove liqudity too fast after adding it")]
        TryingToRemoveLiquidityTooFast = 6133u32,
        #[msg("Invalid Spot Market Vault")]
        InvalidSpotMarketVault = 6134u32,
        #[msg("Invalid Spot Market State")]
        InvalidSpotMarketState = 6135u32,
        #[msg("InvalidSerumProgram")]
        InvalidSerumProgram = 6136u32,
        #[msg("InvalidSerumMarket")]
        InvalidSerumMarket = 6137u32,
        #[msg("InvalidSerumBids")]
        InvalidSerumBids = 6138u32,
        #[msg("InvalidSerumAsks")]
        InvalidSerumAsks = 6139u32,
        #[msg("InvalidSerumOpenOrders")]
        InvalidSerumOpenOrders = 6140u32,
        #[msg("FailedSerumCPI")]
        FailedSerumCPI = 6141u32,
        #[msg("FailedToFillOnExternalMarket")]
        FailedToFillOnExternalMarket = 6142u32,
        #[msg("InvalidFulfillmentConfig")]
        InvalidFulfillmentConfig = 6143u32,
        #[msg("InvalidFeeStructure")]
        InvalidFeeStructure = 6144u32,
        #[msg("Insufficient IF shares")]
        InsufficientIFShares = 6145u32,
        #[msg("the Market has paused this action")]
        MarketActionPaused = 6146u32,
        #[msg("the Market status doesnt allow placing orders")]
        MarketPlaceOrderPaused = 6147u32,
        #[msg("the Market status doesnt allow filling orders")]
        MarketFillOrderPaused = 6148u32,
        #[msg("the Market status doesnt allow withdraws")]
        MarketWithdrawPaused = 6149u32,
        #[msg("Action violates the Protected Asset Tier rules")]
        ProtectedAssetTierViolation = 6150u32,
        #[msg("Action violates the Isolated Asset Tier rules")]
        IsolatedAssetTierViolation = 6151u32,
        #[msg("User Cant Be Deleted")]
        UserCantBeDeleted = 6152u32,
        #[msg("Reduce Only Withdraw Increased Risk")]
        ReduceOnlyWithdrawIncreasedRisk = 6153u32,
        #[msg("Max Open Interest")]
        MaxOpenInterest = 6154u32,
        #[msg("Cant Resolve Perp Bankruptcy")]
        CantResolvePerpBankruptcy = 6155u32,
        #[msg("Liquidation Doesnt Satisfy Limit Price")]
        LiquidationDoesntSatisfyLimitPrice = 6156u32,
        #[msg("Margin Trading Disabled")]
        MarginTradingDisabled = 6157u32,
        #[msg("Invalid Market Status to Settle Perp Pnl")]
        InvalidMarketStatusToSettlePnl = 6158u32,
        #[msg("PerpMarketNotInSettlement")]
        PerpMarketNotInSettlement = 6159u32,
        #[msg("PerpMarketNotInReduceOnly")]
        PerpMarketNotInReduceOnly = 6160u32,
        #[msg("PerpMarketSettlementBufferNotReached")]
        PerpMarketSettlementBufferNotReached = 6161u32,
        #[msg("PerpMarketSettlementUserHasOpenOrders")]
        PerpMarketSettlementUserHasOpenOrders = 6162u32,
        #[msg("PerpMarketSettlementUserHasActiveLP")]
        PerpMarketSettlementUserHasActiveLP = 6163u32,
        #[msg("UnableToSettleExpiredUserPosition")]
        UnableToSettleExpiredUserPosition = 6164u32,
        #[msg("UnequalMarketIndexForSpotTransfer")]
        UnequalMarketIndexForSpotTransfer = 6165u32,
        #[msg("InvalidPerpPositionDetected")]
        InvalidPerpPositionDetected = 6166u32,
        #[msg("InvalidSpotPositionDetected")]
        InvalidSpotPositionDetected = 6167u32,
        #[msg("InvalidAmmDetected")]
        InvalidAmmDetected = 6168u32,
        #[msg("InvalidAmmForFillDetected")]
        InvalidAmmForFillDetected = 6169u32,
        #[msg("InvalidAmmLimitPriceOverride")]
        InvalidAmmLimitPriceOverride = 6170u32,
        #[msg("InvalidOrderFillPrice")]
        InvalidOrderFillPrice = 6171u32,
        #[msg("SpotMarketBalanceInvariantViolated")]
        SpotMarketBalanceInvariantViolated = 6172u32,
        #[msg("SpotMarketVaultInvariantViolated")]
        SpotMarketVaultInvariantViolated = 6173u32,
        #[msg("InvalidPDA")]
        InvalidPDA = 6174u32,
        #[msg("InvalidPDASigner")]
        InvalidPDASigner = 6175u32,
        #[msg("RevenueSettingsCannotSettleToIF")]
        RevenueSettingsCannotSettleToIF = 6176u32,
        #[msg("NoRevenueToSettleToIF")]
        NoRevenueToSettleToIF = 6177u32,
        #[msg("NoAmmPerpPnlDeficit")]
        NoAmmPerpPnlDeficit = 6178u32,
        #[msg("SufficientPerpPnlPool")]
        SufficientPerpPnlPool = 6179u32,
        #[msg("InsufficientPerpPnlPool")]
        InsufficientPerpPnlPool = 6180u32,
        #[msg("PerpPnlDeficitBelowThreshold")]
        PerpPnlDeficitBelowThreshold = 6181u32,
        #[msg("MaxRevenueWithdrawPerPeriodReached")]
        MaxRevenueWithdrawPerPeriodReached = 6182u32,
        #[msg("InvalidSpotPositionDetected")]
        MaxIFWithdrawReached = 6183u32,
        #[msg("NoIFWithdrawAvailable")]
        NoIFWithdrawAvailable = 6184u32,
        #[msg("InvalidIFUnstake")]
        InvalidIFUnstake = 6185u32,
        #[msg("InvalidIFUnstakeSize")]
        InvalidIFUnstakeSize = 6186u32,
        #[msg("InvalidIFUnstakeCancel")]
        InvalidIFUnstakeCancel = 6187u32,
        #[msg("InvalidIFForNewStakes")]
        InvalidIFForNewStakes = 6188u32,
        #[msg("InvalidIFRebase")]
        InvalidIFRebase = 6189u32,
        #[msg("InvalidInsuranceUnstakeSize")]
        InvalidInsuranceUnstakeSize = 6190u32,
        #[msg("InvalidOrderLimitPrice")]
        InvalidOrderLimitPrice = 6191u32,
        #[msg("InvalidIFDetected")]
        InvalidIFDetected = 6192u32,
        #[msg("InvalidAmmMaxSpreadDetected")]
        InvalidAmmMaxSpreadDetected = 6193u32,
        #[msg("InvalidConcentrationCoef")]
        InvalidConcentrationCoef = 6194u32,
        #[msg("InvalidSrmVault")]
        InvalidSrmVault = 6195u32,
        #[msg("InvalidVaultOwner")]
        InvalidVaultOwner = 6196u32,
        #[msg("InvalidMarketStatusForFills")]
        InvalidMarketStatusForFills = 6197u32,
        #[msg("IFWithdrawRequestInProgress")]
        IFWithdrawRequestInProgress = 6198u32,
        #[msg("NoIFWithdrawRequestInProgress")]
        NoIFWithdrawRequestInProgress = 6199u32,
        #[msg("IFWithdrawRequestTooSmall")]
        IFWithdrawRequestTooSmall = 6200u32,
        #[msg("IncorrectSpotMarketAccountPassed")]
        IncorrectSpotMarketAccountPassed = 6201u32,
        #[msg("BlockchainClockInconsistency")]
        BlockchainClockInconsistency = 6202u32,
        #[msg("InvalidIFSharesDetected")]
        InvalidIFSharesDetected = 6203u32,
        #[msg("NewLPSizeTooSmall")]
        NewLPSizeTooSmall = 6204u32,
        #[msg("MarketStatusInvalidForNewLP")]
        MarketStatusInvalidForNewLP = 6205u32,
        #[msg("InvalidMarkTwapUpdateDetected")]
        InvalidMarkTwapUpdateDetected = 6206u32,
        #[msg("MarketSettlementAttemptOnActiveMarket")]
        MarketSettlementAttemptOnActiveMarket = 6207u32,
        #[msg("MarketSettlementRequiresSettledLP")]
        MarketSettlementRequiresSettledLP = 6208u32,
        #[msg("MarketSettlementAttemptTooEarly")]
        MarketSettlementAttemptTooEarly = 6209u32,
        #[msg("MarketSettlementTargetPriceInvalid")]
        MarketSettlementTargetPriceInvalid = 6210u32,
        #[msg("UnsupportedSpotMarket")]
        UnsupportedSpotMarket = 6211u32,
        #[msg("SpotOrdersDisabled")]
        SpotOrdersDisabled = 6212u32,
        #[msg("Market Being Initialized")]
        MarketBeingInitialized = 6213u32,
        #[msg("Invalid Sub Account Id")]
        InvalidUserSubAccountId = 6214u32,
        #[msg("Invalid Trigger Order Condition")]
        InvalidTriggerOrderCondition = 6215u32,
        #[msg("Invalid Spot Position")]
        InvalidSpotPosition = 6216u32,
        #[msg("Cant transfer between same user account")]
        CantTransferBetweenSameUserAccount = 6217u32,
        #[msg("Invalid Perp Position")]
        InvalidPerpPosition = 6218u32,
        #[msg("Unable To Get Limit Price")]
        UnableToGetLimitPrice = 6219u32,
        #[msg("Invalid Liquidation")]
        InvalidLiquidation = 6220u32,
        #[msg("Spot Fulfillment Config Disabled")]
        SpotFulfillmentConfigDisabled = 6221u32,
        #[msg("Invalid Maker")]
        InvalidMaker = 6222u32,
        #[msg("Failed Unwrap")]
        FailedUnwrap = 6223u32,
        #[msg("Max Number Of Users")]
        MaxNumberOfUsers = 6224u32,
        #[msg("InvalidOracleForSettlePnl")]
        InvalidOracleForSettlePnl = 6225u32,
        #[msg("MarginOrdersOpen")]
        MarginOrdersOpen = 6226u32,
        #[msg("TierViolationLiquidatingPerpPnl")]
        TierViolationLiquidatingPerpPnl = 6227u32,
        #[msg("CouldNotLoadUserData")]
        CouldNotLoadUserData = 6228u32,
        #[msg("UserWrongMutability")]
        UserWrongMutability = 6229u32,
        #[msg("InvalidUserAccount")]
        InvalidUserAccount = 6230u32,
        #[msg("CouldNotLoadUserData")]
        CouldNotLoadUserStatsData = 6231u32,
        #[msg("UserWrongMutability")]
        UserStatsWrongMutability = 6232u32,
        #[msg("InvalidUserAccount")]
        InvalidUserStatsAccount = 6233u32,
        #[msg("UserNotFound")]
        UserNotFound = 6234u32,
        #[msg("UnableToLoadUserAccount")]
        UnableToLoadUserAccount = 6235u32,
        #[msg("UserStatsNotFound")]
        UserStatsNotFound = 6236u32,
        #[msg("UnableToLoadUserStatsAccount")]
        UnableToLoadUserStatsAccount = 6237u32,
        #[msg("User Not Inactive")]
        UserNotInactive = 6238u32,
        #[msg("RevertFill")]
        RevertFill = 6239u32,
        #[msg("Invalid MarketAccount for Deletion")]
        InvalidMarketAccountforDeletion = 6240u32,
        #[msg("Invalid Spot Fulfillment Params")]
        InvalidSpotFulfillmentParams = 6241u32,
        #[msg("Failed to Get Mint")]
        FailedToGetMint = 6242u32,
        #[msg("FailedPhoenixCPI")]
        FailedPhoenixCPI = 6243u32,
        #[msg("FailedToDeserializePhoenixMarket")]
        FailedToDeserializePhoenixMarket = 6244u32,
        #[msg("InvalidPricePrecision")]
        InvalidPricePrecision = 6245u32,
        #[msg("InvalidPhoenixProgram")]
        InvalidPhoenixProgram = 6246u32,
        #[msg("InvalidPhoenixMarket")]
        InvalidPhoenixMarket = 6247u32,
        #[msg("InvalidSwap")]
        InvalidSwap = 6248u32,
        #[msg("SwapLimitPriceBreached")]
        SwapLimitPriceBreached = 6249u32,
        #[msg("SpotMarketReduceOnly")]
        SpotMarketReduceOnly = 6250u32,
        #[msg("FundingWasNotUpdated")]
        FundingWasNotUpdated = 6251u32,
        #[msg("ImpossibleFill")]
        ImpossibleFill = 6252u32,
        #[msg("CantUpdatePerpBidAskTwap")]
        CantUpdatePerpBidAskTwap = 6253u32,
        #[msg("UserReduceOnly")]
        UserReduceOnly = 6254u32,
        #[msg("InvalidMarginCalculation")]
        InvalidMarginCalculation = 6255u32,
        #[msg("CantPayUserInitFee")]
        CantPayUserInitFee = 6256u32,
        #[msg("CantReclaimRent")]
        CantReclaimRent = 6257u32,
        #[msg("InsuranceFundOperationPaused")]
        InsuranceFundOperationPaused = 6258u32,
        #[msg("NoUnsettledPnl")]
        NoUnsettledPnl = 6259u32,
        #[msg("PnlPoolCantSettleUser")]
        PnlPoolCantSettleUser = 6260u32,
        #[msg("OracleInvalid")]
        OracleNonPositive = 6261u32,
        #[msg("OracleTooVolatile")]
        OracleTooVolatile = 6262u32,
        #[msg("OracleTooUncertain")]
        OracleTooUncertain = 6263u32,
        #[msg("OracleStaleForMargin")]
        OracleStaleForMargin = 6264u32,
        #[msg("OracleInsufficientDataPoints")]
        OracleInsufficientDataPoints = 6265u32,
        #[msg("OracleStaleForAMM")]
        OracleStaleForAMM = 6266u32,
        #[msg("Unable to parse pull oracle message")]
        UnableToParsePullOracleMessage = 6267u32,
        #[msg("Can not borow more than max borrows")]
        MaxBorrows = 6268u32,
        #[msg("Updates must be monotonically increasing")]
        OracleUpdatesNotMonotonic = 6269u32,
        #[msg("Trying to update price feed with the wrong feed id")]
        OraclePriceFeedMessageMismatch = 6270u32,
        #[msg("The message in the update must be a PriceFeedMessage")]
        OracleUnsupportedMessageType = 6271u32,
        #[msg("Could not deserialize the message in the update")]
        OracleDeserializeMessageFailed = 6272u32,
        #[msg("Wrong guardian set owner in update price atomic")]
        OracleWrongGuardianSetOwner = 6273u32,
        #[msg("Oracle post update atomic price feed account must be drift program")]
        OracleWrongWriteAuthority = 6274u32,
        #[msg("Oracle vaa owner must be wormhole program")]
        OracleWrongVaaOwner = 6275u32,
        #[msg("Multi updates must have 2 or fewer accounts passed in remaining accounts")]
        OracleTooManyPriceAccountUpdates = 6276u32,
        #[msg("Don't have the same remaining accounts number and merkle price updates left")]
        OracleMismatchedVaaAndPriceUpdates = 6277u32,
        #[msg("Remaining account passed is not a valid pda")]
        OracleBadRemainingAccountPublicKey = 6278u32,
        #[msg("FailedOpenbookV2CPI")]
        FailedOpenbookV2CPI = 6279u32,
        #[msg("InvalidOpenbookV2Program")]
        InvalidOpenbookV2Program = 6280u32,
        #[msg("InvalidOpenbookV2Market")]
        InvalidOpenbookV2Market = 6281u32,
        #[msg("Non zero transfer fee")]
        NonZeroTransferFee = 6282u32,
        #[msg("Liquidation order failed to fill")]
        LiquidationOrderFailedToFill = 6283u32,
        #[msg("Invalid prediction market order")]
        InvalidPredictionMarketOrder = 6284u32,
    }
}
pub mod events {
    use super::{types::*, *};
    #[derive(InitSpace)]
    #[event]
    pub struct NewUserRecord {
        pub ts: i64,
        pub user_authority: Pubkey,
        pub user: Pubkey,
        pub sub_account_id: u16,
        pub name: [u8; 32],
        pub referrer: Pubkey,
    }
    #[derive(InitSpace)]
    #[event]
    pub struct DepositRecord {
        pub ts: i64,
        pub user_authority: Pubkey,
        pub user: Pubkey,
        pub direction: DepositDirection,
        pub deposit_record_id: u64,
        pub amount: u64,
        pub market_index: u16,
        pub oracle_price: i64,
        pub market_deposit_balance: u128,
        pub market_withdraw_balance: u128,
        pub market_cumulative_deposit_interest: u128,
        pub market_cumulative_borrow_interest: u128,
        pub total_deposits_after: u64,
        pub total_withdraws_after: u64,
        pub explanation: DepositExplanation,
        pub transfer_user: Option<Pubkey>,
    }
    #[derive(InitSpace)]
    #[event]
    pub struct SpotInterestRecord {
        pub ts: i64,
        pub market_index: u16,
        pub deposit_balance: u128,
        pub cumulative_deposit_interest: u128,
        pub borrow_balance: u128,
        pub cumulative_borrow_interest: u128,
        pub optimal_utilization: u32,
        pub optimal_borrow_rate: u32,
        pub max_borrow_rate: u32,
    }
    #[derive(InitSpace)]
    #[event]
    pub struct FundingPaymentRecord {
        pub ts: i64,
        pub user_authority: Pubkey,
        pub user: Pubkey,
        pub market_index: u16,
        pub funding_payment: i64,
        pub base_asset_amount: i64,
        pub user_last_cumulative_funding: i64,
        pub amm_cumulative_funding_long: i128,
        pub amm_cumulative_funding_short: i128,
    }
    #[derive(InitSpace)]
    #[event]
    pub struct FundingRateRecord {
        pub ts: i64,
        pub record_id: u64,
        pub market_index: u16,
        pub funding_rate: i64,
        pub funding_rate_long: i128,
        pub funding_rate_short: i128,
        pub cumulative_funding_rate_long: i128,
        pub cumulative_funding_rate_short: i128,
        pub oracle_price_twap: i64,
        pub mark_price_twap: u64,
        pub period_revenue: i64,
        pub base_asset_amount_with_amm: i128,
        pub base_asset_amount_with_unsettled_lp: i128,
    }
    #[derive(InitSpace)]
    #[event]
    pub struct CurveRecord {
        pub ts: i64,
        pub record_id: u64,
        pub peg_multiplier_before: u128,
        pub base_asset_reserve_before: u128,
        pub quote_asset_reserve_before: u128,
        pub sqrt_k_before: u128,
        pub peg_multiplier_after: u128,
        pub base_asset_reserve_after: u128,
        pub quote_asset_reserve_after: u128,
        pub sqrt_k_after: u128,
        pub base_asset_amount_long: u128,
        pub base_asset_amount_short: u128,
        pub base_asset_amount_with_amm: i128,
        pub total_fee: i128,
        pub total_fee_minus_distributions: i128,
        pub adjustment_cost: i128,
        pub oracle_price: i64,
        pub fill_record: u128,
        pub number_of_users: u32,
        pub market_index: u16,
    }
    #[derive(InitSpace)]
    #[event]
    pub struct OrderRecord {
        pub ts: i64,
        pub user: Pubkey,
        pub order: Order,
    }
    #[derive(InitSpace)]
    #[event]
    pub struct OrderActionRecord {
        pub ts: i64,
        pub action: OrderAction,
        pub action_explanation: OrderActionExplanation,
        pub market_index: u16,
        pub market_type: MarketType,
        pub filler: Option<Pubkey>,
        pub filler_reward: Option<u64>,
        pub fill_record_id: Option<u64>,
        pub base_asset_amount_filled: Option<u64>,
        pub quote_asset_amount_filled: Option<u64>,
        pub taker_fee: Option<u64>,
        pub maker_fee: Option<i64>,
        pub referrer_reward: Option<u32>,
        pub quote_asset_amount_surplus: Option<i64>,
        pub spot_fulfillment_method_fee: Option<u64>,
        pub taker: Option<Pubkey>,
        pub taker_order_id: Option<u32>,
        pub taker_order_direction: Option<PositionDirection>,
        pub taker_order_base_asset_amount: Option<u64>,
        pub taker_order_cumulative_base_asset_amount_filled: Option<u64>,
        pub taker_order_cumulative_quote_asset_amount_filled: Option<u64>,
        pub maker: Option<Pubkey>,
        pub maker_order_id: Option<u32>,
        pub maker_order_direction: Option<PositionDirection>,
        pub maker_order_base_asset_amount: Option<u64>,
        pub maker_order_cumulative_base_asset_amount_filled: Option<u64>,
        pub maker_order_cumulative_quote_asset_amount_filled: Option<u64>,
        pub oracle_price: i64,
    }
    #[derive(InitSpace)]
    #[event]
    pub struct LPRecord {
        pub ts: i64,
        pub user: Pubkey,
        pub action: LPAction,
        pub n_shares: u64,
        pub market_index: u16,
        pub delta_base_asset_amount: i64,
        pub delta_quote_asset_amount: i64,
        pub pnl: i64,
    }
    #[derive(InitSpace)]
    #[event]
    pub struct LiquidationRecord {
        pub ts: i64,
        pub liquidation_type: LiquidationType,
        pub user: Pubkey,
        pub liquidator: Pubkey,
        pub margin_requirement: u128,
        pub total_collateral: i128,
        pub margin_freed: u64,
        pub liquidation_id: u16,
        pub bankrupt: bool,
        pub canceled_order_ids: Vec<u32>,
        pub liquidate_perp: LiquidatePerpRecord,
        pub liquidate_spot: LiquidateSpotRecord,
        pub liquidate_borrow_for_perp_pnl: LiquidateBorrowForPerpPnlRecord,
        pub liquidate_perp_pnl_for_deposit: LiquidatePerpPnlForDepositRecord,
        pub perp_bankruptcy: PerpBankruptcyRecord,
        pub spot_bankruptcy: SpotBankruptcyRecord,
    }
    #[derive(InitSpace)]
    #[event]
    pub struct SettlePnlRecord {
        pub ts: i64,
        pub user: Pubkey,
        pub market_index: u16,
        pub pnl: i128,
        pub base_asset_amount: i64,
        pub quote_asset_amount_after: i64,
        pub quote_entry_amount: i64,
        pub settle_price: i64,
        pub explanation: SettlePnlExplanation,
    }
    #[derive(InitSpace)]
    #[event]
    pub struct InsuranceFundRecord {
        pub ts: i64,
        pub spot_market_index: u16,
        pub perp_market_index: u16,
        pub user_if_factor: u32,
        pub total_if_factor: u32,
        pub vault_amount_before: u64,
        pub insurance_vault_amount_before: u64,
        pub total_if_shares_before: u128,
        pub total_if_shares_after: u128,
        pub amount: i64,
    }
    #[derive(InitSpace)]
    #[event]
    pub struct InsuranceFundStakeRecord {
        pub ts: i64,
        pub user_authority: Pubkey,
        pub action: StakeAction,
        pub amount: u64,
        pub market_index: u16,
        pub insurance_vault_amount_before: u64,
        pub if_shares_before: u128,
        pub user_if_shares_before: u128,
        pub total_if_shares_before: u128,
        pub if_shares_after: u128,
        pub user_if_shares_after: u128,
        pub total_if_shares_after: u128,
    }
    #[derive(InitSpace)]
    #[event]
    pub struct SwapRecord {
        pub ts: i64,
        pub user: Pubkey,
        pub amount_out: u64,
        pub amount_in: u64,
        pub out_market_index: u16,
        pub in_market_index: u16,
        pub out_oracle_price: i64,
        pub in_oracle_price: i64,
        pub fee: u64,
    }
    #[derive(InitSpace)]
    #[event]
    pub struct SpotMarketVaultDepositRecord {
        pub ts: i64,
        pub market_index: u16,
        pub deposit_balance: u128,
        pub cumulative_deposit_interest_before: u128,
        pub cumulative_deposit_interest_after: u128,
        pub deposit_token_amount_before: u64,
        pub amount: u64,
    }
}
