use std::{
    cell::{BorrowError, BorrowMutError},
    cmp::Ordering,
    str::FromStr,
};

pub use solana_client::rpc_config::RpcSendTransactionConfig;
pub use solana_sdk::{
    commitment_config::CommitmentConfig, message::VersionedMessage,
    transaction::VersionedTransaction,
};
use solana_sdk::{
    instruction::{AccountMeta, InstructionError},
    pubkey::Pubkey,
    transaction::TransactionError,
};
use thiserror::Error;
use tokio::sync::oneshot;
use tokio_tungstenite::tungstenite;

// re-export types in public API
pub use crate::drift_idl::{
    accounts::{self},
    errors::{self},
    events::{self},
    instructions::{self},
    types::*,
};
use crate::{
    constants::{ids, LUT_DEVNET, LUT_MAINNET},
    drift_idl::errors::ErrorCode,
    Wallet,
};

/// Handle for unsubscribing from network updates
pub type UnsubHandle = oneshot::Sender<()>;

pub type SdkResult<T> = Result<T, SdkError>;

pub fn is_one_of_variant<T: PartialEq>(value: &T, variants: &[T]) -> bool {
    variants.iter().any(|variant| value == variant)
}

/// Drift program context
///
/// Contains network specific variables necessary for interacting with drift program
/// on different networks
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Context {
    name: &'static str,
    /// market lookup table
    lut: Pubkey,
    /// pyth program ID
    pyth: Pubkey,
}

impl Context {
    /// Target MainNet context
    #[allow(non_upper_case_globals)]
    pub const MainNet: Context = Self {
        name: "mainnet",
        lut: LUT_MAINNET,
        pyth: ids::pyth_program::ID,
    };
    /// Target DevNet context
    #[allow(non_upper_case_globals)]
    pub const DevNet: Context = Self {
        name: "devnet",
        lut: LUT_DEVNET,
        pyth: ids::pyth_program::ID_DEVNET,
    };

    /// Return drift lookup table address
    pub fn lut(&self) -> Pubkey {
        self.lut
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
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct MarketId {
    index: u16,
    kind: MarketType,
}

// there are derived/auto-generated trait impls for `MarketType` so
// it can be used a key in maps, within `MarketId`
// doing here rather than adding to all structs or special casing in IDL generation
impl core::cmp::Eq for MarketType {}
impl core::hash::Hash for MarketType {
    fn hash<H: core::hash::Hasher>(&self, ra_expand_state: &mut H) {
        core::mem::discriminant(self).hash(ra_expand_state);
        match self {
            MarketType::Spot => {}
            MarketType::Perp => {}
        }
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

/// Provides builder API for Orders
#[derive(Default)]
pub struct NewOrder {
    order_type: OrderType,
    direction: PositionDirection,
    reduce_only: bool,
    market_id: MarketId,
    post_only: PostOnlyParam,
    ioc: bool,
    amount: u64,
    price: u64,
    user_order_id: u8,
}

impl NewOrder {
    /// Create a market order
    pub fn market(market_id: MarketId) -> Self {
        Self {
            order_type: OrderType::Market,
            market_id,
            ..Default::default()
        }
    }
    /// Create a limit order
    pub fn limit(market_id: MarketId) -> Self {
        Self {
            order_type: OrderType::Limit,
            market_id,
            ..Default::default()
        }
    }
    /// Set order amount
    ///
    /// A sub-zero amount indicates a short
    pub fn amount(mut self, amount: i64) -> Self {
        self.direction = if amount >= 0 {
            PositionDirection::Long
        } else {
            PositionDirection::Short
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
    /// Set immediate or cancel (default: false)
    pub fn ioc(mut self, flag: bool) -> Self {
        self.ioc = flag;
        self
    }
    /// Set post-only (default: None)
    pub fn post_only(mut self, value: PostOnlyParam) -> Self {
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
            market_type: self.market_id.kind,
            price: self.price,
            base_asset_amount: self.amount,
            reduce_only: self.reduce_only,
            direction: self.direction,
            immediate_or_cancel: self.ioc,
            post_only: self.post_only,
            user_order_id: self.user_order_id,
            ..Default::default()
        }
    }
}

#[derive(Debug, Error)]
pub enum SdkError {
    #[error("{0}")]
    Rpc(#[from] solana_client::client_error::ClientError),
    #[error("{0}")]
    Ws(#[from] drift_pubsub_client::PubsubClientError),
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
    Signing(#[from] solana_sdk::signer::SignerError),
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
    MathError(String),
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
    WsClient(#[from] tungstenite::Error),
    #[error("libdrift_ffi_sys out-of-date")]
    LibDriftVersion,
}

impl SdkError {
    /// extract anchor error code from the SdkError if it exists
    pub fn to_anchor_error_code(&self) -> Option<ErrorCode> {
        if let SdkError::Rpc(inner) = self {
            if let Some(TransactionError::InstructionError(_, InstructionError::Custom(code))) =
                inner.get_transaction_error()
            {
                // inverse of anchor's 'From<ErrorCode> for u32'
                return Some(unsafe {
                    std::mem::transmute::<u32, ErrorCode>(
                        code - anchor_lang::error::ERROR_CODE_OFFSET,
                    )
                });
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

impl OrderType {
    pub fn as_str(&self) -> &str {
        match self {
            OrderType::Limit => "limit",
            OrderType::Market => "market",
            OrderType::Oracle => "oracle",
            OrderType::TriggerLimit => "trigger_limit",
            OrderType::TriggerMarket => "trigger_market",
        }
    }
}

impl MarketType {
    pub fn as_str(&self) -> &str {
        match self {
            MarketType::Perp => "perp",
            MarketType::Spot => "spot",
        }
    }
}

impl FromStr for MarketType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("perp") {
            Ok(Self::Perp)
        } else if s.eq_ignore_ascii_case("spot") {
            Ok(Self::Spot)
        } else {
            Err(())
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use solana_client::{
        client_error::{ClientError, ClientErrorKind},
        rpc_request::{RpcError, RpcRequest},
        rpc_response::RpcSimulateTransactionResult,
    };
    use solana_sdk::{
        instruction::InstructionError, pubkey::Pubkey, transaction::TransactionError,
    };

    use super::{RemainingAccount, SdkError};
    use crate::{drift_idl::errors::ErrorCode, MarketType};

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
            ClientError {
                request: Some(RpcRequest::SendTransaction),
                kind: ClientErrorKind::RpcError(
                    RpcError::RpcResponseError {
                        code: -32002,
                        message: "Transaction simulation failed: Error processing Instruction 0: custom program error: 0x17b7".to_string(),
                        data: solana_client::rpc_request::RpcResponseErrorData::SendTransactionPreflightFailure(
                            RpcSimulateTransactionResult {
                                err: Some(TransactionError::InstructionError(0, InstructionError::Custom(6071))),
                                logs: None,
                                accounts: None,
                                units_consumed: None,
                                return_data: None,
                                inner_instructions: None,
                                replacement_blockhash: None,
                            }
                        )
                    }
                )
            }
        );

        assert_eq!(
            err.to_anchor_error_code().unwrap(),
            ErrorCode::UserOrderIdAlreadyInUse,
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
