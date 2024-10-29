<div align="center">
  <img height="120x" src="https://uploads-ssl.webflow.com/611580035ad59b20437eb024/616f97a42f5637c4517d0193_Logo%20(1)%20(1).png" />

  <h1 style="margin-top:20px;">drift-rs</h1>

  <p>
    <a href="https://crates.io/crates/drift-rs"><img alt="Crates.io" src="https://img.shields.io/crates/v/drift-rs.img" /></a>
    <a href="https://docs.drift.trade/developer-resources/sdk-documentation"><img alt="Docs" src="https://img.shields.io/badge/docs-tutorials-blueviolet" /></a>
    <a href="https://discord.com/channels/849494028176588802/878700556904980500"><img alt="Discord Chat" src="https://img.shields.io/discord/889577356681945098?color=blueviolet" /></a>
    <a href="https://opensource.org/licenses/Apache-2.0"><img alt="License" src="https://img.shields.io/github/license/project-serum/anchor?color=blueviolet" /></a>
  </p>
</div>

# drift-rs

Experimental, high performance Rust SDK for building offchain clients for [Drift V2](https://github.com/drift-labs/protocol-v2) protocol.


## Install
```toml
# crates.io*
drift-rs = "1.0.0-alpha.3"

# build from source (also builds and links 'libdrift_ffi_sys')
drift-rs = { git = "https://github.com/drift-labs/drift-rs", tag = "v1.0.0-alpha.3" }
```

_*_`drift-rs` uses drift program over ffi.
ensure [libdrift_ffi_sys](https://github.com/drift-labs/drift-ffi-sys/blob/master/README.md#installation) is installed when using via crates.io.

## Use
```rust
use drift_rs::{DriftClient, Wallet};
use solana_sdk::signature::KeyPair;

async fn main() {
    let client = DriftClient::new(
        Context::MainNet,
        RpcClient::new("https://rpc-provider.com"),
        KeyPair::new().into(),
    )
    .await
    .expect("connects");

    /// Subscribe to Ws-based live prices, blockhashes, and oracle updates
    let markets = [MarketId::spot(1), MarketId::perp(0)];
    client.subscribe_markets(&markets).await.unwrap();
}
```
## Setup

### Mac (m-series)

Install rosetta and configure Rust toolchain for `x86_64`

```bash
softwareupdate --install-rosetta
# replace `1.81.0` with preferred stable version
rustup install 1.81.0-x86_64-apple-darwin
rustup override set 1.81.0-x86_64-apple-darwin
```

⚠️ the default toolchain is incompatible due to memory layout differences between solana program (BPF) and aarch64 and will fail at runtime with deserialization errors like: `InvalidSize`.

## Local Development
drift-rs links to the drift program crate via FFI, build from source or optionally install from [drift-ffi-sys](https://github.com/drift-labs/drift-ffi-sys/releases)
```bash
# Build from source
CARGO_DRIFT_FFI_STATIC=1
# Provide a prebuilt drift_ffi_sys lib 
CARGO_DRIFT_FFI_PATH="/path/to/libdrift_ffi_sys"
```
