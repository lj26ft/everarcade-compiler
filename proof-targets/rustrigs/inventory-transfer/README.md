# Proof Target: inventory.transfer

- Source: `crates/rustrigs/inventory/src/lib.rs`
- State model: `InventoryState` / `State` alias
- Input model: `Input` alias
- Output/receipt model: `TransferReceipt` through `Output`
- Mutation function: `apply` and `inventory_transfer`
- Certification report: `reports/rustrig-suite/inventory-transfer-report.txt`

This folder is self-contained for verifier orientation and intentionally excludes `target/`, debug binaries, and compiled artifacts.
