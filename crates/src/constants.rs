use std::sync::OnceLock;

use solana_sdk::{address_lookup_table::AddressLookupTableAccount, pubkey::Pubkey};

use crate::{
    drift_idl::accounts::{PerpMarket, SpotMarket},
    types::Context,
};

/// drift program address
pub const PROGRAM_ID: Pubkey = solana_sdk::pubkey!("dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH");

pub const DEFAULT_PUBKEY: Pubkey = solana_sdk::pubkey!("11111111111111111111111111111111");

static STATE_ACCOUNT: OnceLock<Pubkey> = OnceLock::new();

pub const TOKEN_PROGRAM_ID: Pubkey =
    solana_sdk::pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

/// Return the market lookup table
pub(crate) const fn market_lookup_table(context: Context) -> Pubkey {
    match context {
        Context::DevNet => {
            solana_sdk::pubkey!("FaMS3U4uBojvGn5FSDEPimddcXsCfwkKsFgMVVnDdxGb")
        }
        Context::MainNet => {
            solana_sdk::pubkey!("D9cnvzswDikQDf53k4HpQ3KJ9y1Fv3HGGDFYMXnK5T6c")
        }
    }
}

/// Drift state account
pub fn state_account() -> &'static Pubkey {
    STATE_ACCOUNT.get_or_init(|| {
        let (state_account, _seed) =
            Pubkey::find_program_address(&[&b"drift_state"[..]], &PROGRAM_ID);
        state_account
    })
}

/// calculate the PDA of a drift spot market given index
pub fn derive_spot_market_account(market_index: u16) -> Pubkey {
    let (account, _seed) = Pubkey::find_program_address(
        &[&b"spot_market"[..], &market_index.to_le_bytes()],
        &PROGRAM_ID,
    );
    account
}

/// calculate the PDA of a drift perp market given index
pub fn derive_perp_market_account(market_index: u16) -> Pubkey {
    let (account, _seed) = Pubkey::find_program_address(
        &[&b"perp_market"[..], &market_index.to_le_bytes()],
        &PROGRAM_ID,
    );
    account
}

/// calculate the PDA for a drift spot market vault given index
pub fn derive_spot_market_vault(market_index: u16) -> Pubkey {
    let (account, _seed) = Pubkey::find_program_address(
        &[&b"spot_market_vault"[..], &market_index.to_le_bytes()],
        &PROGRAM_ID,
    );
    account
}

/// calculate the PDA for the drift signer
pub fn derive_drift_signer() -> Pubkey {
    let (account, _seed) = Pubkey::find_program_address(&[&b"drift_signer"[..]], &PROGRAM_ID);
    account
}

/// Helper methods for market data structs
pub trait MarketExt {
    fn market_type(&self) -> &'static str;
    fn symbol(&self) -> &str;
}

impl MarketExt for PerpMarket {
    fn market_type(&self) -> &'static str {
        "perp"
    }
    fn symbol(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.name) }.trim_end()
    }
}

impl MarketExt for SpotMarket {
    fn market_type(&self) -> &'static str {
        "spot"
    }
    fn symbol(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.name) }.trim_end()
    }
}

/// Static-ish metadata from onchain drift program
#[derive(Clone)]
pub struct ProgramData {
    spot_markets: &'static [SpotMarket],
    perp_markets: &'static [PerpMarket],
    pub lookup_table: AddressLookupTableAccount,
}

impl ProgramData {
    /// Return an uninitialized instance of `ProgramData` (useful for bootstrapping)
    pub const fn uninitialized() -> Self {
        Self {
            spot_markets: &[],
            perp_markets: &[],
            lookup_table: AddressLookupTableAccount {
                key: Pubkey::new_from_array([0; 32]),
                addresses: vec![],
            },
        }
    }
    /// Initialize `ProgramData`
    pub fn new(
        mut spot: Vec<SpotMarket>,
        mut perp: Vec<PerpMarket>,
        lookup_table: AddressLookupTableAccount,
    ) -> Self {
        spot.sort_by(|a, b| a.market_index.cmp(&b.market_index));
        perp.sort_by(|a, b| a.market_index.cmp(&b.market_index));
        // other code relies on aligned indexes for fast lookups
        assert!(
            spot.iter()
                .enumerate()
                .all(|(idx, x)| idx == x.market_index as usize),
            "spot indexes unaligned"
        );
        assert!(
            perp.iter()
                .enumerate()
                .all(|(idx, x)| idx == x.market_index as usize),
            "perp indexes unaligned"
        );

        Self {
            spot_markets: Box::leak(spot.into_boxed_slice()),
            perp_markets: Box::leak(perp.into_boxed_slice()),
            lookup_table,
        }
    }

    /// Return known spot markets
    pub fn spot_market_configs(&self) -> &'static [SpotMarket] {
        self.spot_markets
    }

    /// Return known perp markets
    pub fn perp_market_configs(&self) -> &'static [PerpMarket] {
        self.perp_markets
    }

    /// Return the spot market config given a market index
    pub fn spot_market_config_by_index(&self, market_index: u16) -> Option<&'static SpotMarket> {
        self.spot_markets.get(market_index as usize)
    }

    /// Return the perp market config given a market index
    pub fn perp_market_config_by_index(&self, market_index: u16) -> Option<&'static PerpMarket> {
        self.perp_markets.get(market_index as usize)
    }
}

pub mod ids {
    pub mod pyth_program {
        use solana_program::declare_id;
        declare_id!("FsJ3A3u2vn5cTVofAjvy6y5kwABJAqYWpe4975bi2epH");
    }

    pub mod wormhole_program {
        use solana_program::declare_id;
        declare_id!("HDwcJBJXjL9FpJ7UBsYBtaDjsBUhuLCUYoz3zr8SWWaQ");
    }

    pub mod drift_oracle_receiver_program {
        use solana_program::declare_id;
        declare_id!("G6EoTTTgpkNBtVXo96EQp2m6uwwVh2Kt6YidjkmQqoha");
    }

    pub mod switchboard_program {
        use solana_program::declare_id;
        declare_id!("SW1TCH7qEPTdLsDHRgPuMQjbQxKdH2aBStViMFnt64f");
    }

    pub mod switchboard_on_demand {
        use solana_program::declare_id;
        declare_id!("SBondMDrcV3K4kxZR1HNVT7osZxAHVHgYXL5Ze1oMUv");
    }

    pub mod bonk_oracle {
        use solana_program::declare_id;
        declare_id!("8ihFLu5FimgTQ1Unh4dVyEHUGodJ5gJQCrQf4KUVB9bN");
    }

    pub mod bonk_pull_oracle {
        use solana_program::declare_id;
        declare_id!("GojbSnJuPdKDT1ZuHuAM5t9oz6bxTo1xhUKpTua2F72p");
    }

    pub mod pepe_oracle {
        use solana_program::declare_id;
        declare_id!("FSfxunDmjjbDV2QxpyxFCAPKmYJHSLnLuvQXDLkMzLBm");
    }

    pub mod pepe_pull_oracle {
        use solana_program::declare_id;
        declare_id!("CLxofhtzvLiErpn25wvUzpZXEqBhuZ6WMEckEraxyuGt");
    }

    pub mod wen_oracle {
        use solana_program::declare_id;
        declare_id!("6Uo93N83iF5U9KwC8eQpogx4XptMT4wSKfje7hB1Ufko");
    }

    pub mod wen_pull_oracle {
        use solana_program::declare_id;
        declare_id!("F47c7aJgYkfKXQ9gzrJaEpsNwUKHprysregTWXrtYLFp");
    }

    pub mod usdc_oracle {
        use solana_program::declare_id;
        declare_id!("Gnt27xtC473ZT2Mw5u8wZ68Z3gULkSTb5DuxJy7eJotD");
    }

    pub mod usdc_pull_oracle {
        use solana_program::declare_id;
        declare_id!("En8hkHLkRe9d9DraYmBTrus518BvmVH448YcvmrFM6Ce");
    }

    pub mod jupiter_mainnet_6 {
        use solana_program::declare_id;
        declare_id!("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4");
    }
    pub mod jupiter_mainnet_4 {
        use solana_program::declare_id;
        declare_id!("JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB");
    }
    pub mod jupiter_mainnet_3 {
        use solana_program::declare_id;
        declare_id!("JUP3c2Uh3WA4Ng34tw6kPd2G4C5BB21Xo36Je1s32Ph");
    }

    pub mod marinade_mainnet {
        use solana_program::declare_id;
        declare_id!("MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD");
    }

    pub mod usdt_oracle {
        use solana_program::declare_id;
        declare_id!("3vxLXJqLqF3JG5TCbYycbKWRBbCJQLxQmBGCkyqEEefL");
    }

    pub mod usdt_pull_oracle {
        use solana_program::declare_id;
        declare_id!("BekJ3P5G3iFeC97sXHuKnUHofCFj9Sbo7uyF2fkKwvit");
    }

    pub mod admin_hot_wallet {
        use solana_program::declare_id;
        declare_id!("5hMjmxexWu954pX9gB9jkHxMqdjpxArQS2XdvkaevRax");
    }
}
