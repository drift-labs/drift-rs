# drift-ffi

Exposes C-ABI bindings for drift program.

Goals:
1) Enable building SDKs that reuse program logic.  
2) Separate rust SDK from program/solana runtime dependencies. Allows rust SDK to freely update to latest solana-* crates with bug fixes, improvements, etc.

<img src='./architecture.png'/>

## Developer Notes
- this crate must be built with rust <= 1.76.0 to support [128-bit integer C-abi compatibility](https://blog.rust-lang.org/2024/03/30/i128-layout-update.html) to provide best compatibility with onchain data layouts

- for rust users this crate is intended to be linked via compiler flags (not Cargo dependency) as it compiles to a (platform dependent) dynamic lib (`.so/.dylib/.dll`)

- can ignore most of the warnings for FFI safety. The main issue are types containing `u128`/`i128`s which are handled by a custom `compat::u128/i128` type that forces correct alignment where required.