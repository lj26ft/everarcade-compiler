# Tier-2 Verifier Walkthrough

A verifier can reproduce the completion pack without contacting project maintainers.

```bash
git clone <everarcade-tier2-proof-url>
cd everarcade-tier2-proof
cargo test -- --nocapture
```

Targeted checks:

```bash
cargo test replay_harness -- --nocapture
cargo test restore_harness -- --nocapture
cargo test migration_harness -- --nocapture
cargo test federation_harness -- --nocapture
cargo test js_equivalence_harness -- --nocapture
```

Expected success markers:

- `REPLAY VERIFIED`
- `RESTORE VERIFIED`
- `MIGRATION VERIFIED`
- `FEDERATION VERIFIED`
- `JS KERNEL EQUIVALENCE VERIFIED`

The proof pack is self-contained under `proofs/tier2-verification-completion/` and packages `kernel/`, `fixtures/`, `harnesses/`, `certification/`, and `rustrig-standard-library/`.

`Cargo.lock` is committed with the proof crate. Dependency graph pinned for verification reproducibility.
