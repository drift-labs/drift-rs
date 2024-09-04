TODO:

- IDL generated types
- build.rs script to build FFI lib and main SDK at sametime

```bash
RUSTFLAGS="-L $PWD/../drift-ffi/target/release/ -l dylib=drift_ffi" cargo test -- --show-output
```