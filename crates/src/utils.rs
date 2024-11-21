//! SDK utility functions

use base64::Engine;
use serde_json::json;
use solana_sdk::{
    account::Account, address_lookup_table::AddressLookupTableAccount, bs58, pubkey::Pubkey,
    signature::Keypair,
};

use crate::types::{SdkError, SdkResult};

// kudos @wphan
/// Try to parse secret `key` string
///
/// Returns error if the key cannot be parsed
pub fn read_keypair_str_multi_format(key: &str) -> SdkResult<Keypair> {
    // strip out any white spaces and new line/carriage return characters
    let key = key.replace([' ', '\n', '\r', '[', ']'], "");

    // first try to decode as a byte array
    if key.contains(',') {
        // decode the numbers array into json string
        let bytes: Result<Vec<u8>, _> = key.split(',').map(|x| x.parse::<u8>()).collect();
        if let Ok(bytes) = bytes {
            return Keypair::from_bytes(&bytes).map_err(|_| SdkError::InvalidSeed);
        } else {
            return Err(SdkError::InvalidSeed);
        }
    }

    // try to decode as base58 string
    if let Ok(bytes) = bs58::decode(key.as_bytes()).into_vec() {
        return Keypair::from_bytes(&bytes).map_err(|_| SdkError::InvalidSeed);
    }

    // try to decode as base64 string
    if let Ok(bytes) = base64::engine::general_purpose::STANDARD.decode(key.as_bytes()) {
        return Keypair::from_bytes(&bytes).map_err(|_| SdkError::InvalidSeed);
    }

    Err(SdkError::InvalidSeed)
}

/// Try load a `Keypair` from a file path or given string, supports json format and base58 format.
pub fn load_keypair_multi_format(path_or_key: &str) -> SdkResult<Keypair> {
    if let Ok(data) = std::fs::read_to_string(path_or_key) {
        read_keypair_str_multi_format(data.as_str())
    } else {
        read_keypair_str_multi_format(path_or_key)
    }
}

const LOOKUP_TABLE_META_SIZE: usize = 56;

/// modified from sdk.1.17.x
/// https://docs.rs/solana-program/latest/src/solana_program/address_lookup_table/state.rs.html#192
pub fn deserialize_alt(address: Pubkey, account: &Account) -> SdkResult<AddressLookupTableAccount> {
    let raw_addresses_data: &[u8] = account.data.get(LOOKUP_TABLE_META_SIZE..).ok_or({
        // Should be impossible because table accounts must
        // always be LOOKUP_TABLE_META_SIZE in length
        SdkError::InvalidAccount
    })?;
    let addresses = bytemuck::try_cast_slice(raw_addresses_data).map_err(|_| {
        // Should be impossible because raw address data
        // should be aligned and sized in multiples of 32 bytes
        SdkError::InvalidAccount
    })?;

    Ok(AddressLookupTableAccount {
        key: address,
        addresses: addresses.to_vec(),
    })
}

pub fn http_to_ws(url: &str) -> Result<String, &'static str> {
    let base_url = if url.starts_with("http://") {
        url.replacen("http://", "ws://", 1)
    } else if url.starts_with("https://") {
        url.replacen("https://", "wss://", 1)
    } else {
        return Err("Invalid URL scheme");
    };

    Ok(format!("{}/ws", base_url.trim_end_matches('/')))
}

/// Convert a url string into a Ws equivalent
pub fn get_ws_url(url: &str) -> SdkResult<String> {
    if url.starts_with("http://") || url.starts_with("https://") {
        Ok(url.replacen("http", "ws", 1))
    } else if url.starts_with("wss://") || url.starts_with("ws://") {
        Ok(url.to_string())
    } else {
        #[cfg(test)]
        {
            if url.starts_with("MockSender") {
                return Ok("ws://mock.sender.com".into());
            }
        }
        Err(SdkError::InvalidUrl)
    }
}

/// Convert a url string into an Http equivalent
pub fn get_http_url(url: &str) -> SdkResult<String> {
    if url.starts_with("http://") || url.starts_with("https://") {
        Ok(url.to_string())
    } else if url.starts_with("ws://") || url.starts_with("wss://") {
        Ok(url.replacen("ws", "http", 1))
    } else {
        Err(SdkError::InvalidUrl)
    }
}

pub fn dlob_subscribe_ws_json(market: &str) -> String {
    json!({
        "type": "subscribe",
        "marketType": if market.ends_with("perp") {
            "perp"
        } else {
            "spot"
        },
        "channel": "orderbook",
        "market": market,
    })
    .to_string()
}

pub(crate) fn zero_account_to_bytes<T: bytemuck::Pod + anchor_lang::Discriminator>(
    account: T,
) -> Vec<u8> {
    let mut account_data = vec![0; 8 + std::mem::size_of::<T>()];
    account_data[0..8].copy_from_slice(bytemuck::bytes_of(&T::DISCRIMINATOR));
    account_data[8..].copy_from_slice(bytemuck::bytes_of(&account));
    account_data
}

pub mod test_envs {
    //! test env vars
    use solana_sdk::signature::Keypair;

    /// solana mainnet endpoint
    pub fn mainnet_endpoint() -> String {
        std::env::var("TEST_MAINNET_RPC_ENDPOINT").expect("TEST_MAINNET_ENDPOINT set")
    }
    /// solana devnet endpoint
    pub fn devnet_endpoint() -> String {
        std::env::var("TEST_DEVNET_RPC_ENDPOINT")
            .unwrap_or_else(|_| "https://api.devnet.solana.com".to_string())
    }
    /// keypair for integration tests
    pub fn test_keypair() -> Keypair {
        let private_key = std::env::var("TEST_PRIVATE_KEY").expect("TEST_PRIVATE_KEY set");
        Keypair::from_base58_string(private_key.as_str())
    }
}

#[cfg(test)]
pub mod test_utils {
    //! test utilities

    use anchor_lang::Discriminator;
    use bytes::BytesMut;
    // helpers from drift-program test_utils.
    pub fn get_pyth_price(price: i64, expo: i32) -> pyth_test::Price {
        let mut pyth_price = pyth_test::Price::default();
        let price = price * 10_i64.pow(expo as u32);
        pyth_price.agg.price = price;
        pyth_price.twap = price;
        pyth_price.expo = expo;
        pyth_price
    }

    mod pyth_test {
        //! helper structs for pyth oracle prices
        use bytemuck::{Pod, Zeroable};
        use serde::Serialize;

        #[derive(Default, Copy, Clone, Serialize)]
        #[repr(C)]
        pub struct AccKey {
            pub val: [u8; 32],
        }

        #[derive(Copy, Clone, Default, Serialize)]
        #[repr(C)]
        #[allow(dead_code)]
        pub enum PriceStatus {
            Unknown,
            #[default]
            Trading,
            Halted,
            Auction,
        }

        #[derive(Copy, Clone, Default, Serialize)]
        #[repr(C)]
        pub enum CorpAction {
            #[default]
            NoCorpAct,
        }

        #[derive(Default, Copy, Clone, Serialize)]
        #[repr(C)]
        pub struct PriceInfo {
            pub price: i64,
            pub conf: u64,
            pub status: PriceStatus,
            pub corp_act: CorpAction,
            pub pub_slot: u64,
        }
        #[derive(Default, Copy, Clone, Serialize)]
        #[repr(C)]
        pub struct PriceComp {
            publisher: AccKey,
            agg: PriceInfo,
            latest: PriceInfo,
        }

        #[derive(Copy, Clone, Default, Serialize)]
        #[repr(C)]
        #[allow(dead_code, clippy::upper_case_acronyms)]
        pub enum PriceType {
            Unknown,
            #[default]
            Price,
            TWAP,
            Volatility,
        }

        #[derive(Default, Copy, Clone, Serialize)]
        #[repr(C)]
        pub struct Price {
            pub magic: u32,       // Pyth magic number.
            pub ver: u32,         // Program version.
            pub atype: u32,       // Account type.
            pub size: u32,        // Price account size.
            pub ptype: PriceType, // Price or calculation type.
            pub expo: i32,        // Price exponent.
            pub num: u32,         // Number of component prices.
            pub unused: u32,
            pub curr_slot: u64,        // Currently accumulating price slot.
            pub valid_slot: u64,       // Valid slot-time of agg. price.
            pub twap: i64,             // Time-weighted average price.
            pub avol: u64,             // Annualized price volatility.
            pub drv0: i64,             // Space for future derived values.
            pub drv1: i64,             // Space for future derived values.
            pub drv2: i64,             // Space for future derived values.
            pub drv3: i64,             // Space for future derived values.
            pub drv4: i64,             // Space for future derived values.
            pub drv5: i64,             // Space for future derived values.
            pub prod: AccKey,          // Product account key.
            pub next: AccKey,          // Next Price account in linked list.
            pub agg_pub: AccKey,       // Quoter who computed last aggregate price.
            pub agg: PriceInfo,        // Aggregate price info.
            pub comp: [PriceComp; 32], // Price components one per quoter.
        }

        #[cfg(target_endian = "little")]
        unsafe impl Zeroable for Price {}

        #[cfg(target_endian = "little")]
        unsafe impl Pod for Price {}
    }

    pub fn get_account_bytes<T: bytemuck::Pod>(account: &mut T) -> BytesMut {
        let mut bytes = BytesMut::new();
        let data = bytemuck::bytes_of_mut(account);
        bytes.extend_from_slice(data);
        bytes
    }

    pub fn get_anchor_account_bytes<T: bytemuck::Pod + Discriminator>(account: &mut T) -> BytesMut {
        let mut bytes = BytesMut::new();
        bytes.extend_from_slice(&T::discriminator());
        let data = bytemuck::bytes_of_mut(account);
        bytes.extend_from_slice(data);
        bytes
    }

    #[macro_export]
    macro_rules! create_account_info {
        ($account:expr, $pubkey:expr, $owner:expr, $name: ident) => {
            let acc = Account {
                data: crate::utils::test_utils::get_account_bytes(&mut $account).to_vec(),
                owner: $owner,
                ..Default::default()
            };
            let $name: crate::ffi::AccountWithKey = (*$pubkey, acc).into();
        };
    }

    #[macro_export]
    macro_rules! create_anchor_account_info {
        ($account:expr, $pubkey:expr, $type:ident, $name: ident) => {
            let owner = constants::PROGRAM_ID;
            let acc = Account {
                data: crate::utils::test_utils::get_anchor_account_bytes(&mut $account).to_vec(),
                owner,
                ..Default::default()
            };
            let $name: crate::ffi::AccountWithKey = ($pubkey, acc).into();
        };
    }
}

#[cfg(test)]
mod tests {
    use solana_sdk::signer::Signer;

    use super::*;

    #[test]
    fn test_keypair_from_json_numbers_array() {
        let keypair_data = "[17,188,105,73,182,3,56,125,157,20,12,82,88,197,181,202,251,248,97,103,215,165,233,145,114,254,20,89,100,79,207,168,206,103,77,58,215,94,196,155,224,116,73,74,62,200,30,248,101,102,164,126,6,170,77,190,186,142,107,222,3,242,143,155]";

        let keypair = read_keypair_str_multi_format(keypair_data).unwrap();
        assert!(keypair.pubkey().to_string() == "EtiM5qwcrrawQP9FfRErBatNvDgEU656tk5aA8iTgqri");
    }

    #[test]
    fn test_keypair_from_json_comma_separated_numbers() {
        let keypair_data = "17,188,105,73,182,3,56,125,157,20,12,82,88,197,181,202,251,248,97,103,215,165,233,145,114,254,20,89,100,79,207,168,206,103,77,58,215,94,196,155,224,116,73,74,62,200,30,248,101,102,164,126,6,170,77,190,186,142,107,222,3,242,143,155";

        let keypair = read_keypair_str_multi_format(keypair_data).unwrap();
        assert!(keypair.pubkey().to_string() == "EtiM5qwcrrawQP9FfRErBatNvDgEU656tk5aA8iTgqri");
    }

    #[test]
    fn test_keypair_from_base58_string() {
        let keypair_data = "MZsY4Vme2Xa417rhh1MUGCru9oYNDxCjH1TZRWJPNSzRmZmodjczVaGuWKgzBsoKxx2ZLQZjUWTkLu44jE5DhSJ";

        let keypair = read_keypair_str_multi_format(keypair_data).unwrap();
        assert!(keypair.pubkey().to_string() == "EtiM5qwcrrawQP9FfRErBatNvDgEU656tk5aA8iTgqri");
    }

    #[test]
    fn test_keypair_from_base64_string() {
        let keypair_data = "EbxpSbYDOH2dFAxSWMW1yvv4YWfXpemRcv4UWWRPz6jOZ006117Em+B0SUo+yB74ZWakfgaqTb66jmveA/KPmw==";

        let keypair = read_keypair_str_multi_format(keypair_data).unwrap();
        assert!(keypair.pubkey().to_string() == "EtiM5qwcrrawQP9FfRErBatNvDgEU656tk5aA8iTgqri");
    }

    #[test]
    fn test_https_to_ws() {
        let https_url = "https://dlob.drift.trade";
        assert!(http_to_ws(https_url).unwrap() == "wss://dlob.drift.trade/ws");
        let http_url = "http://dlob.drift.trade";
        assert!(http_to_ws(http_url).unwrap() == "ws://dlob.drift.trade/ws")
    }
}
