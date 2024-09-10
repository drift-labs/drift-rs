//! JIT proxy client
//!
//! Routes JIT maker orders via onchain jit-proxy program

use std::borrow::Cow;

use anchor_lang::InstructionData;
pub use jit_proxy::state::{PostOnlyParam, PriceType};
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    instruction::{AccountMeta, Instruction},
    message::v0,
    signature::Signature,
};

pub use crate::drift_idl::math::auction::calculate_auction_price;
use crate::{
    build_accounts,
    constants::{state_account, PROGRAM_ID},
    drift_idl::{accounts::User, math::constants::QUOTE_SPOT_MARKET_INDEX, types::MarketType},
    types::{MarketId, ReferrerInfo, RpcSendTransactionConfig, VersionedMessage},
    AccountProvider, DriftClient, Pubkey, SdkError, SdkResult, TransactionBuilder, Wallet,
};

/// Ix parameters for jit-proxy program
pub struct JitIxParams {
    taker_key: Pubkey,
    taker_stats_key: Pubkey,
    taker: User,
    taker_order_id: u32,
    max_position: i64,
    min_position: i64,
    bid: i64,
    ask: i64,
    price_type: Option<PriceType>,
    referrer_info: Option<ReferrerInfo>,
    post_only: Option<PostOnlyParam>,
}

impl JitIxParams {
    pub fn new(
        taker_key: Pubkey,
        taker_stats_key: Pubkey,
        taker: User,
        taker_order_id: u32,
        max_position: i64,
        min_position: i64,
        bid: i64,
        ask: i64,
        price_type: Option<PriceType>,
        referrer_info: Option<ReferrerInfo>,
        post_only: Option<PostOnlyParam>,
    ) -> Self {
        Self {
            taker_key,
            taker_stats_key,
            taker,
            taker_order_id,
            max_position,
            min_position,
            bid,
            ask,
            price_type,
            referrer_info,
            post_only,
        }
    }
}

#[derive(Clone)]
pub struct JitProxyClient<T: AccountProvider> {
    drift_client: DriftClient,
    config: RpcSendTransactionConfig,
    cu_params: Option<ComputeBudgetParams>,
}

impl<T: AccountProvider> JitProxyClient<T> {
    pub fn new(
        drift_client: DriftClient,
        config: Option<RpcSendTransactionConfig>,
        cu_params: Option<ComputeBudgetParams>,
    ) -> Self {
        Self {
            drift_client,
            config: config.unwrap_or_default(),
            cu_params,
        }
    }

    pub fn update_config(&mut self, config: RpcSendTransactionConfig) {
        self.config = config;
    }

    pub fn update_cu_params(&mut self, cu_params: ComputeBudgetParams) {
        self.cu_params = Some(cu_params);
    }

    /// Build a jit tx
    ///
    /// `params` JIT order params
    /// `authority` drift authority pubkey
    /// `sub_account` tuple (pubkey of the drift user account, data of drift user account)
    pub async fn build_jit_tx(
        &self,
        params: JitIxParams,
        authority: &Pubkey,
        sub_account: (&Pubkey, &User),
    ) -> SdkResult<VersionedMessage> {
        let order = params
            .taker
            .orders
            .iter()
            .find(|order| order.order_id == params.taker_order_id)
            .ok_or(SdkError::JitOrderNotFound)?;

        let tx_builder = TransactionBuilder::new(
            self.drift_client.program_data(),
            *sub_account.0,
            Cow::Borrowed(sub_account.1),
            false,
        );

        let program_data = tx_builder.program_data();
        let account_data = tx_builder.account_data();

        let writable_markets = match order.market_type {
            MarketType::Perp => {
                vec![MarketId::perp(order.market_index)]
            }
            MarketType::Spot => {
                vec![MarketId::spot(order.market_index), MarketId::QUOTE_SPOT]
            }
        };

        let mut accounts = build_accounts(
            program_data,
            jit_proxy::accounts::Jit {
                state: *state_account(),
                user: *sub_account.0,
                user_stats: Wallet::derive_stats_account(authority, &PROGRAM_ID),
                taker: params.taker_key,
                taker_stats: params.taker_stats_key,
                authority: *authority,
                drift_program: PROGRAM_ID,
            },
            &[&params.taker, account_data],
            &[],
            writable_markets.as_slice(),
        );

        if let Some(referrer_info) = params.referrer_info {
            accounts.push(AccountMeta::new(referrer_info.referrer(), false));
            accounts.push(AccountMeta::new(referrer_info.referrer_stats(), false));
        }

        if order.market_type == MarketType::Spot {
            let spot_market_vault = self
                .drift_client
                .get_spot_market_account_and_slot(order.market_index)
                .expect("spot market exists")
                .data
                .vault;
            let quote_spot_market_vault = self
                .drift_client
                .get_spot_market_account_and_slot(QUOTE_SPOT_MARKET_INDEX)
                .expect("quote market exists")
                .data
                .vault;
            accounts.push(AccountMeta::new_readonly(spot_market_vault, false));
            accounts.push(AccountMeta::new_readonly(quote_spot_market_vault, false));
        }

        let jit_params = jit_proxy::instructions::JitParams {
            taker_order_id: params.taker_order_id,
            max_position: params.max_position,
            min_position: params.min_position,
            bid: params.bid,
            ask: params.ask,
            price_type: params.price_type.unwrap(),
            post_only: params.post_only,
        };

        let ix = Instruction {
            program_id: jit_proxy::id(),
            accounts,
            data: jit_proxy::instruction::Jit { params: jit_params }.data(),
        };

        let mut ixs = Vec::with_capacity(3);
        if let Some(cu_params) = self.cu_params {
            let cu_limit_ix =
                ComputeBudgetInstruction::set_compute_unit_price(cu_params.microlamports_per_cu());
            let cu_price_ix =
                ComputeBudgetInstruction::set_compute_unit_limit(cu_params.cu_limit());

            ixs.push(cu_limit_ix);
            ixs.push(cu_price_ix);
        }
        ixs.push(ix);

        let lut = program_data.lookup_table.clone();

        let message =
            v0::Message::try_compile(authority, ixs.as_slice(), &[lut], Default::default())
                .expect("failed to compile message");

        Ok(VersionedMessage::V0(message))
    }

    /// Send a jit tx given params
    pub async fn jit(
        &self,
        params: JitIxParams,
        authority: &Pubkey,
        sub_account_id: Option<u16>,
    ) -> SdkResult<Signature> {
        let sub_account =
            Wallet::derive_user_account(authority, sub_account_id.unwrap_or_default(), &PROGRAM_ID);
        let sub_account_data = self.drift_client.get_user_account(&sub_account).await?;
        let tx = self
            .build_jit_tx(params, authority, (&sub_account, &sub_account_data))
            .await?;
        self.drift_client
            .sign_and_send_with_config(tx, self.config)
            .await
    }
}

#[derive(Clone, Copy)]
pub struct ComputeBudgetParams {
    microlamports_per_cu: u64,
    cu_limit: u32,
}

impl ComputeBudgetParams {
    pub fn new(microlamports_per_cu: u64, cu_limit: u32) -> Self {
        Self {
            microlamports_per_cu,
            cu_limit,
        }
    }

    pub fn microlamports_per_cu(&self) -> u64 {
        self.microlamports_per_cu
    }

    pub fn cu_limit(&self) -> u32 {
        self.cu_limit
    }
}
