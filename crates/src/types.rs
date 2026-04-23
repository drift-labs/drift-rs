use std::{
    cell::{BorrowError, BorrowMutError},
    cmp::Ordering,
    fmt::Display,
    str::FromStr,
};

pub use crate::solana_sdk::{
    commitment_config::CommitmentConfig, message::VersionedMessage,
    transaction::versioned::VersionedTransaction,
};
use crate::solana_sdk::{
    instruction::{error::InstructionError, AccountMeta},
    pubkey::Pubkey,
    transaction::TransactionError,
};
use dashmap::DashMap;
pub use solana_rpc_client_api::config::RpcSendTransactionConfig;
use thiserror::Error;
use tokio::sync::oneshot;
use tokio_tungstenite::tungstenite;

// Public type surface: drift native types are the source of truth. The IDL
// still supplies instruction-context accounts, events, and a handful of
// deprecated/removed types (see bottom of this block).
pub mod accounts {
    //! Drift on-chain account types + IDL-generated instruction-context accounts
    //! (`PlacePerpOrder`, `CancelOrder`, etc. used by TransactionBuilder until
    //! Phase 4 swaps them for anchor 1.0's derived ix builders).
    //!
    //! Glob first so drift native state types shadow any IDL equivalents below.
    pub use crate::drift_idl::accounts::*;
    pub use drift::state::amm_cache::AmmCache;
    pub use drift::state::fulfillment_params::openbook_v2::OpenbookV2FulfillmentConfig;
    pub use drift::state::fulfillment_params::phoenix::PhoenixV1FulfillmentConfig;
    pub use drift::state::fulfillment_params::serum::SerumV3FulfillmentConfig;
    pub use drift::state::if_rebalance_config::IfRebalanceConfig;
    pub use drift::state::insurance_fund_stake::{
        InsuranceFundStake, ProtocolIfSharesTransferConfig,
    };
    pub use drift::state::lp_pool::{
        AmmConstituentMapping, Constituent, ConstituentCorrelations, ConstituentTargetBase, LPPool,
    };
    pub use drift::state::oracle::PrelaunchOracle;
    pub use drift::state::perp_market::PerpMarket;
    pub use drift::state::protected_maker_mode_config::ProtectedMakerModeConfig;
    pub use drift::state::pyth_lazer_oracle::PythLazerOracle;
    pub use drift::state::revenue_share::{RevenueShare, RevenueShareEscrow};
    pub use drift::state::signed_msg_user::{SignedMsgUserOrders, SignedMsgWsDelegates};
    pub use drift::state::spot_market::SpotMarket;
    pub use drift::state::user::{FuelOverflow, ReferrerName, User, UserStats};
    // Note: `State` stays on IDL (supplied by the glob above). drift's State is
    // `#[account]` (Borsh) but drift-rs's `AccountMap::account_data::<T>`
    // requires `T: Pod`. drift-idl-gen emits an `unsafe impl Pod for State`
    // that the existing call path relies on. Phase 3 (or an upstream PR making
    // drift::State zero-copy) revisits this.
}
pub mod events {
    // Keep IDL events in public API: drift's `#[event]` types lack serde.
    pub use crate::drift_idl::events::*;
}
pub mod errors {
    pub use drift::error::ErrorCode;
}
pub use crate::drift_idl::instructions::{self};
pub use drift::controller::position::PositionDirection;
pub use drift::error::ErrorCode;
pub use drift::math::margin::MarginRequirementType;
pub use drift::math::oracle::OracleValidity;
pub use drift::state::events::OrderActionExplanation;
pub use drift::state::fill_mode::FillMode;
pub use drift::state::fulfillment::PerpFulfillmentMethod;
pub use drift::state::margin_calculation::{
    MarginCalculationMode, MarginContext, MarketIdentifier,
};
pub use drift::state::market_status::MarketStatus;
pub use drift::state::oracle::{
    HistoricalIndexData, HistoricalOracleData, MMOraclePriceData, OraclePriceData, OracleSource,
    PrelaunchOracleParams, StrictOraclePrice,
};
pub use drift::state::order_params::{
    OrderParamsBitFlag, PlaceAndTakeOrderSuccessCondition, PostOnlyParam,
    SignedMsgTriggerOrderParams,
};
pub use drift::state::paused_operations::{InsuranceFundOperation, PerpOperation, SpotOperation};
pub use drift::state::perp_market::{ContractTier, ContractType, InsuranceClaim, PoolBalance, AMM};
pub use drift::state::revenue_share::{BuilderInfo, RevenueShareOrder};
pub use drift::state::settle_pnl_mode::SettlePnlMode;
pub use drift::state::spot_market::{AssetTier, SpotBalanceType, TokenProgramFlag};
pub use drift::state::state::{
    ExchangeStatus, FeeStructure, FeeTier, OracleGuardRails, PriceDivergenceGuardRails,
    ValidityGuardRails,
};
pub use drift::state::user::{
    AssetType, MarketType, Order, OrderStatus, OrderTriggerCondition, OrderType, PerpPosition,
    SpotPosition, UserStatus,
};
// IDL-only fallbacks kept in the public API until Phase 4 rewires instruction
// building onto anchor 1.0's derived builders.
// - OrderParams, ModifyOrderParams, SignedMsgOrderParams* are Borsh-serialized
//   into instruction data; drift native versions would require conversion at
//   every tx-building site.
// - MarginMode, HighLeverageModeConfig, ModifyOrderPolicy, SwapReduceOnly are
//   deprecated upstream (no drift equivalent).
// - SpotFulfillmentMethod is IDL-only; drift has PerpFulfillmentMethod only.
// - Padding, Signature are plumbing helpers.
pub use crate::drift_idl::types::{
    MarginMode, ModifyOrderParams, ModifyOrderPolicy, OrderParams, Padding, Signature,
    SignedMsgOrderParamsDelegateMessage, SignedMsgOrderParamsMessage, SpotFulfillmentMethod,
    SpotFulfillmentType, SwapReduceOnly,
};
use crate::{
    accounts::UserStats,
    constants::{ids, LUTS_DEVNET, LUTS_MAINNET, TOKEN_2022_PROGRAM_ID, TOKEN_PROGRAM_ID},
    grpc::grpc_subscriber::GrpcError,
    Wallet,
};

pub mod solana_sdk {
    //! convenience type re-exports for solana v3 crates in v2 format
    pub use solana_account as account;
    pub mod clock {
        pub type Slot = u64;
        pub type Epoch = u64;
    }
    pub use solana_commitment_config as commitment_config;
    pub use solana_compute_budget_interface as compute_budget;
    pub use solana_instruction as instruction;
    pub use solana_keypair as keypair;
    pub use solana_message as message;
    pub use solana_pubkey as pubkey;
    pub use solana_signature as signature;
    pub use solana_signer as signer;
    pub use solana_transaction as transaction;
}

use self::accounts::SpotMarket;

/// Map from K => V
pub type MapOf<K, V> = DashMap<K, V, ahash::RandomState>;

/// Handle for unsubscribing from network updates
pub type UnsubHandle = oneshot::Sender<()>;

pub type SdkResult<T> = Result<T, SdkError>;

pub fn is_one_of_variant<T: PartialEq>(value: &T, variants: &[T]) -> bool {
    variants.iter().any(|variant| value == variant)
}

/// Conversions between drift native enums and their IDL-generated equivalents.
///
/// These exist while Phase 4 (instruction-building rewrite) is pending:
/// TransactionBuilder still uses IDL-generated instruction structs that embed
/// `drift_idl::types::{MarketType, PositionDirection, OracleSource, ...}`, so
/// drift-native values need a shim on their way into those structs.
pub mod idl_conv {
    use crate::drift_idl::types as idl;
    pub fn market_type_to_idl(v: super::MarketType) -> idl::MarketType {
        match v {
            super::MarketType::Perp => idl::MarketType::Perp,
            super::MarketType::Spot => idl::MarketType::Spot,
        }
    }
    pub fn market_type_from_idl(v: idl::MarketType) -> super::MarketType {
        match v {
            idl::MarketType::Perp => super::MarketType::Perp,
            idl::MarketType::Spot => super::MarketType::Spot,
        }
    }
    pub fn position_direction_to_idl(v: super::PositionDirection) -> idl::PositionDirection {
        match v {
            super::PositionDirection::Long => idl::PositionDirection::Long,
            super::PositionDirection::Short => idl::PositionDirection::Short,
        }
    }
    pub fn position_direction_from_idl(v: idl::PositionDirection) -> super::PositionDirection {
        match v {
            idl::PositionDirection::Long => super::PositionDirection::Long,
            idl::PositionDirection::Short => super::PositionDirection::Short,
        }
    }
    pub fn oracle_source_to_idl(v: super::OracleSource) -> idl::OracleSource {
        // Match on discriminant ordering (drift's IDL is the source of truth).
        match v {
            super::OracleSource::Pyth => idl::OracleSource::Pyth,
            super::OracleSource::DeprecatedSwitchboard => idl::OracleSource::Switchboard,
            super::OracleSource::QuoteAsset => idl::OracleSource::QuoteAsset,
            super::OracleSource::Pyth1K => idl::OracleSource::Pyth1K,
            super::OracleSource::Pyth1M => idl::OracleSource::Pyth1M,
            super::OracleSource::PythStableCoin => idl::OracleSource::PythStableCoin,
            super::OracleSource::Prelaunch => idl::OracleSource::Prelaunch,
            super::OracleSource::PythPull => idl::OracleSource::PythPull,
            super::OracleSource::Pyth1KPull => idl::OracleSource::Pyth1KPull,
            super::OracleSource::Pyth1MPull => idl::OracleSource::Pyth1MPull,
            super::OracleSource::PythStableCoinPull => idl::OracleSource::PythStableCoinPull,
            super::OracleSource::DeprecatedSwitchboardOnDemand => {
                idl::OracleSource::SwitchboardOnDemand
            }
            super::OracleSource::PythLazer => idl::OracleSource::PythLazer,
            super::OracleSource::PythLazer1K => idl::OracleSource::PythLazer1K,
            super::OracleSource::PythLazer1M => idl::OracleSource::PythLazer1M,
            super::OracleSource::PythLazerStableCoin => idl::OracleSource::PythLazerStableCoin,
        }
    }
    pub fn oracle_source_from_idl(v: idl::OracleSource) -> super::OracleSource {
        match v {
            idl::OracleSource::Pyth => super::OracleSource::Pyth,
            idl::OracleSource::Switchboard => super::OracleSource::DeprecatedSwitchboard,
            idl::OracleSource::QuoteAsset => super::OracleSource::QuoteAsset,
            idl::OracleSource::Pyth1K => super::OracleSource::Pyth1K,
            idl::OracleSource::Pyth1M => super::OracleSource::Pyth1M,
            idl::OracleSource::PythStableCoin => super::OracleSource::PythStableCoin,
            idl::OracleSource::Prelaunch => super::OracleSource::Prelaunch,
            idl::OracleSource::PythPull => super::OracleSource::PythPull,
            idl::OracleSource::Pyth1KPull => super::OracleSource::Pyth1KPull,
            idl::OracleSource::Pyth1MPull => super::OracleSource::Pyth1MPull,
            idl::OracleSource::PythStableCoinPull => super::OracleSource::PythStableCoinPull,
            idl::OracleSource::SwitchboardOnDemand => {
                super::OracleSource::DeprecatedSwitchboardOnDemand
            }
            idl::OracleSource::PythLazer => super::OracleSource::PythLazer,
            idl::OracleSource::PythLazer1K => super::OracleSource::PythLazer1K,
            idl::OracleSource::PythLazer1M => super::OracleSource::PythLazer1M,
            idl::OracleSource::PythLazerStableCoin => super::OracleSource::PythLazerStableCoin,
        }
    }
}

/// SDK-side helpers for drift's `SpotMarket` (extension trait — orphan rule).
pub trait SpotMarketExt {
    fn token_program(&self) -> Pubkey;
    fn is_token_2022_program(&self) -> bool;
    fn has_transfer_hook(&self) -> bool;
}
impl SpotMarketExt for SpotMarket {
    fn token_program(&self) -> Pubkey {
        if self.is_token_2022_program() {
            TOKEN_2022_PROGRAM_ID
        } else {
            TOKEN_PROGRAM_ID
        }
    }
    fn is_token_2022_program(&self) -> bool {
        self.token_program_flag & TokenProgramFlag::Token2022 as u8 != 0
    }
    fn has_transfer_hook(&self) -> bool {
        self.token_program_flag & TokenProgramFlag::TransferHook as u8 != 0
    }
}

/// Drift program context
///
/// Contains network specific variables necessary for interacting with drift program
/// on different networks
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Context {
    name: &'static str,
    /// market lookup table
    luts: &'static [Pubkey],
    /// pyth program ID
    pyth: Pubkey,
}

impl Context {
    /// Target MainNet context
    #[allow(non_upper_case_globals)]
    pub const MainNet: Context = Self {
        name: "mainnet",
        luts: LUTS_MAINNET,
        pyth: ids::pyth_program::ID,
    };
    /// Target DevNet context
    #[allow(non_upper_case_globals)]
    pub const DevNet: Context = Self {
        name: "devnet",
        luts: LUTS_DEVNET,
        pyth: ids::pyth_program::ID_DEVNET,
    };

    /// Return drift lookup table address(es)
    pub fn luts(&self) -> &[Pubkey] {
        self.luts
    }

    /// Return pyth owner address
    pub fn pyth(&self) -> Pubkey {
        self.pyth
    }

    /// Return name
    pub fn name(&self) -> &'static str {
        self.name
    }
}

/// Some data from chain along with the retreived slot
#[derive(Debug, Clone)]
pub struct DataAndSlot<T> {
    pub slot: u64,
    pub data: T,
}

/// Id of a Drift market
#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub struct MarketId {
    index: u16,
    kind: MarketType,
}

impl core::hash::Hash for MarketId {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        (self.kind as u8).hash(state);
    }
}

impl MarketId {
    /// Create a new `MarketId` from parts
    pub fn new(index: u16, kind: MarketType) -> Self {
        Self { index, kind }
    }
    /// `MarketId` for the USDC Spot Market
    pub const QUOTE_SPOT: Self = Self {
        index: 0,
        kind: MarketType::Spot,
    };
    /// Id of a perp market
    pub const fn perp(index: u16) -> Self {
        Self {
            index,
            kind: MarketType::Perp,
        }
    }
    /// Id of a spot market
    pub const fn spot(index: u16) -> Self {
        Self {
            index,
            kind: MarketType::Spot,
        }
    }
    /// uint index of the market
    pub fn index(&self) -> u16 {
        self.index
    }
    /// type of the market
    pub fn kind(&self) -> MarketType {
        self.kind
    }
    /// Convert self into its parts
    pub fn to_parts(self) -> (u16, MarketType) {
        (self.index, self.kind)
    }
    pub fn is_perp(self) -> bool {
        self.kind == MarketType::Perp
    }
    pub fn is_spot(self) -> bool {
        self.kind == MarketType::Spot
    }
}

impl std::fmt::Debug for MarketId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.kind {
            MarketType::Perp => {
                write!(f, "perp/{}", self.index)
            }
            MarketType::Spot => {
                write!(f, "spot/{}", self.index)
            }
        }
    }
}

impl From<(u16, MarketType)> for MarketId {
    fn from(value: (u16, MarketType)) -> Self {
        Self {
            index: value.0,
            kind: value.1,
        }
    }
}

impl From<(u16, crate::drift_idl::types::MarketType)> for MarketId {
    fn from(value: (u16, crate::drift_idl::types::MarketType)) -> Self {
        Self {
            index: value.0,
            kind: match value.1 {
                crate::drift_idl::types::MarketType::Perp => MarketType::Perp,
                crate::drift_idl::types::MarketType::Spot => MarketType::Spot,
            },
        }
    }
}

/// Provides builder API for Orders.
///
/// Uses IDL-generated types internally so the produced `OrderParams` matches
/// what `TransactionBuilder`'s instruction builders expect. Converts to/from
/// drift native at the external boundaries.
#[derive(Default)]
pub struct NewOrder {
    order_type: crate::drift_idl::types::OrderType,
    direction: crate::drift_idl::types::PositionDirection,
    reduce_only: bool,
    market_id: MarketId,
    post_only: crate::drift_idl::types::PostOnlyParam,
    ioc: u8,
    amount: u64,
    price: u64,
    user_order_id: u8,
}

impl NewOrder {
    /// Create a market order
    pub fn market(market_id: MarketId) -> Self {
        Self {
            order_type: crate::drift_idl::types::OrderType::Market,
            market_id,
            ..Default::default()
        }
    }
    /// Create an oracle order
    pub fn oracle(market_id: MarketId) -> Self {
        Self {
            order_type: crate::drift_idl::types::OrderType::Oracle,
            market_id,
            ..Default::default()
        }
    }
    /// Create a limit order
    pub fn limit(market_id: MarketId) -> Self {
        Self {
            order_type: crate::drift_idl::types::OrderType::Limit,
            market_id,
            ..Default::default()
        }
    }
    /// Set order amount
    ///
    /// A sub-zero amount indicates a short
    pub fn amount(mut self, amount: i64) -> Self {
        self.direction = if amount >= 0 {
            crate::drift_idl::types::PositionDirection::Long
        } else {
            crate::drift_idl::types::PositionDirection::Short
        };
        self.amount = amount.unsigned_abs();

        self
    }
    /// Set order price
    pub fn price(mut self, price: u64) -> Self {
        self.price = price;
        self
    }
    /// Set reduce only (default: false)
    pub fn reduce_only(mut self, flag: bool) -> Self {
        self.reduce_only = flag;
        self
    }
    /// Set immediate or cancel and other flags (default: false)
    pub fn bit_flags(mut self, flags: u8) -> Self {
        self.ioc = flags;
        self
    }
    /// Set post-only (default: None)
    pub fn post_only(mut self, value: crate::drift_idl::types::PostOnlyParam) -> Self {
        self.post_only = value;
        self
    }
    /// Set user order id
    pub fn user_order_id(mut self, user_order_id: u8) -> Self {
        self.user_order_id = user_order_id;
        self
    }
    /// Call to complete building the Order
    pub fn build(self) -> OrderParams {
        OrderParams {
            order_type: self.order_type,
            market_index: self.market_id.index,
            market_type: match self.market_id.kind {
                MarketType::Perp => crate::drift_idl::types::MarketType::Perp,
                MarketType::Spot => crate::drift_idl::types::MarketType::Spot,
            },
            price: self.price,
            base_asset_amount: self.amount,
            reduce_only: self.reduce_only,
            direction: self.direction,
            bit_flags: self.ioc,
            post_only: self.post_only,
            user_order_id: self.user_order_id,
            ..Default::default()
        }
    }
}

#[derive(Debug, Error)]
pub enum SdkError {
    #[error("{0}")]
    Rpc(#[from] Box<solana_rpc_client_api::client_error::Error>),
    #[error("{0}")]
    Ws(#[from] Box<drift_pubsub_client::PubsubClientError>),
    #[error("{0}")]
    Anchor(#[from] Box<anchor_lang::error::Error>),
    #[error("error while deserializing")]
    Deserializing,
    #[error("invalid drift account")]
    InvalidAccount,
    #[error("invalid oracle account")]
    InvalidOracle,
    #[error("invalid keypair seed")]
    InvalidSeed,
    #[error("invalid base58 value")]
    InvalidBase58,
    #[error("user does not have position: {0}")]
    NoPosition(u16),
    #[error("insufficient SOL balance for fees")]
    OutOfSOL,
    #[error("{0}")]
    Signing(#[from] Box<solana_sdk::signer::SignerError>),
    #[error("Received Error from websocket")]
    WebsocketError,
    #[error("Missed DLOB heartbeat")]
    MissedHeartbeat,
    #[error("Unsupported account data format")]
    UnsupportedAccountData,
    #[error("Could not decode data: {0}")]
    CouldntDecode(#[from] base64::DecodeError),
    #[error("Couldn't join task: {0}")]
    CouldntJoin(#[from] tokio::task::JoinError),
    #[error("Couldn't send unsubscribe message")]
    CouldntUnsubscribe,
    #[error("MathError")]
    MathError(&'static str),
    #[error("{0}")]
    BorrowMutError(#[from] BorrowMutError),
    #[error("{0}")]
    BorrowError(#[from] BorrowError),
    #[error("{0}")]
    Generic(String),
    #[error("max connection attempts reached")]
    MaxReconnectionAttemptsReached,
    #[error("jit taker order not found")]
    JitOrderNotFound,
    #[error("market data unavailable. subscribe market: {0:?}")]
    NoMarketData(MarketId),
    #[error("account data unavailable. subscribe account: {0:?}")]
    NoAccountData(Pubkey),
    #[error("component is already subscribed")]
    AlreadySubscribed,
    #[error("invalid URL")]
    InvalidUrl,
    #[error("{0}")]
    WsClient(#[from] Box<tungstenite::Error>),
    #[error("libdrift_ffi_sys out-of-date")]
    LibDriftVersion,
    #[error("wallet signing disabled")]
    WalletSigningDisabled,
    #[error("{0}")]
    Grpc(#[from] Box<GrpcError>),
}

// Manual From implementations for unboxed error types to avoid breaking changes
impl From<solana_rpc_client_api::client_error::Error> for SdkError {
    fn from(e: solana_rpc_client_api::client_error::Error) -> Self {
        SdkError::Rpc(Box::new(e))
    }
}
impl From<drift_pubsub_client::PubsubClientError> for SdkError {
    fn from(e: drift_pubsub_client::PubsubClientError) -> Self {
        SdkError::Ws(Box::new(e))
    }
}
impl From<anchor_lang::error::Error> for SdkError {
    fn from(e: anchor_lang::error::Error) -> Self {
        SdkError::Anchor(Box::new(e))
    }
}
impl From<crate::solana_sdk::signer::SignerError> for SdkError {
    fn from(e: crate::solana_sdk::signer::SignerError) -> Self {
        SdkError::Signing(Box::new(e))
    }
}
impl From<tungstenite::Error> for SdkError {
    fn from(e: tungstenite::Error) -> Self {
        SdkError::WsClient(Box::new(e))
    }
}
impl From<GrpcError> for SdkError {
    fn from(e: GrpcError) -> Self {
        SdkError::Grpc(Box::new(e))
    }
}

#[derive(Debug, PartialEq)]
/// Solana program execution error
pub enum ProgramError {
    /// instruction error from Drift
    Drift(ErrorCode),
    /// instruction error from another program
    Other { ix_idx: u8, code: u32 },
}

impl Display for ProgramError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Drift(code) => f.write_fmt(format_args!("drift: {code}")),
            Self::Other { ix_idx, code } => {
                f.write_fmt(format_args!("ix_idx: {ix_idx}, code: {code}"))
            }
        }
    }
}

impl SdkError {
    /// extract anchor error code from the SdkError if it exists
    pub fn to_anchor_error_code(&self) -> Option<ProgramError> {
        if let SdkError::Rpc(inner) = self {
            if let Some(TransactionError::InstructionError(
                ix_idx,
                InstructionError::Custom(code),
            )) = inner.get_transaction_error()
            {
                // inverse of anchor's 'From<ErrorCode> for u32'
                let err = match code.checked_sub(anchor_lang::error::ERROR_CODE_OFFSET) {
                    Some(code) => {
                        // this will saturate e.g. if u32 > |ErrorCode\ then it always returns the
                        // highest idx variant
                        ProgramError::Drift(unsafe { std::mem::transmute::<u32, ErrorCode>(code) })
                    }
                    None => ProgramError::Other { ix_idx, code },
                };
                return Some(err);
            }
        }
        None
    }
    /// convert to 'out of sol' error is possible
    pub fn to_out_of_sol_error(&self) -> Option<SdkError> {
        if let SdkError::Rpc(inner) = self {
            if let Some(
                TransactionError::InsufficientFundsForFee
                | TransactionError::InsufficientFundsForRent { account_index: _ },
            ) = inner.get_transaction_error()
            {
                return Some(Self::OutOfSOL);
            }
        }
        None
    }
}

/// Helper type for Accounts included in drift instructions
///
/// Provides sorting implementation matching drift program
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum RemainingAccount {
    Oracle { pubkey: Pubkey },
    Spot { pubkey: Pubkey, writable: bool },
    Perp { pubkey: Pubkey, writable: bool },
}

impl RemainingAccount {
    fn pubkey(&self) -> &Pubkey {
        match self {
            Self::Oracle { pubkey } => pubkey,
            Self::Spot { pubkey, .. } => pubkey,
            Self::Perp { pubkey, .. } => pubkey,
        }
    }
    fn parts(self) -> (Pubkey, bool) {
        match self {
            Self::Oracle { pubkey } => (pubkey, false),
            Self::Spot {
                pubkey, writable, ..
            } => (pubkey, writable),
            Self::Perp {
                pubkey, writable, ..
            } => (pubkey, writable),
        }
    }
    fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)` `union`
        // between `repr(C)` structs, each of which has the `u8` discriminant as its first
        // field, so we can read the discriminant without offsetting the pointer.
        let ptr = <*const RemainingAccount>::from(self);
        unsafe { *ptr.cast::<u8>() }
    }
}

impl Ord for RemainingAccount {
    fn cmp(&self, other: &Self) -> Ordering {
        let type_order = self.discriminant().cmp(&other.discriminant());
        if let Ordering::Equal = type_order {
            self.pubkey().cmp(other.pubkey())
        } else {
            type_order
        }
    }
}

impl PartialOrd for RemainingAccount {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<RemainingAccount> for AccountMeta {
    fn from(value: RemainingAccount) -> Self {
        let (pubkey, is_writable) = value.parts();
        AccountMeta {
            pubkey,
            is_writable,
            is_signer: false,
        }
    }
}

/// Provide market precision information
pub trait MarketPrecision {
    // prices must be a multiple of this
    fn price_tick(&self) -> u64;
    // order sizes must be a multiple of this
    fn quantity_tick(&self) -> u64;
    /// smallest order size
    fn min_order_size(&self) -> u64;
}

impl MarketPrecision for accounts::SpotMarket {
    fn min_order_size(&self) -> u64 {
        self.min_order_size
    }
    fn price_tick(&self) -> u64 {
        self.order_tick_size
    }
    fn quantity_tick(&self) -> u64 {
        self.order_step_size
    }
}

impl MarketPrecision for accounts::PerpMarket {
    fn min_order_size(&self) -> u64 {
        self.amm.min_order_size
    }
    fn price_tick(&self) -> u64 {
        self.amm.order_tick_size
    }
    fn quantity_tick(&self) -> u64 {
        self.amm.order_step_size
    }
}

#[derive(Copy, Clone)]
pub struct ReferrerInfo {
    referrer: Pubkey,
    referrer_stats: Pubkey,
}

impl ReferrerInfo {
    pub fn new(referrer: Pubkey, referrer_stats: Pubkey) -> Self {
        Self {
            referrer,
            referrer_stats,
        }
    }

    pub fn referrer(&self) -> Pubkey {
        self.referrer
    }

    pub fn referrer_stats(&self) -> Pubkey {
        self.referrer_stats
    }

    pub fn get_referrer_info(taker_stats: accounts::UserStats) -> Option<Self> {
        if taker_stats.referrer == Pubkey::default() {
            return None;
        }

        let user_account_pubkey = Wallet::derive_user_account(&taker_stats.referrer, 0);
        let user_stats_pubkey = Wallet::derive_stats_account(&taker_stats.referrer);

        Some(Self {
            referrer: user_account_pubkey,
            referrer_stats: user_stats_pubkey,
        })
    }
}

/// SDK-side helpers for drift's `Order` (extension trait — orphan rule).
pub trait OrderExt {
    const ORACLE_TRIGGER_MARKET_FLAG: u8 = 0b0000_0010;
    const SAFE_TRIGGER_ORDER_FLAG: u8 = 0b0000_0100;
    const NEW_TRIGGER_REDUCE_ONLY_FLAG: u8 = 0b0000_1000;
    const HAS_BUILDER_FLAG: u8 = 0b0001_0000;
    fn is_oracle_trigger_market(&self) -> bool;
    fn has_builder(&self) -> bool;
}
impl OrderExt for Order {
    fn is_oracle_trigger_market(&self) -> bool {
        (self.bit_flags & Self::ORACLE_TRIGGER_MARKET_FLAG) != 0
    }
    fn has_builder(&self) -> bool {
        (self.bit_flags & Self::HAS_BUILDER_FLAG) != 0
    }
}

/// SDK-side helpers for drift's `OrderParams`.
pub trait OrderParamsExt {
    const IMMEDIATE_OR_CANCEL_FLAG: u8 = 0b0000_0001;
    const HIGH_LEVERAGE_MODE_FLAG: u8 = 0b0000_0010;
    fn immediate_or_cancel(&self) -> bool;
    fn high_leverage_mode(&self) -> bool;
}
impl OrderParamsExt for OrderParams {
    fn immediate_or_cancel(&self) -> bool {
        (self.bit_flags & Self::IMMEDIATE_OR_CANCEL_FLAG) > 0
    }
    fn high_leverage_mode(&self) -> bool {
        (self.bit_flags & Self::HIGH_LEVERAGE_MODE_FLAG) > 0
    }
}

/// SDK-side helpers for drift's `OrderType`.
pub trait OrderTypeExt {
    fn as_str(&self) -> &str;
}
impl OrderTypeExt for OrderType {
    fn as_str(&self) -> &str {
        match self {
            OrderType::Limit => "limit",
            OrderType::Market => "market",
            OrderType::Oracle => "oracle",
            OrderType::TriggerLimit => "trigger_limit",
            OrderType::TriggerMarket => "trigger_market",
        }
    }
}

/// SDK-side helpers for drift's `MarketType`.
pub trait MarketTypeExt {
    fn as_str(&self) -> &str;
}
impl MarketTypeExt for MarketType {
    fn as_str(&self) -> &str {
        match self {
            MarketType::Perp => "perp",
            MarketType::Spot => "spot",
        }
    }
}

/// Parse a market kind string (orphan rule blocks `impl FromStr for MarketType`).
pub fn market_type_from_str(s: &str) -> Result<MarketType, ()> {
    if s.eq_ignore_ascii_case("perp") {
        Ok(MarketType::Perp)
    } else if s.eq_ignore_ascii_case("spot") {
        Ok(MarketType::Spot)
    } else {
        Err(())
    }
}

#[derive(Clone, Copy, Default)]
pub struct ProtectedMakerParams {
    pub limit_price_divisor: u8,
    pub dynamic_offset: u64,
    pub tick_size: u64,
}

/// SDK-side helpers for drift's `UserStats`.
pub trait UserStatsExt {
    fn is_referrer(&self) -> bool;
    fn is_referred(&self) -> bool;
}
impl UserStatsExt for UserStats {
    fn is_referrer(&self) -> bool {
        self.referrer_status & 0b0000_0001 != 0
    }
    fn is_referred(&self) -> bool {
        self.referrer_status & 0b0000_0010 != 0
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::solana_sdk::{
        instruction::error::InstructionError, pubkey::Pubkey, transaction::TransactionError,
    };
    use solana_rpc_client_api::{
        client_error::{Error as ClientError, ErrorKind as ClientErrorKind},
        request::{RpcError, RpcRequest, RpcResponseErrorData},
        response::RpcSimulateTransactionResult,
    };

    use super::{RemainingAccount, SdkError};
    use crate::{drift_idl::errors::ErrorCode, types::ProgramError, MarketType};

    #[test]
    fn market_type_str() {
        assert_eq!(MarketType::from_str("PERP").unwrap(), MarketType::Perp,);
        assert_eq!(MarketType::from_str("spot").unwrap(), MarketType::Spot,);
        assert_eq!("perp", MarketType::Perp.as_str());
        assert_eq!("spot", MarketType::Spot.as_str());
    }

    #[test]
    fn extract_anchor_error() {
        let err = SdkError::Rpc(
            Box::new(ClientError {
                request: Some(RpcRequest::SendTransaction),
                kind: Box::new(ClientErrorKind::RpcError(
                    RpcError::RpcResponseError {
                        code: -32002,
                        message: "Transaction simulation failed: Error processing Instruction 0: custom program error: 0x17b7".to_string(),
                        data: RpcResponseErrorData::SendTransactionPreflightFailure(
                            RpcSimulateTransactionResult {
                                err: Some(TransactionError::InstructionError(0, InstructionError::Custom(6071)).into()),
                                logs: None,
                                accounts: None,
                                units_consumed: None,
                                loaded_accounts_data_size: None,
                                return_data: None,
                                inner_instructions: None,
                                replacement_blockhash: None,
                                fee: None,
                                pre_balances: None,
                                post_balances: None,
                                pre_token_balances: None,
                                post_token_balances: None,
                                loaded_addresses: None,
                            }
                        )
                    }
                ))
            })
        );

        assert_eq!(
            err.to_anchor_error_code().unwrap(),
            ProgramError::Drift(ErrorCode::UserOrderIdAlreadyInUse),
        );
    }

    #[test]
    fn account_type_sorting() {
        let mut accounts = vec![
            RemainingAccount::Perp {
                pubkey: Pubkey::new_from_array([4_u8; 32]),
                writable: false,
            },
            RemainingAccount::Oracle {
                pubkey: Pubkey::new_from_array([2_u8; 32]),
            },
            RemainingAccount::Oracle {
                pubkey: Pubkey::new_from_array([1_u8; 32]),
            },
            RemainingAccount::Spot {
                pubkey: Pubkey::new_from_array([3_u8; 32]),
                writable: true,
            },
        ];
        accounts.sort();

        assert_eq!(
            accounts,
            vec![
                RemainingAccount::Oracle {
                    pubkey: Pubkey::new_from_array([1_u8; 32])
                },
                RemainingAccount::Oracle {
                    pubkey: Pubkey::new_from_array([2_u8; 32])
                },
                RemainingAccount::Spot {
                    pubkey: Pubkey::new_from_array([3_u8; 32]),
                    writable: true
                },
                RemainingAccount::Perp {
                    pubkey: Pubkey::new_from_array([4_u8; 32]),
                    writable: false
                },
            ]
        )
    }
}

#[derive(Clone, Debug)]
pub struct AccountUpdate {
    /// Address of the account
    pub pubkey: Pubkey,
    /// Owner of the account
    pub owner: Pubkey,
    pub lamports: u64,
    /// Serialized account data (e.g. Anchor/Borsh)
    pub data: Vec<u8>,
    /// Slot retrieved
    pub slot: u64,
}

pub type OnAccountFn = dyn Fn(&AccountUpdate) + Send + Sync + 'static;

/// Empty callback function pointer that does nothing - useful as a no-op callback
pub const EMPTY_ACCOUNT_CALLBACK: fn(&AccountUpdate) = |_: &AccountUpdate| {};

/// SDK-side feature-flag helpers for drift's `State`.
pub trait StateExt {
    const MM_ORACLE_UPDATE_FLAG: u8 = 0b0000_0001;
    const MEDIAN_TRIGGER_PRICE_FLAG: u8 = 0b0000_0010;
    const BUILDER_CODES_FLAG: u8 = 0b0000_0100;
    const BUILDER_REFERRAL_FLAG: u8 = 0b0000_1000;
    fn has_mm_oracle_update_feature(&self) -> bool;
    fn has_median_trigger_price_feature(&self) -> bool;
    fn has_builder_codes_feature(&self) -> bool;
    fn has_builder_referral_feature(&self) -> bool;
}
impl StateExt for accounts::State {
    fn has_mm_oracle_update_feature(&self) -> bool {
        (self.feature_bit_flags & Self::MM_ORACLE_UPDATE_FLAG) != 0
    }
    fn has_median_trigger_price_feature(&self) -> bool {
        (self.feature_bit_flags & Self::MEDIAN_TRIGGER_PRICE_FLAG) != 0
    }
    fn has_builder_codes_feature(&self) -> bool {
        (self.feature_bit_flags & Self::BUILDER_CODES_FLAG) != 0
    }
    fn has_builder_referral_feature(&self) -> bool {
        (self.feature_bit_flags & Self::BUILDER_REFERRAL_FLAG) != 0
    }
}
