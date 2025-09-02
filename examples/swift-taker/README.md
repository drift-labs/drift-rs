# Swift Example Taker

Example swift taker client.
Runs on devnet by default

## Run
Run on devnet
```shell
PRIVATE_KEY="<base58 private key>" RUST_LOG=swift=debug cargo run --release

# Deposit Trade
PRIVATE_KEY="<base58 private key>" RUST_LOG=swift=debug cargo run --release -- --deposit-trade

```

Run on mainnet
```shell
PRIVATE_KEY="<base58 private key>" MAINNET=1 RPC_URL="mainnet-rpc.example.com" RUST_LOG=swift=debug cargo run --release
```

alternatively use a `.env` file
