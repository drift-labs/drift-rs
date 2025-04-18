use jupiter_swap_api_client::{
    quote::{QuoteResponse, SwapMode},
    swap::SwapInstructionsResponse,
    transaction_config::TransactionConfig,
    JupiterSwapApiClient,
};
use solana_sdk::pubkey::Pubkey;

use crate::types::{SdkError, SdkResult};

/// Default Jupiter API url
const DEFAULT_JUPITER_API_URL: &str = "https://lite-api.jup.ag/v1";

/// Queries jupiter API for a quote and subsequent swap
///
/// Returns (quote, swap ixs) on success
pub async fn get_jupiter_swap_ixs(
    user_authority: Pubkey,
    amount: u64,
    swap_mode: SwapMode,
    slippage_bps: u16,
    input_mint: Pubkey,
    output_mint: Pubkey,
    transaction_config: Option<TransactionConfig>,
) -> SdkResult<(QuoteResponse, SwapInstructionsResponse)> {
    let jupiter_url = std::env::var("JUPITER_API_URL").unwrap_or(DEFAULT_JUPITER_API_URL.into());
    let jup_client = JupiterSwapApiClient::new(jupiter_url);

    // GET /quote
    let quote_request = jupiter_swap_api_client::quote::QuoteRequest {
        amount,
        swap_mode: Some(swap_mode),
        input_mint,
        output_mint,
        dexes: Some("Whirlpool,Meteora DLMM,Raydium CLMM".into()),
        slippage_bps,
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

    Ok((quote_response, swap_instructions))
}
