#![allow(unused_imports)]
#![doc = r""]
#![doc = r" Auto-generated IDL types, manual edits do not persist (see `crates/drift-idl-gen`)"]
#![doc = r""]
use anchor_lang::{
    prelude::{
        account,
        borsh::{self},
        error_code, event, msg, AnchorDeserialize, AnchorSerialize, InitSpace,
    },
    Discriminator,
};
use serde::{Deserialize, Serialize};
use solana_sdk::{instruction::AccountMeta, pubkey::Pubkey};
pub const IDL_VERSION: &str = "2.137.0";
use self::traits::ToAccountMetas;
pub mod traits {
    use solana_sdk::instruction::AccountMeta;
    #[doc = r" This is distinct from the anchor_lang version of the trait"]
    #[doc = r" reimplemented to ensure the types used are from `solana`` crates _not_ the anchor_lang vendored versions which may be lagging behind"]
    pub trait ToAccountMetas {
        fn to_account_metas(&self) -> Vec<AccountMeta>;
    }
}
pub mod instructions {
    #![doc = r" IDL instruction types"]
    use super::{types::*, *};
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializeUser {
        pub sub_account_id: u16,
        pub name: [u8; 32],
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeUser {
        const DISCRIMINATOR: &[u8] = &[111, 17, 185, 250, 60, 122, 38, 254];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeUser {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializeUserStats {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeUserStats {
        const DISCRIMINATOR: &[u8] = &[254, 243, 72, 98, 251, 130, 168, 213];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeUserStats {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializeSignedMsgUserOrders {
        pub num_orders: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeSignedMsgUserOrders {
        const DISCRIMINATOR: &[u8] = &[164, 99, 156, 126, 156, 57, 99, 180];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeSignedMsgUserOrders {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct ResizeSignedMsgUserOrders {
        pub num_orders: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for ResizeSignedMsgUserOrders {
        const DISCRIMINATOR: &[u8] = &[137, 10, 87, 150, 18, 115, 79, 168];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for ResizeSignedMsgUserOrders {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializeSignedMsgWsDelegates {
        pub delegates: Vec<Pubkey>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeSignedMsgWsDelegates {
        const DISCRIMINATOR: &[u8] = &[40, 132, 96, 219, 184, 193, 80, 8];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeSignedMsgWsDelegates {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct ChangeSignedMsgWsDelegateStatus {
        pub delegate: Pubkey,
        pub add: bool,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for ChangeSignedMsgWsDelegateStatus {
        const DISCRIMINATOR: &[u8] = &[252, 202, 252, 219, 179, 27, 84, 138];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for ChangeSignedMsgWsDelegateStatus {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializeFuelOverflow {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeFuelOverflow {
        const DISCRIMINATOR: &[u8] = &[88, 223, 132, 161, 208, 88, 142, 42];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeFuelOverflow {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct SweepFuel {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for SweepFuel {
        const DISCRIMINATOR: &[u8] = &[175, 107, 19, 56, 165, 241, 43, 69];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for SweepFuel {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct ResetFuelSeason {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for ResetFuelSeason {
        const DISCRIMINATOR: &[u8] = &[199, 122, 192, 255, 32, 99, 63, 200];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for ResetFuelSeason {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializeReferrerName {
        pub name: [u8; 32],
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeReferrerName {
        const DISCRIMINATOR: &[u8] = &[235, 126, 231, 10, 42, 164, 26, 61];
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
        const DISCRIMINATOR: &[u8] = &[242, 35, 198, 137, 82, 225, 242, 182];
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
        const DISCRIMINATOR: &[u8] = &[183, 18, 70, 156, 148, 109, 161, 34];
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
        const DISCRIMINATOR: &[u8] = &[20, 20, 147, 223, 41, 63, 204, 111];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for TransferDeposit {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct TransferPools {
        pub deposit_from_market_index: u16,
        pub deposit_to_market_index: u16,
        pub borrow_from_market_index: u16,
        pub borrow_to_market_index: u16,
        pub deposit_amount: Option<u64>,
        pub borrow_amount: Option<u64>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for TransferPools {
        const DISCRIMINATOR: &[u8] = &[197, 103, 154, 25, 107, 90, 60, 94];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for TransferPools {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct TransferPerpPosition {
        pub market_index: u16,
        pub amount: Option<i64>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for TransferPerpPosition {
        const DISCRIMINATOR: &[u8] = &[23, 172, 188, 168, 134, 210, 3, 108];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for TransferPerpPosition {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PlacePerpOrder {
        pub params: OrderParams,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PlacePerpOrder {
        const DISCRIMINATOR: &[u8] = &[69, 161, 93, 202, 120, 126, 76, 185];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for PlacePerpOrder {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct CancelOrder {
        pub order_id: Option<u32>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for CancelOrder {
        const DISCRIMINATOR: &[u8] = &[95, 129, 237, 240, 8, 49, 223, 132];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for CancelOrder {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct CancelOrderByUserId {
        pub user_order_id: u8,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for CancelOrderByUserId {
        const DISCRIMINATOR: &[u8] = &[107, 211, 250, 133, 18, 37, 57, 100];
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
        const DISCRIMINATOR: &[u8] = &[238, 225, 95, 158, 227, 103, 8, 194];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for CancelOrders {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct CancelOrdersByIds {
        pub order_ids: Vec<u32>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for CancelOrdersByIds {
        const DISCRIMINATOR: &[u8] = &[134, 19, 144, 165, 94, 240, 210, 94];
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
        const DISCRIMINATOR: &[u8] = &[47, 124, 117, 255, 201, 197, 130, 94];
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
        const DISCRIMINATOR: &[u8] = &[158, 77, 4, 253, 252, 194, 161, 179];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for ModifyOrderByUserId {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PlaceAndTakePerpOrder {
        pub params: OrderParams,
        pub success_condition: Option<u32>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PlaceAndTakePerpOrder {
        const DISCRIMINATOR: &[u8] = &[213, 51, 1, 187, 108, 220, 230, 224];
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
        const DISCRIMINATOR: &[u8] = &[149, 117, 11, 237, 47, 95, 89, 237];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for PlaceAndMakePerpOrder {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PlaceAndMakeSignedMsgPerpOrder {
        pub params: OrderParams,
        pub signed_msg_order_uuid: [u8; 8],
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PlaceAndMakeSignedMsgPerpOrder {
        const DISCRIMINATOR: &[u8] = &[16, 26, 123, 131, 94, 29, 175, 98];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for PlaceAndMakeSignedMsgPerpOrder {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PlaceSignedMsgTakerOrder {
        pub signed_msg_order_params_message_bytes: Vec<u8>,
        pub is_delegate_signer: bool,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PlaceSignedMsgTakerOrder {
        const DISCRIMINATOR: &[u8] = &[32, 79, 101, 139, 25, 6, 98, 15];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for PlaceSignedMsgTakerOrder {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PlaceSpotOrder {
        pub params: OrderParams,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PlaceSpotOrder {
        const DISCRIMINATOR: &[u8] = &[45, 79, 81, 160, 248, 90, 91, 220];
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
        const DISCRIMINATOR: &[u8] = &[191, 3, 138, 71, 114, 198, 202, 100];
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
        const DISCRIMINATOR: &[u8] = &[149, 158, 85, 66, 239, 9, 243, 98];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for PlaceAndMakeSpotOrder {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PlaceOrders {
        pub params: Vec<OrderParams>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PlaceOrders {
        const DISCRIMINATOR: &[u8] = &[60, 63, 50, 123, 12, 197, 60, 190];
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
        const DISCRIMINATOR: &[u8] = &[174, 109, 228, 1, 242, 105, 232, 105];
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
        const DISCRIMINATOR: &[u8] = &[177, 184, 27, 193, 34, 13, 210, 145];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for EndSwap {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateUserName {
        pub sub_account_id: u16,
        pub name: [u8; 32],
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserName {
        const DISCRIMINATOR: &[u8] = &[135, 25, 185, 56, 165, 53, 34, 136];
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
        const DISCRIMINATOR: &[u8] = &[21, 221, 140, 187, 32, 129, 11, 123];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserCustomMarginRatio {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateUserPerpPositionCustomMarginRatio {
        pub sub_account_id: u16,
        pub perp_market_index: u16,
        pub margin_ratio: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserPerpPositionCustomMarginRatio {
        const DISCRIMINATOR: &[u8] = &[121, 137, 157, 155, 89, 186, 145, 113];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserPerpPositionCustomMarginRatio {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateUserMarginTradingEnabled {
        pub sub_account_id: u16,
        pub margin_trading_enabled: bool,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserMarginTradingEnabled {
        const DISCRIMINATOR: &[u8] = &[194, 92, 204, 223, 246, 188, 31, 203];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserMarginTradingEnabled {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateUserPoolId {
        pub sub_account_id: u16,
        pub pool_id: u8,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserPoolId {
        const DISCRIMINATOR: &[u8] = &[219, 86, 73, 106, 56, 218, 128, 109];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserPoolId {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateUserDelegate {
        pub sub_account_id: u16,
        pub delegate: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserDelegate {
        const DISCRIMINATOR: &[u8] = &[139, 205, 141, 141, 113, 36, 94, 187];
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
        const DISCRIMINATOR: &[u8] = &[199, 71, 42, 67, 144, 19, 86, 109];
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
        const DISCRIMINATOR: &[u8] = &[66, 80, 107, 186, 27, 242, 66, 95];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserAdvancedLp {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateUserProtectedMakerOrders {
        pub sub_account_id: u16,
        pub protected_maker_orders: bool,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserProtectedMakerOrders {
        const DISCRIMINATOR: &[u8] = &[114, 39, 123, 198, 187, 25, 90, 219];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserProtectedMakerOrders {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct DeleteUser {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for DeleteUser {
        const DISCRIMINATOR: &[u8] = &[186, 85, 17, 249, 219, 231, 98, 251];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for DeleteUser {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct ForceDeleteUser {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for ForceDeleteUser {
        const DISCRIMINATOR: &[u8] = &[2, 241, 195, 172, 227, 24, 254, 158];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for ForceDeleteUser {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct DeleteSignedMsgUserOrders {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for DeleteSignedMsgUserOrders {
        const DISCRIMINATOR: &[u8] = &[221, 247, 128, 253, 212, 254, 46, 153];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for DeleteSignedMsgUserOrders {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct ReclaimRent {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for ReclaimRent {
        const DISCRIMINATOR: &[u8] = &[218, 200, 19, 197, 227, 89, 192, 22];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for ReclaimRent {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct EnableUserHighLeverageMode {
        pub sub_account_id: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for EnableUserHighLeverageMode {
        const DISCRIMINATOR: &[u8] = &[231, 24, 230, 112, 201, 173, 73, 184];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for EnableUserHighLeverageMode {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct FillPerpOrder {
        pub order_id: Option<u32>,
        pub maker_order_id: Option<u32>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for FillPerpOrder {
        const DISCRIMINATOR: &[u8] = &[13, 188, 248, 103, 134, 217, 106, 240];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for FillPerpOrder {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct RevertFill {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for RevertFill {
        const DISCRIMINATOR: &[u8] = &[236, 238, 176, 69, 239, 10, 181, 193];
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
        const DISCRIMINATOR: &[u8] = &[212, 206, 130, 173, 21, 34, 199, 40];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for FillSpotOrder {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct TriggerOrder {
        pub order_id: u32,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for TriggerOrder {
        const DISCRIMINATOR: &[u8] = &[63, 112, 51, 233, 232, 47, 240, 199];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for TriggerOrder {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct ForceCancelOrders {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for ForceCancelOrders {
        const DISCRIMINATOR: &[u8] = &[64, 181, 196, 63, 222, 72, 64, 232];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for ForceCancelOrders {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateUserIdle {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserIdle {
        const DISCRIMINATOR: &[u8] = &[253, 133, 67, 22, 103, 161, 20, 100];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserIdle {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct LogUserBalances {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for LogUserBalances {
        const DISCRIMINATOR: &[u8] = &[162, 21, 35, 251, 32, 57, 161, 210];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for LogUserBalances {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct DisableUserHighLeverageMode {
        pub disable_maintenance: bool,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for DisableUserHighLeverageMode {
        const DISCRIMINATOR: &[u8] = &[183, 155, 45, 0, 226, 85, 213, 69];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for DisableUserHighLeverageMode {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateUserFuelBonus {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserFuelBonus {
        const DISCRIMINATOR: &[u8] = &[88, 175, 201, 190, 222, 100, 143, 57];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserFuelBonus {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateUserStatsReferrerStatus {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserStatsReferrerStatus {
        const DISCRIMINATOR: &[u8] = &[174, 154, 72, 42, 191, 148, 145, 205];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserStatsReferrerStatus {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateUserOpenOrdersCount {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserOpenOrdersCount {
        const DISCRIMINATOR: &[u8] = &[104, 39, 65, 210, 250, 163, 100, 134];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserOpenOrdersCount {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct AdminDisableUpdatePerpBidAskTwap {
        pub disable: bool,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for AdminDisableUpdatePerpBidAskTwap {
        const DISCRIMINATOR: &[u8] = &[17, 164, 82, 45, 183, 86, 191, 199];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for AdminDisableUpdatePerpBidAskTwap {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct SettlePnl {
        pub market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for SettlePnl {
        const DISCRIMINATOR: &[u8] = &[43, 61, 234, 45, 15, 95, 152, 153];
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
        const DISCRIMINATOR: &[u8] = &[127, 66, 117, 57, 40, 50, 152, 127];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for SettleMultiplePnls {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct SettleFundingPayment {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for SettleFundingPayment {
        const DISCRIMINATOR: &[u8] = &[222, 90, 202, 94, 28, 45, 115, 183];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for SettleFundingPayment {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct SettleExpiredMarket {
        pub market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for SettleExpiredMarket {
        const DISCRIMINATOR: &[u8] = &[120, 89, 11, 25, 122, 77, 72, 193];
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
        const DISCRIMINATOR: &[u8] = &[75, 35, 119, 247, 191, 18, 139, 2];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for LiquidatePerp {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct LiquidatePerpWithFill {
        pub market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for LiquidatePerpWithFill {
        const DISCRIMINATOR: &[u8] = &[95, 111, 124, 105, 86, 169, 187, 34];
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
        const DISCRIMINATOR: &[u8] = &[107, 0, 128, 41, 35, 229, 251, 18];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for LiquidateSpot {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct LiquidateSpotWithSwapBegin {
        pub asset_market_index: u16,
        pub liability_market_index: u16,
        pub swap_amount: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for LiquidateSpotWithSwapBegin {
        const DISCRIMINATOR: &[u8] = &[12, 43, 176, 83, 156, 251, 117, 13];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for LiquidateSpotWithSwapBegin {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct LiquidateSpotWithSwapEnd {
        pub asset_market_index: u16,
        pub liability_market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for LiquidateSpotWithSwapEnd {
        const DISCRIMINATOR: &[u8] = &[142, 88, 163, 160, 223, 75, 55, 225];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for LiquidateSpotWithSwapEnd {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct LiquidateBorrowForPerpPnl {
        pub perp_market_index: u16,
        pub spot_market_index: u16,
        pub liquidator_max_liability_transfer: u128,
        pub limit_price: Option<u64>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for LiquidateBorrowForPerpPnl {
        const DISCRIMINATOR: &[u8] = &[169, 17, 32, 90, 207, 148, 209, 27];
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
        const DISCRIMINATOR: &[u8] = &[237, 75, 198, 235, 233, 186, 75, 35];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for LiquidatePerpPnlForDeposit {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct SetUserStatusToBeingLiquidated {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for SetUserStatusToBeingLiquidated {
        const DISCRIMINATOR: &[u8] = &[106, 133, 160, 206, 193, 171, 192, 194];
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
        const DISCRIMINATOR: &[u8] = &[168, 204, 68, 150, 159, 126, 95, 148];
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
        const DISCRIMINATOR: &[u8] = &[224, 16, 176, 214, 162, 213, 183, 222];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for ResolvePerpBankruptcy {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct ResolveSpotBankruptcy {
        pub market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for ResolveSpotBankruptcy {
        const DISCRIMINATOR: &[u8] = &[124, 194, 240, 254, 198, 213, 52, 122];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for ResolveSpotBankruptcy {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct SettleRevenueToInsuranceFund {
        pub spot_market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for SettleRevenueToInsuranceFund {
        const DISCRIMINATOR: &[u8] = &[200, 120, 93, 136, 69, 38, 199, 159];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for SettleRevenueToInsuranceFund {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateFundingRate {
        pub market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateFundingRate {
        const DISCRIMINATOR: &[u8] = &[201, 178, 116, 212, 166, 144, 72, 238];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateFundingRate {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePrelaunchOracle {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePrelaunchOracle {
        const DISCRIMINATOR: &[u8] = &[220, 132, 27, 27, 233, 220, 61, 219];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePrelaunchOracle {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpBidAskTwap {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpBidAskTwap {
        const DISCRIMINATOR: &[u8] = &[247, 23, 255, 65, 212, 90, 221, 194];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpBidAskTwap {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketCumulativeInterest {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketCumulativeInterest {
        const DISCRIMINATOR: &[u8] = &[39, 166, 139, 243, 158, 165, 155, 225];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketCumulativeInterest {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateAmms {
        pub market_indexes: Vec<u16>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateAmms {
        const DISCRIMINATOR: &[u8] = &[201, 106, 217, 253, 4, 175, 228, 97];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateAmms {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketExpiry {
        pub expiry_ts: i64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketExpiry {
        const DISCRIMINATOR: &[u8] = &[208, 11, 211, 159, 226, 24, 11, 247];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketExpiry {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateUserQuoteAssetInsuranceStake {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserQuoteAssetInsuranceStake {
        const DISCRIMINATOR: &[u8] = &[251, 101, 156, 7, 2, 63, 30, 23];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserQuoteAssetInsuranceStake {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateUserGovTokenInsuranceStake {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserGovTokenInsuranceStake {
        const DISCRIMINATOR: &[u8] = &[143, 99, 235, 187, 20, 159, 184, 84];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserGovTokenInsuranceStake {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateUserGovTokenInsuranceStakeDevnet {
        pub gov_stake_amount: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserGovTokenInsuranceStakeDevnet {
        const DISCRIMINATOR: &[u8] = &[129, 185, 243, 183, 228, 111, 64, 175];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserGovTokenInsuranceStakeDevnet {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializeInsuranceFundStake {
        pub market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeInsuranceFundStake {
        const DISCRIMINATOR: &[u8] = &[187, 179, 243, 70, 248, 90, 92, 147];
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
        const DISCRIMINATOR: &[u8] = &[251, 144, 115, 11, 222, 47, 62, 236];
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
        const DISCRIMINATOR: &[u8] = &[142, 70, 204, 92, 73, 106, 180, 52];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for RequestRemoveInsuranceFundStake {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct CancelRequestRemoveInsuranceFundStake {
        pub market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for CancelRequestRemoveInsuranceFundStake {
        const DISCRIMINATOR: &[u8] = &[97, 235, 78, 62, 212, 42, 241, 127];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for CancelRequestRemoveInsuranceFundStake {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct RemoveInsuranceFundStake {
        pub market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for RemoveInsuranceFundStake {
        const DISCRIMINATOR: &[u8] = &[128, 166, 142, 9, 254, 187, 143, 174];
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
        const DISCRIMINATOR: &[u8] = &[94, 93, 226, 240, 195, 201, 184, 109];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for TransferProtocolIfShares {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct BeginInsuranceFundSwap {
        pub in_market_index: u16,
        pub out_market_index: u16,
        pub amount_in: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for BeginInsuranceFundSwap {
        const DISCRIMINATOR: &[u8] = &[176, 69, 143, 205, 32, 132, 163, 0];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for BeginInsuranceFundSwap {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct EndInsuranceFundSwap {
        pub in_market_index: u16,
        pub out_market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for EndInsuranceFundSwap {
        const DISCRIMINATOR: &[u8] = &[206, 230, 98, 8, 249, 158, 169, 167];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for EndInsuranceFundSwap {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct TransferProtocolIfSharesToRevenuePool {
        pub market_index: u16,
        pub amount: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for TransferProtocolIfSharesToRevenuePool {
        const DISCRIMINATOR: &[u8] = &[236, 136, 147, 153, 146, 205, 104, 29];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for TransferProtocolIfSharesToRevenuePool {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePythPullOracle {
        pub feed_id: [u8; 32],
        pub params: Vec<u8>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePythPullOracle {
        const DISCRIMINATOR: &[u8] = &[230, 191, 189, 94, 108, 59, 74, 197];
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
        const DISCRIMINATOR: &[u8] = &[116, 122, 137, 158, 224, 195, 173, 119];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for PostPythPullOracleUpdateAtomic {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PostMultiPythPullOracleUpdatesAtomic {
        pub params: Vec<u8>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PostMultiPythPullOracleUpdatesAtomic {
        const DISCRIMINATOR: &[u8] = &[243, 79, 204, 228, 227, 208, 100, 244];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for PostMultiPythPullOracleUpdatesAtomic {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PauseSpotMarketDepositWithdraw {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for PauseSpotMarketDepositWithdraw {
        const DISCRIMINATOR: &[u8] = &[183, 119, 59, 170, 137, 35, 242, 86];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for PauseSpotMarketDepositWithdraw {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct Initialize {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for Initialize {
        const DISCRIMINATOR: &[u8] = &[175, 175, 109, 31, 13, 152, 155, 237];
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
        const DISCRIMINATOR: &[u8] = &[234, 196, 128, 44, 94, 15, 48, 201];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeSpotMarket {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct DeleteInitializedSpotMarket {
        pub market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for DeleteInitializedSpotMarket {
        const DISCRIMINATOR: &[u8] = &[31, 140, 67, 191, 189, 20, 101, 221];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for DeleteInitializedSpotMarket {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializeSerumFulfillmentConfig {
        pub market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeSerumFulfillmentConfig {
        const DISCRIMINATOR: &[u8] = &[193, 211, 132, 172, 70, 171, 7, 94];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeSerumFulfillmentConfig {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSerumFulfillmentConfigStatus {
        pub status: SpotFulfillmentConfigStatus,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSerumFulfillmentConfigStatus {
        const DISCRIMINATOR: &[u8] = &[171, 109, 240, 251, 95, 1, 149, 89];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSerumFulfillmentConfigStatus {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializeOpenbookV2FulfillmentConfig {
        pub market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeOpenbookV2FulfillmentConfig {
        const DISCRIMINATOR: &[u8] = &[7, 221, 103, 153, 107, 57, 27, 197];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeOpenbookV2FulfillmentConfig {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct OpenbookV2FulfillmentConfigStatus {
        pub status: SpotFulfillmentConfigStatus,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for OpenbookV2FulfillmentConfigStatus {
        const DISCRIMINATOR: &[u8] = &[25, 173, 19, 189, 4, 211, 64, 238];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for OpenbookV2FulfillmentConfigStatus {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializePhoenixFulfillmentConfig {
        pub market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializePhoenixFulfillmentConfig {
        const DISCRIMINATOR: &[u8] = &[135, 132, 110, 107, 185, 160, 169, 154];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializePhoenixFulfillmentConfig {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PhoenixFulfillmentConfigStatus {
        pub status: SpotFulfillmentConfigStatus,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PhoenixFulfillmentConfigStatus {
        const DISCRIMINATOR: &[u8] = &[96, 31, 113, 32, 12, 203, 7, 154];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for PhoenixFulfillmentConfigStatus {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSerumVault {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSerumVault {
        const DISCRIMINATOR: &[u8] = &[219, 8, 246, 96, 169, 121, 91, 110];
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
        const DISCRIMINATOR: &[u8] = &[132, 9, 229, 118, 117, 118, 117, 62];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializePerpMarket {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializePredictionMarket {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializePredictionMarket {
        const DISCRIMINATOR: &[u8] = &[248, 70, 198, 224, 224, 105, 125, 195];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializePredictionMarket {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct DeleteInitializedPerpMarket {
        pub market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for DeleteInitializedPerpMarket {
        const DISCRIMINATOR: &[u8] = &[91, 154, 24, 87, 106, 59, 190, 66];
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
        const DISCRIMINATOR: &[u8] = &[235, 109, 2, 82, 219, 118, 6, 159];
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
        const DISCRIMINATOR: &[u8] = &[24, 87, 10, 115, 165, 190, 80, 139];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for RecenterPerpMarketAmm {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct RecenterPerpMarketAmmCrank {
        pub depth: Option<u128>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for RecenterPerpMarketAmmCrank {
        const DISCRIMINATOR: &[u8] = &[166, 19, 64, 10, 14, 51, 101, 122];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for RecenterPerpMarketAmmCrank {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketAmmSummaryStats {
        pub params: UpdatePerpMarketSummaryStatsParams,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketAmmSummaryStats {
        const DISCRIMINATOR: &[u8] = &[122, 101, 249, 238, 209, 9, 241, 245];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketAmmSummaryStats {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketExpiry {
        pub expiry_ts: i64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketExpiry {
        const DISCRIMINATOR: &[u8] = &[44, 221, 227, 151, 131, 140, 22, 110];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketExpiry {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct SettleExpiredMarketPoolsToRevenuePool {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for SettleExpiredMarketPoolsToRevenuePool {
        const DISCRIMINATOR: &[u8] = &[55, 19, 238, 169, 227, 90, 200, 184];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for SettleExpiredMarketPoolsToRevenuePool {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct DepositIntoPerpMarketFeePool {
        pub amount: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for DepositIntoPerpMarketFeePool {
        const DISCRIMINATOR: &[u8] = &[34, 58, 57, 68, 97, 80, 244, 6];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for DepositIntoPerpMarketFeePool {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketPnlPool {
        pub amount: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketPnlPool {
        const DISCRIMINATOR: &[u8] = &[50, 202, 249, 224, 166, 184, 13, 143];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketPnlPool {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct DepositIntoSpotMarketVault {
        pub amount: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for DepositIntoSpotMarketVault {
        const DISCRIMINATOR: &[u8] = &[48, 252, 119, 73, 255, 205, 174, 247];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for DepositIntoSpotMarketVault {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct DepositIntoSpotMarketRevenuePool {
        pub amount: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for DepositIntoSpotMarketRevenuePool {
        const DISCRIMINATOR: &[u8] = &[92, 40, 151, 42, 122, 254, 139, 246];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for DepositIntoSpotMarketRevenuePool {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct RepegAmmCurve {
        pub new_peg_candidate: u128,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for RepegAmmCurve {
        const DISCRIMINATOR: &[u8] = &[3, 36, 102, 89, 180, 128, 120, 213];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for RepegAmmCurve {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketAmmOracleTwap {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketAmmOracleTwap {
        const DISCRIMINATOR: &[u8] = &[241, 74, 114, 123, 206, 153, 24, 202];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketAmmOracleTwap {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct ResetPerpMarketAmmOracleTwap {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for ResetPerpMarketAmmOracleTwap {
        const DISCRIMINATOR: &[u8] = &[127, 10, 55, 164, 123, 226, 47, 24];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for ResetPerpMarketAmmOracleTwap {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateK {
        pub sqrt_k: u128,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateK {
        const DISCRIMINATOR: &[u8] = &[72, 98, 9, 139, 129, 229, 172, 56];
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
        const DISCRIMINATOR: &[u8] = &[130, 173, 107, 45, 119, 105, 26, 113];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketMarginRatio {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketHighLeverageMarginRatio {
        pub margin_ratio_initial: u16,
        pub margin_ratio_maintenance: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketHighLeverageMarginRatio {
        const DISCRIMINATOR: &[u8] = &[88, 112, 86, 49, 24, 116, 74, 157];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketHighLeverageMarginRatio {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketFundingPeriod {
        pub funding_period: i64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketFundingPeriod {
        const DISCRIMINATOR: &[u8] = &[171, 161, 69, 91, 129, 139, 161, 28];
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
        const DISCRIMINATOR: &[u8] = &[15, 206, 73, 133, 60, 8, 86, 89];
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
        const DISCRIMINATOR: &[u8] = &[90, 137, 9, 145, 41, 8, 148, 117];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketLiquidationFee {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateInsuranceFundUnstakingPeriod {
        pub insurance_fund_unstaking_period: i64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateInsuranceFundUnstakingPeriod {
        const DISCRIMINATOR: &[u8] = &[44, 69, 43, 226, 204, 223, 202, 52];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateInsuranceFundUnstakingPeriod {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketPoolId {
        pub pool_id: u8,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketPoolId {
        const DISCRIMINATOR: &[u8] = &[22, 213, 197, 160, 139, 193, 81, 149];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketPoolId {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketLiquidationFee {
        pub liquidator_fee: u32,
        pub if_liquidation_fee: u32,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketLiquidationFee {
        const DISCRIMINATOR: &[u8] = &[11, 13, 255, 53, 56, 136, 104, 177];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketLiquidationFee {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateWithdrawGuardThreshold {
        pub withdraw_guard_threshold: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateWithdrawGuardThreshold {
        const DISCRIMINATOR: &[u8] = &[56, 18, 39, 61, 155, 211, 44, 133];
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
        const DISCRIMINATOR: &[u8] = &[147, 30, 224, 34, 18, 230, 105, 4];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketIfFactor {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketRevenueSettlePeriod {
        pub revenue_settle_period: i64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketRevenueSettlePeriod {
        const DISCRIMINATOR: &[u8] = &[81, 92, 126, 41, 250, 225, 156, 219];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketRevenueSettlePeriod {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketStatus {
        pub status: MarketStatus,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketStatus {
        const DISCRIMINATOR: &[u8] = &[78, 94, 16, 188, 193, 110, 231, 31];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketStatus {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketPausedOperations {
        pub paused_operations: u8,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketPausedOperations {
        const DISCRIMINATOR: &[u8] = &[100, 61, 153, 81, 180, 12, 6, 248];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketPausedOperations {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketAssetTier {
        pub asset_tier: AssetTier,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketAssetTier {
        const DISCRIMINATOR: &[u8] = &[253, 209, 231, 14, 242, 208, 243, 130];
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
        const DISCRIMINATOR: &[u8] = &[109, 33, 87, 195, 255, 36, 6, 81];
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
        const DISCRIMINATOR: &[u8] = &[71, 239, 236, 153, 210, 62, 254, 76];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketBorrowRate {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketMaxTokenDeposits {
        pub max_token_deposits: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketMaxTokenDeposits {
        const DISCRIMINATOR: &[u8] = &[56, 191, 79, 18, 26, 121, 80, 208];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketMaxTokenDeposits {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketMaxTokenBorrows {
        pub max_token_borrows_fraction: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketMaxTokenBorrows {
        const DISCRIMINATOR: &[u8] = &[57, 102, 204, 212, 253, 95, 13, 199];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketMaxTokenBorrows {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketScaleInitialAssetWeightStart {
        pub scale_initial_asset_weight_start: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketScaleInitialAssetWeightStart {
        const DISCRIMINATOR: &[u8] = &[217, 204, 204, 118, 204, 130, 225, 147];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketScaleInitialAssetWeightStart {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketOracle {
        pub oracle: Pubkey,
        pub oracle_source: OracleSource,
        pub skip_invariant_check: bool,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketOracle {
        const DISCRIMINATOR: &[u8] = &[114, 184, 102, 37, 246, 186, 180, 99];
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
        const DISCRIMINATOR: &[u8] = &[238, 153, 137, 80, 206, 59, 250, 61];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketStepSizeAndTickSize {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketMinOrderSize {
        pub order_size: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketMinOrderSize {
        const DISCRIMINATOR: &[u8] = &[93, 128, 11, 119, 26, 20, 181, 50];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketMinOrderSize {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketOrdersEnabled {
        pub orders_enabled: bool,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketOrdersEnabled {
        const DISCRIMINATOR: &[u8] = &[190, 79, 206, 15, 26, 229, 229, 43];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketOrdersEnabled {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketIfPausedOperations {
        pub paused_operations: u8,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketIfPausedOperations {
        const DISCRIMINATOR: &[u8] = &[101, 215, 79, 74, 59, 41, 79, 12];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketIfPausedOperations {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketName {
        pub name: [u8; 32],
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketName {
        const DISCRIMINATOR: &[u8] = &[17, 208, 1, 1, 162, 211, 188, 224];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketName {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketStatus {
        pub status: MarketStatus,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketStatus {
        const DISCRIMINATOR: &[u8] = &[71, 201, 175, 122, 255, 207, 196, 207];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketStatus {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketPausedOperations {
        pub paused_operations: u8,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketPausedOperations {
        const DISCRIMINATOR: &[u8] = &[53, 16, 136, 132, 30, 220, 121, 85];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketPausedOperations {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketContractTier {
        pub contract_tier: ContractTier,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketContractTier {
        const DISCRIMINATOR: &[u8] = &[236, 128, 15, 95, 203, 214, 68, 117];
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
        const DISCRIMINATOR: &[u8] = &[207, 194, 56, 132, 35, 67, 71, 244];
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
        const DISCRIMINATOR: &[u8] = &[135, 132, 205, 165, 109, 150, 166, 106];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketUnrealizedAssetWeight {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketConcentrationCoef {
        pub concentration_scale: u128,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketConcentrationCoef {
        const DISCRIMINATOR: &[u8] = &[24, 78, 232, 126, 169, 176, 230, 16];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketConcentrationCoef {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketCurveUpdateIntensity {
        pub curve_update_intensity: u8,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketCurveUpdateIntensity {
        const DISCRIMINATOR: &[u8] = &[50, 131, 6, 156, 226, 231, 189, 72];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketCurveUpdateIntensity {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateLpCooldownTime {
        pub lp_cooldown_time: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateLpCooldownTime {
        const DISCRIMINATOR: &[u8] = &[198, 133, 88, 41, 241, 119, 61, 14];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateLpCooldownTime {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpFeeStructure {
        pub fee_structure: FeeStructure,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpFeeStructure {
        const DISCRIMINATOR: &[u8] = &[23, 178, 111, 203, 73, 22, 140, 75];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpFeeStructure {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotFeeStructure {
        pub fee_structure: FeeStructure,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotFeeStructure {
        const DISCRIMINATOR: &[u8] = &[97, 216, 105, 131, 113, 246, 142, 141];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotFeeStructure {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateInitialPctToLiquidate {
        pub initial_pct_to_liquidate: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateInitialPctToLiquidate {
        const DISCRIMINATOR: &[u8] = &[210, 133, 225, 128, 194, 50, 13, 109];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateInitialPctToLiquidate {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateLiquidationDuration {
        pub liquidation_duration: u8,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateLiquidationDuration {
        const DISCRIMINATOR: &[u8] = &[28, 154, 20, 249, 102, 192, 73, 71];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateLiquidationDuration {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateLiquidationMarginBufferRatio {
        pub liquidation_margin_buffer_ratio: u32,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateLiquidationMarginBufferRatio {
        const DISCRIMINATOR: &[u8] = &[132, 224, 243, 160, 154, 82, 97, 215];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateLiquidationMarginBufferRatio {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateOracleGuardRails {
        pub oracle_guard_rails: OracleGuardRails,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateOracleGuardRails {
        const DISCRIMINATOR: &[u8] = &[131, 112, 10, 59, 32, 54, 40, 164];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateOracleGuardRails {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateStateSettlementDuration {
        pub settlement_duration: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateStateSettlementDuration {
        const DISCRIMINATOR: &[u8] = &[97, 68, 199, 235, 131, 80, 61, 173];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateStateSettlementDuration {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateStateMaxNumberOfSubAccounts {
        pub max_number_of_sub_accounts: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateStateMaxNumberOfSubAccounts {
        const DISCRIMINATOR: &[u8] = &[155, 123, 214, 2, 221, 166, 204, 85];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateStateMaxNumberOfSubAccounts {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateStateMaxInitializeUserFee {
        pub max_initialize_user_fee: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateStateMaxInitializeUserFee {
        const DISCRIMINATOR: &[u8] = &[237, 225, 25, 237, 193, 45, 77, 97];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateStateMaxInitializeUserFee {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketOracle {
        pub oracle: Pubkey,
        pub oracle_source: OracleSource,
        pub skip_invariant_check: bool,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketOracle {
        const DISCRIMINATOR: &[u8] = &[182, 113, 111, 160, 67, 174, 89, 191];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketOracle {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketBaseSpread {
        pub base_spread: u32,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketBaseSpread {
        const DISCRIMINATOR: &[u8] = &[71, 95, 84, 168, 9, 157, 198, 65];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketBaseSpread {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateAmmJitIntensity {
        pub amm_jit_intensity: u8,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateAmmJitIntensity {
        const DISCRIMINATOR: &[u8] = &[181, 191, 53, 109, 166, 249, 55, 142];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateAmmJitIntensity {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketMaxSpread {
        pub max_spread: u32,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketMaxSpread {
        const DISCRIMINATOR: &[u8] = &[80, 252, 122, 62, 40, 218, 91, 100];
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
        const DISCRIMINATOR: &[u8] = &[231, 255, 97, 25, 146, 139, 174, 4];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketStepSizeAndTickSize {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketName {
        pub name: [u8; 32],
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketName {
        const DISCRIMINATOR: &[u8] = &[211, 31, 21, 210, 64, 108, 66, 201];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketName {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketMinOrderSize {
        pub order_size: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketMinOrderSize {
        const DISCRIMINATOR: &[u8] = &[226, 74, 5, 89, 108, 223, 46, 141];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketMinOrderSize {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketMaxSlippageRatio {
        pub max_slippage_ratio: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketMaxSlippageRatio {
        const DISCRIMINATOR: &[u8] = &[235, 37, 40, 196, 70, 146, 54, 201];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketMaxSlippageRatio {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketMaxFillReserveFraction {
        pub max_fill_reserve_fraction: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketMaxFillReserveFraction {
        const DISCRIMINATOR: &[u8] = &[19, 172, 114, 154, 42, 135, 161, 133];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketMaxFillReserveFraction {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketMaxOpenInterest {
        pub max_open_interest: u128,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketMaxOpenInterest {
        const DISCRIMINATOR: &[u8] = &[194, 79, 149, 224, 246, 102, 186, 140];
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
        const DISCRIMINATOR: &[u8] = &[35, 62, 144, 177, 180, 62, 215, 196];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketNumberOfUsers {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketFeeAdjustment {
        pub fee_adjustment: i16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketFeeAdjustment {
        const DISCRIMINATOR: &[u8] = &[194, 174, 87, 102, 43, 148, 32, 112];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketFeeAdjustment {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketFeeAdjustment {
        pub fee_adjustment: i16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketFeeAdjustment {
        const DISCRIMINATOR: &[u8] = &[148, 182, 3, 126, 157, 114, 220, 99];
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
        const DISCRIMINATOR: &[u8] = &[252, 141, 110, 101, 27, 99, 182, 21];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketFuel {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketProtectedMakerParams {
        pub protected_maker_limit_price_divisor: Option<u8>,
        pub protected_maker_dynamic_divisor: Option<u8>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketProtectedMakerParams {
        const DISCRIMINATOR: &[u8] = &[249, 213, 115, 34, 253, 239, 75, 173];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketProtectedMakerParams {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketTakerSpeedBumpOverride {
        pub taker_speed_bump_override: i8,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketTakerSpeedBumpOverride {
        const DISCRIMINATOR: &[u8] = &[31, 39, 5, 25, 228, 50, 1, 0];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketTakerSpeedBumpOverride {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketAmmSpreadAdjustment {
        pub amm_spread_adjustment: i8,
        pub amm_inventory_spread_adjustment: i8,
        pub reference_price_offset: i32,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketAmmSpreadAdjustment {
        const DISCRIMINATOR: &[u8] = &[155, 195, 149, 43, 220, 82, 173, 205];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketAmmSpreadAdjustment {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketOracleSlotDelayOverride {
        pub oracle_slot_delay_override: i8,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketOracleSlotDelayOverride {
        const DISCRIMINATOR: &[u8] = &[165, 91, 239, 227, 63, 172, 227, 8];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketOracleSlotDelayOverride {}
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
        const DISCRIMINATOR: &[u8] = &[226, 253, 76, 71, 17, 2, 171, 169];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketFuel {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitUserFuel {
        pub fuel_boost_deposits: Option<i32>,
        pub fuel_boost_borrows: Option<u32>,
        pub fuel_boost_taker: Option<u32>,
        pub fuel_boost_maker: Option<u32>,
        pub fuel_boost_insurance: Option<u32>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitUserFuel {
        const DISCRIMINATOR: &[u8] = &[132, 191, 228, 141, 201, 138, 60, 48];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitUserFuel {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateAdmin {
        pub admin: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateAdmin {
        const DISCRIMINATOR: &[u8] = &[161, 176, 40, 213, 60, 184, 179, 228];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateAdmin {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateWhitelistMint {
        pub whitelist_mint: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateWhitelistMint {
        const DISCRIMINATOR: &[u8] = &[161, 15, 162, 19, 148, 120, 144, 151];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateWhitelistMint {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateDiscountMint {
        pub discount_mint: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateDiscountMint {
        const DISCRIMINATOR: &[u8] = &[32, 252, 122, 211, 66, 31, 47, 241];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateDiscountMint {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateExchangeStatus {
        pub exchange_status: u8,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateExchangeStatus {
        const DISCRIMINATOR: &[u8] = &[83, 160, 252, 250, 129, 116, 49, 223];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateExchangeStatus {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpAuctionDuration {
        pub min_perp_auction_duration: u8,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpAuctionDuration {
        const DISCRIMINATOR: &[u8] = &[126, 110, 52, 174, 30, 206, 215, 90];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpAuctionDuration {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotAuctionDuration {
        pub default_spot_auction_duration: u8,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotAuctionDuration {
        const DISCRIMINATOR: &[u8] = &[182, 178, 203, 72, 187, 143, 157, 107];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotAuctionDuration {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializeProtocolIfSharesTransferConfig {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeProtocolIfSharesTransferConfig {
        const DISCRIMINATOR: &[u8] = &[89, 131, 239, 200, 178, 141, 106, 194];
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
        const DISCRIMINATOR: &[u8] = &[34, 135, 47, 91, 220, 24, 212, 53];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateProtocolIfSharesTransferConfig {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializePrelaunchOracle {
        pub params: PrelaunchOracleParams,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializePrelaunchOracle {
        const DISCRIMINATOR: &[u8] = &[169, 178, 84, 25, 175, 62, 29, 247];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializePrelaunchOracle {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePrelaunchOracleParams {
        pub params: PrelaunchOracleParams,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePrelaunchOracleParams {
        const DISCRIMINATOR: &[u8] = &[98, 205, 147, 243, 18, 75, 83, 207];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePrelaunchOracleParams {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct DeletePrelaunchOracle {
        pub perp_market_index: u16,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for DeletePrelaunchOracle {
        const DISCRIMINATOR: &[u8] = &[59, 169, 100, 49, 69, 17, 173, 253];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for DeletePrelaunchOracle {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializePythPullOracle {
        pub feed_id: [u8; 32],
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializePythPullOracle {
        const DISCRIMINATOR: &[u8] = &[249, 140, 253, 243, 248, 74, 240, 238];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializePythPullOracle {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializePythLazerOracle {
        pub feed_id: u32,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializePythLazerOracle {
        const DISCRIMINATOR: &[u8] = &[140, 107, 33, 214, 235, 219, 103, 20];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializePythLazerOracle {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PostPythLazerOracleUpdate {
        pub pyth_message: Vec<u8>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PostPythLazerOracleUpdate {
        const DISCRIMINATOR: &[u8] = &[218, 237, 170, 245, 39, 143, 166, 33];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for PostPythLazerOracleUpdate {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializeHighLeverageModeConfig {
        pub max_users: u32,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeHighLeverageModeConfig {
        const DISCRIMINATOR: &[u8] = &[213, 167, 93, 246, 208, 130, 90, 248];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeHighLeverageModeConfig {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateHighLeverageModeConfig {
        pub max_users: u32,
        pub reduce_only: bool,
        pub current_users: Option<u32>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateHighLeverageModeConfig {
        const DISCRIMINATOR: &[u8] = &[64, 122, 212, 93, 141, 217, 202, 55];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateHighLeverageModeConfig {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializeProtectedMakerModeConfig {
        pub max_users: u32,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeProtectedMakerModeConfig {
        const DISCRIMINATOR: &[u8] = &[67, 103, 220, 67, 88, 32, 252, 8];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeProtectedMakerModeConfig {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateProtectedMakerModeConfig {
        pub max_users: u32,
        pub reduce_only: bool,
        pub current_users: Option<u32>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateProtectedMakerModeConfig {
        const DISCRIMINATOR: &[u8] = &[86, 166, 235, 253, 67, 202, 223, 17];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateProtectedMakerModeConfig {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct AdminDeposit {
        pub market_index: u16,
        pub amount: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for AdminDeposit {
        const DISCRIMINATOR: &[u8] = &[210, 66, 65, 182, 102, 214, 176, 30];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for AdminDeposit {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializeIfRebalanceConfig {
        pub params: IfRebalanceConfigParams,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeIfRebalanceConfig {
        const DISCRIMINATOR: &[u8] = &[8, 85, 184, 167, 176, 61, 173, 226];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeIfRebalanceConfig {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateIfRebalanceConfig {
        pub params: IfRebalanceConfigParams,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateIfRebalanceConfig {
        const DISCRIMINATOR: &[u8] = &[142, 245, 249, 66, 249, 181, 22, 83];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateIfRebalanceConfig {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateFeatureBitFlagsMmOracle {
        pub enable: bool,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateFeatureBitFlagsMmOracle {
        const DISCRIMINATOR: &[u8] = &[218, 134, 33, 186, 231, 59, 130, 149];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateFeatureBitFlagsMmOracle {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct ZeroMmOracleFields {}
    #[automatically_derived]
    impl anchor_lang::Discriminator for ZeroMmOracleFields {
        const DISCRIMINATOR: &[u8] = &[192, 226, 39, 204, 207, 120, 148, 250];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for ZeroMmOracleFields {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateFeatureBitFlagsMedianTriggerPrice {
        pub enable: bool,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateFeatureBitFlagsMedianTriggerPrice {
        const DISCRIMINATOR: &[u8] = &[64, 185, 221, 45, 87, 147, 12, 19];
    }
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateFeatureBitFlagsMedianTriggerPrice {}
}
pub mod types {
    #![doc = r" IDL types"]
    use super::*;
    use std::ops::Mul;
    #[doc = ""]
    #[doc = " backwards compatible u128 deserializing data from rust <=1.76.0 when u/i128 was 8-byte aligned"]
    #[doc = " https://solana.stackexchange.com/questions/7720/using-u128-without-sacrificing-alignment-8"]
    #[derive(
        Default,
        PartialEq,
        AnchorSerialize,
        AnchorDeserialize,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        bytemuck :: Zeroable,
        bytemuck :: Pod,
        Debug,
    )]
    #[repr(C)]
    pub struct u128(pub [u8; 16]);
    impl u128 {
        #[doc = " convert self into the std `u128` type"]
        pub fn as_u128(&self) -> std::primitive::u128 {
            std::primitive::u128::from_le_bytes(self.0)
        }
    }
    impl From<std::primitive::u128> for self::u128 {
        fn from(value: std::primitive::u128) -> Self {
            Self(value.to_le_bytes())
        }
    }
    #[doc = " backwards compatible i128 deserializing data from rust <=1.76.0 when u/i128 was 8-byte aligned"]
    #[doc = " https://solana.stackexchange.com/questions/7720/using-u128-without-sacrificing-alignment-8"]
    #[derive(
        Default,
        PartialEq,
        AnchorSerialize,
        AnchorDeserialize,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        bytemuck :: Zeroable,
        bytemuck :: Pod,
        Debug,
    )]
    #[repr(C)]
    pub struct i128(pub [u8; 16]);
    impl i128 {
        #[doc = " convert self into the std `i128` type"]
        pub fn as_i128(&self) -> core::primitive::i128 {
            core::primitive::i128::from_le_bytes(self.0)
        }
    }
    impl From<core::primitive::i128> for i128 {
        fn from(value: core::primitive::i128) -> Self {
            Self(value.to_le_bytes())
        }
    }
    #[repr(transparent)]
    #[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone, PartialEq, Debug)]
    pub struct Signature(pub [u8; 64]);
    impl Default for Signature {
        fn default() -> Self {
            Self([0_u8; 64])
        }
    }
    impl serde::Serialize for Signature {
        fn serialize<S: serde::Serializer>(
            &self,
            serializer: S,
        ) -> std::result::Result<S::Ok, S::Error> {
            serializer.serialize_bytes(&self.0)
        }
    }
    impl<'de> serde::Deserialize<'de> for Signature {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
            let s = <&[u8]>::deserialize(d)?;
            s.try_into()
                .map(Signature)
                .map_err(serde::de::Error::custom)
        }
    }
    impl anchor_lang::Space for Signature {
        const INIT_SPACE: usize = 8 * 64;
    }
    #[doc = " wrapper around fixed array types used for padding with `Default` implementation"]
    #[repr(transparent)]
    #[derive(AnchorDeserialize, AnchorSerialize, Copy, Clone, PartialEq)]
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
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub struct UpdatePerpMarketSummaryStatsParams {
        pub quote_asset_amount_with_unsettled_lp: Option<i64>,
        pub net_unsettled_funding_pnl: Option<i64>,
        pub update_amm_summary_stats: Option<bool>,
        pub exclude_total_liq_fee: Option<bool>,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
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
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
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
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
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
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
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
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
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
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub struct SpotBankruptcyRecord {
        pub market_index: u16,
        pub borrow_amount: u128,
        pub if_payment: u128,
        pub cumulative_deposit_interest_delta: u128,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub struct IfRebalanceConfigParams {
        pub total_in_amount: u64,
        pub epoch_max_in_amount: u64,
        pub epoch_duration: i64,
        pub out_market_index: u16,
        pub in_market_index: u16,
        pub max_slippage_bps: u16,
        pub swap_mode: u8,
        pub status: u8,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub struct MarketIdentifier {
        pub market_type: MarketType,
        pub market_index: u16,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
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
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
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
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub struct PrelaunchOracleParams {
        pub perp_market_index: u16,
        pub price: Option<i64>,
        pub max_price: Option<i64>,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
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
        pub bit_flags: u8,
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
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub struct SignedMsgOrderParamsMessage {
        pub signed_msg_order_params: OrderParams,
        pub sub_account_id: u16,
        pub slot: u64,
        pub uuid: [u8; 8],
        pub take_profit_order_params: Option<SignedMsgTriggerOrderParams>,
        pub stop_loss_order_params: Option<SignedMsgTriggerOrderParams>,
        pub max_margin_ratio: Option<u16>,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub struct SignedMsgOrderParamsDelegateMessage {
        pub signed_msg_order_params: OrderParams,
        pub taker_pubkey: Pubkey,
        pub slot: u64,
        pub uuid: [u8; 8],
        pub take_profit_order_params: Option<SignedMsgTriggerOrderParams>,
        pub stop_loss_order_params: Option<SignedMsgTriggerOrderParams>,
        pub max_margin_ratio: Option<u16>,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub struct SignedMsgTriggerOrderParams {
        pub trigger_price: u64,
        pub base_asset_amount: u64,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub struct ModifyOrderParams {
        pub direction: Option<PositionDirection>,
        pub base_asset_amount: Option<u64>,
        pub price: Option<u64>,
        pub reduce_only: Option<bool>,
        pub post_only: Option<PostOnlyParam>,
        pub bit_flags: Option<u8>,
        pub max_ts: Option<i64>,
        pub trigger_price: Option<u64>,
        pub trigger_condition: Option<OrderTriggerCondition>,
        pub oracle_price_offset: Option<i32>,
        pub auction_duration: Option<u8>,
        pub auction_start_price: Option<i64>,
        pub auction_end_price: Option<i64>,
        pub policy: Option<u8>,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
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
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub struct PoolBalance {
        pub scaled_balance: u128,
        pub market_index: u16,
        pub padding: [u8; 6],
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
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
        pub mm_oracle_slot: u64,
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
        pub mm_oracle_price: i64,
        pub max_fill_reserve_fraction: u16,
        pub max_slippage_ratio: u16,
        pub curve_update_intensity: u8,
        pub amm_jit_intensity: u8,
        pub oracle_source: OracleSource,
        pub last_oracle_valid: bool,
        pub target_base_asset_amount_per_lp: i32,
        pub per_lp_base: i8,
        pub taker_speed_bump_override: i8,
        pub amm_spread_adjustment: i8,
        pub oracle_slot_delay_override: i8,
        pub mm_oracle_sequence_id: u64,
        pub net_unsettled_funding_pnl: i64,
        pub quote_asset_amount_with_unsettled_lp: i64,
        pub reference_price_offset: i32,
        pub amm_inventory_spread_adjustment: i8,
        pub padding: [u8; 3],
        pub last_funding_oracle_twap: i64,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub struct SignedMsgOrderId {
        pub uuid: [u8; 8],
        pub max_slot: u64,
        pub order_id: u32,
        pub padding: u32,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub struct SignedMsgUserOrdersFixed {
        pub user_pubkey: Pubkey,
        pub padding: u32,
        pub len: u32,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
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
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub struct OracleGuardRails {
        pub price_divergence: PriceDivergenceGuardRails,
        pub validity: ValidityGuardRails,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub struct PriceDivergenceGuardRails {
        pub mark_oracle_percent_divergence: u64,
        pub oracle_twap5min_percent_divergence: u64,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub struct ValidityGuardRails {
        pub slots_before_stale_for_amm: i64,
        pub slots_before_stale_for_margin: i64,
        pub confidence_interval_max_size: u64,
        pub too_volatile_ratio: i64,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub struct FeeStructure {
        pub fee_tiers: [FeeTier; 10],
        pub filler_reward_structure: OrderFillerRewardStructure,
        pub referrer_reward_epoch_upper_bound: u64,
        pub flat_filler_fee: u64,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
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
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub struct OrderFillerRewardStructure {
        pub reward_numerator: u32,
        pub reward_denominator: u32,
        pub time_based_reward_lower_bound: u128,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
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
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
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
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
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
        pub padding: [u8; 2],
        pub max_margin_ratio: u16,
        pub market_index: u16,
        pub open_orders: u8,
        pub per_lp_base: i8,
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
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
        pub posted_slot_tail: u8,
        pub bit_flags: u8,
        pub padding: [u8; 1],
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum SwapDirection {
        #[default]
        Add,
        Remove,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum ModifyOrderId {
        #[default]
        UserOrderId,
        OrderId,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum PositionDirection {
        #[default]
        Long,
        Short,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum SpotFulfillmentType {
        #[default]
        SerumV3,
        Match,
        PhoenixV1,
        OpenbookV2,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum SwapReduceOnly {
        #[default]
        In,
        Out,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum TwapPeriod {
        #[default]
        FundingPeriod,
        FiveMin,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum LiquidationMultiplierType {
        #[default]
        Discount,
        Premium,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum MarginRequirementType {
        #[default]
        Initial,
        Fill,
        Maintenance,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
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
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
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
        UseMMOraclePrice,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum LogMode {
        #[default]
        None,
        ExchangeOracle,
        MMOracle,
        SafeMMOracle,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum PositionUpdateType {
        #[default]
        Open,
        Increase,
        Reduce,
        Close,
        Flip,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum DepositExplanation {
        #[default]
        None,
        Transfer,
        Borrow,
        RepayBorrow,
        Reward,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum DepositDirection {
        #[default]
        Deposit,
        Withdraw,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum OrderAction {
        #[default]
        Place,
        Cancel,
        Fill,
        Trigger,
        Expire,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
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
        TransferPerpPosition,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum LPAction {
        #[default]
        AddLiquidity,
        RemoveLiquidity,
        SettleLiquidity,
        RemoveLiquidityDerisk,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
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
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum SettlePnlExplanation {
        #[default]
        None,
        ExpiredPosition,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
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
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum FillMode {
        #[default]
        Fill,
        PlaceAndMake,
        PlaceAndTake,
        Liquidation,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum PerpFulfillmentMethod {
        #[default]
        AMM,
        Match,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum SpotFulfillmentMethod {
        #[default]
        ExternalMarket,
        Match,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Debug,
        PartialEq,
    )]
    pub enum MarginCalculationMode {
        Standard {
            track_open_orders_fraction: bool,
        },
        Liquidation {
            market_to_track_margin_requirement: Option<MarketIdentifier>,
        },
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
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
        PythLazer,
        PythLazer1K,
        PythLazer1M,
        PythLazerStableCoin,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum OrderParamsBitFlag {
        #[default]
        ImmediateOrCancel,
        UpdateHighLeverageMode,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum PostOnlyParam {
        #[default]
        None,
        MustPostOnly,
        TryPostOnly,
        Slide,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum ModifyOrderPolicy {
        #[default]
        MustModify,
        ExcludePreviousFill,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum PlaceAndTakeOrderSuccessCondition {
        #[default]
        PartialFill,
        FullFill,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum PerpOperation {
        #[default]
        UpdateFunding,
        AmmFill,
        Fill,
        SettlePnl,
        SettlePnlWithPosition,
        Liquidation,
        AmmImmediateFill,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum SpotOperation {
        #[default]
        UpdateCumulativeInterest,
        Fill,
        Deposit,
        Withdraw,
        Liquidation,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum InsuranceFundOperation {
        #[default]
        Init,
        Add,
        RequestRemove,
        Remove,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
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
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum ContractType {
        #[default]
        Perpetual,
        Future,
        Prediction,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
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
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum AMMAvailability {
        #[default]
        Immediate,
        AfterMinDuration,
        Unavailable,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum SettlePnlMode {
        #[default]
        MustSettle,
        TrySettle,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum SpotBalanceType {
        #[default]
        Deposit,
        Borrow,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum SpotFulfillmentConfigStatus {
        #[default]
        Enabled,
        Disabled,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum AssetTier {
        #[default]
        Collateral,
        Protected,
        Cross,
        Isolated,
        Unlisted,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum TokenProgramFlag {
        #[default]
        Token2022,
        TransferHook,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
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
        AmmImmediateFillPaused,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum FeatureBitFlags {
        #[default]
        MmOracleUpdate,
        MedianTriggerPrice,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum UserStatus {
        #[default]
        BeingLiquidated,
        Bankrupt,
        ReduceOnly,
        AdvancedLp,
        ProtectedMakerOrders,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum AssetType {
        #[default]
        Base,
        Quote,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum OrderStatus {
        #[default]
        Init,
        Open,
        Filled,
        Canceled,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum OrderType {
        #[default]
        Market,
        Limit,
        TriggerMarket,
        TriggerLimit,
        Oracle,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum OrderTriggerCondition {
        #[default]
        Above,
        Below,
        TriggeredAbove,
        TriggeredBelow,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum MarketType {
        #[default]
        Spot,
        Perp,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum OrderBitFlag {
        #[default]
        SignedMessage,
        OracleTriggerMarket,
        SafeTriggerOrder,
        NewTriggerReduceOnly,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum ReferrerStatus {
        #[default]
        IsReferrer,
        IsReferred,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum MarginMode {
        #[default]
        Default,
        HighLeverage,
        HighLeverageMaintenance,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum FuelOverflowStatus {
        #[default]
        Exists,
    }
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub enum SignatureVerificationError {
        #[default]
        InvalidEd25519InstructionProgramId,
        InvalidEd25519InstructionDataLength,
        InvalidSignatureIndex,
        InvalidSignatureOffset,
        InvalidPublicKeyOffset,
        InvalidMessageOffset,
        InvalidMessageDataSize,
        InvalidInstructionIndex,
        MessageOffsetOverflow,
        InvalidMessageHex,
        InvalidMessageData,
        LoadInstructionAtFailed,
    }
}
pub mod accounts {
    #![doc = r" IDL Account types"]
    use super::{types::*, *};
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
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
        #[serde(skip)]
        pub padding: Padding<4>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for OpenbookV2FulfillmentConfig {
        const DISCRIMINATOR: &[u8] = &[3, 43, 58, 106, 131, 132, 199, 171];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for OpenbookV2FulfillmentConfig {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for OpenbookV2FulfillmentConfig {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for OpenbookV2FulfillmentConfig {}
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for OpenbookV2FulfillmentConfig {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for OpenbookV2FulfillmentConfig {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
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
        #[serde(skip)]
        pub padding: Padding<4>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PhoenixV1FulfillmentConfig {
        const DISCRIMINATOR: &[u8] = &[233, 45, 62, 40, 35, 129, 48, 72];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for PhoenixV1FulfillmentConfig {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for PhoenixV1FulfillmentConfig {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for PhoenixV1FulfillmentConfig {}
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for PhoenixV1FulfillmentConfig {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for PhoenixV1FulfillmentConfig {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
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
        #[serde(skip)]
        pub padding: Padding<4>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for SerumV3FulfillmentConfig {
        const DISCRIMINATOR: &[u8] = &[65, 160, 197, 112, 239, 168, 103, 185];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for SerumV3FulfillmentConfig {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for SerumV3FulfillmentConfig {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for SerumV3FulfillmentConfig {}
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for SerumV3FulfillmentConfig {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for SerumV3FulfillmentConfig {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub struct HighLeverageModeConfig {
        pub max_users: u32,
        pub current_users: u32,
        pub reduce_only: u8,
        pub padding1: [u8; 3],
        pub current_maintenance_users: u32,
        pub padding2: [u8; 24],
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for HighLeverageModeConfig {
        const DISCRIMINATOR: &[u8] = &[3, 196, 90, 189, 193, 64, 228, 234];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for HighLeverageModeConfig {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for HighLeverageModeConfig {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for HighLeverageModeConfig {}
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for HighLeverageModeConfig {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for HighLeverageModeConfig {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub struct IfRebalanceConfig {
        pub pubkey: Pubkey,
        pub total_in_amount: u64,
        pub current_in_amount: u64,
        pub current_out_amount: u64,
        pub current_out_amount_transferred: u64,
        pub current_in_amount_since_last_transfer: u64,
        pub epoch_start_ts: i64,
        pub epoch_in_amount: u64,
        pub epoch_max_in_amount: u64,
        pub epoch_duration: i64,
        pub out_market_index: u16,
        pub in_market_index: u16,
        pub max_slippage_bps: u16,
        pub swap_mode: u8,
        pub status: u8,
        pub padding2: [u8; 32],
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for IfRebalanceConfig {
        const DISCRIMINATOR: &[u8] = &[214, 84, 40, 251, 107, 144, 173, 239];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for IfRebalanceConfig {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for IfRebalanceConfig {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for IfRebalanceConfig {}
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for IfRebalanceConfig {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for IfRebalanceConfig {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
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
        #[serde(skip)]
        pub padding: Padding<14>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InsuranceFundStake {
        const DISCRIMINATOR: &[u8] = &[110, 202, 14, 42, 95, 73, 90, 95];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for InsuranceFundStake {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for InsuranceFundStake {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for InsuranceFundStake {}
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for InsuranceFundStake {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for InsuranceFundStake {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub struct ProtocolIfSharesTransferConfig {
        pub whitelisted_signers: [Pubkey; 4],
        pub max_transfer_per_epoch: u128,
        pub current_epoch_transfer: u128,
        pub next_epoch_ts: i64,
        #[serde(skip)]
        pub padding: Padding<8>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for ProtocolIfSharesTransferConfig {
        const DISCRIMINATOR: &[u8] = &[188, 1, 213, 98, 23, 148, 30, 1];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for ProtocolIfSharesTransferConfig {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for ProtocolIfSharesTransferConfig {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for ProtocolIfSharesTransferConfig {}
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for ProtocolIfSharesTransferConfig {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for ProtocolIfSharesTransferConfig {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub struct PrelaunchOracle {
        pub price: i64,
        pub max_price: i64,
        pub confidence: u64,
        pub last_update_slot: u64,
        pub amm_last_update_slot: u64,
        pub perp_market_index: u16,
        #[serde(skip)]
        pub padding: Padding<70>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PrelaunchOracle {
        const DISCRIMINATOR: &[u8] = &[92, 14, 139, 234, 72, 244, 68, 26];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for PrelaunchOracle {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for PrelaunchOracle {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for PrelaunchOracle {}
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for PrelaunchOracle {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for PrelaunchOracle {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
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
        pub pool_id: u8,
        pub high_leverage_margin_ratio_initial: u16,
        pub high_leverage_margin_ratio_maintenance: u16,
        pub protected_maker_limit_price_divisor: u8,
        pub protected_maker_dynamic_divisor: u8,
        pub padding1: u32,
        pub last_fill_price: u64,
        #[serde(skip)]
        pub padding: Padding<24>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PerpMarket {
        const DISCRIMINATOR: &[u8] = &[10, 223, 12, 44, 107, 245, 55, 247];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for PerpMarket {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for PerpMarket {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for PerpMarket {}
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for PerpMarket {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for PerpMarket {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub struct ProtectedMakerModeConfig {
        pub max_users: u32,
        pub current_users: u32,
        pub reduce_only: u8,
        #[serde(skip)]
        pub padding: Padding<31>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for ProtectedMakerModeConfig {
        const DISCRIMINATOR: &[u8] = &[47, 86, 90, 9, 224, 255, 10, 69];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for ProtectedMakerModeConfig {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for ProtectedMakerModeConfig {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for ProtectedMakerModeConfig {}
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for ProtectedMakerModeConfig {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for ProtectedMakerModeConfig {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub struct PythLazerOracle {
        pub price: i64,
        pub publish_time: u64,
        pub posted_slot: u64,
        pub exponent: i32,
        #[serde(skip)]
        pub padding: Padding<4>,
        pub conf: u64,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PythLazerOracle {
        const DISCRIMINATOR: &[u8] = &[159, 7, 161, 249, 34, 81, 121, 133];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for PythLazerOracle {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for PythLazerOracle {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for PythLazerOracle {}
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for PythLazerOracle {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for PythLazerOracle {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, Serialize, Deserialize, Clone, Default, Debug, PartialEq,
    )]
    pub struct SignedMsgUserOrders {
        pub authority_pubkey: Pubkey,
        pub padding: u32,
        pub signed_msg_order_data: Vec<SignedMsgOrderId>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for SignedMsgUserOrders {
        const DISCRIMINATOR: &[u8] = &[70, 6, 50, 248, 222, 1, 143, 49];
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for SignedMsgUserOrders {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for SignedMsgUserOrders {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize, AnchorDeserialize, Serialize, Deserialize, Clone, Default, Debug, PartialEq,
    )]
    pub struct SignedMsgWsDelegates {
        pub delegates: Vec<Pubkey>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for SignedMsgWsDelegates {
        const DISCRIMINATOR: &[u8] = &[190, 115, 111, 44, 216, 252, 108, 85];
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for SignedMsgWsDelegates {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for SignedMsgWsDelegates {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
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
        pub token_program_flag: u8,
        pub pool_id: u8,
        #[serde(skip)]
        pub padding: Padding<40>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for SpotMarket {
        const DISCRIMINATOR: &[u8] = &[100, 177, 8, 107, 168, 65, 65, 39];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for SpotMarket {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for SpotMarket {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for SpotMarket {}
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for SpotMarket {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for SpotMarket {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
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
        pub feature_bit_flags: u8,
        #[serde(skip)]
        pub padding: Padding<9>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for State {
        const DISCRIMINATOR: &[u8] = &[216, 146, 107, 94, 104, 75, 182, 177];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for State {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for State {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for State {}
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for State {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for State {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
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
        pub margin_mode: MarginMode,
        pub pool_id: u8,
        pub padding1: [u8; 3],
        pub last_fuel_bonus_update_ts: u32,
        #[serde(skip)]
        pub padding: Padding<12>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for User {
        const DISCRIMINATOR: &[u8] = &[159, 117, 95, 227, 239, 151, 58, 236];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for User {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for User {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for User {}
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for User {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for User {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
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
        pub referrer_status: u8,
        pub disable_update_perp_bid_ask_twap: bool,
        pub padding1: [u8; 1],
        pub fuel_overflow_status: u8,
        pub fuel_insurance: u32,
        pub fuel_deposits: u32,
        pub fuel_borrows: u32,
        pub fuel_positions: u32,
        pub fuel_taker: u32,
        pub fuel_maker: u32,
        pub if_staked_gov_token_amount: u64,
        pub last_fuel_if_bonus_update_ts: u32,
        #[serde(skip)]
        pub padding: Padding<12>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UserStats {
        const DISCRIMINATOR: &[u8] = &[176, 223, 136, 27, 122, 79, 32, 227];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UserStats {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UserStats {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UserStats {}
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UserStats {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UserStats {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub struct ReferrerName {
        pub authority: Pubkey,
        pub user: Pubkey,
        pub user_stats: Pubkey,
        pub name: [u8; 32],
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for ReferrerName {
        const DISCRIMINATOR: &[u8] = &[105, 133, 170, 110, 52, 42, 28, 182];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for ReferrerName {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for ReferrerName {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for ReferrerName {}
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for ReferrerName {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for ReferrerName {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(
        AnchorSerialize,
        AnchorDeserialize,
        InitSpace,
        Serialize,
        Deserialize,
        Copy,
        Clone,
        Default,
        Debug,
        PartialEq,
    )]
    pub struct FuelOverflow {
        pub authority: Pubkey,
        pub fuel_insurance: u128,
        pub fuel_deposits: u128,
        pub fuel_borrows: u128,
        pub fuel_positions: u128,
        pub fuel_taker: u128,
        pub fuel_maker: u128,
        pub last_fuel_sweep_ts: u32,
        pub last_reset_ts: u32,
        #[serde(skip)]
        pub padding: Padding<6>,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for FuelOverflow {
        const DISCRIMINATOR: &[u8] = &[182, 64, 231, 177, 226, 142, 69, 58];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for FuelOverflow {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for FuelOverflow {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for FuelOverflow {}
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for FuelOverflow {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for FuelOverflow {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[203, 62, 186, 181, 109, 250, 240, 193];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for InitializeUser {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for InitializeUser {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for InitializeUser {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for InitializeUser {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for InitializeUser {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[193, 37, 127, 56, 10, 44, 65, 13];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for InitializeUserStats {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for InitializeUserStats {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for InitializeUserStats {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for InitializeUserStats {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for InitializeUserStats {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct InitializeSignedMsgUserOrders {
        pub signed_msg_user_orders: Pubkey,
        pub authority: Pubkey,
        pub payer: Pubkey,
        pub rent: Pubkey,
        pub system_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeSignedMsgUserOrders {
        const DISCRIMINATOR: &[u8] = &[49, 255, 141, 186, 72, 235, 186, 237];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for InitializeSignedMsgUserOrders {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for InitializeSignedMsgUserOrders {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for InitializeSignedMsgUserOrders {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeSignedMsgUserOrders {}
    #[automatically_derived]
    impl ToAccountMetas for InitializeSignedMsgUserOrders {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.signed_msg_user_orders,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: false,
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for InitializeSignedMsgUserOrders {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for InitializeSignedMsgUserOrders {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct ResizeSignedMsgUserOrders {
        pub signed_msg_user_orders: Pubkey,
        pub authority: Pubkey,
        pub user: Pubkey,
        pub payer: Pubkey,
        pub system_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for ResizeSignedMsgUserOrders {
        const DISCRIMINATOR: &[u8] = &[125, 177, 239, 3, 35, 33, 152, 87];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for ResizeSignedMsgUserOrders {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for ResizeSignedMsgUserOrders {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for ResizeSignedMsgUserOrders {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for ResizeSignedMsgUserOrders {}
    #[automatically_derived]
    impl ToAccountMetas for ResizeSignedMsgUserOrders {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.signed_msg_user_orders,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.payer,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.system_program,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for ResizeSignedMsgUserOrders {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for ResizeSignedMsgUserOrders {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct InitializeSignedMsgWsDelegates {
        pub signed_msg_ws_delegates: Pubkey,
        pub authority: Pubkey,
        pub rent: Pubkey,
        pub system_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeSignedMsgWsDelegates {
        const DISCRIMINATOR: &[u8] = &[171, 35, 226, 71, 228, 189, 130, 139];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for InitializeSignedMsgWsDelegates {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for InitializeSignedMsgWsDelegates {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for InitializeSignedMsgWsDelegates {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeSignedMsgWsDelegates {}
    #[automatically_derived]
    impl ToAccountMetas for InitializeSignedMsgWsDelegates {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.signed_msg_ws_delegates,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for InitializeSignedMsgWsDelegates {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for InitializeSignedMsgWsDelegates {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct ChangeSignedMsgWsDelegateStatus {
        pub signed_msg_ws_delegates: Pubkey,
        pub authority: Pubkey,
        pub system_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for ChangeSignedMsgWsDelegateStatus {
        const DISCRIMINATOR: &[u8] = &[115, 165, 130, 151, 247, 6, 159, 9];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for ChangeSignedMsgWsDelegateStatus {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for ChangeSignedMsgWsDelegateStatus {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for ChangeSignedMsgWsDelegateStatus {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for ChangeSignedMsgWsDelegateStatus {}
    #[automatically_derived]
    impl ToAccountMetas for ChangeSignedMsgWsDelegateStatus {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.signed_msg_ws_delegates,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.system_program,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for ChangeSignedMsgWsDelegateStatus {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for ChangeSignedMsgWsDelegateStatus {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct InitializeFuelOverflow {
        pub fuel_overflow: Pubkey,
        pub user_stats: Pubkey,
        pub authority: Pubkey,
        pub payer: Pubkey,
        pub rent: Pubkey,
        pub system_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeFuelOverflow {
        const DISCRIMINATOR: &[u8] = &[87, 122, 96, 232, 83, 190, 67, 60];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for InitializeFuelOverflow {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for InitializeFuelOverflow {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for InitializeFuelOverflow {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeFuelOverflow {}
    #[automatically_derived]
    impl ToAccountMetas for InitializeFuelOverflow {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.fuel_overflow,
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
                    is_signer: false,
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for InitializeFuelOverflow {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for InitializeFuelOverflow {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct SweepFuel {
        pub fuel_overflow: Pubkey,
        pub user_stats: Pubkey,
        pub authority: Pubkey,
        pub signer: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for SweepFuel {
        const DISCRIMINATOR: &[u8] = &[213, 69, 211, 253, 19, 221, 144, 63];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for SweepFuel {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for SweepFuel {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for SweepFuel {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for SweepFuel {}
    #[automatically_derived]
    impl ToAccountMetas for SweepFuel {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.fuel_overflow,
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
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.signer,
                    is_signer: true,
                    is_writable: false,
                },
            ]
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for SweepFuel {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for SweepFuel {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct ResetFuelSeason {
        pub user_stats: Pubkey,
        pub authority: Pubkey,
        pub state: Pubkey,
        pub admin: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for ResetFuelSeason {
        const DISCRIMINATOR: &[u8] = &[1, 202, 193, 87, 106, 234, 121, 179];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for ResetFuelSeason {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for ResetFuelSeason {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for ResetFuelSeason {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for ResetFuelSeason {}
    #[automatically_derived]
    impl ToAccountMetas for ResetFuelSeason {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: false,
                    is_writable: false,
                },
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
            ]
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for ResetFuelSeason {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for ResetFuelSeason {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[162, 6, 98, 89, 149, 201, 160, 208];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for InitializeReferrerName {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for InitializeReferrerName {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for InitializeReferrerName {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for InitializeReferrerName {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for InitializeReferrerName {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[148, 146, 121, 66, 207, 173, 21, 227];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for Deposit {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for Deposit {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for Deposit {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for Deposit {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for Deposit {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[250, 14, 222, 36, 223, 62, 75, 248];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for Withdraw {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for Withdraw {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for Withdraw {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for Withdraw {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for Withdraw {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[71, 147, 10, 190, 58, 115, 100, 21];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for TransferDeposit {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for TransferDeposit {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for TransferDeposit {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for TransferDeposit {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for TransferDeposit {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct TransferPools {
        pub from_user: Pubkey,
        pub to_user: Pubkey,
        pub user_stats: Pubkey,
        pub authority: Pubkey,
        pub state: Pubkey,
        pub deposit_from_spot_market_vault: Pubkey,
        pub deposit_to_spot_market_vault: Pubkey,
        pub borrow_from_spot_market_vault: Pubkey,
        pub borrow_to_spot_market_vault: Pubkey,
        pub drift_signer: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for TransferPools {
        const DISCRIMINATOR: &[u8] = &[95, 222, 82, 35, 146, 141, 77, 239];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for TransferPools {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for TransferPools {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for TransferPools {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for TransferPools {}
    #[automatically_derived]
    impl ToAccountMetas for TransferPools {
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
                    pubkey: self.deposit_from_spot_market_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.deposit_to_spot_market_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.borrow_from_spot_market_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.borrow_to_spot_market_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.drift_signer,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for TransferPools {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for TransferPools {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct TransferPerpPosition {
        pub from_user: Pubkey,
        pub to_user: Pubkey,
        pub user_stats: Pubkey,
        pub authority: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for TransferPerpPosition {
        const DISCRIMINATOR: &[u8] = &[73, 4, 221, 41, 202, 239, 84, 16];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for TransferPerpPosition {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for TransferPerpPosition {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for TransferPerpPosition {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for TransferPerpPosition {}
    #[automatically_derived]
    impl ToAccountMetas for TransferPerpPosition {
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
            ]
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for TransferPerpPosition {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for TransferPerpPosition {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct PlacePerpOrder {
        pub state: Pubkey,
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PlacePerpOrder {
        const DISCRIMINATOR: &[u8] = &[246, 108, 77, 100, 111, 83, 209, 236];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for PlacePerpOrder {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for PlacePerpOrder {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for PlacePerpOrder {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for PlacePerpOrder {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for PlacePerpOrder {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct CancelOrder {
        pub state: Pubkey,
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for CancelOrder {
        const DISCRIMINATOR: &[u8] = &[113, 49, 205, 244, 82, 104, 158, 85];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for CancelOrder {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for CancelOrder {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for CancelOrder {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for CancelOrder {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for CancelOrder {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct CancelOrderByUserId {
        pub state: Pubkey,
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for CancelOrderByUserId {
        const DISCRIMINATOR: &[u8] = &[242, 196, 53, 34, 121, 232, 149, 144];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for CancelOrderByUserId {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for CancelOrderByUserId {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for CancelOrderByUserId {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for CancelOrderByUserId {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for CancelOrderByUserId {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct CancelOrders {
        pub state: Pubkey,
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for CancelOrders {
        const DISCRIMINATOR: &[u8] = &[91, 217, 110, 30, 16, 2, 55, 83];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for CancelOrders {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for CancelOrders {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for CancelOrders {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for CancelOrders {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for CancelOrders {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct CancelOrdersByIds {
        pub state: Pubkey,
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for CancelOrdersByIds {
        const DISCRIMINATOR: &[u8] = &[111, 71, 138, 14, 8, 97, 11, 44];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for CancelOrdersByIds {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for CancelOrdersByIds {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for CancelOrdersByIds {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for CancelOrdersByIds {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for CancelOrdersByIds {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct ModifyOrder {
        pub state: Pubkey,
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for ModifyOrder {
        const DISCRIMINATOR: &[u8] = &[123, 5, 147, 45, 173, 23, 156, 131];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for ModifyOrder {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for ModifyOrder {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for ModifyOrder {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for ModifyOrder {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for ModifyOrder {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct ModifyOrderByUserId {
        pub state: Pubkey,
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for ModifyOrderByUserId {
        const DISCRIMINATOR: &[u8] = &[87, 55, 202, 75, 9, 179, 204, 73];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for ModifyOrderByUserId {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for ModifyOrderByUserId {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for ModifyOrderByUserId {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for ModifyOrderByUserId {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for ModifyOrderByUserId {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct PlaceAndTakePerpOrder {
        pub state: Pubkey,
        pub user: Pubkey,
        pub user_stats: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PlaceAndTakePerpOrder {
        const DISCRIMINATOR: &[u8] = &[106, 246, 39, 196, 36, 127, 12, 84];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for PlaceAndTakePerpOrder {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for PlaceAndTakePerpOrder {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for PlaceAndTakePerpOrder {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for PlaceAndTakePerpOrder {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for PlaceAndTakePerpOrder {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[139, 129, 243, 60, 209, 20, 25, 195];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for PlaceAndMakePerpOrder {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for PlaceAndMakePerpOrder {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for PlaceAndMakePerpOrder {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for PlaceAndMakePerpOrder {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for PlaceAndMakePerpOrder {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct PlaceAndMakeSignedMsgPerpOrder {
        pub state: Pubkey,
        pub user: Pubkey,
        pub user_stats: Pubkey,
        pub taker: Pubkey,
        pub taker_stats: Pubkey,
        pub taker_signed_msg_user_orders: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PlaceAndMakeSignedMsgPerpOrder {
        const DISCRIMINATOR: &[u8] = &[240, 219, 156, 22, 147, 139, 152, 165];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for PlaceAndMakeSignedMsgPerpOrder {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for PlaceAndMakeSignedMsgPerpOrder {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for PlaceAndMakeSignedMsgPerpOrder {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for PlaceAndMakeSignedMsgPerpOrder {}
    #[automatically_derived]
    impl ToAccountMetas for PlaceAndMakeSignedMsgPerpOrder {
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
                    pubkey: self.taker_signed_msg_user_orders,
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for PlaceAndMakeSignedMsgPerpOrder {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for PlaceAndMakeSignedMsgPerpOrder {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct PlaceSignedMsgTakerOrder {
        pub state: Pubkey,
        pub user: Pubkey,
        pub user_stats: Pubkey,
        pub signed_msg_user_orders: Pubkey,
        pub authority: Pubkey,
        pub ix_sysvar: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PlaceSignedMsgTakerOrder {
        const DISCRIMINATOR: &[u8] = &[202, 134, 30, 82, 84, 234, 248, 40];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for PlaceSignedMsgTakerOrder {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for PlaceSignedMsgTakerOrder {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for PlaceSignedMsgTakerOrder {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for PlaceSignedMsgTakerOrder {}
    #[automatically_derived]
    impl ToAccountMetas for PlaceSignedMsgTakerOrder {
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
                    pubkey: self.signed_msg_user_orders,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.ix_sysvar,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for PlaceSignedMsgTakerOrder {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for PlaceSignedMsgTakerOrder {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct PlaceSpotOrder {
        pub state: Pubkey,
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PlaceSpotOrder {
        const DISCRIMINATOR: &[u8] = &[174, 248, 135, 120, 62, 178, 165, 88];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for PlaceSpotOrder {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for PlaceSpotOrder {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for PlaceSpotOrder {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for PlaceSpotOrder {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for PlaceSpotOrder {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct PlaceAndTakeSpotOrder {
        pub state: Pubkey,
        pub user: Pubkey,
        pub user_stats: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PlaceAndTakeSpotOrder {
        const DISCRIMINATOR: &[u8] = &[114, 201, 131, 176, 15, 188, 94, 123];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for PlaceAndTakeSpotOrder {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for PlaceAndTakeSpotOrder {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for PlaceAndTakeSpotOrder {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for PlaceAndTakeSpotOrder {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for PlaceAndTakeSpotOrder {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[136, 102, 203, 251, 133, 233, 3, 195];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for PlaceAndMakeSpotOrder {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for PlaceAndMakeSpotOrder {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for PlaceAndMakeSpotOrder {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for PlaceAndMakeSpotOrder {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for PlaceAndMakeSpotOrder {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct PlaceOrders {
        pub state: Pubkey,
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PlaceOrders {
        const DISCRIMINATOR: &[u8] = &[124, 246, 105, 210, 163, 120, 252, 61];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for PlaceOrders {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for PlaceOrders {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for PlaceOrders {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for PlaceOrders {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for PlaceOrders {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[219, 64, 214, 146, 204, 171, 39, 63];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for BeginSwap {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for BeginSwap {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for BeginSwap {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for BeginSwap {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for BeginSwap {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[169, 78, 41, 75, 127, 169, 211, 205];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for EndSwap {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for EndSwap {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for EndSwap {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for EndSwap {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for EndSwap {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateUserName {
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserName {
        const DISCRIMINATOR: &[u8] = &[110, 237, 80, 83, 89, 231, 185, 154];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateUserName {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateUserName {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateUserName {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateUserName {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateUserName {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateUserCustomMarginRatio {
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserCustomMarginRatio {
        const DISCRIMINATOR: &[u8] = &[102, 94, 49, 231, 23, 142, 117, 224];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateUserCustomMarginRatio {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateUserCustomMarginRatio {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateUserCustomMarginRatio {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateUserCustomMarginRatio {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateUserCustomMarginRatio {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateUserPerpPositionCustomMarginRatio {
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserPerpPositionCustomMarginRatio {
        const DISCRIMINATOR: &[u8] = &[200, 88, 83, 62, 55, 227, 50, 252];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateUserPerpPositionCustomMarginRatio {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateUserPerpPositionCustomMarginRatio {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateUserPerpPositionCustomMarginRatio {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserPerpPositionCustomMarginRatio {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateUserPerpPositionCustomMarginRatio {
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateUserPerpPositionCustomMarginRatio {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateUserPerpPositionCustomMarginRatio {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateUserMarginTradingEnabled {
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserMarginTradingEnabled {
        const DISCRIMINATOR: &[u8] = &[89, 5, 206, 157, 104, 243, 243, 104];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateUserMarginTradingEnabled {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateUserMarginTradingEnabled {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateUserMarginTradingEnabled {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateUserMarginTradingEnabled {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateUserMarginTradingEnabled {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateUserPoolId {
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserPoolId {
        const DISCRIMINATOR: &[u8] = &[215, 193, 254, 33, 60, 226, 249, 100];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateUserPoolId {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateUserPoolId {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateUserPoolId {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserPoolId {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateUserPoolId {
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateUserPoolId {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateUserPoolId {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateUserDelegate {
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserDelegate {
        const DISCRIMINATOR: &[u8] = &[32, 244, 37, 163, 236, 179, 10, 208];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateUserDelegate {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateUserDelegate {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateUserDelegate {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateUserDelegate {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateUserDelegate {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateUserReduceOnly {
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserReduceOnly {
        const DISCRIMINATOR: &[u8] = &[213, 230, 138, 228, 171, 118, 20, 105];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateUserReduceOnly {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateUserReduceOnly {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateUserReduceOnly {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateUserReduceOnly {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateUserReduceOnly {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateUserAdvancedLp {
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserAdvancedLp {
        const DISCRIMINATOR: &[u8] = &[227, 146, 68, 197, 45, 160, 163, 72];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateUserAdvancedLp {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateUserAdvancedLp {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateUserAdvancedLp {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateUserAdvancedLp {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateUserAdvancedLp {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateUserProtectedMakerOrders {
        pub state: Pubkey,
        pub user: Pubkey,
        pub authority: Pubkey,
        pub protected_maker_mode_config: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserProtectedMakerOrders {
        const DISCRIMINATOR: &[u8] = &[220, 255, 63, 84, 125, 9, 84, 92];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateUserProtectedMakerOrders {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateUserProtectedMakerOrders {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateUserProtectedMakerOrders {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserProtectedMakerOrders {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateUserProtectedMakerOrders {
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
                    pubkey: self.protected_maker_mode_config,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateUserProtectedMakerOrders {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateUserProtectedMakerOrders {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct DeleteUser {
        pub user: Pubkey,
        pub user_stats: Pubkey,
        pub state: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for DeleteUser {
        const DISCRIMINATOR: &[u8] = &[138, 7, 216, 138, 241, 248, 199, 228];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for DeleteUser {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for DeleteUser {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for DeleteUser {}
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
                    is_writable: true,
                },
            ]
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for DeleteUser {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for DeleteUser {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct ForceDeleteUser {
        pub user: Pubkey,
        pub user_stats: Pubkey,
        pub state: Pubkey,
        pub authority: Pubkey,
        pub keeper: Pubkey,
        pub drift_signer: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for ForceDeleteUser {
        const DISCRIMINATOR: &[u8] = &[224, 206, 23, 192, 99, 164, 175, 251];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for ForceDeleteUser {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for ForceDeleteUser {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for ForceDeleteUser {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for ForceDeleteUser {}
    #[automatically_derived]
    impl ToAccountMetas for ForceDeleteUser {
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
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.keeper,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.drift_signer,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for ForceDeleteUser {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for ForceDeleteUser {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct DeleteSignedMsgUserOrders {
        pub signed_msg_user_orders: Pubkey,
        pub state: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for DeleteSignedMsgUserOrders {
        const DISCRIMINATOR: &[u8] = &[134, 162, 251, 123, 234, 231, 227, 119];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for DeleteSignedMsgUserOrders {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for DeleteSignedMsgUserOrders {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for DeleteSignedMsgUserOrders {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for DeleteSignedMsgUserOrders {}
    #[automatically_derived]
    impl ToAccountMetas for DeleteSignedMsgUserOrders {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.signed_msg_user_orders,
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for DeleteSignedMsgUserOrders {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for DeleteSignedMsgUserOrders {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct ReclaimRent {
        pub user: Pubkey,
        pub user_stats: Pubkey,
        pub state: Pubkey,
        pub authority: Pubkey,
        pub rent: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for ReclaimRent {
        const DISCRIMINATOR: &[u8] = &[245, 126, 60, 211, 102, 85, 171, 126];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for ReclaimRent {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for ReclaimRent {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for ReclaimRent {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for ReclaimRent {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for ReclaimRent {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct EnableUserHighLeverageMode {
        pub state: Pubkey,
        pub user: Pubkey,
        pub authority: Pubkey,
        pub high_leverage_mode_config: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for EnableUserHighLeverageMode {
        const DISCRIMINATOR: &[u8] = &[87, 74, 202, 252, 83, 254, 102, 158];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for EnableUserHighLeverageMode {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for EnableUserHighLeverageMode {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for EnableUserHighLeverageMode {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for EnableUserHighLeverageMode {}
    #[automatically_derived]
    impl ToAccountMetas for EnableUserHighLeverageMode {
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
                    pubkey: self.high_leverage_mode_config,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for EnableUserHighLeverageMode {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for EnableUserHighLeverageMode {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[196, 125, 144, 95, 242, 149, 179, 234];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for FillPerpOrder {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for FillPerpOrder {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for FillPerpOrder {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for FillPerpOrder {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for FillPerpOrder {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct RevertFill {
        pub state: Pubkey,
        pub authority: Pubkey,
        pub filler: Pubkey,
        pub filler_stats: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for RevertFill {
        const DISCRIMINATOR: &[u8] = &[119, 31, 174, 155, 246, 22, 28, 126];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for RevertFill {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for RevertFill {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for RevertFill {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for RevertFill {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for RevertFill {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[105, 64, 114, 55, 230, 90, 153, 28];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for FillSpotOrder {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for FillSpotOrder {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for FillSpotOrder {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for FillSpotOrder {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for FillSpotOrder {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct TriggerOrder {
        pub state: Pubkey,
        pub authority: Pubkey,
        pub filler: Pubkey,
        pub user: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for TriggerOrder {
        const DISCRIMINATOR: &[u8] = &[236, 61, 42, 190, 152, 12, 106, 116];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for TriggerOrder {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for TriggerOrder {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for TriggerOrder {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for TriggerOrder {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for TriggerOrder {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct ForceCancelOrders {
        pub state: Pubkey,
        pub authority: Pubkey,
        pub filler: Pubkey,
        pub user: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for ForceCancelOrders {
        const DISCRIMINATOR: &[u8] = &[108, 153, 180, 51, 37, 158, 99, 93];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for ForceCancelOrders {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for ForceCancelOrders {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for ForceCancelOrders {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for ForceCancelOrders {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for ForceCancelOrders {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateUserIdle {
        pub state: Pubkey,
        pub authority: Pubkey,
        pub filler: Pubkey,
        pub user: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserIdle {
        const DISCRIMINATOR: &[u8] = &[229, 30, 7, 22, 26, 184, 224, 191];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateUserIdle {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateUserIdle {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateUserIdle {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateUserIdle {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateUserIdle {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct LogUserBalances {
        pub state: Pubkey,
        pub authority: Pubkey,
        pub user: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for LogUserBalances {
        const DISCRIMINATOR: &[u8] = &[121, 191, 93, 132, 153, 217, 15, 171];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for LogUserBalances {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for LogUserBalances {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for LogUserBalances {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for LogUserBalances {}
    #[automatically_derived]
    impl ToAccountMetas for LogUserBalances {
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
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for LogUserBalances {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for LogUserBalances {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct DisableUserHighLeverageMode {
        pub state: Pubkey,
        pub authority: Pubkey,
        pub user: Pubkey,
        pub high_leverage_mode_config: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for DisableUserHighLeverageMode {
        const DISCRIMINATOR: &[u8] = &[126, 242, 88, 155, 81, 152, 143, 68];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for DisableUserHighLeverageMode {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for DisableUserHighLeverageMode {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for DisableUserHighLeverageMode {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for DisableUserHighLeverageMode {}
    #[automatically_derived]
    impl ToAccountMetas for DisableUserHighLeverageMode {
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
                    pubkey: self.user,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.high_leverage_mode_config,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for DisableUserHighLeverageMode {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for DisableUserHighLeverageMode {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateUserFuelBonus {
        pub state: Pubkey,
        pub authority: Pubkey,
        pub user: Pubkey,
        pub user_stats: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserFuelBonus {
        const DISCRIMINATOR: &[u8] = &[179, 14, 130, 214, 107, 254, 33, 235];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateUserFuelBonus {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateUserFuelBonus {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateUserFuelBonus {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserFuelBonus {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateUserFuelBonus {
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateUserFuelBonus {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateUserFuelBonus {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateUserStatsReferrerStatus {
        pub state: Pubkey,
        pub authority: Pubkey,
        pub user_stats: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserStatsReferrerStatus {
        const DISCRIMINATOR: &[u8] = &[88, 125, 77, 90, 13, 11, 141, 158];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateUserStatsReferrerStatus {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateUserStatsReferrerStatus {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateUserStatsReferrerStatus {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserStatsReferrerStatus {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateUserStatsReferrerStatus {
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
                    pubkey: self.user_stats,
                    is_signer: false,
                    is_writable: true,
                },
            ]
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateUserStatsReferrerStatus {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateUserStatsReferrerStatus {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateUserOpenOrdersCount {
        pub state: Pubkey,
        pub authority: Pubkey,
        pub filler: Pubkey,
        pub user: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserOpenOrdersCount {
        const DISCRIMINATOR: &[u8] = &[21, 201, 16, 50, 34, 238, 126, 254];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateUserOpenOrdersCount {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateUserOpenOrdersCount {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateUserOpenOrdersCount {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateUserOpenOrdersCount {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateUserOpenOrdersCount {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct AdminDisableUpdatePerpBidAskTwap {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub user_stats: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for AdminDisableUpdatePerpBidAskTwap {
        const DISCRIMINATOR: &[u8] = &[253, 223, 202, 93, 246, 209, 209, 26];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for AdminDisableUpdatePerpBidAskTwap {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for AdminDisableUpdatePerpBidAskTwap {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for AdminDisableUpdatePerpBidAskTwap {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for AdminDisableUpdatePerpBidAskTwap {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for AdminDisableUpdatePerpBidAskTwap {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct SettlePnl {
        pub state: Pubkey,
        pub user: Pubkey,
        pub authority: Pubkey,
        pub spot_market_vault: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for SettlePnl {
        const DISCRIMINATOR: &[u8] = &[216, 232, 154, 114, 103, 217, 85, 46];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for SettlePnl {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for SettlePnl {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for SettlePnl {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for SettlePnl {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for SettlePnl {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct SettleMultiplePnls {
        pub state: Pubkey,
        pub user: Pubkey,
        pub authority: Pubkey,
        pub spot_market_vault: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for SettleMultiplePnls {
        const DISCRIMINATOR: &[u8] = &[6, 98, 112, 180, 76, 44, 249, 23];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for SettleMultiplePnls {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for SettleMultiplePnls {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for SettleMultiplePnls {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for SettleMultiplePnls {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for SettleMultiplePnls {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct SettleFundingPayment {
        pub state: Pubkey,
        pub user: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for SettleFundingPayment {
        const DISCRIMINATOR: &[u8] = &[138, 197, 211, 110, 76, 124, 91, 34];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for SettleFundingPayment {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for SettleFundingPayment {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for SettleFundingPayment {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for SettleFundingPayment {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for SettleFundingPayment {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct SettleExpiredMarket {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for SettleExpiredMarket {
        const DISCRIMINATOR: &[u8] = &[208, 148, 90, 99, 162, 85, 158, 236];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for SettleExpiredMarket {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for SettleExpiredMarket {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for SettleExpiredMarket {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for SettleExpiredMarket {}
    #[automatically_derived]
    impl ToAccountMetas for SettleExpiredMarket {
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for SettleExpiredMarket {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for SettleExpiredMarket {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[167, 66, 155, 127, 112, 246, 147, 196];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for LiquidatePerp {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for LiquidatePerp {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for LiquidatePerp {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for LiquidatePerp {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for LiquidatePerp {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[236, 199, 136, 156, 22, 138, 41, 225];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for LiquidatePerpWithFill {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for LiquidatePerpWithFill {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for LiquidatePerpWithFill {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for LiquidatePerpWithFill {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for LiquidatePerpWithFill {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[89, 79, 84, 154, 215, 7, 211, 253];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for LiquidateSpot {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for LiquidateSpot {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for LiquidateSpot {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for LiquidateSpot {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for LiquidateSpot {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct LiquidateSpotWithSwapBegin {
        pub state: Pubkey,
        pub authority: Pubkey,
        pub liquidator: Pubkey,
        pub liquidator_stats: Pubkey,
        pub user: Pubkey,
        pub user_stats: Pubkey,
        pub liability_spot_market_vault: Pubkey,
        pub asset_spot_market_vault: Pubkey,
        pub liability_token_account: Pubkey,
        pub asset_token_account: Pubkey,
        pub token_program: Pubkey,
        pub drift_signer: Pubkey,
        pub instructions: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for LiquidateSpotWithSwapBegin {
        const DISCRIMINATOR: &[u8] = &[58, 245, 239, 110, 253, 194, 212, 67];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for LiquidateSpotWithSwapBegin {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for LiquidateSpotWithSwapBegin {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for LiquidateSpotWithSwapBegin {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for LiquidateSpotWithSwapBegin {}
    #[automatically_derived]
    impl ToAccountMetas for LiquidateSpotWithSwapBegin {
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
                    pubkey: self.liability_spot_market_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.asset_spot_market_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.liability_token_account,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.asset_token_account,
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for LiquidateSpotWithSwapBegin {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for LiquidateSpotWithSwapBegin {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct LiquidateSpotWithSwapEnd {
        pub state: Pubkey,
        pub authority: Pubkey,
        pub liquidator: Pubkey,
        pub liquidator_stats: Pubkey,
        pub user: Pubkey,
        pub user_stats: Pubkey,
        pub liability_spot_market_vault: Pubkey,
        pub asset_spot_market_vault: Pubkey,
        pub liability_token_account: Pubkey,
        pub asset_token_account: Pubkey,
        pub token_program: Pubkey,
        pub drift_signer: Pubkey,
        pub instructions: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for LiquidateSpotWithSwapEnd {
        const DISCRIMINATOR: &[u8] = &[157, 1, 82, 217, 233, 241, 137, 175];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for LiquidateSpotWithSwapEnd {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for LiquidateSpotWithSwapEnd {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for LiquidateSpotWithSwapEnd {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for LiquidateSpotWithSwapEnd {}
    #[automatically_derived]
    impl ToAccountMetas for LiquidateSpotWithSwapEnd {
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
                    pubkey: self.liability_spot_market_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.asset_spot_market_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.liability_token_account,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.asset_token_account,
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for LiquidateSpotWithSwapEnd {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for LiquidateSpotWithSwapEnd {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[188, 143, 170, 71, 28, 50, 50, 50];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for LiquidateBorrowForPerpPnl {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for LiquidateBorrowForPerpPnl {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for LiquidateBorrowForPerpPnl {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for LiquidateBorrowForPerpPnl {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for LiquidateBorrowForPerpPnl {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[66, 46, 37, 140, 100, 242, 118, 224];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for LiquidatePerpPnlForDeposit {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for LiquidatePerpPnlForDeposit {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for LiquidatePerpPnlForDeposit {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for LiquidatePerpPnlForDeposit {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for LiquidatePerpPnlForDeposit {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct SetUserStatusToBeingLiquidated {
        pub state: Pubkey,
        pub user: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for SetUserStatusToBeingLiquidated {
        const DISCRIMINATOR: &[u8] = &[150, 171, 70, 240, 192, 22, 158, 128];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for SetUserStatusToBeingLiquidated {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for SetUserStatusToBeingLiquidated {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for SetUserStatusToBeingLiquidated {}
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
                AccountMeta {
                    pubkey: self.authority,
                    is_signer: true,
                    is_writable: false,
                },
            ]
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for SetUserStatusToBeingLiquidated {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for SetUserStatusToBeingLiquidated {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[11, 51, 56, 223, 39, 129, 129, 31];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for ResolvePerpPnlDeficit {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for ResolvePerpPnlDeficit {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for ResolvePerpPnlDeficit {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for ResolvePerpPnlDeficit {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for ResolvePerpPnlDeficit {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[232, 117, 49, 236, 173, 89, 78, 52];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for ResolvePerpBankruptcy {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for ResolvePerpBankruptcy {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for ResolvePerpBankruptcy {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for ResolvePerpBankruptcy {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for ResolvePerpBankruptcy {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[186, 115, 37, 134, 49, 132, 252, 139];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for ResolveSpotBankruptcy {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for ResolveSpotBankruptcy {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for ResolveSpotBankruptcy {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for ResolveSpotBankruptcy {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for ResolveSpotBankruptcy {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[49, 54, 211, 157, 180, 157, 182, 14];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for SettleRevenueToInsuranceFund {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for SettleRevenueToInsuranceFund {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for SettleRevenueToInsuranceFund {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for SettleRevenueToInsuranceFund {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for SettleRevenueToInsuranceFund {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateFundingRate {
        pub state: Pubkey,
        pub perp_market: Pubkey,
        pub oracle: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateFundingRate {
        const DISCRIMINATOR: &[u8] = &[147, 129, 170, 235, 221, 137, 139, 160];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateFundingRate {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateFundingRate {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateFundingRate {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateFundingRate {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateFundingRate {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePrelaunchOracle {
        pub state: Pubkey,
        pub perp_market: Pubkey,
        pub oracle: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePrelaunchOracle {
        const DISCRIMINATOR: &[u8] = &[246, 29, 3, 127, 200, 172, 45, 171];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePrelaunchOracle {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePrelaunchOracle {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePrelaunchOracle {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePrelaunchOracle {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePrelaunchOracle {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpBidAskTwap {
        pub state: Pubkey,
        pub perp_market: Pubkey,
        pub oracle: Pubkey,
        pub keeper_stats: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpBidAskTwap {
        const DISCRIMINATOR: &[u8] = &[75, 78, 22, 179, 238, 41, 216, 90];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpBidAskTwap {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpBidAskTwap {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpBidAskTwap {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpBidAskTwap {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpBidAskTwap {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateSpotMarketCumulativeInterest {
        pub state: Pubkey,
        pub spot_market: Pubkey,
        pub oracle: Pubkey,
        pub spot_market_vault: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketCumulativeInterest {
        const DISCRIMINATOR: &[u8] = &[103, 124, 233, 228, 65, 182, 52, 216];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateSpotMarketCumulativeInterest {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateSpotMarketCumulativeInterest {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateSpotMarketCumulativeInterest {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateSpotMarketCumulativeInterest {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateSpotMarketCumulativeInterest {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateAmms {
        pub state: Pubkey,
        pub authority: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateAmms {
        const DISCRIMINATOR: &[u8] = &[199, 237, 7, 202, 212, 96, 167, 243];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateAmms {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateAmms {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateAmms {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateAmms {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateAmms {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateSpotMarketExpiry {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketExpiry {
        const DISCRIMINATOR: &[u8] = &[124, 96, 183, 195, 2, 49, 222, 97];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateSpotMarketExpiry {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateSpotMarketExpiry {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateSpotMarketExpiry {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateSpotMarketExpiry {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateSpotMarketExpiry {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[78, 21, 169, 183, 105, 218, 162, 67];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateUserQuoteAssetInsuranceStake {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateUserQuoteAssetInsuranceStake {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateUserQuoteAssetInsuranceStake {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateUserQuoteAssetInsuranceStake {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateUserQuoteAssetInsuranceStake {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[43, 203, 49, 187, 213, 150, 189, 95];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateUserGovTokenInsuranceStake {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateUserGovTokenInsuranceStake {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateUserGovTokenInsuranceStake {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateUserGovTokenInsuranceStake {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateUserGovTokenInsuranceStake {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateUserGovTokenInsuranceStakeDevnet {
        pub user_stats: Pubkey,
        pub signer: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateUserGovTokenInsuranceStakeDevnet {
        const DISCRIMINATOR: &[u8] = &[7, 243, 156, 21, 134, 61, 166, 81];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateUserGovTokenInsuranceStakeDevnet {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateUserGovTokenInsuranceStakeDevnet {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateUserGovTokenInsuranceStakeDevnet {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateUserGovTokenInsuranceStakeDevnet {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateUserGovTokenInsuranceStakeDevnet {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
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
            ]
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateUserGovTokenInsuranceStakeDevnet {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateUserGovTokenInsuranceStakeDevnet {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[99, 171, 38, 232, 118, 110, 9, 182];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for InitializeInsuranceFundStake {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for InitializeInsuranceFundStake {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for InitializeInsuranceFundStake {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for InitializeInsuranceFundStake {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for InitializeInsuranceFundStake {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[0, 49, 78, 210, 146, 42, 143, 8];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for AddInsuranceFundStake {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for AddInsuranceFundStake {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for AddInsuranceFundStake {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for AddInsuranceFundStake {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for AddInsuranceFundStake {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct RequestRemoveInsuranceFundStake {
        pub spot_market: Pubkey,
        pub insurance_fund_stake: Pubkey,
        pub user_stats: Pubkey,
        pub authority: Pubkey,
        pub insurance_fund_vault: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for RequestRemoveInsuranceFundStake {
        const DISCRIMINATOR: &[u8] = &[19, 15, 248, 88, 171, 187, 152, 24];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for RequestRemoveInsuranceFundStake {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for RequestRemoveInsuranceFundStake {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for RequestRemoveInsuranceFundStake {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for RequestRemoveInsuranceFundStake {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for RequestRemoveInsuranceFundStake {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct CancelRequestRemoveInsuranceFundStake {
        pub spot_market: Pubkey,
        pub insurance_fund_stake: Pubkey,
        pub user_stats: Pubkey,
        pub authority: Pubkey,
        pub insurance_fund_vault: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for CancelRequestRemoveInsuranceFundStake {
        const DISCRIMINATOR: &[u8] = &[143, 95, 196, 57, 1, 17, 40, 153];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for CancelRequestRemoveInsuranceFundStake {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for CancelRequestRemoveInsuranceFundStake {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for CancelRequestRemoveInsuranceFundStake {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for CancelRequestRemoveInsuranceFundStake {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for CancelRequestRemoveInsuranceFundStake {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[64, 21, 182, 166, 31, 13, 139, 114];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for RemoveInsuranceFundStake {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for RemoveInsuranceFundStake {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for RemoveInsuranceFundStake {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for RemoveInsuranceFundStake {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for RemoveInsuranceFundStake {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[113, 16, 209, 191, 115, 24, 39, 146];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for TransferProtocolIfShares {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for TransferProtocolIfShares {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for TransferProtocolIfShares {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for TransferProtocolIfShares {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for TransferProtocolIfShares {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct BeginInsuranceFundSwap {
        pub state: Pubkey,
        pub authority: Pubkey,
        pub out_insurance_fund_vault: Pubkey,
        pub in_insurance_fund_vault: Pubkey,
        pub out_token_account: Pubkey,
        pub in_token_account: Pubkey,
        pub if_rebalance_config: Pubkey,
        pub token_program: Pubkey,
        pub drift_signer: Pubkey,
        pub instructions: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for BeginInsuranceFundSwap {
        const DISCRIMINATOR: &[u8] = &[82, 226, 182, 52, 133, 70, 71, 186];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for BeginInsuranceFundSwap {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for BeginInsuranceFundSwap {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for BeginInsuranceFundSwap {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for BeginInsuranceFundSwap {}
    #[automatically_derived]
    impl ToAccountMetas for BeginInsuranceFundSwap {
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
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.out_insurance_fund_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.in_insurance_fund_vault,
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
                    pubkey: self.if_rebalance_config,
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for BeginInsuranceFundSwap {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for BeginInsuranceFundSwap {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct EndInsuranceFundSwap {
        pub state: Pubkey,
        pub authority: Pubkey,
        pub out_insurance_fund_vault: Pubkey,
        pub in_insurance_fund_vault: Pubkey,
        pub out_token_account: Pubkey,
        pub in_token_account: Pubkey,
        pub if_rebalance_config: Pubkey,
        pub token_program: Pubkey,
        pub drift_signer: Pubkey,
        pub instructions: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for EndInsuranceFundSwap {
        const DISCRIMINATOR: &[u8] = &[248, 118, 124, 42, 161, 233, 183, 133];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for EndInsuranceFundSwap {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for EndInsuranceFundSwap {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for EndInsuranceFundSwap {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for EndInsuranceFundSwap {}
    #[automatically_derived]
    impl ToAccountMetas for EndInsuranceFundSwap {
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
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.out_insurance_fund_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.in_insurance_fund_vault,
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
                    pubkey: self.if_rebalance_config,
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for EndInsuranceFundSwap {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for EndInsuranceFundSwap {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct TransferProtocolIfSharesToRevenuePool {
        pub state: Pubkey,
        pub authority: Pubkey,
        pub insurance_fund_vault: Pubkey,
        pub spot_market_vault: Pubkey,
        pub if_rebalance_config: Pubkey,
        pub token_program: Pubkey,
        pub drift_signer: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for TransferProtocolIfSharesToRevenuePool {
        const DISCRIMINATOR: &[u8] = &[137, 197, 241, 46, 104, 88, 83, 50];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for TransferProtocolIfSharesToRevenuePool {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for TransferProtocolIfSharesToRevenuePool {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for TransferProtocolIfSharesToRevenuePool {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for TransferProtocolIfSharesToRevenuePool {}
    #[automatically_derived]
    impl ToAccountMetas for TransferProtocolIfSharesToRevenuePool {
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
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.insurance_fund_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.spot_market_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.if_rebalance_config,
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
            ]
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for TransferProtocolIfSharesToRevenuePool {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for TransferProtocolIfSharesToRevenuePool {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePythPullOracle {
        pub keeper: Pubkey,
        pub pyth_solana_receiver: Pubkey,
        pub encoded_vaa: Pubkey,
        pub price_feed: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePythPullOracle {
        const DISCRIMINATOR: &[u8] = &[163, 16, 49, 37, 171, 99, 61, 60];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePythPullOracle {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePythPullOracle {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePythPullOracle {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePythPullOracle {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePythPullOracle {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct PostPythPullOracleUpdateAtomic {
        pub keeper: Pubkey,
        pub pyth_solana_receiver: Pubkey,
        pub guardian_set: Pubkey,
        pub price_feed: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PostPythPullOracleUpdateAtomic {
        const DISCRIMINATOR: &[u8] = &[14, 125, 28, 5, 52, 143, 144, 18];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for PostPythPullOracleUpdateAtomic {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for PostPythPullOracleUpdateAtomic {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for PostPythPullOracleUpdateAtomic {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for PostPythPullOracleUpdateAtomic {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for PostPythPullOracleUpdateAtomic {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct PostMultiPythPullOracleUpdatesAtomic {
        pub keeper: Pubkey,
        pub pyth_solana_receiver: Pubkey,
        pub guardian_set: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PostMultiPythPullOracleUpdatesAtomic {
        const DISCRIMINATOR: &[u8] = &[110, 234, 167, 14, 68, 55, 110, 122];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for PostMultiPythPullOracleUpdatesAtomic {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for PostMultiPythPullOracleUpdatesAtomic {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for PostMultiPythPullOracleUpdatesAtomic {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for PostMultiPythPullOracleUpdatesAtomic {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for PostMultiPythPullOracleUpdatesAtomic {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct PauseSpotMarketDepositWithdraw {
        pub state: Pubkey,
        pub keeper: Pubkey,
        pub spot_market: Pubkey,
        pub spot_market_vault: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PauseSpotMarketDepositWithdraw {
        const DISCRIMINATOR: &[u8] = &[229, 56, 238, 247, 130, 249, 245, 152];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for PauseSpotMarketDepositWithdraw {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for PauseSpotMarketDepositWithdraw {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for PauseSpotMarketDepositWithdraw {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for PauseSpotMarketDepositWithdraw {}
    #[automatically_derived]
    impl ToAccountMetas for PauseSpotMarketDepositWithdraw {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.state,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.keeper,
                    is_signer: true,
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
                    is_writable: false,
                },
            ]
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for PauseSpotMarketDepositWithdraw {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for PauseSpotMarketDepositWithdraw {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[131, 246, 167, 36, 232, 249, 207, 142];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for Initialize {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for Initialize {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for Initialize {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for Initialize {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for Initialize {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[71, 64, 197, 212, 23, 44, 156, 75];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for InitializeSpotMarket {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for InitializeSpotMarket {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for InitializeSpotMarket {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for InitializeSpotMarket {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for InitializeSpotMarket {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[239, 247, 78, 81, 92, 141, 135, 107];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for DeleteInitializedSpotMarket {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for DeleteInitializedSpotMarket {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for DeleteInitializedSpotMarket {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for DeleteInitializedSpotMarket {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for DeleteInitializedSpotMarket {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[19, 53, 42, 248, 46, 49, 92, 179];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for InitializeSerumFulfillmentConfig {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for InitializeSerumFulfillmentConfig {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for InitializeSerumFulfillmentConfig {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for InitializeSerumFulfillmentConfig {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for InitializeSerumFulfillmentConfig {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateSerumFulfillmentConfigStatus {
        pub state: Pubkey,
        pub serum_fulfillment_config: Pubkey,
        pub admin: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSerumFulfillmentConfigStatus {
        const DISCRIMINATOR: &[u8] = &[132, 84, 50, 193, 9, 204, 122, 230];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateSerumFulfillmentConfigStatus {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateSerumFulfillmentConfigStatus {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateSerumFulfillmentConfigStatus {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateSerumFulfillmentConfigStatus {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateSerumFulfillmentConfigStatus {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[22, 199, 68, 220, 120, 204, 78, 80];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for InitializeOpenbookV2FulfillmentConfig {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for InitializeOpenbookV2FulfillmentConfig {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for InitializeOpenbookV2FulfillmentConfig {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for InitializeOpenbookV2FulfillmentConfig {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for InitializeOpenbookV2FulfillmentConfig {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct OpenbookV2FulfillmentConfigStatus {
        pub state: Pubkey,
        pub openbook_v2_fulfillment_config: Pubkey,
        pub admin: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for OpenbookV2FulfillmentConfigStatus {
        const DISCRIMINATOR: &[u8] = &[165, 142, 230, 255, 126, 234, 45, 16];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for OpenbookV2FulfillmentConfigStatus {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for OpenbookV2FulfillmentConfigStatus {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for OpenbookV2FulfillmentConfigStatus {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for OpenbookV2FulfillmentConfigStatus {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for OpenbookV2FulfillmentConfigStatus {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[62, 152, 127, 242, 21, 146, 146, 126];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for InitializePhoenixFulfillmentConfig {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for InitializePhoenixFulfillmentConfig {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for InitializePhoenixFulfillmentConfig {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for InitializePhoenixFulfillmentConfig {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for InitializePhoenixFulfillmentConfig {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct PhoenixFulfillmentConfigStatus {
        pub state: Pubkey,
        pub phoenix_fulfillment_config: Pubkey,
        pub admin: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PhoenixFulfillmentConfigStatus {
        const DISCRIMINATOR: &[u8] = &[220, 133, 48, 129, 233, 8, 182, 172];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for PhoenixFulfillmentConfigStatus {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for PhoenixFulfillmentConfigStatus {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for PhoenixFulfillmentConfigStatus {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for PhoenixFulfillmentConfigStatus {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for PhoenixFulfillmentConfigStatus {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateSerumVault {
        pub state: Pubkey,
        pub admin: Pubkey,
        pub srm_vault: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSerumVault {
        const DISCRIMINATOR: &[u8] = &[156, 242, 103, 240, 181, 141, 22, 33];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateSerumVault {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateSerumVault {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateSerumVault {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateSerumVault {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateSerumVault {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[25, 16, 69, 186, 57, 158, 209, 76];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for InitializePerpMarket {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for InitializePerpMarket {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for InitializePerpMarket {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for InitializePerpMarket {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for InitializePerpMarket {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct InitializePredictionMarket {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializePredictionMarket {
        const DISCRIMINATOR: &[u8] = &[20, 122, 255, 2, 124, 75, 145, 6];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for InitializePredictionMarket {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for InitializePredictionMarket {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for InitializePredictionMarket {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for InitializePredictionMarket {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for InitializePredictionMarket {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct DeleteInitializedPerpMarket {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for DeleteInitializedPerpMarket {
        const DISCRIMINATOR: &[u8] = &[40, 7, 193, 147, 191, 12, 100, 252];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for DeleteInitializedPerpMarket {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for DeleteInitializedPerpMarket {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for DeleteInitializedPerpMarket {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for DeleteInitializedPerpMarket {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for DeleteInitializedPerpMarket {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct MoveAmmPrice {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for MoveAmmPrice {
        const DISCRIMINATOR: &[u8] = &[236, 153, 176, 246, 97, 108, 58, 10];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for MoveAmmPrice {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for MoveAmmPrice {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for MoveAmmPrice {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for MoveAmmPrice {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for MoveAmmPrice {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct RecenterPerpMarketAmm {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for RecenterPerpMarketAmm {
        const DISCRIMINATOR: &[u8] = &[152, 45, 182, 137, 52, 102, 161, 100];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for RecenterPerpMarketAmm {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for RecenterPerpMarketAmm {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for RecenterPerpMarketAmm {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for RecenterPerpMarketAmm {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for RecenterPerpMarketAmm {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct RecenterPerpMarketAmmCrank {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
        pub spot_market: Pubkey,
        pub oracle: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for RecenterPerpMarketAmmCrank {
        const DISCRIMINATOR: &[u8] = &[38, 29, 78, 100, 174, 48, 152, 253];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for RecenterPerpMarketAmmCrank {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for RecenterPerpMarketAmmCrank {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for RecenterPerpMarketAmmCrank {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for RecenterPerpMarketAmmCrank {}
    #[automatically_derived]
    impl ToAccountMetas for RecenterPerpMarketAmmCrank {
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for RecenterPerpMarketAmmCrank {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for RecenterPerpMarketAmmCrank {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpMarketAmmSummaryStats {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
        pub spot_market: Pubkey,
        pub oracle: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketAmmSummaryStats {
        const DISCRIMINATOR: &[u8] = &[99, 157, 213, 53, 50, 43, 173, 112];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpMarketAmmSummaryStats {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpMarketAmmSummaryStats {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpMarketAmmSummaryStats {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpMarketAmmSummaryStats {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpMarketAmmSummaryStats {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpMarketExpiry {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketExpiry {
        const DISCRIMINATOR: &[u8] = &[0, 190, 50, 234, 33, 65, 89, 123];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpMarketExpiry {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpMarketExpiry {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpMarketExpiry {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpMarketExpiry {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpMarketExpiry {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct SettleExpiredMarketPoolsToRevenuePool {
        pub state: Pubkey,
        pub admin: Pubkey,
        pub spot_market: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for SettleExpiredMarketPoolsToRevenuePool {
        const DISCRIMINATOR: &[u8] = &[94, 227, 127, 9, 147, 38, 93, 45];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for SettleExpiredMarketPoolsToRevenuePool {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for SettleExpiredMarketPoolsToRevenuePool {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for SettleExpiredMarketPoolsToRevenuePool {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for SettleExpiredMarketPoolsToRevenuePool {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for SettleExpiredMarketPoolsToRevenuePool {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[135, 163, 183, 80, 184, 65, 88, 104];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for DepositIntoPerpMarketFeePool {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for DepositIntoPerpMarketFeePool {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for DepositIntoPerpMarketFeePool {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for DepositIntoPerpMarketFeePool {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for DepositIntoPerpMarketFeePool {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpMarketPnlPool {
        pub state: Pubkey,
        pub admin: Pubkey,
        pub spot_market: Pubkey,
        pub spot_market_vault: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketPnlPool {
        const DISCRIMINATOR: &[u8] = &[104, 120, 127, 193, 115, 51, 229, 51];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpMarketPnlPool {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpMarketPnlPool {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpMarketPnlPool {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketPnlPool {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketPnlPool {
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
                    pubkey: self.spot_market_vault,
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpMarketPnlPool {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpMarketPnlPool {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[56, 123, 23, 107, 140, 39, 66, 245];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for DepositIntoSpotMarketVault {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for DepositIntoSpotMarketVault {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for DepositIntoSpotMarketVault {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for DepositIntoSpotMarketVault {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for DepositIntoSpotMarketVault {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
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
        const DISCRIMINATOR: &[u8] = &[120, 221, 129, 235, 106, 205, 195, 210];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for DepositIntoSpotMarketRevenuePool {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for DepositIntoSpotMarketRevenuePool {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for DepositIntoSpotMarketRevenuePool {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for DepositIntoSpotMarketRevenuePool {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for DepositIntoSpotMarketRevenuePool {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct RepegAmmCurve {
        pub state: Pubkey,
        pub perp_market: Pubkey,
        pub oracle: Pubkey,
        pub admin: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for RepegAmmCurve {
        const DISCRIMINATOR: &[u8] = &[183, 96, 186, 13, 27, 52, 226, 194];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for RepegAmmCurve {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for RepegAmmCurve {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for RepegAmmCurve {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for RepegAmmCurve {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for RepegAmmCurve {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpMarketAmmOracleTwap {
        pub state: Pubkey,
        pub perp_market: Pubkey,
        pub oracle: Pubkey,
        pub admin: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketAmmOracleTwap {
        const DISCRIMINATOR: &[u8] = &[4, 62, 151, 214, 85, 102, 165, 154];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpMarketAmmOracleTwap {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpMarketAmmOracleTwap {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpMarketAmmOracleTwap {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpMarketAmmOracleTwap {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpMarketAmmOracleTwap {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct ResetPerpMarketAmmOracleTwap {
        pub state: Pubkey,
        pub perp_market: Pubkey,
        pub oracle: Pubkey,
        pub admin: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for ResetPerpMarketAmmOracleTwap {
        const DISCRIMINATOR: &[u8] = &[31, 97, 152, 72, 18, 20, 35, 195];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for ResetPerpMarketAmmOracleTwap {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for ResetPerpMarketAmmOracleTwap {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for ResetPerpMarketAmmOracleTwap {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for ResetPerpMarketAmmOracleTwap {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for ResetPerpMarketAmmOracleTwap {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateK {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
        pub oracle: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateK {
        const DISCRIMINATOR: &[u8] = &[173, 8, 27, 196, 81, 191, 36, 109];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateK {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateK {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateK {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateK {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateK {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpMarketMarginRatio {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketMarginRatio {
        const DISCRIMINATOR: &[u8] = &[221, 168, 98, 81, 42, 207, 199, 104];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpMarketMarginRatio {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpMarketMarginRatio {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpMarketMarginRatio {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpMarketMarginRatio {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpMarketMarginRatio {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpMarketHighLeverageMarginRatio {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketHighLeverageMarginRatio {
        const DISCRIMINATOR: &[u8] = &[94, 44, 114, 224, 250, 149, 47, 90];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpMarketHighLeverageMarginRatio {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpMarketHighLeverageMarginRatio {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpMarketHighLeverageMarginRatio {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketHighLeverageMarginRatio {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketHighLeverageMarginRatio {
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpMarketHighLeverageMarginRatio {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpMarketHighLeverageMarginRatio {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpMarketFundingPeriod {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketFundingPeriod {
        const DISCRIMINATOR: &[u8] = &[143, 196, 48, 65, 237, 226, 95, 77];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpMarketFundingPeriod {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpMarketFundingPeriod {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpMarketFundingPeriod {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpMarketFundingPeriod {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpMarketFundingPeriod {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpMarketMaxImbalances {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketMaxImbalances {
        const DISCRIMINATOR: &[u8] = &[166, 19, 134, 181, 163, 163, 221, 128];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpMarketMaxImbalances {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpMarketMaxImbalances {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpMarketMaxImbalances {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpMarketMaxImbalances {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpMarketMaxImbalances {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpMarketLiquidationFee {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketLiquidationFee {
        const DISCRIMINATOR: &[u8] = &[12, 91, 76, 183, 11, 62, 192, 215];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpMarketLiquidationFee {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpMarketLiquidationFee {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpMarketLiquidationFee {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpMarketLiquidationFee {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpMarketLiquidationFee {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateInsuranceFundUnstakingPeriod {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateInsuranceFundUnstakingPeriod {
        const DISCRIMINATOR: &[u8] = &[221, 191, 114, 7, 101, 250, 31, 201];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateInsuranceFundUnstakingPeriod {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateInsuranceFundUnstakingPeriod {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateInsuranceFundUnstakingPeriod {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateInsuranceFundUnstakingPeriod {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateInsuranceFundUnstakingPeriod {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateSpotMarketPoolId {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketPoolId {
        const DISCRIMINATOR: &[u8] = &[221, 222, 116, 19, 147, 70, 109, 228];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateSpotMarketPoolId {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateSpotMarketPoolId {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateSpotMarketPoolId {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateSpotMarketPoolId {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateSpotMarketPoolId {
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateSpotMarketPoolId {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateSpotMarketPoolId {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateSpotMarketLiquidationFee {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketLiquidationFee {
        const DISCRIMINATOR: &[u8] = &[233, 115, 0, 78, 143, 75, 33, 75];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateSpotMarketLiquidationFee {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateSpotMarketLiquidationFee {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateSpotMarketLiquidationFee {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateSpotMarketLiquidationFee {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateSpotMarketLiquidationFee {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateWithdrawGuardThreshold {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateWithdrawGuardThreshold {
        const DISCRIMINATOR: &[u8] = &[164, 77, 57, 122, 175, 179, 81, 84];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateWithdrawGuardThreshold {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateWithdrawGuardThreshold {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateWithdrawGuardThreshold {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateWithdrawGuardThreshold {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateWithdrawGuardThreshold {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateSpotMarketIfFactor {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketIfFactor {
        const DISCRIMINATOR: &[u8] = &[235, 61, 57, 159, 24, 10, 159, 53];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateSpotMarketIfFactor {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateSpotMarketIfFactor {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateSpotMarketIfFactor {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateSpotMarketIfFactor {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateSpotMarketIfFactor {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateSpotMarketRevenueSettlePeriod {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketRevenueSettlePeriod {
        const DISCRIMINATOR: &[u8] = &[251, 114, 78, 11, 27, 117, 179, 21];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateSpotMarketRevenueSettlePeriod {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateSpotMarketRevenueSettlePeriod {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateSpotMarketRevenueSettlePeriod {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateSpotMarketRevenueSettlePeriod {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateSpotMarketRevenueSettlePeriod {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateSpotMarketStatus {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketStatus {
        const DISCRIMINATOR: &[u8] = &[39, 227, 245, 172, 81, 243, 74, 239];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateSpotMarketStatus {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateSpotMarketStatus {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateSpotMarketStatus {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateSpotMarketStatus {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateSpotMarketStatus {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateSpotMarketPausedOperations {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketPausedOperations {
        const DISCRIMINATOR: &[u8] = &[177, 42, 203, 86, 124, 85, 32, 39];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateSpotMarketPausedOperations {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateSpotMarketPausedOperations {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateSpotMarketPausedOperations {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateSpotMarketPausedOperations {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateSpotMarketPausedOperations {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateSpotMarketAssetTier {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketAssetTier {
        const DISCRIMINATOR: &[u8] = &[216, 218, 15, 206, 208, 164, 24, 163];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateSpotMarketAssetTier {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateSpotMarketAssetTier {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateSpotMarketAssetTier {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateSpotMarketAssetTier {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateSpotMarketAssetTier {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateSpotMarketMarginWeights {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketMarginWeights {
        const DISCRIMINATOR: &[u8] = &[32, 75, 107, 180, 16, 197, 1, 38];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateSpotMarketMarginWeights {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateSpotMarketMarginWeights {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateSpotMarketMarginWeights {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateSpotMarketMarginWeights {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateSpotMarketMarginWeights {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateSpotMarketBorrowRate {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketBorrowRate {
        const DISCRIMINATOR: &[u8] = &[216, 47, 114, 248, 0, 78, 173, 186];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateSpotMarketBorrowRate {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateSpotMarketBorrowRate {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateSpotMarketBorrowRate {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateSpotMarketBorrowRate {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateSpotMarketBorrowRate {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateSpotMarketMaxTokenDeposits {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketMaxTokenDeposits {
        const DISCRIMINATOR: &[u8] = &[89, 241, 13, 153, 229, 206, 230, 40];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateSpotMarketMaxTokenDeposits {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateSpotMarketMaxTokenDeposits {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateSpotMarketMaxTokenDeposits {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateSpotMarketMaxTokenDeposits {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateSpotMarketMaxTokenDeposits {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateSpotMarketMaxTokenBorrows {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketMaxTokenBorrows {
        const DISCRIMINATOR: &[u8] = &[76, 222, 78, 65, 19, 17, 10, 26];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateSpotMarketMaxTokenBorrows {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateSpotMarketMaxTokenBorrows {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateSpotMarketMaxTokenBorrows {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateSpotMarketMaxTokenBorrows {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateSpotMarketMaxTokenBorrows {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateSpotMarketScaleInitialAssetWeightStart {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketScaleInitialAssetWeightStart {
        const DISCRIMINATOR: &[u8] = &[154, 211, 71, 7, 119, 171, 98, 72];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateSpotMarketScaleInitialAssetWeightStart {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable
        for UpdateSpotMarketScaleInitialAssetWeightStart
    {
    }
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateSpotMarketScaleInitialAssetWeightStart {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateSpotMarketScaleInitialAssetWeightStart {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateSpotMarketScaleInitialAssetWeightStart {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateSpotMarketOracle {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
        pub oracle: Pubkey,
        pub old_oracle: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketOracle {
        const DISCRIMINATOR: &[u8] = &[36, 174, 101, 206, 244, 70, 211, 189];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateSpotMarketOracle {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateSpotMarketOracle {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateSpotMarketOracle {}
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
                AccountMeta {
                    pubkey: self.old_oracle,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateSpotMarketOracle {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateSpotMarketOracle {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateSpotMarketStepSizeAndTickSize {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketStepSizeAndTickSize {
        const DISCRIMINATOR: &[u8] = &[129, 204, 160, 151, 133, 198, 153, 119];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateSpotMarketStepSizeAndTickSize {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateSpotMarketStepSizeAndTickSize {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateSpotMarketStepSizeAndTickSize {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateSpotMarketStepSizeAndTickSize {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateSpotMarketStepSizeAndTickSize {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateSpotMarketMinOrderSize {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketMinOrderSize {
        const DISCRIMINATOR: &[u8] = &[7, 136, 27, 144, 153, 150, 80, 125];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateSpotMarketMinOrderSize {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateSpotMarketMinOrderSize {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateSpotMarketMinOrderSize {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateSpotMarketMinOrderSize {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateSpotMarketMinOrderSize {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateSpotMarketOrdersEnabled {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketOrdersEnabled {
        const DISCRIMINATOR: &[u8] = &[19, 211, 29, 85, 150, 174, 176, 255];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateSpotMarketOrdersEnabled {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateSpotMarketOrdersEnabled {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateSpotMarketOrdersEnabled {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateSpotMarketOrdersEnabled {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateSpotMarketOrdersEnabled {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateSpotMarketIfPausedOperations {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketIfPausedOperations {
        const DISCRIMINATOR: &[u8] = &[198, 225, 237, 109, 145, 233, 119, 66];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateSpotMarketIfPausedOperations {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateSpotMarketIfPausedOperations {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateSpotMarketIfPausedOperations {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateSpotMarketIfPausedOperations {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateSpotMarketIfPausedOperations {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateSpotMarketName {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketName {
        const DISCRIMINATOR: &[u8] = &[132, 15, 105, 156, 160, 4, 62, 223];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateSpotMarketName {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateSpotMarketName {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateSpotMarketName {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateSpotMarketName {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateSpotMarketName {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpMarketStatus {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketStatus {
        const DISCRIMINATOR: &[u8] = &[224, 52, 220, 78, 238, 167, 101, 138];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpMarketStatus {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpMarketStatus {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpMarketStatus {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpMarketStatus {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpMarketStatus {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpMarketPausedOperations {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketPausedOperations {
        const DISCRIMINATOR: &[u8] = &[51, 168, 150, 68, 102, 63, 155, 3];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpMarketPausedOperations {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpMarketPausedOperations {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpMarketPausedOperations {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpMarketPausedOperations {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpMarketPausedOperations {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpMarketContractTier {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketContractTier {
        const DISCRIMINATOR: &[u8] = &[222, 107, 185, 64, 127, 171, 165, 198];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpMarketContractTier {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpMarketContractTier {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpMarketContractTier {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpMarketContractTier {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpMarketContractTier {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpMarketImfFactor {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketImfFactor {
        const DISCRIMINATOR: &[u8] = &[225, 32, 202, 238, 175, 168, 66, 111];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpMarketImfFactor {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpMarketImfFactor {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpMarketImfFactor {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpMarketImfFactor {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpMarketImfFactor {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpMarketUnrealizedAssetWeight {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketUnrealizedAssetWeight {
        const DISCRIMINATOR: &[u8] = &[96, 83, 120, 200, 98, 169, 198, 236];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpMarketUnrealizedAssetWeight {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpMarketUnrealizedAssetWeight {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpMarketUnrealizedAssetWeight {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpMarketUnrealizedAssetWeight {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpMarketUnrealizedAssetWeight {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpMarketConcentrationCoef {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketConcentrationCoef {
        const DISCRIMINATOR: &[u8] = &[216, 212, 111, 9, 250, 64, 121, 75];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpMarketConcentrationCoef {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpMarketConcentrationCoef {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpMarketConcentrationCoef {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpMarketConcentrationCoef {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpMarketConcentrationCoef {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpMarketCurveUpdateIntensity {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketCurveUpdateIntensity {
        const DISCRIMINATOR: &[u8] = &[174, 119, 97, 13, 217, 130, 136, 102];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpMarketCurveUpdateIntensity {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpMarketCurveUpdateIntensity {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpMarketCurveUpdateIntensity {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpMarketCurveUpdateIntensity {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpMarketCurveUpdateIntensity {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateLpCooldownTime {
        pub admin: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateLpCooldownTime {
        const DISCRIMINATOR: &[u8] = &[176, 192, 230, 203, 208, 6, 140, 65];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateLpCooldownTime {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateLpCooldownTime {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateLpCooldownTime {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateLpCooldownTime {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateLpCooldownTime {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpFeeStructure {
        pub admin: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpFeeStructure {
        const DISCRIMINATOR: &[u8] = &[200, 56, 109, 180, 172, 159, 143, 220];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpFeeStructure {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpFeeStructure {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpFeeStructure {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpFeeStructure {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpFeeStructure {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateSpotFeeStructure {
        pub admin: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotFeeStructure {
        const DISCRIMINATOR: &[u8] = &[24, 12, 37, 124, 132, 63, 160, 6];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateSpotFeeStructure {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateSpotFeeStructure {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateSpotFeeStructure {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateSpotFeeStructure {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateSpotFeeStructure {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateInitialPctToLiquidate {
        pub admin: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateInitialPctToLiquidate {
        const DISCRIMINATOR: &[u8] = &[245, 97, 64, 31, 129, 51, 40, 245];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateInitialPctToLiquidate {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateInitialPctToLiquidate {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateInitialPctToLiquidate {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateInitialPctToLiquidate {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateInitialPctToLiquidate {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateLiquidationDuration {
        pub admin: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateLiquidationDuration {
        const DISCRIMINATOR: &[u8] = &[195, 45, 228, 75, 98, 127, 63, 63];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateLiquidationDuration {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateLiquidationDuration {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateLiquidationDuration {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateLiquidationDuration {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateLiquidationDuration {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateLiquidationMarginBufferRatio {
        pub admin: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateLiquidationMarginBufferRatio {
        const DISCRIMINATOR: &[u8] = &[5, 215, 155, 184, 19, 125, 142, 66];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateLiquidationMarginBufferRatio {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateLiquidationMarginBufferRatio {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateLiquidationMarginBufferRatio {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateLiquidationMarginBufferRatio {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateLiquidationMarginBufferRatio {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateOracleGuardRails {
        pub admin: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateOracleGuardRails {
        const DISCRIMINATOR: &[u8] = &[141, 30, 9, 240, 136, 75, 69, 245];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateOracleGuardRails {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateOracleGuardRails {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateOracleGuardRails {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateOracleGuardRails {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateOracleGuardRails {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateStateSettlementDuration {
        pub admin: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateStateSettlementDuration {
        const DISCRIMINATOR: &[u8] = &[117, 8, 77, 34, 122, 119, 61, 165];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateStateSettlementDuration {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateStateSettlementDuration {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateStateSettlementDuration {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateStateSettlementDuration {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateStateSettlementDuration {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateStateMaxNumberOfSubAccounts {
        pub admin: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateStateMaxNumberOfSubAccounts {
        const DISCRIMINATOR: &[u8] = &[105, 137, 80, 95, 51, 50, 190, 95];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateStateMaxNumberOfSubAccounts {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateStateMaxNumberOfSubAccounts {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateStateMaxNumberOfSubAccounts {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateStateMaxNumberOfSubAccounts {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateStateMaxNumberOfSubAccounts {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateStateMaxInitializeUserFee {
        pub admin: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateStateMaxInitializeUserFee {
        const DISCRIMINATOR: &[u8] = &[183, 72, 183, 217, 46, 152, 38, 41];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateStateMaxInitializeUserFee {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateStateMaxInitializeUserFee {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateStateMaxInitializeUserFee {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateStateMaxInitializeUserFee {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateStateMaxInitializeUserFee {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpMarketOracle {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
        pub oracle: Pubkey,
        pub old_oracle: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketOracle {
        const DISCRIMINATOR: &[u8] = &[145, 236, 122, 74, 26, 16, 123, 173];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpMarketOracle {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpMarketOracle {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpMarketOracle {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketOracle {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketOracle {
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
                AccountMeta {
                    pubkey: self.old_oracle,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpMarketOracle {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpMarketOracle {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpMarketBaseSpread {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketBaseSpread {
        const DISCRIMINATOR: &[u8] = &[103, 167, 141, 61, 192, 229, 25, 38];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpMarketBaseSpread {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpMarketBaseSpread {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpMarketBaseSpread {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpMarketBaseSpread {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpMarketBaseSpread {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateAmmJitIntensity {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateAmmJitIntensity {
        const DISCRIMINATOR: &[u8] = &[53, 216, 81, 248, 60, 1, 222, 134];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateAmmJitIntensity {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateAmmJitIntensity {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateAmmJitIntensity {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateAmmJitIntensity {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateAmmJitIntensity {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpMarketMaxSpread {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketMaxSpread {
        const DISCRIMINATOR: &[u8] = &[6, 132, 169, 40, 124, 227, 156, 212];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpMarketMaxSpread {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpMarketMaxSpread {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpMarketMaxSpread {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpMarketMaxSpread {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpMarketMaxSpread {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpMarketStepSizeAndTickSize {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketStepSizeAndTickSize {
        const DISCRIMINATOR: &[u8] = &[97, 22, 195, 229, 181, 121, 32, 14];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpMarketStepSizeAndTickSize {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpMarketStepSizeAndTickSize {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpMarketStepSizeAndTickSize {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpMarketStepSizeAndTickSize {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpMarketStepSizeAndTickSize {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpMarketName {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketName {
        const DISCRIMINATOR: &[u8] = &[120, 12, 150, 158, 194, 231, 152, 183];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpMarketName {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpMarketName {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpMarketName {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpMarketName {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpMarketName {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpMarketMinOrderSize {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketMinOrderSize {
        const DISCRIMINATOR: &[u8] = &[148, 182, 143, 25, 247, 254, 104, 102];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpMarketMinOrderSize {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpMarketMinOrderSize {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpMarketMinOrderSize {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpMarketMinOrderSize {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpMarketMinOrderSize {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpMarketMaxSlippageRatio {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketMaxSlippageRatio {
        const DISCRIMINATOR: &[u8] = &[56, 192, 174, 214, 217, 227, 164, 25];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpMarketMaxSlippageRatio {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpMarketMaxSlippageRatio {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpMarketMaxSlippageRatio {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpMarketMaxSlippageRatio {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpMarketMaxSlippageRatio {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpMarketMaxFillReserveFraction {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketMaxFillReserveFraction {
        const DISCRIMINATOR: &[u8] = &[158, 50, 175, 240, 17, 123, 186, 41];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpMarketMaxFillReserveFraction {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpMarketMaxFillReserveFraction {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpMarketMaxFillReserveFraction {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpMarketMaxFillReserveFraction {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpMarketMaxFillReserveFraction {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpMarketMaxOpenInterest {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketMaxOpenInterest {
        const DISCRIMINATOR: &[u8] = &[211, 243, 37, 54, 206, 192, 240, 221];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpMarketMaxOpenInterest {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpMarketMaxOpenInterest {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpMarketMaxOpenInterest {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpMarketMaxOpenInterest {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpMarketMaxOpenInterest {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpMarketNumberOfUsers {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketNumberOfUsers {
        const DISCRIMINATOR: &[u8] = &[221, 66, 246, 127, 23, 14, 214, 137];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpMarketNumberOfUsers {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpMarketNumberOfUsers {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpMarketNumberOfUsers {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpMarketNumberOfUsers {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpMarketNumberOfUsers {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpMarketFeeAdjustment {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketFeeAdjustment {
        const DISCRIMINATOR: &[u8] = &[110, 120, 82, 31, 217, 115, 121, 11];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpMarketFeeAdjustment {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpMarketFeeAdjustment {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpMarketFeeAdjustment {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpMarketFeeAdjustment {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpMarketFeeAdjustment {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateSpotMarketFeeAdjustment {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketFeeAdjustment {
        const DISCRIMINATOR: &[u8] = &[124, 184, 119, 171, 139, 199, 27, 40];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateSpotMarketFeeAdjustment {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateSpotMarketFeeAdjustment {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateSpotMarketFeeAdjustment {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateSpotMarketFeeAdjustment {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateSpotMarketFeeAdjustment {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpMarketFuel {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketFuel {
        const DISCRIMINATOR: &[u8] = &[128, 34, 207, 128, 207, 145, 65, 46];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpMarketFuel {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpMarketFuel {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpMarketFuel {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpMarketFuel {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpMarketFuel {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpMarketProtectedMakerParams {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketProtectedMakerParams {
        const DISCRIMINATOR: &[u8] = &[31, 187, 231, 244, 165, 74, 103, 137];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpMarketProtectedMakerParams {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpMarketProtectedMakerParams {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpMarketProtectedMakerParams {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketProtectedMakerParams {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketProtectedMakerParams {
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpMarketProtectedMakerParams {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpMarketProtectedMakerParams {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpMarketTakerSpeedBumpOverride {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketTakerSpeedBumpOverride {
        const DISCRIMINATOR: &[u8] = &[17, 22, 210, 50, 255, 103, 140, 64];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpMarketTakerSpeedBumpOverride {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpMarketTakerSpeedBumpOverride {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpMarketTakerSpeedBumpOverride {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketTakerSpeedBumpOverride {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketTakerSpeedBumpOverride {
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpMarketTakerSpeedBumpOverride {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpMarketTakerSpeedBumpOverride {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpMarketAmmSpreadAdjustment {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketAmmSpreadAdjustment {
        const DISCRIMINATOR: &[u8] = &[33, 229, 109, 138, 163, 201, 26, 95];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpMarketAmmSpreadAdjustment {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpMarketAmmSpreadAdjustment {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpMarketAmmSpreadAdjustment {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketAmmSpreadAdjustment {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketAmmSpreadAdjustment {
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpMarketAmmSpreadAdjustment {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpMarketAmmSpreadAdjustment {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpMarketOracleSlotDelayOverride {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpMarketOracleSlotDelayOverride {
        const DISCRIMINATOR: &[u8] = &[2, 164, 70, 196, 79, 149, 182, 73];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpMarketOracleSlotDelayOverride {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpMarketOracleSlotDelayOverride {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpMarketOracleSlotDelayOverride {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdatePerpMarketOracleSlotDelayOverride {}
    #[automatically_derived]
    impl ToAccountMetas for UpdatePerpMarketOracleSlotDelayOverride {
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpMarketOracleSlotDelayOverride {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpMarketOracleSlotDelayOverride {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateSpotMarketFuel {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub spot_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotMarketFuel {
        const DISCRIMINATOR: &[u8] = &[169, 6, 208, 85, 23, 115, 22, 75];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateSpotMarketFuel {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateSpotMarketFuel {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateSpotMarketFuel {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateSpotMarketFuel {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateSpotMarketFuel {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct InitUserFuel {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub user: Pubkey,
        pub user_stats: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitUserFuel {
        const DISCRIMINATOR: &[u8] = &[78, 54, 127, 169, 245, 179, 149, 35];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for InitUserFuel {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for InitUserFuel {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for InitUserFuel {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for InitUserFuel {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for InitUserFuel {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateAdmin {
        pub admin: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateAdmin {
        const DISCRIMINATOR: &[u8] = &[226, 255, 184, 234, 67, 191, 5, 182];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateAdmin {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateAdmin {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateAdmin {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateAdmin {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateAdmin {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateWhitelistMint {
        pub admin: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateWhitelistMint {
        const DISCRIMINATOR: &[u8] = &[227, 158, 63, 228, 73, 69, 23, 54];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateWhitelistMint {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateWhitelistMint {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateWhitelistMint {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateWhitelistMint {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateWhitelistMint {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateDiscountMint {
        pub admin: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateDiscountMint {
        const DISCRIMINATOR: &[u8] = &[26, 238, 24, 203, 91, 255, 142, 129];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateDiscountMint {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateDiscountMint {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateDiscountMint {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateDiscountMint {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateDiscountMint {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateExchangeStatus {
        pub admin: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateExchangeStatus {
        const DISCRIMINATOR: &[u8] = &[54, 251, 5, 25, 149, 14, 206, 156];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateExchangeStatus {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateExchangeStatus {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateExchangeStatus {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateExchangeStatus {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateExchangeStatus {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePerpAuctionDuration {
        pub admin: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePerpAuctionDuration {
        const DISCRIMINATOR: &[u8] = &[220, 51, 72, 247, 206, 206, 238, 110];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePerpAuctionDuration {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePerpAuctionDuration {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePerpAuctionDuration {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePerpAuctionDuration {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePerpAuctionDuration {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateSpotAuctionDuration {
        pub admin: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateSpotAuctionDuration {
        const DISCRIMINATOR: &[u8] = &[227, 57, 154, 80, 55, 26, 182, 148];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateSpotAuctionDuration {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateSpotAuctionDuration {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateSpotAuctionDuration {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateSpotAuctionDuration {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateSpotAuctionDuration {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct InitializeProtocolIfSharesTransferConfig {
        pub admin: Pubkey,
        pub protocol_if_shares_transfer_config: Pubkey,
        pub state: Pubkey,
        pub rent: Pubkey,
        pub system_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeProtocolIfSharesTransferConfig {
        const DISCRIMINATOR: &[u8] = &[204, 47, 201, 74, 217, 201, 130, 232];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for InitializeProtocolIfSharesTransferConfig {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable
        for InitializeProtocolIfSharesTransferConfig
    {
    }
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for InitializeProtocolIfSharesTransferConfig {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for InitializeProtocolIfSharesTransferConfig {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for InitializeProtocolIfSharesTransferConfig {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateProtocolIfSharesTransferConfig {
        pub admin: Pubkey,
        pub protocol_if_shares_transfer_config: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateProtocolIfSharesTransferConfig {
        const DISCRIMINATOR: &[u8] = &[244, 48, 134, 239, 41, 44, 96, 131];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateProtocolIfSharesTransferConfig {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateProtocolIfSharesTransferConfig {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateProtocolIfSharesTransferConfig {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateProtocolIfSharesTransferConfig {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateProtocolIfSharesTransferConfig {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct InitializePrelaunchOracle {
        pub admin: Pubkey,
        pub prelaunch_oracle: Pubkey,
        pub state: Pubkey,
        pub rent: Pubkey,
        pub system_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializePrelaunchOracle {
        const DISCRIMINATOR: &[u8] = &[130, 152, 19, 253, 63, 72, 246, 220];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for InitializePrelaunchOracle {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for InitializePrelaunchOracle {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for InitializePrelaunchOracle {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for InitializePrelaunchOracle {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for InitializePrelaunchOracle {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdatePrelaunchOracleParams {
        pub admin: Pubkey,
        pub prelaunch_oracle: Pubkey,
        pub perp_market: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdatePrelaunchOracleParams {
        const DISCRIMINATOR: &[u8] = &[197, 77, 11, 137, 115, 92, 181, 124];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdatePrelaunchOracleParams {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdatePrelaunchOracleParams {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdatePrelaunchOracleParams {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdatePrelaunchOracleParams {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdatePrelaunchOracleParams {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct DeletePrelaunchOracle {
        pub admin: Pubkey,
        pub prelaunch_oracle: Pubkey,
        pub perp_market: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for DeletePrelaunchOracle {
        const DISCRIMINATOR: &[u8] = &[124, 0, 150, 210, 115, 171, 215, 202];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for DeletePrelaunchOracle {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for DeletePrelaunchOracle {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for DeletePrelaunchOracle {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for DeletePrelaunchOracle {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for DeletePrelaunchOracle {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct InitializePythPullOracle {
        pub admin: Pubkey,
        pub pyth_solana_receiver: Pubkey,
        pub price_feed: Pubkey,
        pub system_program: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializePythPullOracle {
        const DISCRIMINATOR: &[u8] = &[11, 151, 145, 80, 21, 164, 2, 147];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for InitializePythPullOracle {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for InitializePythPullOracle {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for InitializePythPullOracle {}
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for InitializePythPullOracle {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for InitializePythPullOracle {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct InitializePythLazerOracle {
        pub admin: Pubkey,
        pub lazer_oracle: Pubkey,
        pub state: Pubkey,
        pub rent: Pubkey,
        pub system_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializePythLazerOracle {
        const DISCRIMINATOR: &[u8] = &[89, 72, 144, 241, 94, 171, 28, 143];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for InitializePythLazerOracle {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for InitializePythLazerOracle {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for InitializePythLazerOracle {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializePythLazerOracle {}
    #[automatically_derived]
    impl ToAccountMetas for InitializePythLazerOracle {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.lazer_oracle,
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for InitializePythLazerOracle {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for InitializePythLazerOracle {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct PostPythLazerOracleUpdate {
        pub keeper: Pubkey,
        pub pyth_lazer_storage: Pubkey,
        pub ix_sysvar: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for PostPythLazerOracleUpdate {
        const DISCRIMINATOR: &[u8] = &[168, 250, 82, 74, 96, 140, 128, 207];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for PostPythLazerOracleUpdate {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for PostPythLazerOracleUpdate {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for PostPythLazerOracleUpdate {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for PostPythLazerOracleUpdate {}
    #[automatically_derived]
    impl ToAccountMetas for PostPythLazerOracleUpdate {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.keeper,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.pyth_lazer_storage,
                    is_signer: false,
                    is_writable: false,
                },
                AccountMeta {
                    pubkey: self.ix_sysvar,
                    is_signer: false,
                    is_writable: false,
                },
            ]
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for PostPythLazerOracleUpdate {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for PostPythLazerOracleUpdate {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct InitializeHighLeverageModeConfig {
        pub admin: Pubkey,
        pub high_leverage_mode_config: Pubkey,
        pub state: Pubkey,
        pub rent: Pubkey,
        pub system_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeHighLeverageModeConfig {
        const DISCRIMINATOR: &[u8] = &[125, 235, 77, 45, 130, 90, 134, 48];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for InitializeHighLeverageModeConfig {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for InitializeHighLeverageModeConfig {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for InitializeHighLeverageModeConfig {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeHighLeverageModeConfig {}
    #[automatically_derived]
    impl ToAccountMetas for InitializeHighLeverageModeConfig {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.high_leverage_mode_config,
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for InitializeHighLeverageModeConfig {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for InitializeHighLeverageModeConfig {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateHighLeverageModeConfig {
        pub admin: Pubkey,
        pub high_leverage_mode_config: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateHighLeverageModeConfig {
        const DISCRIMINATOR: &[u8] = &[254, 192, 159, 254, 254, 74, 141, 70];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateHighLeverageModeConfig {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateHighLeverageModeConfig {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateHighLeverageModeConfig {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateHighLeverageModeConfig {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateHighLeverageModeConfig {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.high_leverage_mode_config,
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateHighLeverageModeConfig {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateHighLeverageModeConfig {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct InitializeProtectedMakerModeConfig {
        pub admin: Pubkey,
        pub protected_maker_mode_config: Pubkey,
        pub state: Pubkey,
        pub rent: Pubkey,
        pub system_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeProtectedMakerModeConfig {
        const DISCRIMINATOR: &[u8] = &[71, 150, 108, 182, 19, 30, 72, 149];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for InitializeProtectedMakerModeConfig {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for InitializeProtectedMakerModeConfig {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for InitializeProtectedMakerModeConfig {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeProtectedMakerModeConfig {}
    #[automatically_derived]
    impl ToAccountMetas for InitializeProtectedMakerModeConfig {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.protected_maker_mode_config,
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for InitializeProtectedMakerModeConfig {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for InitializeProtectedMakerModeConfig {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateProtectedMakerModeConfig {
        pub admin: Pubkey,
        pub protected_maker_mode_config: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateProtectedMakerModeConfig {
        const DISCRIMINATOR: &[u8] = &[189, 135, 186, 140, 137, 238, 182, 65];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateProtectedMakerModeConfig {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateProtectedMakerModeConfig {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateProtectedMakerModeConfig {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateProtectedMakerModeConfig {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateProtectedMakerModeConfig {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.protected_maker_mode_config,
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateProtectedMakerModeConfig {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateProtectedMakerModeConfig {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct AdminDeposit {
        pub state: Pubkey,
        pub user: Pubkey,
        pub admin: Pubkey,
        pub spot_market_vault: Pubkey,
        pub admin_token_account: Pubkey,
        pub token_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for AdminDeposit {
        const DISCRIMINATOR: &[u8] = &[164, 41, 145, 198, 178, 181, 43, 8];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for AdminDeposit {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for AdminDeposit {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for AdminDeposit {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for AdminDeposit {}
    #[automatically_derived]
    impl ToAccountMetas for AdminDeposit {
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
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.spot_market_vault,
                    is_signer: false,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.admin_token_account,
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for AdminDeposit {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for AdminDeposit {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct InitializeIfRebalanceConfig {
        pub admin: Pubkey,
        pub if_rebalance_config: Pubkey,
        pub state: Pubkey,
        pub rent: Pubkey,
        pub system_program: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for InitializeIfRebalanceConfig {
        const DISCRIMINATOR: &[u8] = &[35, 170, 219, 129, 208, 77, 60, 46];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for InitializeIfRebalanceConfig {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for InitializeIfRebalanceConfig {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for InitializeIfRebalanceConfig {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for InitializeIfRebalanceConfig {}
    #[automatically_derived]
    impl ToAccountMetas for InitializeIfRebalanceConfig {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.if_rebalance_config,
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for InitializeIfRebalanceConfig {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for InitializeIfRebalanceConfig {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateIfRebalanceConfig {
        pub admin: Pubkey,
        pub if_rebalance_config: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateIfRebalanceConfig {
        const DISCRIMINATOR: &[u8] = &[190, 159, 23, 62, 227, 167, 30, 78];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateIfRebalanceConfig {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateIfRebalanceConfig {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateIfRebalanceConfig {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateIfRebalanceConfig {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateIfRebalanceConfig {
        fn to_account_metas(&self) -> Vec<AccountMeta> {
            vec![
                AccountMeta {
                    pubkey: self.admin,
                    is_signer: true,
                    is_writable: true,
                },
                AccountMeta {
                    pubkey: self.if_rebalance_config,
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateIfRebalanceConfig {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateIfRebalanceConfig {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateFeatureBitFlagsMmOracle {
        pub admin: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateFeatureBitFlagsMmOracle {
        const DISCRIMINATOR: &[u8] = &[88, 1, 170, 20, 116, 55, 171, 64];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateFeatureBitFlagsMmOracle {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateFeatureBitFlagsMmOracle {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateFeatureBitFlagsMmOracle {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateFeatureBitFlagsMmOracle {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateFeatureBitFlagsMmOracle {
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateFeatureBitFlagsMmOracle {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateFeatureBitFlagsMmOracle {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct ZeroMmOracleFields {
        pub admin: Pubkey,
        pub state: Pubkey,
        pub perp_market: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for ZeroMmOracleFields {
        const DISCRIMINATOR: &[u8] = &[163, 39, 36, 8, 37, 81, 249, 83];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for ZeroMmOracleFields {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for ZeroMmOracleFields {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for ZeroMmOracleFields {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for ZeroMmOracleFields {}
    #[automatically_derived]
    impl ToAccountMetas for ZeroMmOracleFields {
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for ZeroMmOracleFields {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for ZeroMmOracleFields {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
    #[repr(C)]
    #[derive(Copy, Clone, Default, AnchorSerialize, AnchorDeserialize, Serialize, Deserialize)]
    pub struct UpdateFeatureBitFlagsMedianTriggerPrice {
        pub admin: Pubkey,
        pub state: Pubkey,
    }
    #[automatically_derived]
    impl anchor_lang::Discriminator for UpdateFeatureBitFlagsMedianTriggerPrice {
        const DISCRIMINATOR: &[u8] = &[45, 231, 165, 199, 62, 17, 64, 24];
    }
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Pod for UpdateFeatureBitFlagsMedianTriggerPrice {}
    #[automatically_derived]
    unsafe impl anchor_lang::__private::bytemuck::Zeroable for UpdateFeatureBitFlagsMedianTriggerPrice {}
    #[automatically_derived]
    impl anchor_lang::ZeroCopy for UpdateFeatureBitFlagsMedianTriggerPrice {}
    #[automatically_derived]
    impl anchor_lang::InstructionData for UpdateFeatureBitFlagsMedianTriggerPrice {}
    #[automatically_derived]
    impl ToAccountMetas for UpdateFeatureBitFlagsMedianTriggerPrice {
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
    #[automatically_derived]
    impl anchor_lang::AccountSerialize for UpdateFeatureBitFlagsMedianTriggerPrice {
        fn try_serialize<W: std::io::Write>(&self, writer: &mut W) -> anchor_lang::Result<()> {
            if writer.write_all(Self::DISCRIMINATOR).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            if AnchorSerialize::serialize(self, writer).is_err() {
                return Err(anchor_lang::error::ErrorCode::AccountDidNotSerialize.into());
            }
            Ok(())
        }
    }
    #[automatically_derived]
    impl anchor_lang::AccountDeserialize for UpdateFeatureBitFlagsMedianTriggerPrice {
        fn try_deserialize(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let given_disc = &buf[..8];
            if Self::DISCRIMINATOR != given_disc {
                return Err(anchor_lang::error!(
                    anchor_lang::error::ErrorCode::AccountDiscriminatorMismatch
                ));
            }
            Self::try_deserialize_unchecked(buf)
        }
        fn try_deserialize_unchecked(buf: &mut &[u8]) -> anchor_lang::Result<Self> {
            let mut data: &[u8] = &buf[8..];
            AnchorDeserialize::deserialize(&mut data)
                .map_err(|_| anchor_lang::error::ErrorCode::AccountDidNotDeserialize.into())
        }
    }
}
pub mod errors {
    #![doc = r" IDL error types"]
    use super::{types::*, *};
    #[derive(PartialEq)]
    #[error_code]
    pub enum ErrorCode {
        #[msg("Invalid Spot Market Authority")]
        InvalidSpotMarketAuthority,
        #[msg("Clearing house not insurance fund authority")]
        InvalidInsuranceFundAuthority,
        #[msg("Insufficient deposit")]
        InsufficientDeposit,
        #[msg("Insufficient collateral")]
        InsufficientCollateral,
        #[msg("Sufficient collateral")]
        SufficientCollateral,
        #[msg("Max number of positions taken")]
        MaxNumberOfPositions,
        #[msg("Admin Controls Prices Disabled")]
        AdminControlsPricesDisabled,
        #[msg("Market Delisted")]
        MarketDelisted,
        #[msg("Market Index Already Initialized")]
        MarketIndexAlreadyInitialized,
        #[msg("User Account And User Positions Account Mismatch")]
        UserAccountAndUserPositionsAccountMismatch,
        #[msg("User Has No Position In Market")]
        UserHasNoPositionInMarket,
        #[msg("Invalid Initial Peg")]
        InvalidInitialPeg,
        #[msg("AMM repeg already configured with amt given")]
        InvalidRepegRedundant,
        #[msg("AMM repeg incorrect repeg direction")]
        InvalidRepegDirection,
        #[msg("AMM repeg out of bounds pnl")]
        InvalidRepegProfitability,
        #[msg("Slippage Outside Limit Price")]
        SlippageOutsideLimit,
        #[msg("Order Size Too Small")]
        OrderSizeTooSmall,
        #[msg("Price change too large when updating K")]
        InvalidUpdateK,
        #[msg("Admin tried to withdraw amount larger than fees collected")]
        AdminWithdrawTooLarge,
        #[msg("Math Error")]
        MathError,
        #[msg("Conversion to u128/u64 failed with an overflow or underflow")]
        BnConversionError,
        #[msg("Clock unavailable")]
        ClockUnavailable,
        #[msg("Unable To Load Oracles")]
        UnableToLoadOracle,
        #[msg("Price Bands Breached")]
        PriceBandsBreached,
        #[msg("Exchange is paused")]
        ExchangePaused,
        #[msg("Invalid whitelist token")]
        InvalidWhitelistToken,
        #[msg("Whitelist token not found")]
        WhitelistTokenNotFound,
        #[msg("Invalid discount token")]
        InvalidDiscountToken,
        #[msg("Discount token not found")]
        DiscountTokenNotFound,
        #[msg("Referrer not found")]
        ReferrerNotFound,
        #[msg("ReferrerNotFound")]
        ReferrerStatsNotFound,
        #[msg("ReferrerMustBeWritable")]
        ReferrerMustBeWritable,
        #[msg("ReferrerMustBeWritable")]
        ReferrerStatsMustBeWritable,
        #[msg("ReferrerAndReferrerStatsAuthorityUnequal")]
        ReferrerAndReferrerStatsAuthorityUnequal,
        #[msg("InvalidReferrer")]
        InvalidReferrer,
        #[msg("InvalidOracle")]
        InvalidOracle,
        #[msg("OracleNotFound")]
        OracleNotFound,
        #[msg("Liquidations Blocked By Oracle")]
        LiquidationsBlockedByOracle,
        #[msg("Can not deposit more than max deposit")]
        MaxDeposit,
        #[msg("Can not delete user that still has collateral")]
        CantDeleteUserWithCollateral,
        #[msg("AMM funding out of bounds pnl")]
        InvalidFundingProfitability,
        #[msg("Casting Failure")]
        CastingFailure,
        #[msg("InvalidOrder")]
        InvalidOrder,
        #[msg("InvalidOrderMaxTs")]
        InvalidOrderMaxTs,
        #[msg("InvalidOrderMarketType")]
        InvalidOrderMarketType,
        #[msg("InvalidOrderForInitialMarginReq")]
        InvalidOrderForInitialMarginReq,
        #[msg("InvalidOrderNotRiskReducing")]
        InvalidOrderNotRiskReducing,
        #[msg("InvalidOrderSizeTooSmall")]
        InvalidOrderSizeTooSmall,
        #[msg("InvalidOrderNotStepSizeMultiple")]
        InvalidOrderNotStepSizeMultiple,
        #[msg("InvalidOrderBaseQuoteAsset")]
        InvalidOrderBaseQuoteAsset,
        #[msg("InvalidOrderIOC")]
        InvalidOrderIOC,
        #[msg("InvalidOrderPostOnly")]
        InvalidOrderPostOnly,
        #[msg("InvalidOrderIOCPostOnly")]
        InvalidOrderIOCPostOnly,
        #[msg("InvalidOrderTrigger")]
        InvalidOrderTrigger,
        #[msg("InvalidOrderAuction")]
        InvalidOrderAuction,
        #[msg("InvalidOrderOracleOffset")]
        InvalidOrderOracleOffset,
        #[msg("InvalidOrderMinOrderSize")]
        InvalidOrderMinOrderSize,
        #[msg("Failed to Place Post-Only Limit Order")]
        PlacePostOnlyLimitFailure,
        #[msg("User has no order")]
        UserHasNoOrder,
        #[msg("Order Amount Too Small")]
        OrderAmountTooSmall,
        #[msg("Max number of orders taken")]
        MaxNumberOfOrders,
        #[msg("Order does not exist")]
        OrderDoesNotExist,
        #[msg("Order not open")]
        OrderNotOpen,
        #[msg("FillOrderDidNotUpdateState")]
        FillOrderDidNotUpdateState,
        #[msg("Reduce only order increased risk")]
        ReduceOnlyOrderIncreasedRisk,
        #[msg("Unable to load AccountLoader")]
        UnableToLoadAccountLoader,
        #[msg("Trade Size Too Large")]
        TradeSizeTooLarge,
        #[msg("User cant refer themselves")]
        UserCantReferThemselves,
        #[msg("Did not receive expected referrer")]
        DidNotReceiveExpectedReferrer,
        #[msg("Could not deserialize referrer")]
        CouldNotDeserializeReferrer,
        #[msg("Could not deserialize referrer stats")]
        CouldNotDeserializeReferrerStats,
        #[msg("User Order Id Already In Use")]
        UserOrderIdAlreadyInUse,
        #[msg("No positions liquidatable")]
        NoPositionsLiquidatable,
        #[msg("Invalid Margin Ratio")]
        InvalidMarginRatio,
        #[msg("Cant Cancel Post Only Order")]
        CantCancelPostOnlyOrder,
        #[msg("InvalidOracleOffset")]
        InvalidOracleOffset,
        #[msg("CantExpireOrders")]
        CantExpireOrders,
        #[msg("CouldNotLoadMarketData")]
        CouldNotLoadMarketData,
        #[msg("PerpMarketNotFound")]
        PerpMarketNotFound,
        #[msg("InvalidMarketAccount")]
        InvalidMarketAccount,
        #[msg("UnableToLoadMarketAccount")]
        UnableToLoadPerpMarketAccount,
        #[msg("MarketWrongMutability")]
        MarketWrongMutability,
        #[msg("UnableToCastUnixTime")]
        UnableToCastUnixTime,
        #[msg("CouldNotFindSpotPosition")]
        CouldNotFindSpotPosition,
        #[msg("NoSpotPositionAvailable")]
        NoSpotPositionAvailable,
        #[msg("InvalidSpotMarketInitialization")]
        InvalidSpotMarketInitialization,
        #[msg("CouldNotLoadSpotMarketData")]
        CouldNotLoadSpotMarketData,
        #[msg("SpotMarketNotFound")]
        SpotMarketNotFound,
        #[msg("InvalidSpotMarketAccount")]
        InvalidSpotMarketAccount,
        #[msg("UnableToLoadSpotMarketAccount")]
        UnableToLoadSpotMarketAccount,
        #[msg("SpotMarketWrongMutability")]
        SpotMarketWrongMutability,
        #[msg("SpotInterestNotUpToDate")]
        SpotMarketInterestNotUpToDate,
        #[msg("SpotMarketInsufficientDeposits")]
        SpotMarketInsufficientDeposits,
        #[msg("UserMustSettleTheirOwnPositiveUnsettledPNL")]
        UserMustSettleTheirOwnPositiveUnsettledPNL,
        #[msg("CantUpdatePoolBalanceType")]
        CantUpdatePoolBalanceType,
        #[msg("InsufficientCollateralForSettlingPNL")]
        InsufficientCollateralForSettlingPNL,
        #[msg("AMMNotUpdatedInSameSlot")]
        AMMNotUpdatedInSameSlot,
        #[msg("AuctionNotComplete")]
        AuctionNotComplete,
        #[msg("MakerNotFound")]
        MakerNotFound,
        #[msg("MakerNotFound")]
        MakerStatsNotFound,
        #[msg("MakerMustBeWritable")]
        MakerMustBeWritable,
        #[msg("MakerMustBeWritable")]
        MakerStatsMustBeWritable,
        #[msg("MakerOrderNotFound")]
        MakerOrderNotFound,
        #[msg("CouldNotDeserializeMaker")]
        CouldNotDeserializeMaker,
        #[msg("CouldNotDeserializeMaker")]
        CouldNotDeserializeMakerStats,
        #[msg("AuctionPriceDoesNotSatisfyMaker")]
        AuctionPriceDoesNotSatisfyMaker,
        #[msg("MakerCantFulfillOwnOrder")]
        MakerCantFulfillOwnOrder,
        #[msg("MakerOrderMustBePostOnly")]
        MakerOrderMustBePostOnly,
        #[msg("CantMatchTwoPostOnlys")]
        CantMatchTwoPostOnlys,
        #[msg("OrderBreachesOraclePriceLimits")]
        OrderBreachesOraclePriceLimits,
        #[msg("OrderMustBeTriggeredFirst")]
        OrderMustBeTriggeredFirst,
        #[msg("OrderNotTriggerable")]
        OrderNotTriggerable,
        #[msg("OrderDidNotSatisfyTriggerCondition")]
        OrderDidNotSatisfyTriggerCondition,
        #[msg("PositionAlreadyBeingLiquidated")]
        PositionAlreadyBeingLiquidated,
        #[msg("PositionDoesntHaveOpenPositionOrOrders")]
        PositionDoesntHaveOpenPositionOrOrders,
        #[msg("AllOrdersAreAlreadyLiquidations")]
        AllOrdersAreAlreadyLiquidations,
        #[msg("CantCancelLiquidationOrder")]
        CantCancelLiquidationOrder,
        #[msg("UserIsBeingLiquidated")]
        UserIsBeingLiquidated,
        #[msg("LiquidationsOngoing")]
        LiquidationsOngoing,
        #[msg("WrongSpotBalanceType")]
        WrongSpotBalanceType,
        #[msg("UserCantLiquidateThemself")]
        UserCantLiquidateThemself,
        #[msg("InvalidPerpPositionToLiquidate")]
        InvalidPerpPositionToLiquidate,
        #[msg("InvalidBaseAssetAmountForLiquidatePerp")]
        InvalidBaseAssetAmountForLiquidatePerp,
        #[msg("InvalidPositionLastFundingRate")]
        InvalidPositionLastFundingRate,
        #[msg("InvalidPositionDelta")]
        InvalidPositionDelta,
        #[msg("UserBankrupt")]
        UserBankrupt,
        #[msg("UserNotBankrupt")]
        UserNotBankrupt,
        #[msg("UserHasInvalidBorrow")]
        UserHasInvalidBorrow,
        #[msg("DailyWithdrawLimit")]
        DailyWithdrawLimit,
        #[msg("DefaultError")]
        DefaultError,
        #[msg("Insufficient LP tokens")]
        InsufficientLPTokens,
        #[msg("Cant LP with a market position")]
        CantLPWithPerpPosition,
        #[msg("Unable to burn LP tokens")]
        UnableToBurnLPTokens,
        #[msg("Trying to remove liqudity too fast after adding it")]
        TryingToRemoveLiquidityTooFast,
        #[msg("Invalid Spot Market Vault")]
        InvalidSpotMarketVault,
        #[msg("Invalid Spot Market State")]
        InvalidSpotMarketState,
        #[msg("InvalidSerumProgram")]
        InvalidSerumProgram,
        #[msg("InvalidSerumMarket")]
        InvalidSerumMarket,
        #[msg("InvalidSerumBids")]
        InvalidSerumBids,
        #[msg("InvalidSerumAsks")]
        InvalidSerumAsks,
        #[msg("InvalidSerumOpenOrders")]
        InvalidSerumOpenOrders,
        #[msg("FailedSerumCPI")]
        FailedSerumCPI,
        #[msg("FailedToFillOnExternalMarket")]
        FailedToFillOnExternalMarket,
        #[msg("InvalidFulfillmentConfig")]
        InvalidFulfillmentConfig,
        #[msg("InvalidFeeStructure")]
        InvalidFeeStructure,
        #[msg("Insufficient IF shares")]
        InsufficientIFShares,
        #[msg("the Market has paused this action")]
        MarketActionPaused,
        #[msg("the Market status doesnt allow placing orders")]
        MarketPlaceOrderPaused,
        #[msg("the Market status doesnt allow filling orders")]
        MarketFillOrderPaused,
        #[msg("the Market status doesnt allow withdraws")]
        MarketWithdrawPaused,
        #[msg("Action violates the Protected Asset Tier rules")]
        ProtectedAssetTierViolation,
        #[msg("Action violates the Isolated Asset Tier rules")]
        IsolatedAssetTierViolation,
        #[msg("User Cant Be Deleted")]
        UserCantBeDeleted,
        #[msg("Reduce Only Withdraw Increased Risk")]
        ReduceOnlyWithdrawIncreasedRisk,
        #[msg("Max Open Interest")]
        MaxOpenInterest,
        #[msg("Cant Resolve Perp Bankruptcy")]
        CantResolvePerpBankruptcy,
        #[msg("Liquidation Doesnt Satisfy Limit Price")]
        LiquidationDoesntSatisfyLimitPrice,
        #[msg("Margin Trading Disabled")]
        MarginTradingDisabled,
        #[msg("Invalid Market Status to Settle Perp Pnl")]
        InvalidMarketStatusToSettlePnl,
        #[msg("PerpMarketNotInSettlement")]
        PerpMarketNotInSettlement,
        #[msg("PerpMarketNotInReduceOnly")]
        PerpMarketNotInReduceOnly,
        #[msg("PerpMarketSettlementBufferNotReached")]
        PerpMarketSettlementBufferNotReached,
        #[msg("PerpMarketSettlementUserHasOpenOrders")]
        PerpMarketSettlementUserHasOpenOrders,
        #[msg("PerpMarketSettlementUserHasActiveLP")]
        PerpMarketSettlementUserHasActiveLP,
        #[msg("UnableToSettleExpiredUserPosition")]
        UnableToSettleExpiredUserPosition,
        #[msg("UnequalMarketIndexForSpotTransfer")]
        UnequalMarketIndexForSpotTransfer,
        #[msg("InvalidPerpPositionDetected")]
        InvalidPerpPositionDetected,
        #[msg("InvalidSpotPositionDetected")]
        InvalidSpotPositionDetected,
        #[msg("InvalidAmmDetected")]
        InvalidAmmDetected,
        #[msg("InvalidAmmForFillDetected")]
        InvalidAmmForFillDetected,
        #[msg("InvalidAmmLimitPriceOverride")]
        InvalidAmmLimitPriceOverride,
        #[msg("InvalidOrderFillPrice")]
        InvalidOrderFillPrice,
        #[msg("SpotMarketBalanceInvariantViolated")]
        SpotMarketBalanceInvariantViolated,
        #[msg("SpotMarketVaultInvariantViolated")]
        SpotMarketVaultInvariantViolated,
        #[msg("InvalidPDA")]
        InvalidPDA,
        #[msg("InvalidPDASigner")]
        InvalidPDASigner,
        #[msg("RevenueSettingsCannotSettleToIF")]
        RevenueSettingsCannotSettleToIF,
        #[msg("NoRevenueToSettleToIF")]
        NoRevenueToSettleToIF,
        #[msg("NoAmmPerpPnlDeficit")]
        NoAmmPerpPnlDeficit,
        #[msg("SufficientPerpPnlPool")]
        SufficientPerpPnlPool,
        #[msg("InsufficientPerpPnlPool")]
        InsufficientPerpPnlPool,
        #[msg("PerpPnlDeficitBelowThreshold")]
        PerpPnlDeficitBelowThreshold,
        #[msg("MaxRevenueWithdrawPerPeriodReached")]
        MaxRevenueWithdrawPerPeriodReached,
        #[msg("InvalidSpotPositionDetected")]
        MaxIFWithdrawReached,
        #[msg("NoIFWithdrawAvailable")]
        NoIFWithdrawAvailable,
        #[msg("InvalidIFUnstake")]
        InvalidIFUnstake,
        #[msg("InvalidIFUnstakeSize")]
        InvalidIFUnstakeSize,
        #[msg("InvalidIFUnstakeCancel")]
        InvalidIFUnstakeCancel,
        #[msg("InvalidIFForNewStakes")]
        InvalidIFForNewStakes,
        #[msg("InvalidIFRebase")]
        InvalidIFRebase,
        #[msg("InvalidInsuranceUnstakeSize")]
        InvalidInsuranceUnstakeSize,
        #[msg("InvalidOrderLimitPrice")]
        InvalidOrderLimitPrice,
        #[msg("InvalidIFDetected")]
        InvalidIFDetected,
        #[msg("InvalidAmmMaxSpreadDetected")]
        InvalidAmmMaxSpreadDetected,
        #[msg("InvalidConcentrationCoef")]
        InvalidConcentrationCoef,
        #[msg("InvalidSrmVault")]
        InvalidSrmVault,
        #[msg("InvalidVaultOwner")]
        InvalidVaultOwner,
        #[msg("InvalidMarketStatusForFills")]
        InvalidMarketStatusForFills,
        #[msg("IFWithdrawRequestInProgress")]
        IFWithdrawRequestInProgress,
        #[msg("NoIFWithdrawRequestInProgress")]
        NoIFWithdrawRequestInProgress,
        #[msg("IFWithdrawRequestTooSmall")]
        IFWithdrawRequestTooSmall,
        #[msg("IncorrectSpotMarketAccountPassed")]
        IncorrectSpotMarketAccountPassed,
        #[msg("BlockchainClockInconsistency")]
        BlockchainClockInconsistency,
        #[msg("InvalidIFSharesDetected")]
        InvalidIFSharesDetected,
        #[msg("NewLPSizeTooSmall")]
        NewLPSizeTooSmall,
        #[msg("MarketStatusInvalidForNewLP")]
        MarketStatusInvalidForNewLP,
        #[msg("InvalidMarkTwapUpdateDetected")]
        InvalidMarkTwapUpdateDetected,
        #[msg("MarketSettlementAttemptOnActiveMarket")]
        MarketSettlementAttemptOnActiveMarket,
        #[msg("MarketSettlementRequiresSettledLP")]
        MarketSettlementRequiresSettledLP,
        #[msg("MarketSettlementAttemptTooEarly")]
        MarketSettlementAttemptTooEarly,
        #[msg("MarketSettlementTargetPriceInvalid")]
        MarketSettlementTargetPriceInvalid,
        #[msg("UnsupportedSpotMarket")]
        UnsupportedSpotMarket,
        #[msg("SpotOrdersDisabled")]
        SpotOrdersDisabled,
        #[msg("Market Being Initialized")]
        MarketBeingInitialized,
        #[msg("Invalid Sub Account Id")]
        InvalidUserSubAccountId,
        #[msg("Invalid Trigger Order Condition")]
        InvalidTriggerOrderCondition,
        #[msg("Invalid Spot Position")]
        InvalidSpotPosition,
        #[msg("Cant transfer between same user account")]
        CantTransferBetweenSameUserAccount,
        #[msg("Invalid Perp Position")]
        InvalidPerpPosition,
        #[msg("Unable To Get Limit Price")]
        UnableToGetLimitPrice,
        #[msg("Invalid Liquidation")]
        InvalidLiquidation,
        #[msg("Spot Fulfillment Config Disabled")]
        SpotFulfillmentConfigDisabled,
        #[msg("Invalid Maker")]
        InvalidMaker,
        #[msg("Failed Unwrap")]
        FailedUnwrap,
        #[msg("Max Number Of Users")]
        MaxNumberOfUsers,
        #[msg("InvalidOracleForSettlePnl")]
        InvalidOracleForSettlePnl,
        #[msg("MarginOrdersOpen")]
        MarginOrdersOpen,
        #[msg("TierViolationLiquidatingPerpPnl")]
        TierViolationLiquidatingPerpPnl,
        #[msg("CouldNotLoadUserData")]
        CouldNotLoadUserData,
        #[msg("UserWrongMutability")]
        UserWrongMutability,
        #[msg("InvalidUserAccount")]
        InvalidUserAccount,
        #[msg("CouldNotLoadUserData")]
        CouldNotLoadUserStatsData,
        #[msg("UserWrongMutability")]
        UserStatsWrongMutability,
        #[msg("InvalidUserAccount")]
        InvalidUserStatsAccount,
        #[msg("UserNotFound")]
        UserNotFound,
        #[msg("UnableToLoadUserAccount")]
        UnableToLoadUserAccount,
        #[msg("UserStatsNotFound")]
        UserStatsNotFound,
        #[msg("UnableToLoadUserStatsAccount")]
        UnableToLoadUserStatsAccount,
        #[msg("User Not Inactive")]
        UserNotInactive,
        #[msg("RevertFill")]
        RevertFill,
        #[msg("Invalid MarketAccount for Deletion")]
        InvalidMarketAccountforDeletion,
        #[msg("Invalid Spot Fulfillment Params")]
        InvalidSpotFulfillmentParams,
        #[msg("Failed to Get Mint")]
        FailedToGetMint,
        #[msg("FailedPhoenixCPI")]
        FailedPhoenixCPI,
        #[msg("FailedToDeserializePhoenixMarket")]
        FailedToDeserializePhoenixMarket,
        #[msg("InvalidPricePrecision")]
        InvalidPricePrecision,
        #[msg("InvalidPhoenixProgram")]
        InvalidPhoenixProgram,
        #[msg("InvalidPhoenixMarket")]
        InvalidPhoenixMarket,
        #[msg("InvalidSwap")]
        InvalidSwap,
        #[msg("SwapLimitPriceBreached")]
        SwapLimitPriceBreached,
        #[msg("SpotMarketReduceOnly")]
        SpotMarketReduceOnly,
        #[msg("FundingWasNotUpdated")]
        FundingWasNotUpdated,
        #[msg("ImpossibleFill")]
        ImpossibleFill,
        #[msg("CantUpdatePerpBidAskTwap")]
        CantUpdatePerpBidAskTwap,
        #[msg("UserReduceOnly")]
        UserReduceOnly,
        #[msg("InvalidMarginCalculation")]
        InvalidMarginCalculation,
        #[msg("CantPayUserInitFee")]
        CantPayUserInitFee,
        #[msg("CantReclaimRent")]
        CantReclaimRent,
        #[msg("InsuranceFundOperationPaused")]
        InsuranceFundOperationPaused,
        #[msg("NoUnsettledPnl")]
        NoUnsettledPnl,
        #[msg("PnlPoolCantSettleUser")]
        PnlPoolCantSettleUser,
        #[msg("OracleInvalid")]
        OracleNonPositive,
        #[msg("OracleTooVolatile")]
        OracleTooVolatile,
        #[msg("OracleTooUncertain")]
        OracleTooUncertain,
        #[msg("OracleStaleForMargin")]
        OracleStaleForMargin,
        #[msg("OracleInsufficientDataPoints")]
        OracleInsufficientDataPoints,
        #[msg("OracleStaleForAMM")]
        OracleStaleForAMM,
        #[msg("Unable to parse pull oracle message")]
        UnableToParsePullOracleMessage,
        #[msg("Can not borow more than max borrows")]
        MaxBorrows,
        #[msg("Updates must be monotonically increasing")]
        OracleUpdatesNotMonotonic,
        #[msg("Trying to update price feed with the wrong feed id")]
        OraclePriceFeedMessageMismatch,
        #[msg("The message in the update must be a PriceFeedMessage")]
        OracleUnsupportedMessageType,
        #[msg("Could not deserialize the message in the update")]
        OracleDeserializeMessageFailed,
        #[msg("Wrong guardian set owner in update price atomic")]
        OracleWrongGuardianSetOwner,
        #[msg("Oracle post update atomic price feed account must be drift program")]
        OracleWrongWriteAuthority,
        #[msg("Oracle vaa owner must be wormhole program")]
        OracleWrongVaaOwner,
        #[msg("Multi updates must have 2 or fewer accounts passed in remaining accounts")]
        OracleTooManyPriceAccountUpdates,
        #[msg("Don't have the same remaining accounts number and pyth updates left")]
        OracleMismatchedVaaAndPriceUpdates,
        #[msg("Remaining account passed does not match oracle update derived pda")]
        OracleBadRemainingAccountPublicKey,
        #[msg("FailedOpenbookV2CPI")]
        FailedOpenbookV2CPI,
        #[msg("InvalidOpenbookV2Program")]
        InvalidOpenbookV2Program,
        #[msg("InvalidOpenbookV2Market")]
        InvalidOpenbookV2Market,
        #[msg("Non zero transfer fee")]
        NonZeroTransferFee,
        #[msg("Liquidation order failed to fill")]
        LiquidationOrderFailedToFill,
        #[msg("Invalid prediction market order")]
        InvalidPredictionMarketOrder,
        #[msg("Ed25519 Ix must be before place and make SignedMsg order ix")]
        InvalidVerificationIxIndex,
        #[msg("SignedMsg message verificaiton failed")]
        SigVerificationFailed,
        #[msg("Market index mismatched b/w taker and maker SignedMsg order params")]
        MismatchedSignedMsgOrderParamsMarketIndex,
        #[msg("Invalid SignedMsg order param")]
        InvalidSignedMsgOrderParam,
        #[msg("Place and take order success condition failed")]
        PlaceAndTakeOrderSuccessConditionFailed,
        #[msg("Invalid High Leverage Mode Config")]
        InvalidHighLeverageModeConfig,
        #[msg("Invalid RFQ User Account")]
        InvalidRFQUserAccount,
        #[msg("RFQUserAccount should be mutable")]
        RFQUserAccountWrongMutability,
        #[msg("RFQUserAccount has too many active RFQs")]
        RFQUserAccountFull,
        #[msg("RFQ order not filled as expected")]
        RFQOrderNotFilled,
        #[msg("RFQ orders must be jit makers")]
        InvalidRFQOrder,
        #[msg("RFQ matches must be valid")]
        InvalidRFQMatch,
        #[msg("Invalid SignedMsg user account")]
        InvalidSignedMsgUserAccount,
        #[msg("SignedMsg account wrong mutability")]
        SignedMsgUserAccountWrongMutability,
        #[msg("SignedMsgUserAccount has too many active orders")]
        SignedMsgUserOrdersAccountFull,
        #[msg("Order with SignedMsg uuid does not exist")]
        SignedMsgOrderDoesNotExist,
        #[msg("SignedMsg order id cannot be 0s")]
        InvalidSignedMsgOrderId,
        #[msg("Invalid pool id")]
        InvalidPoolId,
        #[msg("Invalid Protected Maker Mode Config")]
        InvalidProtectedMakerModeConfig,
        #[msg("Invalid pyth lazer storage owner")]
        InvalidPythLazerStorageOwner,
        #[msg("Verification of pyth lazer message failed")]
        UnverifiedPythLazerMessage,
        #[msg("Invalid pyth lazer message")]
        InvalidPythLazerMessage,
        #[msg("Pyth lazer message does not correspond to correct fed id")]
        PythLazerMessagePriceFeedMismatch,
        #[msg("InvalidLiquidateSpotWithSwap")]
        InvalidLiquidateSpotWithSwap,
        #[msg("User in SignedMsg message does not match user in ix context")]
        SignedMsgUserContextUserMismatch,
        #[msg("User fuel overflow threshold not met")]
        UserFuelOverflowThresholdNotMet,
        #[msg("FuelOverflow account not found")]
        FuelOverflowAccountNotFound,
        #[msg("Invalid Transfer Perp Position")]
        InvalidTransferPerpPosition,
        #[msg("Invalid SignedMsgUserOrders resize")]
        InvalidSignedMsgUserOrdersResize,
        #[msg("Could not deserialize high leverage mode config")]
        CouldNotDeserializeHighLeverageModeConfig,
        #[msg("Invalid If Rebalance Config")]
        InvalidIfRebalanceConfig,
        #[msg("Invalid If Rebalance Swap")]
        InvalidIfRebalanceSwap,
    }
}
pub mod events {
    #![doc = r" IDL event types"]
    use super::{types::*, *};
    #[derive(Clone, Debug, PartialEq, Default)]
    #[event]
    pub struct NewUserRecord {
        pub ts: i64,
        pub user_authority: Pubkey,
        pub user: Pubkey,
        pub sub_account_id: u16,
        pub name: [u8; 32],
        pub referrer: Pubkey,
    }
    #[derive(Clone, Debug, PartialEq, Default)]
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
    #[derive(Clone, Debug, PartialEq, Default)]
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
    #[derive(Clone, Debug, PartialEq, Default)]
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
    #[derive(Clone, Debug, PartialEq, Default)]
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
    #[derive(Clone, Debug, PartialEq, Default)]
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
    #[derive(Clone, Debug, PartialEq, Default)]
    #[event]
    pub struct SignedMsgOrderRecord {
        pub user: Pubkey,
        pub hash: String,
        pub matching_order_params: OrderParams,
        pub user_order_id: u32,
        pub signed_msg_order_max_slot: u64,
        pub signed_msg_order_uuid: [u8; 8],
        pub ts: i64,
    }
    #[derive(Clone, Debug, PartialEq, Default)]
    #[event]
    pub struct OrderRecord {
        pub ts: i64,
        pub user: Pubkey,
        pub order: Order,
    }
    #[derive(Clone, Debug, PartialEq, Default)]
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
        pub bit_flags: u8,
        pub taker_existing_quote_entry_amount: Option<u64>,
        pub taker_existing_base_asset_amount: Option<u64>,
        pub maker_existing_quote_entry_amount: Option<u64>,
        pub maker_existing_base_asset_amount: Option<u64>,
        pub trigger_price: Option<u64>,
    }
    #[derive(Clone, Debug, PartialEq, Default)]
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
    #[derive(Clone, Debug, PartialEq, Default)]
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
    #[derive(Clone, Debug, PartialEq, Default)]
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
    #[derive(Clone, Debug, PartialEq, Default)]
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
    #[derive(Clone, Debug, PartialEq, Default)]
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
    #[derive(Clone, Debug, PartialEq, Default)]
    #[event]
    pub struct InsuranceFundSwapRecord {
        pub rebalance_config: Pubkey,
        pub in_if_total_shares_before: u128,
        pub out_if_total_shares_before: u128,
        pub in_if_user_shares_before: u128,
        pub out_if_user_shares_before: u128,
        pub in_if_total_shares_after: u128,
        pub out_if_total_shares_after: u128,
        pub in_if_user_shares_after: u128,
        pub out_if_user_shares_after: u128,
        pub ts: i64,
        pub in_amount: u64,
        pub out_amount: u64,
        pub out_oracle_price: u64,
        pub out_oracle_price_twap: i64,
        pub in_vault_amount_before: u64,
        pub out_vault_amount_before: u64,
        pub in_fund_vault_amount_after: u64,
        pub out_fund_vault_amount_after: u64,
        pub in_market_index: u16,
        pub out_market_index: u16,
    }
    #[derive(Clone, Debug, PartialEq, Default)]
    #[event]
    pub struct TransferProtocolIfSharesToRevenuePoolRecord {
        pub ts: i64,
        pub market_index: u16,
        pub amount: u64,
        pub shares: u128,
        pub if_vault_amount_before: u64,
        pub protocol_shares_before: u128,
        pub transfer_amount: u64,
    }
    #[derive(Clone, Debug, PartialEq, Default)]
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
    #[derive(Clone, Debug, PartialEq, Default)]
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
    #[derive(Clone, Debug, PartialEq, Default)]
    #[event]
    pub struct DeleteUserRecord {
        pub ts: i64,
        pub user_authority: Pubkey,
        pub user: Pubkey,
        pub sub_account_id: u16,
        pub keeper: Option<Pubkey>,
    }
    #[derive(Clone, Debug, PartialEq, Default)]
    #[event]
    pub struct FuelSweepRecord {
        pub ts: i64,
        pub authority: Pubkey,
        pub user_stats_fuel_insurance: u32,
        pub user_stats_fuel_deposits: u32,
        pub user_stats_fuel_borrows: u32,
        pub user_stats_fuel_positions: u32,
        pub user_stats_fuel_taker: u32,
        pub user_stats_fuel_maker: u32,
        pub fuel_overflow_fuel_insurance: u128,
        pub fuel_overflow_fuel_deposits: u128,
        pub fuel_overflow_fuel_borrows: u128,
        pub fuel_overflow_fuel_positions: u128,
        pub fuel_overflow_fuel_taker: u128,
        pub fuel_overflow_fuel_maker: u128,
    }
    #[derive(Clone, Debug, PartialEq, Default)]
    #[event]
    pub struct FuelSeasonRecord {
        pub ts: i64,
        pub authority: Pubkey,
        pub fuel_insurance: u128,
        pub fuel_deposits: u128,
        pub fuel_borrows: u128,
        pub fuel_positions: u128,
        pub fuel_taker: u128,
        pub fuel_maker: u128,
        pub fuel_total: u128,
    }
    #[derive(Clone, Debug, PartialEq, Default)]
    #[event]
    pub struct LPSettleRecord {
        pub record_id: u64,
        pub last_ts: i64,
        pub last_slot: u64,
        pub ts: i64,
        pub slot: u64,
        pub perp_market_index: u16,
        pub settle_to_lp_amount: i64,
        pub perp_amm_pnl_delta: i64,
        pub perp_amm_ex_fee_delta: i64,
        pub lp_aum: u128,
        pub lp_price: u128,
    }
    #[derive(Clone, Debug, PartialEq, Default)]
    #[event]
    pub struct LPSwapRecord {
        pub ts: i64,
        pub slot: u64,
        pub authority: Pubkey,
        pub out_amount: u128,
        pub in_amount: u128,
        pub out_fee: i128,
        pub in_fee: i128,
        pub out_spot_market_index: u16,
        pub in_spot_market_index: u16,
        pub out_constituent_index: u16,
        pub in_constituent_index: u16,
        pub out_oracle_price: i64,
        pub in_oracle_price: i64,
        pub last_aum: u128,
        pub last_aum_slot: u64,
        pub in_market_current_weight: i64,
        pub out_market_current_weight: i64,
        pub in_market_target_weight: i64,
        pub out_market_target_weight: i64,
        pub in_swap_id: u64,
        pub out_swap_id: u64,
    }
    #[derive(Clone, Debug, PartialEq, Default)]
    #[event]
    pub struct LPMintRedeemRecord {
        pub ts: i64,
        pub slot: u64,
        pub authority: Pubkey,
        pub description: u8,
        pub amount: u128,
        pub fee: i128,
        pub spot_market_index: u16,
        pub constituent_index: u16,
        pub oracle_price: i64,
        pub mint: Pubkey,
        pub lp_amount: u64,
        pub lp_fee: i64,
        pub lp_price: u128,
        pub mint_redeem_id: u64,
        pub last_aum: u128,
        pub last_aum_slot: u64,
        pub in_market_current_weight: i64,
        pub in_market_target_weight: i64,
    }
}
