# Proof Target: combat.attack

- Source: `crates/rustrigs/combat/src/lib.rs`
- State model: `CombatState` / `State` alias
- Input model: `Input` alias
- Output/receipt model: `CombatReceipt` through `Output`
- Mutation function: `apply` and `combat_attack`
- Certification report: `reports/rustrig-suite/combat-attack-report.txt`

This folder is self-contained for verifier orientation and intentionally excludes `target/`, debug binaries, and compiled artifacts.
