# Run

```bash
RUSTFLAGS="-L native=$PWD/../target/release/ -l dylib=drift_ffi" cargo test -- --show-output
```