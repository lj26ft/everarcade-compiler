# Proof Target: market.trade

- Source: `crates/rustrigs/market/src/lib.rs`
- State model: `MarketState` / `State` alias
- Input model: `Input` alias
- Output/receipt model: `TradeReceipt` through `Output`
- Mutation function: `apply` and `market_trade`
- Certification report: `reports/rustrig-suite/market-trade-report.txt`

This folder is self-contained for verifier orientation and intentionally excludes `target/`, debug binaries, and compiled artifacts.
