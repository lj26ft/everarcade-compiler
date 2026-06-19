# Proof Target: governance.vote

- Source: `crates/rustrigs/governance/src/lib.rs`
- State model: `GovernanceState` / `State` alias
- Input model: `Input` alias
- Output/receipt model: `VoteReceipt` through `Output`
- Mutation function: `apply` and `governance_vote`
- Certification report: `reports/rustrig-suite/governance-vote-report.txt`

This folder is self-contained for verifier orientation and intentionally excludes `target/`, debug binaries, and compiled artifacts.
