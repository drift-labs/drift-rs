#![allow(unused_imports)]
use anchor_lang::prelude::*;
use solana_program::pubkey::Pubkey;
pub mod instructions {
    use super::{types::*, *};
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializeUser {
        pub sub_account_id: u16,
        pub name: [u8; 32],
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializeUserStats {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializeReferrerName {
        pub name: [u8; 32],
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct Deposit {
        pub market_index: u16,
        pub amount: u64,
        pub reduce_only: bool,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct Withdraw {
        pub market_index: u16,
        pub amount: u64,
        pub reduce_only: bool,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct TransferDeposit {
        pub market_index: u16,
        pub amount: u64,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PlacePerpOrder {
        pub params: OrderParams,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct CancelOrder {
        pub order_id: Option<u32>,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct CancelOrderByUserId {
        pub user_order_id: u8,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct CancelOrders {
        pub market_type: Option<MarketType>,
        pub market_index: Option<u16>,
        pub direction: Option<PositionDirection>,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct CancelOrdersByIds {
        pub order_ids: Vec<u32>,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct ModifyOrder {
        pub order_id: Option<u32>,
        pub modify_order_params: ModifyOrderParams,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct ModifyOrderByUserId {
        pub user_order_id: u8,
        pub modify_order_params: ModifyOrderParams,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PlaceAndTakePerpOrder {
        pub params: OrderParams,
        pub maker_order_id: Option<u32>,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PlaceAndMakePerpOrder {
        pub params: OrderParams,
        pub taker_order_id: u32,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PlaceSpotOrder {
        pub params: OrderParams,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PlaceAndTakeSpotOrder {
        pub params: OrderParams,
        pub fulfillment_type: Option<SpotFulfillmentType>,
        pub maker_order_id: Option<u32>,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PlaceAndMakeSpotOrder {
        pub params: OrderParams,
        pub taker_order_id: u32,
        pub fulfillment_type: Option<SpotFulfillmentType>,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PlaceOrders {
        pub params: Vec<OrderParams>,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct BeginSwap {
        pub in_market_index: u16,
        pub out_market_index: u16,
        pub amount_in: u64,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct EndSwap {
        pub in_market_index: u16,
        pub out_market_index: u16,
        pub limit_price: Option<u64>,
        pub reduce_only: Option<SwapReduceOnly>,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct AddPerpLpShares {
        pub n_shares: u64,
        pub market_index: u16,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct RemovePerpLpShares {
        pub shares_to_burn: u64,
        pub market_index: u16,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct RemovePerpLpSharesInExpiringMarket {
        pub shares_to_burn: u64,
        pub market_index: u16,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateUserName {
        pub sub_account_id: u16,
        pub name: [u8; 32],
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateUserCustomMarginRatio {
        pub sub_account_id: u16,
        pub margin_ratio: u32,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateUserMarginTradingEnabled {
        pub sub_account_id: u16,
        pub margin_trading_enabled: bool,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateUserDelegate {
        pub sub_account_id: u16,
        pub delegate: Pubkey,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateUserReduceOnly {
        pub sub_account_id: u16,
        pub reduce_only: bool,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateUserAdvancedLp {
        pub sub_account_id: u16,
        pub advanced_lp: bool,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct DeleteUser {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct ReclaimRent {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct FillPerpOrder {
        pub order_id: Option<u32>,
        pub maker_order_id: Option<u32>,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct RevertFill {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct FillSpotOrder {
        pub order_id: Option<u32>,
        pub fulfillment_type: Option<SpotFulfillmentType>,
        pub maker_order_id: Option<u32>,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct TriggerOrder {
        pub order_id: u32,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct ForceCancelOrders {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateUserIdle {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateUserOpenOrdersCount {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct AdminDisableUpdatePerpBidAskTwap {
        pub disable: bool,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct SettlePnl {
        pub market_index: u16,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct SettleMultiplePnls {
        pub market_indexes: Vec<u16>,
        pub mode: SettlePnlMode,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct SettleFundingPayment {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct SettleLp {
        pub market_index: u16,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct SettleExpiredMarket {
        pub market_index: u16,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct LiquidatePerp {
        pub market_index: u16,
        pub liquidator_max_base_asset_amount: u64,
        pub limit_price: Option<u64>,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct LiquidatePerpWithFill {
        pub market_index: u16,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct LiquidateSpot {
        pub asset_market_index: u16,
        pub liability_market_index: u16,
        pub liquidator_max_liability_transfer: u128,
        pub limit_price: Option<u64>,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct LiquidateBorrowForPerpPnl {
        pub perp_market_index: u16,
        pub spot_market_index: u16,
        pub liquidator_max_liability_transfer: u128,
        pub limit_price: Option<u64>,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct LiquidatePerpPnlForDeposit {
        pub perp_market_index: u16,
        pub spot_market_index: u16,
        pub liquidator_max_pnl_transfer: u128,
        pub limit_price: Option<u64>,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct SetUserStatusToBeingLiquidated {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct ResolvePerpPnlDeficit {
        pub spot_market_index: u16,
        pub perp_market_index: u16,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct ResolvePerpBankruptcy {
        pub quote_spot_market_index: u16,
        pub market_index: u16,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct ResolveSpotBankruptcy {
        pub market_index: u16,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct SettleRevenueToInsuranceFund {
        pub spot_market_index: u16,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateFundingRate {
        pub market_index: u16,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePrelaunchOracle {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpBidAskTwap {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketCumulativeInterest {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateAmms {
        pub market_indexes: [u16; 5],
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketExpiry {
        pub expiry_ts: i64,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateUserQuoteAssetInsuranceStake {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateUserGovTokenInsuranceStake {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializeInsuranceFundStake {
        pub market_index: u16,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct AddInsuranceFundStake {
        pub market_index: u16,
        pub amount: u64,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct RequestRemoveInsuranceFundStake {
        pub market_index: u16,
        pub amount: u64,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct CancelRequestRemoveInsuranceFundStake {
        pub market_index: u16,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct RemoveInsuranceFundStake {
        pub market_index: u16,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct TransferProtocolIfShares {
        pub market_index: u16,
        pub shares: u128,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePythPullOracle {
        pub feed_id: [u8; 32],
        pub params: Vec<u8>,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PostPythPullOracleUpdateAtomic {
        pub feed_id: [u8; 32],
        pub params: Vec<u8>,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PostMultiPythPullOracleUpdatesAtomic {
        pub params: Vec<u8>,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct Initialize {}
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
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct DeleteInitializedSpotMarket {
        pub market_index: u16,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializeSerumFulfillmentConfig {
        pub market_index: u16,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSerumFulfillmentConfigStatus {
        pub status: SpotFulfillmentConfigStatus,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializeOpenbookV2FulfillmentConfig {
        pub market_index: u16,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct OpenbookV2FulfillmentConfigStatus {
        pub status: SpotFulfillmentConfigStatus,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializePhoenixFulfillmentConfig {
        pub market_index: u16,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PhoenixFulfillmentConfigStatus {
        pub status: SpotFulfillmentConfigStatus,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSerumVault {}
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
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializePredictionMarket {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct DeleteInitializedPerpMarket {
        pub market_index: u16,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct MoveAmmPrice {
        pub base_asset_reserve: u128,
        pub quote_asset_reserve: u128,
        pub sqrt_k: u128,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct RecenterPerpMarketAmm {
        pub peg_multiplier: u128,
        pub sqrt_k: u128,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketAmmSummaryStats {
        pub params: UpdatePerpMarketSummaryStatsParams,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketExpiry {
        pub expiry_ts: i64,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct SettleExpiredMarketPoolsToRevenuePool {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct DepositIntoPerpMarketFeePool {
        pub amount: u64,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct DepositIntoSpotMarketVault {
        pub amount: u64,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct DepositIntoSpotMarketRevenuePool {
        pub amount: u64,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct RepegAmmCurve {
        pub new_peg_candidate: u128,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketAmmOracleTwap {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct ResetPerpMarketAmmOracleTwap {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateK {
        pub sqrt_k: u128,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketMarginRatio {
        pub margin_ratio_initial: u32,
        pub margin_ratio_maintenance: u32,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketFundingPeriod {
        pub funding_period: i64,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketMaxImbalances {
        pub unrealized_max_imbalance: u64,
        pub max_revenue_withdraw_per_period: u64,
        pub quote_max_insurance: u64,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketLiquidationFee {
        pub liquidator_fee: u32,
        pub if_liquidation_fee: u32,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateInsuranceFundUnstakingPeriod {
        pub insurance_fund_unstaking_period: i64,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketLiquidationFee {
        pub liquidator_fee: u32,
        pub if_liquidation_fee: u32,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateWithdrawGuardThreshold {
        pub withdraw_guard_threshold: u64,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketIfFactor {
        pub spot_market_index: u16,
        pub user_if_factor: u32,
        pub total_if_factor: u32,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketRevenueSettlePeriod {
        pub revenue_settle_period: i64,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketStatus {
        pub status: MarketStatus,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketPausedOperations {
        pub paused_operations: u8,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketAssetTier {
        pub asset_tier: AssetTier,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketMarginWeights {
        pub initial_asset_weight: u32,
        pub maintenance_asset_weight: u32,
        pub initial_liability_weight: u32,
        pub maintenance_liability_weight: u32,
        pub imf_factor: u32,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketBorrowRate {
        pub optimal_utilization: u32,
        pub optimal_borrow_rate: u32,
        pub max_borrow_rate: u32,
        pub min_borrow_rate: Option<u8>,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketMaxTokenDeposits {
        pub max_token_deposits: u64,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketMaxTokenBorrows {
        pub max_token_borrows_fraction: u16,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketScaleInitialAssetWeightStart {
        pub scale_initial_asset_weight_start: u64,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketOracle {
        pub oracle: Pubkey,
        pub oracle_source: OracleSource,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketStepSizeAndTickSize {
        pub step_size: u64,
        pub tick_size: u64,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketMinOrderSize {
        pub order_size: u64,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketOrdersEnabled {
        pub orders_enabled: bool,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketIfPausedOperations {
        pub paused_operations: u8,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketName {
        pub name: [u8; 32],
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketStatus {
        pub status: MarketStatus,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketPausedOperations {
        pub paused_operations: u8,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketContractTier {
        pub contract_tier: ContractTier,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketImfFactor {
        pub imf_factor: u32,
        pub unrealized_pnl_imf_factor: u32,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketUnrealizedAssetWeight {
        pub unrealized_initial_asset_weight: u32,
        pub unrealized_maintenance_asset_weight: u32,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketConcentrationCoef {
        pub concentration_scale: u128,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketCurveUpdateIntensity {
        pub curve_update_intensity: u8,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketTargetBaseAssetAmountPerLp {
        pub target_base_asset_amount_per_lp: i32,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketPerLpBase {
        pub per_lp_base: i8,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateLpCooldownTime {
        pub lp_cooldown_time: u64,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpFeeStructure {
        pub fee_structure: FeeStructure,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotFeeStructure {
        pub fee_structure: FeeStructure,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateInitialPctToLiquidate {
        pub initial_pct_to_liquidate: u16,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateLiquidationDuration {
        pub liquidation_duration: u8,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateLiquidationMarginBufferRatio {
        pub liquidation_margin_buffer_ratio: u32,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateOracleGuardRails {
        pub oracle_guard_rails: OracleGuardRails,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateStateSettlementDuration {
        pub settlement_duration: u16,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateStateMaxNumberOfSubAccounts {
        pub max_number_of_sub_accounts: u16,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateStateMaxInitializeUserFee {
        pub max_initialize_user_fee: u16,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketOracle {
        pub oracle: Pubkey,
        pub oracle_source: OracleSource,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketBaseSpread {
        pub base_spread: u32,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateAmmJitIntensity {
        pub amm_jit_intensity: u8,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketMaxSpread {
        pub max_spread: u32,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketStepSizeAndTickSize {
        pub step_size: u64,
        pub tick_size: u64,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketName {
        pub name: [u8; 32],
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketMinOrderSize {
        pub order_size: u64,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketMaxSlippageRatio {
        pub max_slippage_ratio: u16,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketMaxFillReserveFraction {
        pub max_fill_reserve_fraction: u16,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketMaxOpenInterest {
        pub max_open_interest: u128,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketNumberOfUsers {
        pub number_of_users: Option<u32>,
        pub number_of_users_with_base: Option<u32>,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketFeeAdjustment {
        pub fee_adjustment: i16,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketFeeAdjustment {
        pub fee_adjustment: i16,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketFuel {
        pub fuel_boost_taker: Option<u8>,
        pub fuel_boost_maker: Option<u8>,
        pub fuel_boost_position: Option<u8>,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotMarketFuel {
        pub fuel_boost_deposits: Option<u8>,
        pub fuel_boost_borrows: Option<u8>,
        pub fuel_boost_taker: Option<u8>,
        pub fuel_boost_maker: Option<u8>,
        pub fuel_boost_insurance: Option<u8>,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitUserFuel {
        pub fuel_boost_deposits: Option<u32>,
        pub fuel_boost_borrows: Option<u32>,
        pub fuel_boost_taker: Option<u32>,
        pub fuel_boost_maker: Option<u32>,
        pub fuel_boost_insurance: Option<u32>,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateAdmin {
        pub admin: Pubkey,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateWhitelistMint {
        pub whitelist_mint: Pubkey,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateDiscountMint {
        pub discount_mint: Pubkey,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateExchangeStatus {
        pub exchange_status: u8,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpAuctionDuration {
        pub min_perp_auction_duration: u8,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateSpotAuctionDuration {
        pub default_spot_auction_duration: u8,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializeProtocolIfSharesTransferConfig {}
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdateProtocolIfSharesTransferConfig {
        pub whitelisted_signers: Option<[Pubkey; 4]>,
        pub max_transfer_per_epoch: Option<u128>,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializePrelaunchOracle {
        pub params: PrelaunchOracleParams,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePrelaunchOracleParams {
        pub params: PrelaunchOracleParams,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct DeletePrelaunchOracle {
        pub perp_market_index: u16,
    }
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InitializePythPullOracle {
        pub feed_id: [u8; 32],
    }
}
pub mod types {
    use super::*;
    #[doc = r" wrapper around fixed array types used for padding with `Default` implementation"]
    #[repr(transparent)]
    #[derive(AnchorDeserialize, AnchorSerialize, Clone)]
    pub struct Padding<const N: usize>([u8; N]);
    impl<const N: usize> Default for Padding<N> {
        fn default() -> Self {
            Self([0u8; N])
        }
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UpdatePerpMarketSummaryStatsParams {
        pub quote_asset_amount_with_unsettled_lp: Option<i64>,
        pub net_unsettled_funding_pnl: Option<i64>,
        pub update_amm_summary_stats: Option<bool>,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
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
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
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
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct LiquidateBorrowForPerpPnlRecord {
        pub perp_market_index: u16,
        pub market_oracle_price: i64,
        pub pnl_transfer: u128,
        pub liability_market_index: u16,
        pub liability_price: i64,
        pub liability_transfer: u128,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct LiquidatePerpPnlForDepositRecord {
        pub perp_market_index: u16,
        pub market_oracle_price: i64,
        pub pnl_transfer: u128,
        pub asset_market_index: u16,
        pub asset_price: i64,
        pub asset_transfer: u128,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PerpBankruptcyRecord {
        pub market_index: u16,
        pub pnl: i128,
        pub if_payment: u128,
        pub clawback_user: Option<Pubkey>,
        pub clawback_user_payment: Option<u128>,
        pub cumulative_funding_rate_delta: i128,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct SpotBankruptcyRecord {
        pub market_index: u16,
        pub borrow_amount: u128,
        pub if_payment: u128,
        pub cumulative_deposit_interest_delta: u128,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct MarketIdentifier {
        pub market_type: MarketType,
        pub market_index: u16,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct HistoricalOracleData {
        pub last_oracle_price: i64,
        pub last_oracle_conf: u64,
        pub last_oracle_delay: i64,
        pub last_oracle_price_twap: i64,
        pub last_oracle_price_twap5min: i64,
        pub last_oracle_price_twap_ts: i64,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct HistoricalIndexData {
        pub last_index_bid_price: u64,
        pub last_index_ask_price: u64,
        pub last_index_price_twap: u64,
        pub last_index_price_twap5min: u64,
        pub last_index_price_twap_ts: i64,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PrelaunchOracleParams {
        pub perp_market_index: u16,
        pub price: Option<i64>,
        pub max_price: Option<i64>,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
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
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
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
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct InsuranceClaim {
        pub revenue_withdraw_since_last_settle: i64,
        pub max_revenue_withdraw_per_period: u64,
        pub quote_max_insurance: u64,
        pub quote_settled_insurance: u64,
        pub last_revenue_withdraw_ts: i64,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PoolBalance {
        pub scaled_balance: u128,
        pub market_index: u16,
        pub padding: [u8; 6],
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
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
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
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
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct OracleGuardRails {
        pub price_divergence: PriceDivergenceGuardRails,
        pub validity: ValidityGuardRails,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PriceDivergenceGuardRails {
        pub mark_oracle_percent_divergence: u64,
        pub oracle_twap5min_percent_divergence: u64,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct ValidityGuardRails {
        pub slots_before_stale_for_amm: i64,
        pub slots_before_stale_for_margin: i64,
        pub confidence_interval_max_size: u64,
        pub too_volatile_ratio: i64,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct FeeStructure {
        pub fee_tiers: [FeeTier; 10],
        pub filler_reward_structure: OrderFillerRewardStructure,
        pub referrer_reward_epoch_upper_bound: u64,
        pub flat_filler_fee: u64,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
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
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct OrderFillerRewardStructure {
        pub reward_numerator: u32,
        pub reward_denominator: u32,
        pub time_based_reward_lower_bound: u128,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct UserFees {
        pub total_fee_paid: u64,
        pub total_fee_rebate: u64,
        pub total_token_discount: u64,
        pub total_referee_discount: u64,
        pub total_referrer_reward: u64,
        pub current_epoch_referrer_reward: u64,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
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
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
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
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
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
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub enum SwapDirection {
        #[default]
        Add,
        Remove,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub enum ModifyOrderId {
        #[default]
        UserOrderId,
        OrderId,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub enum PositionDirection {
        #[default]
        Long,
        Short,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub enum SpotFulfillmentType {
        #[default]
        SerumV3,
        Match,
        PhoenixV1,
        OpenbookV2,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub enum SwapReduceOnly {
        #[default]
        In,
        Out,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub enum TwapPeriod {
        #[default]
        FundingPeriod,
        FiveMin,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub enum LiquidationMultiplierType {
        #[default]
        Discount,
        Premium,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub enum MarginRequirementType {
        #[default]
        Initial,
        Fill,
        Maintenance,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
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
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
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
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub enum PositionUpdateType {
        #[default]
        Open,
        Increase,
        Reduce,
        Close,
        Flip,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub enum DepositExplanation {
        #[default]
        None,
        Transfer,
        Borrow,
        RepayBorrow,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub enum DepositDirection {
        #[default]
        Deposit,
        Withdraw,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub enum OrderAction {
        #[default]
        Place,
        Cancel,
        Fill,
        Trigger,
        Expire,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
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
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub enum LPAction {
        #[default]
        AddLiquidity,
        RemoveLiquidity,
        SettleLiquidity,
        RemoveLiquidityDerisk,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
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
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub enum SettlePnlExplanation {
        #[default]
        None,
        ExpiredPosition,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
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
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub enum FillMode {
        #[default]
        Fill,
        PlaceAndMake,
        PlaceAndTake,
        Liquidation,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub enum PerpFulfillmentMethod {
        #[default]
        AMM,
        Match,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub enum SpotFulfillmentMethod {
        #[default]
        ExternalMarket,
        Match,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub enum MarginCalculationMode {
        #[default]
        Standard,
        Liquidation,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
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
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub enum PostOnlyParam {
        #[default]
        None,
        MustPostOnly,
        TryPostOnly,
        Slide,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub enum ModifyOrderPolicy {
        #[default]
        TryModify,
        MustModify,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
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
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub enum SpotOperation {
        #[default]
        UpdateCumulativeInterest,
        Fill,
        Deposit,
        Withdraw,
        Liquidation,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub enum InsuranceFundOperation {
        #[default]
        Init,
        Add,
        RequestRemove,
        Remove,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
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
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub enum ContractType {
        #[default]
        Perpetual,
        Future,
        Prediction,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
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
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub enum AMMLiquiditySplit {
        #[default]
        ProtocolOwned,
        LPOwned,
        Shared,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub enum SettlePnlMode {
        #[default]
        MustSettle,
        TrySettle,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub enum SpotBalanceType {
        #[default]
        Deposit,
        Borrow,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub enum SpotFulfillmentConfigStatus {
        #[default]
        Enabled,
        Disabled,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub enum AssetTier {
        #[default]
        Collateral,
        Protected,
        Cross,
        Isolated,
        Unlisted,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
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
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub enum UserStatus {
        #[default]
        BeingLiquidated,
        Bankrupt,
        ReduceOnly,
        AdvancedLp,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub enum AssetType {
        #[default]
        Base,
        Quote,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub enum OrderStatus {
        #[default]
        Init,
        Open,
        Filled,
        Canceled,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub enum OrderType {
        #[default]
        Market,
        Limit,
        TriggerMarket,
        TriggerLimit,
        Oracle,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub enum OrderTriggerCondition {
        #[default]
        Above,
        Below,
        TriggeredAbove,
        TriggeredBelow,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub enum MarketType {
        #[default]
        Spot,
        Perp,
    }
}
pub mod accounts {
    use super::{types::*, *};
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
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
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
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
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
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
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
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
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct ProtocolIfSharesTransferConfig {
        pub whitelisted_signers: [Pubkey; 4],
        pub max_transfer_per_epoch: u128,
        pub current_epoch_transfer: u128,
        pub next_epoch_ts: i64,
        pub padding: Padding<8>,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct PrelaunchOracle {
        pub price: i64,
        pub max_price: i64,
        pub confidence: u64,
        pub last_update_slot: u64,
        pub amm_last_update_slot: u64,
        pub perp_market_index: u16,
        pub padding: Padding<70>,
    }
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
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
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
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
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
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
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
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
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
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
    #[repr(C)]
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
    pub struct ReferrerName {
        pub authority: Pubkey,
        pub user: Pubkey,
        pub user_stats: Pubkey,
        pub name: [u8; 32],
    }
}
pub mod errors {
    use super::{types::*, *};
    #[error_code]
    pub enum ProgramError {
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
    #[event]
    pub struct NewUserRecord {
        pub ts: i64,
        pub user_authority: Pubkey,
        pub user: Pubkey,
        pub sub_account_id: u16,
        pub name: [u8; 32],
        pub referrer: Pubkey,
    }
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
    #[event]
    pub struct OrderRecord {
        pub ts: i64,
        pub user: Pubkey,
        pub order: Order,
    }
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
