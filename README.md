<div align="center">
  <img height="120x" src="https://uploads-ssl.webflow.com/611580035ad59b20437eb024/616f97a42f5637c4517d0193_Logo%20(1)%20(1).png" />

  <h1 style="margin-top:20px;">drift-rs</h1>

  <p>
    <a href="https://crates.io/crates/drift-sdk"><img alt="Crates.io" src="https://img.shields.io/crates/v/drift-sdk.img" /></a>
    <a href="https://docs.drift.trade/developer-resources/sdk-documentation"><img alt="Docs" src="https://img.shields.io/badge/docs-tutorials-blueviolet" /></a>
    <a href="https://discord.com/channels/849494028176588802/878700556904980500"><img alt="Discord Chat" src="https://img.shields.io/discord/889577356681945098?color=blueviolet" /></a>
    <a href="https://opensource.org/licenses/Apache-2.0"><img alt="License" src="https://img.shields.io/github/license/project-serum/anchor?color=blueviolet" /></a>
  </p>
</div>

# drift-rs

Experimental, high performance Rust SDK for building off chain clients for interacting with the [Drift V2](https://github.com/drift-labs/protocol-v2) protocol.

## Setup

### Mac (m-series)

Install rosetta and configure build for `x86_64`

```bash
softwareupdate --install-rosetta
# replace `1.81.0` with preferred stable version
rustup install 1.81.0-x86_64-apple-darwin
rustup override set 1.81.0-x86_64-apple-darwin
```

the native build is incompatible due to memory layout differences between solana program (BPF) and aarch64 and will fail at runtime with errors like `InvalidSize`.

## Build
```bash
# Provide a prebuilt drift_ffi_sys lib 
CARGO_DRIFT_FFI_PATH=/"path/to/libdrift_ffi_sys"
# Build from source (default)
CARGO_DRIFT_FFI_STATIC=1
```
