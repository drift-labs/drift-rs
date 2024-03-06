# drift-rs

## Mac (m-series)

Install rosetta and configure build for x86

```bash
softwareupdate --install-rosetta
rustup install 1.76.0-x86_64-apple-darwin
rustup override set 1.76.0-x86_64-apple-darwin
```

the native build is incompatible due to memory layout differences between solana program (BPF) and aarch64 and will fail at runtime with errors like `InvalidSize`.
