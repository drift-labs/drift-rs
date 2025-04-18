use jupiter_swap_api_client::{
    quote::{QuoteResponse, SwapMode},
    swap::SwapInstructionsResponse,
    transaction_config::TransactionConfig,
    JupiterSwapApiClient,
};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{message::AddressLookupTableAccount, pubkey::Pubkey};

use crate::{
    types::{SdkError, SdkResult},
    utils,
};

/// Default Jupiter API url
const DEFAULT_JUPITER_API_URL: &str = "https://lite-api.jup.ag/v1";

/// jupiter swap IXs and metadata for building a swap Tx
pub struct JupiterSwapInfo {
    pub quote: QuoteResponse,
    pub ixs: SwapInstructionsResponse,
    pub luts: Vec<AddressLookupTableAccount>,
}

/// Fetches Jupiter swap instructions for token swaps on Solana
///
/// This function queries Jupiter API to get the optimal swap route and corresponding instructions
/// for swapping between two tokens.
///
/// # Arguments
///
/// * `rpc` - A Solana RPC client
/// * `user_authority` - The public key of the user's wallet that will execute the swap
/// * `amount` - The amount of input tokens to swap, in native units (smallest denomination)
/// * `swap_mode` - The type of swap to perform (e.g. ExactIn, ExactOut)
/// * `slippage_bps` - Maximum allowed slippage in basis points (1 bp = 0.01%)
/// * `input_mint` - The mint address of the token to swap from
/// * `output_mint` - The mint address of the token to swap to
/// * `only_direct_routes` - If Some(true), only consider direct swap routes between the tokens
/// * `excluded_dexes` - Optional comma-separated string of DEX names to exclude from routing
/// * `transaction_config` - Optional configuration for the swap transaction
///
/// # Returns
///
/// Returns a `Result` containing `JupiterSwapInfo` with the swap instructions and route details
/// if successful, or a `SdkError` if the operation fails.
///
/// # Example
///
/// ```no_run
/// use solana_sdk::pubkey::Pubkey;
///
/// let swap_info = get_jupiter_swap_ixs(
///     rpc_client,
///     user_wallet.pubkey(),
///     1_000_000, // 1 USDC
///     SwapMode::ExactIn,
///     50, // 0.5% slippage
///     usdc_mint,
///     sol_mint,
///     Some(true),
///     None,
///     None
/// ).await?;
/// ```
pub async fn get_jupiter_swap_ixs(
    rpc: RpcClient,
    user_authority: Pubkey,
    amount: u64,
    swap_mode: SwapMode,
    slippage_bps: u16,
    input_mint: Pubkey,
    output_mint: Pubkey,
    only_direct_routes: Option<bool>,
    excluded_dexes: Option<String>,
    transaction_config: Option<TransactionConfig>,
) -> SdkResult<JupiterSwapInfo> {
    let jupiter_url = std::env::var("JUPITER_API_URL").unwrap_or(DEFAULT_JUPITER_API_URL.into());
    let jup_client = JupiterSwapApiClient::new(jupiter_url);

    // GET /quote
    let quote_request = jupiter_swap_api_client::quote::QuoteRequest {
        amount,
        swap_mode: Some(swap_mode),
        input_mint,
        output_mint,
        slippage_bps,
        only_direct_routes,
        excluded_dexes,
        ..Default::default()
    };

    let quote_response = jup_client.quote(&quote_request).await.map_err(|err| {
        log::error!("jupiter api request: {err:?}");
        SdkError::Generic(err.to_string())
    })?;
    // POST /swap-instructions
    let swap_instructions = jup_client
        .swap_instructions(&jupiter_swap_api_client::swap::SwapRequest {
            user_public_key: user_authority,
            quote_response: quote_response.clone(),
            config: transaction_config.unwrap_or_default(),
        })
        .await
        .map_err(|err| {
            log::error!("jupiter api request: {err:?}");
            SdkError::Generic(err.to_string())
        })?;

    let res = rpc
        .get_multiple_accounts(swap_instructions.address_lookup_table_addresses.as_slice())
        .await?;

    let luts = res
        .iter()
        .zip(swap_instructions.address_lookup_table_addresses.iter())
        .map(|(acc, key)| {
            utils::deserialize_alt(*key, acc.as_ref().expect("deser LUT")).expect("deser LUT")
        })
        .collect();

    Ok(JupiterSwapInfo {
        luts,
        quote: quote_response,
        ixs: swap_instructions,
    })
}
