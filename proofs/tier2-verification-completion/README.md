# EverArcade Tier-2 Verification Completion Pack v1

This directory is the clone-and-run proof artifact for independent Tier-2 reproduction. It contains a standalone Rust proof kernel, packaged fixtures, harness tests, certification notes, and local reports for replay, restore, migration, federation, and JS ↔ Rust kernel equivalence.

Verifier commands from this directory:

```bash
cargo test replay_harness -- --nocapture
cargo test restore_harness -- --nocapture
cargo test migration_harness -- --nocapture
cargo test federation_harness -- --nocapture
cargo test js_equivalence_harness -- --nocapture
cargo test -- --nocapture
```

The pack has no parent-repository imports, no monorepo imports, and no network dependency.
