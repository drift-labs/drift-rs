use std::{collections::HashMap, str::FromStr};

use reqwest::Client;
use serde::{Deserialize, Serialize};
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_config::RpcAccountInfoConfig};
use solana_sdk::{
    account::ReadableAccount,
    address_lookup_table_account::AddressLookupTableAccount,
    instruction::Instruction,
    message::{v0, VersionedMessage},
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    transaction::VersionedTransaction,
};

use crate::{
    jupiter::serde_helpers::field_as_string,
    types::{SdkError, SdkResult},
};

use self::{
    swap::{SwapInstructionsResponse, SwapInstructionsResponseInternal, SwapRequest, SwapResponse},
    transaction_config::TransactionConfig,
};

mod serde_helpers;
mod swap;
mod transaction_config;

#[derive(Serialize, Deserialize, Default, PartialEq, Clone, Debug)]
pub enum SwapMode {
    #[default]
    ExactIn,
    ExactOut,
}

impl FromStr for SwapMode {
    type Err = SdkError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "ExactIn" => Ok(Self::ExactIn),
            "ExactOut" => Ok(Self::ExactOut),
            _ => Err(SdkError::Generic(format!("{} is not a valid SwapMode", s))),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RoutePlanStep {
    pub swap_info: SwapInfo,
    pub percent: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SwapInfo {
    #[serde(with = "field_as_string")]
    pub amm_key: Pubkey,
    pub label: String,
    #[serde(with = "field_as_string")]
    pub input_mint: Pubkey,
    #[serde(with = "field_as_string")]
    pub output_mint: Pubkey,
    /// An estimation of the input amount into the AMM
    #[serde(with = "field_as_string")]
    pub in_amount: u64,
    /// An estimation of the output amount into the AMM
    #[serde(with = "field_as_string")]
    pub out_amount: u64,
    #[serde(with = "field_as_string")]
    pub fee_amount: u64,
    #[serde(with = "field_as_string")]
    pub fee_mint: Pubkey,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlatformFee {
    #[serde(with = "field_as_string")]
    pub amount: u64,
    pub fee_bps: u8,
}

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct QuoteRequest {
    #[serde(with = "field_as_string")]
    pub input_mint: Pubkey,
    #[serde(with = "field_as_string")]
    pub output_mint: Pubkey,
    #[serde(with = "field_as_string")]
    pub amount: u64,
    pub swap_mode: Option<SwapMode>,
    /// Allowed slippage in basis points
    pub slippage_bps: u16,
    /// Platform fee in basis points
    pub platform_fee_bps: Option<u8>,
    pub dexes: Option<Vec<String>>,
    pub excluded_dexes: Option<Vec<String>>,
    /// Quote only direct routes
    pub only_direct_routes: Option<bool>,
    /// Quote fit into legacy transaction
    pub as_legacy_transaction: Option<bool>,
    /// Find a route given a maximum number of accounts involved,
    /// this might dangerously limit routing ending up giving a bad price.
    /// The max is an estimation and not the exact count
    pub max_accounts: Option<usize>,
    // Quote type to be used for routing, switches the algorithm
    pub quote_type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QuoteResponse {
    #[serde(with = "field_as_string")]
    pub input_mint: Pubkey,
    #[serde(with = "field_as_string")]
    pub in_amount: u64,
    #[serde(with = "field_as_string")]
    pub output_mint: Pubkey,
    #[serde(with = "field_as_string")]
    pub out_amount: u64,
    /// Not used by build transaction
    #[serde(with = "field_as_string")]
    pub other_amount_threshold: u64,
    pub swap_mode: SwapMode,
    pub slippage_bps: u16,
    pub platform_fee: Option<PlatformFee>,
    pub price_impact_pct: String,
    pub route_plan: Vec<RoutePlanStep>,
    #[serde(default)]
    pub context_slot: u64,
    #[serde(default)]
    pub time_taken: f64,
}

pub struct JupiterClient {
    url: String,
    rpc_client: RpcClient,
    lookup_table_cache: HashMap<String, AddressLookupTableAccount>,
}

impl JupiterClient {
    pub fn new(rpc_client: RpcClient, url: Option<String>) -> Self {
        let url = match url {
            Some(url) => url,
            None => "https://quote-api.jup.ag".to_string(),
        };

        Self {
            url,
            rpc_client,
            lookup_table_cache: HashMap::new(),
        }
    }

    /// Get routes for a swap
    #[allow(clippy::too_many_arguments)]
    pub async fn get_quote(
        &self,
        input_mint: Pubkey,
        output_mint: Pubkey,
        amount: u64,
        max_accounts: Option<usize>,
        slippage_bps: u16,
        swap_mode: Option<SwapMode>,
        only_direct_routes: Option<bool>,
        excluded_dexes: Option<Vec<String>>,
    ) -> SdkResult<QuoteResponse> {
        let quote_request = QuoteRequest {
            input_mint,
            output_mint,
            amount,
            swap_mode,
            slippage_bps,
            platform_fee_bps: None,
            dexes: None,
            excluded_dexes,
            only_direct_routes,
            as_legacy_transaction: None,
            max_accounts,
            quote_type: None,
        };
        let query = serde_qs::to_string(&quote_request)
            .map_err(|e| SdkError::Generic(format!("failed to serialize: {e}")))?;
        let api_version_param = if self.url == "https://quote-api.jup.ag" {
            "/v6"
        } else {
            ""
        };

        let response = Client::new()
            .get(format!("{}{api_version_param}/quote?{query}", self.url))
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response
                .json::<QuoteResponse>()
                .await
                .map_err(|e| SdkError::Generic(format!("failed to get json: {e}")))?)
        } else {
            Err(SdkError::Generic(format!(
                "Request status not ok: {}, body: {}",
                response.status(),
                response
                    .text()
                    .await
                    .map_err(|e| SdkError::Generic(format!("failed to get text: {e}")))?
            )))
        }
    }

    /// Get a swap transaction for quote
    pub async fn get_swap(
        &self,
        mut quote_response: QuoteResponse,
        user_public_key: Pubkey,
        slippage_bps: Option<u16>,
    ) -> SdkResult<VersionedTransaction> {
        let slippage_bps = slippage_bps.unwrap_or(50);
        let api_version_param = if self.url == "https://quote-api.jup.ag" {
            "/v6"
        } else {
            ""
        };

        quote_response.slippage_bps = slippage_bps;
        let swap_request = SwapRequest {
            user_public_key,
            quote_response,
            config: TransactionConfig::default(),
        };
        let response = Client::new()
            .post(format!("{}{api_version_param}/swap", self.url))
            .json(&swap_request)
            .send()
            .await?;

        if response.status().is_success() {
            let res = response
                .json::<SwapResponse>()
                .await
                .map_err(|e| SdkError::Generic(format!("failed to get json: {e}")))?;

            let versioned_transaction: VersionedTransaction =
                bincode::deserialize(&res.swap_transaction)
                    .map_err(|_e| SdkError::Deserializing)?;

            Ok(versioned_transaction)
        } else {
            Err(SdkError::Generic(format!(
                "Request status not ok: {}, body: {}",
                response.status(),
                response
                    .text()
                    .await
                    .map_err(|e| SdkError::Generic(format!("failed to get text: {e}")))?
            )))
        }
    }

    /// Get the transaction message and lookup tables for a transaction
    /// https://solana.stackexchange.com/questions/12811/lookuptables-in-rust
    pub async fn get_transaction_message_and_lookup_tables(
        &self,
        transaction: VersionedTransaction,
        instruction: Instruction,
        payer: &Keypair,
    ) -> SdkResult<(VersionedTransaction, Vec<AddressLookupTableAccount>)> {
        let message = transaction.message;

        let lookup_tables_futures = match message.address_table_lookups() {
            Some(lookups) => lookups
                .iter()
                .map(|lookup| self.get_lookup_table(lookup.account_key))
                .collect(),
            None => vec![],
        };

        let lookup_tables: Vec<AddressLookupTableAccount> =
            futures_util::future::join_all(lookup_tables_futures)
                .await
                .into_iter()
                .filter_map(|result| match result {
                    Ok(Some(account)) => Some(account),
                    _ => None,
                })
                .collect();

        let recent_blockhash = self.rpc_client.get_latest_blockhash().await?;
        let tx = VersionedTransaction::try_new(
            VersionedMessage::V0(
                v0::Message::try_compile(
                    &payer.pubkey(),
                    &[instruction],
                    &lookup_tables,
                    recent_blockhash,
                )
                .map_err(|e| SdkError::Generic(format!("failed to compile: {e}")))?,
            ),
            &[payer],
        )?;

        Ok((tx, lookup_tables))
    }

    async fn get_lookup_table(
        &self,
        account_key: Pubkey,
    ) -> SdkResult<Option<AddressLookupTableAccount>> {
        if let Some(table_account) = self.lookup_table_cache.get(&account_key.to_string()) {
            return Ok(Some(table_account.clone()));
        }

        let account_info = self
            .rpc_client
            .get_account_with_config(&account_key, RpcAccountInfoConfig::default())
            .await?;

        let mut value = None;
        if let Some(account) = account_info.value {
            let table =
                solana_address_lookup_table_program::state::AddressLookupTable::deserialize(
                    account.data(),
                )
                .map_err(|_e| SdkError::Deserializing)?;
            value = Some(AddressLookupTableAccount {
                key: account_key,
                addresses: table.addresses.to_vec(),
            });
        }

        Ok(value)
    }

    pub async fn get_swap_instructions(
        &self,
        quote_response: QuoteResponse,
        user_public_key: Pubkey,
    ) -> SdkResult<SwapInstructionsResponse> {
        let api_version_param = if self.url == "https://quote-api.jup.ag" {
            "/v6"
        } else {
            ""
        };

        let swap_request = SwapRequest {
            user_public_key,
            quote_response,
            config: TransactionConfig::default(),
        };
        let response = Client::new()
            .post(format!("{}{api_version_param}/swap-instructions", self.url))
            .json(&swap_request)
            .send()
            .await?;

        if response.status().is_success() {
            let swap_instruction_res_internal = response
                .json::<SwapInstructionsResponseInternal>()
                .await
                .map_err(|e| SdkError::Generic(format!("failed to get json: {e}")))?;

            Ok(swap_instruction_res_internal.into())
        } else {
            Err(SdkError::Generic(format!(
                "Request status not ok: {}, body: {}",
                response.status(),
                response
                    .text()
                    .await
                    .map_err(|e| SdkError::Generic(format!("failed to get text: {e}")))?
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use solana_client::nonblocking::rpc_client::RpcClient;
    use solana_sdk::pubkey;
    use solana_sdk::pubkey::Pubkey;

    use crate::jupiter::JupiterClient;
    use crate::types::SdkResult;

    use super::QuoteResponse;

    const USDC_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
    const NATIVE_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
    const TEST_WALLET: Pubkey = pubkey!("2AQdpHJ2JpcEgPiATUXjQxA8QmafFegfQwSLWSprPicm");

    async fn request_get_quote(client: &JupiterClient) -> SdkResult<QuoteResponse> {
        let quote_response = client
            .get_quote(
                USDC_MINT,
                NATIVE_MINT,
                1_000_000,
                None,
                50,
                None,
                None,
                None,
            )
            .await;

        quote_response
    }

    #[tokio::test]
    async fn test_get_quote() {
        let rpc_client = RpcClient::new("".to_string());
        let jupiter_client = JupiterClient::new(rpc_client, None);

        // GET /quote
        let quote_response = request_get_quote(&jupiter_client).await;

        assert!(quote_response.is_ok());
    }

    #[tokio::test]
    async fn test_get_swap() {
        let rpc_client = RpcClient::new("".to_string());
        let jupiter_client = JupiterClient::new(rpc_client, None);

        let quote_response = request_get_quote(&jupiter_client)
            .await
            .expect("failed to get quote");

        // POST /swap
        let swap_response = jupiter_client
            .get_swap(quote_response, TEST_WALLET, None)
            .await;

        assert!(swap_response.is_ok());
    }

    #[tokio::test]
    async fn test_get_swap_instructions() {
        let rpc_client = RpcClient::new("".to_string());
        let jupiter_client = JupiterClient::new(rpc_client, None);

        let quote_response = request_get_quote(&jupiter_client)
            .await
            .expect("failed to get quote");

        // POST /swap-instructions
        let swap_instructions = jupiter_client
            .get_swap_instructions(quote_response, TEST_WALLET)
            .await;

        assert!(swap_instructions.is_ok());
    }
}
