use std::sync::OnceLock;

use solana_sdk::{address_lookup_table::AddressLookupTableAccount, pubkey::Pubkey};

use crate::{
    drift_idl::accounts::{PerpMarket, SpotMarket},
    types::{accounts::State, Context},
    MarketId, MarketType, OracleSource,
};

/// https://github.com/solana-labs/solana-web3.js/blob/4e9988cfc561f3ed11f4c5016a29090a61d129a8/src/sysvar.ts#L11
pub const SYSVAR_INSTRUCTIONS_PUBKEY: Pubkey =
    solana_sdk::pubkey!("Sysvar1nstructions1111111111111111111111111");

/// https://github.com/solana-foundation/solana-web3.js/blob/4e9988cfc561f3ed11f4c5016a29090a61d129a8/src/sysvar.ts#L19
pub const SYSVAR_RENT_PUBKEY: Pubkey =
    solana_sdk::pubkey!("SysvarRent111111111111111111111111111111111");

/// Drift program address
pub const PROGRAM_ID: Pubkey = solana_sdk::pubkey!("dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH");

/// Vault program address
pub const VAULT_PROGRAM_ID: Pubkey =
    solana_sdk::pubkey!("vAuLTsyrvSfZRuRB3XgvkPwNGgYSs9YRYymVebLKoxR");

/// JIT proxy program address
pub const JIT_PROXY_ID: Pubkey =
    solana_sdk::pubkey!("J1TnP8zvVxbtF5KFp5xRmWuvG9McnhzmBd9XGfCyuxFP");
/// Empty pubkey
pub const DEFAULT_PUBKEY: Pubkey = solana_sdk::pubkey!("11111111111111111111111111111111");

pub const SYSTEM_PROGRAM_ID: Pubkey = DEFAULT_PUBKEY;

pub const PYTH_LAZER_STORAGE_ACCOUNT_KEY: Pubkey =
    solana_sdk::pubkey!("3rdJbqfnagQ4yx9HXJViD4zc4xpiSqmFsKpPuSCQVyQL");

static STATE_ACCOUNT: OnceLock<Pubkey> = OnceLock::new();
static HIGH_LEVERAGE_MODE_ACCOUNT: OnceLock<Pubkey> = OnceLock::new();

/// Address of the SPL Token program
pub const TOKEN_PROGRAM_ID: Pubkey =
    solana_sdk::pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

/// Address of the SPL Token 2022 program
pub const TOKEN_2022_PROGRAM_ID: Pubkey =
    solana_sdk::pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");

/// Address of Associated Token Program
pub const ASSOCIATED_TOKEN_PROGRAM_ID: Pubkey =
    solana_sdk::pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

/// Drift market lookup table (DevNet)
pub const LUTS_DEVNET: &[Pubkey] = &[solana_sdk::pubkey!(
    "FaMS3U4uBojvGn5FSDEPimddcXsCfwkKsFgMVVnDdxGb"
)];
/// Drift market lookup table (MainNet)
pub const LUTS_MAINNET: &[Pubkey] = &[
    solana_sdk::pubkey!("Fpys8GRa5RBWfyeN7AaDUwFGD1zkDCA4z3t4CJLV8dfL"),
    solana_sdk::pubkey!("EiWSskK5HXnBTptiS5DH6gpAJRVNQ3cAhTKBGaiaysAb"),
];

/// Drift state account
pub fn state_account() -> &'static Pubkey {
    STATE_ACCOUNT.get_or_init(|| {
        let (state_account, _seed) =
            Pubkey::find_program_address(&[&b"drift_state"[..]], &PROGRAM_ID);
        state_account
    })
}

/// Returns the program's HLM config address
pub fn high_leverage_mode_account() -> &'static Pubkey {
    HIGH_LEVERAGE_MODE_ACCOUNT.get_or_init(|| {
        let (account_drift_pda, _seed) =
            Pubkey::find_program_address(&[&b"high_leverage_mode_config"[..]], &PROGRAM_ID);
        account_drift_pda
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

pub fn derive_revenue_share(authority: &Pubkey) -> Pubkey {
    let (account, _seed) =
        Pubkey::find_program_address(&[&b"REV_SHARE"[..], authority.as_ref()], &PROGRAM_ID);
    account
}

pub fn derive_revenue_share_escrow(authority: &Pubkey) -> Pubkey {
    let (account, _seed) =
        Pubkey::find_program_address(&[&b"REV_ESCROW"[..], authority.as_ref()], &PROGRAM_ID);
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
///
/// useful for market info suchas as pubkeys, decimal places, which rarely change.
///
/// it should not be relied upon for live values such as OI, total borrows, etc.
/// instead subscribe to a marketmap
#[derive(Clone)]
pub struct ProgramData {
    spot_markets: &'static [SpotMarket],
    perp_markets: &'static [PerpMarket],
    pub lookup_tables: &'static [AddressLookupTableAccount],
    // drift state account
    state: State,
}

impl ProgramData {
    /// Return an uninitialized instance of `ProgramData` (useful for bootstrapping)
    pub const fn uninitialized() -> Self {
        Self {
            spot_markets: &[],
            perp_markets: &[],
            lookup_tables: &[],
            state: unsafe { std::mem::zeroed() },
        }
    }
    /// Initialize `ProgramData`
    pub fn new(
        mut spot: Vec<SpotMarket>,
        mut perp: Vec<PerpMarket>,
        lookup_tables: Vec<AddressLookupTableAccount>,
        state: State,
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
            lookup_tables: Box::leak(lookup_tables.into_boxed_slice()),
            state,
        }
    }

    /// Return drift `State` account (cached)
    ///
    /// prefer live
    pub fn state(&self) -> &State {
        &self.state
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
    ///
    /// Useful for static metadata e.g. token program address
    pub fn spot_market_config_by_index(&self, market_index: u16) -> Option<&'static SpotMarket> {
        self.spot_markets.get(market_index as usize)
    }

    /// Return the perp market config given a market index
    ///
    /// Useful for static metadata e.g. token program address
    pub fn perp_market_config_by_index(&self, market_index: u16) -> Option<&'static PerpMarket> {
        self.perp_markets.get(market_index as usize)
    }

    /// Given some drift `MarketId`'s maps them to associated public keys
    pub fn markets_to_accounts(&self, markets: &[MarketId]) -> Vec<Pubkey> {
        let accounts: Vec<Pubkey> = markets
            .iter()
            .filter_map(|x| match x.kind() {
                MarketType::Spot => self
                    .spot_market_config_by_index(x.index())
                    .map(|x| x.pubkey),
                MarketType::Perp => self
                    .perp_market_config_by_index(x.index())
                    .map(|x| x.pubkey),
            })
            .collect();

        accounts
    }
}

/// Map oracle `source` to its owner pubkey (network dependent)
pub fn oracle_source_to_owner(context: Context, source: OracleSource) -> Pubkey {
    match source {
        OracleSource::Pyth
        | OracleSource::Pyth1K
        | OracleSource::Pyth1M
        | OracleSource::PythStableCoin => context.pyth(),
        OracleSource::PythPull
        | OracleSource::Pyth1KPull
        | OracleSource::Pyth1MPull
        | OracleSource::PythStableCoinPull => ids::drift_oracle_receiver_program::ID,
        OracleSource::Switchboard => ids::switchboard_program::ID,
        OracleSource::SwitchboardOnDemand => ids::switchboard_on_demand::ID,
        OracleSource::QuoteAsset => DEFAULT_PUBKEY,
        OracleSource::Prelaunch
        | OracleSource::PythLazer
        | OracleSource::PythLazer1K
        | OracleSource::PythLazer1M
        | OracleSource::PythLazerStableCoin => PROGRAM_ID,
    }
}

pub mod ids {
    pub mod pyth_program {
        use solana_sdk::pubkey::Pubkey;
        pub const ID: Pubkey = solana_sdk::pubkey!("FsJ3A3u2vn5cTVofAjvy6y5kwABJAqYWpe4975bi2epH");
        pub const ID_DEVNET: Pubkey =
            solana_sdk::pubkey!("gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s");
    }

    pub mod wormhole_program {
        use solana_sdk::pubkey::Pubkey;

        pub const ID: Pubkey = solana_sdk::pubkey!("HDwcJBJXjL9FpJ7UBsYBtaDjsBUhuLCUYoz3zr8SWWaQ");
    }

    pub mod drift_oracle_receiver_program {
        use solana_sdk::pubkey::Pubkey;

        pub const ID: Pubkey = solana_sdk::pubkey!("G6EoTTTgpkNBtVXo96EQp2m6uwwVh2Kt6YidjkmQqoha");
    }

    pub mod switchboard_program {
        use solana_sdk::pubkey::Pubkey;

        pub const ID: Pubkey = solana_sdk::pubkey!("SW1TCH7qEPTdLsDHRgPuMQjbQxKdH2aBStViMFnt64f");
    }

    pub mod switchboard_on_demand {
        use solana_sdk::pubkey::Pubkey;

        pub const ID: Pubkey = solana_sdk::pubkey!("SBondMDrcV3K4kxZR1HNVT7osZxAHVHgYXL5Ze1oMUv");
    }

    pub mod bonk_oracle {
        use solana_sdk::pubkey::Pubkey;

        pub const ID: Pubkey = solana_sdk::pubkey!("8ihFLu5FimgTQ1Unh4dVyEHUGodJ5gJQCrQf4KUVB9bN");
    }

    pub mod bonk_pull_oracle {
        use solana_sdk::pubkey::Pubkey;

        pub const ID: Pubkey = solana_sdk::pubkey!("GojbSnJuPdKDT1ZuHuAM5t9oz6bxTo1xhUKpTua2F72p");
    }

    pub mod pepe_oracle {
        use solana_sdk::pubkey::Pubkey;

        pub const ID: Pubkey = solana_sdk::pubkey!("FSfxunDmjjbDV2QxpyxFCAPKmYJHSLnLuvQXDLkMzLBm");
    }

    pub mod pepe_pull_oracle {
        use solana_sdk::pubkey::Pubkey;

        pub const ID: Pubkey = solana_sdk::pubkey!("CLxofhtzvLiErpn25wvUzpZXEqBhuZ6WMEckEraxyuGt");
    }

    pub mod wen_oracle {
        use solana_sdk::pubkey::Pubkey;

        pub const ID: Pubkey = solana_sdk::pubkey!("6Uo93N83iF5U9KwC8eQpogx4XptMT4wSKfje7hB1Ufko");
    }

    pub mod wen_pull_oracle {
        use solana_sdk::pubkey::Pubkey;

        pub const ID: Pubkey = solana_sdk::pubkey!("F47c7aJgYkfKXQ9gzrJaEpsNwUKHprysregTWXrtYLFp");
    }

    pub mod usdc_oracle {
        use solana_sdk::pubkey::Pubkey;

        pub const ID: Pubkey = solana_sdk::pubkey!("Gnt27xtC473ZT2Mw5u8wZ68Z3gULkSTb5DuxJy7eJotD");
    }

    pub mod usdc_pull_oracle {
        use solana_sdk::pubkey::Pubkey;

        pub const ID: Pubkey = solana_sdk::pubkey!("En8hkHLkRe9d9DraYmBTrus518BvmVH448YcvmrFM6Ce");
    }

    pub mod jupiter_mainnet_6 {
        use solana_sdk::pubkey::Pubkey;

        pub const ID: Pubkey = solana_sdk::pubkey!("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4");
    }
    pub mod jupiter_mainnet_4 {
        use solana_sdk::pubkey::Pubkey;

        pub const ID: Pubkey = solana_sdk::pubkey!("JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB");
    }
    pub mod jupiter_mainnet_3 {
        use solana_sdk::pubkey::Pubkey;

        pub const ID: Pubkey = solana_sdk::pubkey!("JUP3c2Uh3WA4Ng34tw6kPd2G4C5BB21Xo36Je1s32Ph");
    }

    pub mod marinade_mainnet {
        use solana_sdk::pubkey::Pubkey;

        pub const ID: Pubkey = solana_sdk::pubkey!("MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD");
    }

    pub mod usdt_oracle {
        use solana_sdk::pubkey::Pubkey;

        pub const ID: Pubkey = solana_sdk::pubkey!("3vxLXJqLqF3JG5TCbYycbKWRBbCJQLxQmBGCkyqEEefL");
    }

    pub mod usdt_pull_oracle {
        use solana_sdk::pubkey::Pubkey;

        pub const ID: Pubkey = solana_sdk::pubkey!("BekJ3P5G3iFeC97sXHuKnUHofCFj9Sbo7uyF2fkKwvit");
    }

    pub mod admin_hot_wallet {
        use solana_sdk::pubkey::Pubkey;

        pub const ID: Pubkey = solana_sdk::pubkey!("5hMjmxexWu954pX9gB9jkHxMqdjpxArQS2XdvkaevRax");
    }
}

macro_rules! generate_pyth_lazer_mappings {
    (
        const $array_name:ident: [ $( ($feed_id:expr, $market_index:expr) ),* $(,)? ];
    ) => {
        pub const $array_name: &[(u32, u16)] = &[
            $( ($feed_id, $market_index), )*
        ];

        /// Map from pyth lazer `feed_id `to mainnet perp market index
        pub const fn pyth_lazer_feed_id_to_perp_market_index(feed_id: u32) -> Option<u16> {
            match feed_id {
                $(
                    $feed_id => Some($market_index),
                )*
                _ => None,
            }
        }

        /// Map from mainnet perp market index to pyth lazer `feed_id`
        pub const fn perp_market_index_to_pyth_lazer_feed_id(market_index: u16) -> Option<u32> {
            match market_index {
                $(
                    $market_index => Some($feed_id),
                )*
                _ => None,
            }
        }
    };
}

// DEV: append new markets to this list (feed_id, market_index)
generate_pyth_lazer_mappings! {
    const PYTH_LAZER_FEED_ID_TO_PERP_MARKET_MAINNET: [
        (6, 0),
        (1, 1),
        (2, 2),
        (28, 3),
        (9, 4),
        (32, 5),
        (37, 6),
        (13, 7),
        (15, 8),
        (11, 9),
        (4, 10),
        (41, 11),
        (34, 12),
        (14, 13),
        (168, 14),
        (46, 15),
        (19, 16),
        (3, 18),
        (48, 19),
        (91, 20),
        (51, 21),
        (18, 22),
        (10, 23),
        (92, 24),
        (83, 25),
        (36, 26),
        (102, 27),
        (99, 29),
        (130, 34),
        (12, 42),
        (137, 51),
        (77, 55),
        (54, 56),
        (110, 59),
        (26, 60),
        (93, 61),
        (97, 62),
        (171, 63),
        (203, 64),
        (145, 65),
        (308, 66),
        (306, 69),
        (309, 70),
        (182, 71),
        (16, 72),
        (163, 73),
        (1578, 75),
    ];
}
