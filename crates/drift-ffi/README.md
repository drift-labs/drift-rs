# drift-ffi

Exposes C-abi bindings for drift programs.  

Goals:
1) Enable building SDKs that reuse program logic.  
2) Allow rust SDK (drift-rs) to freely upgrade its solana-* crates to receive latest fixes and improvements.  

## Developer Notes
- this crate must be built with rust >= 1.77.0 to support [128-bit integer C-abi compatibility](https://blog.rust-lang.org/2024/03/30/i128-layout-update.html)

- for rust users this crate is intended to be linked via compiler flags (not Cargo dependency) as it compiles to a dynamic lib

- can ignore warnings spuriously warning about 128 integer types e.g. `extern` fn uses type `i128`, which is not FFI-safe` there is copmile time check to assert the layout is correct (if using rust >= 1.77.0)

## TODO:
- [] Check: while rust >=1.77.0 allows 128-bit integerC-abi compatibility the data from onchain program uses a layout <=1.77.0, so there will likely be zero-copy deserialization issues for types which include 128-bit integers...