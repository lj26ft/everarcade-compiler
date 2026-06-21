# Evidence Map Update

| Claim | Implementation artifacts |
| --- | --- |
| Independent kernel reproduction | `proofs/tier2-verification-completion/kernel/src/lib.rs`; `proofs/tier2-verification-completion/fixtures/*.fixture`; `proofs/tier2-verification-completion/reports/final-report.txt` |
| Tier-2 completion pack | `proofs/tier2-verification-completion/README.md`; `proofs/tier2-verification-completion/Cargo.toml`; `proofs/tier2-verification-completion/Cargo.lock`; `proofs/tier2-verification-completion/harnesses/tier2_harnesses.rs`; `reports/tier2-verification-completion/*.txt` |
| Creator SDK validation run | `docs/creator-sdk-v1.1.md`; `creator-sdk/cli/everarcade.mjs`; `scripts/validate_developer_onboarding.sh`; `reports/developer_onboarding_validation_report.txt` when regenerated |
| World package certification | `creator-sdk/cli/everarcade.mjs`; generated `dist/certification/world-package-certificate.json`; `examples/reference-certified-world/README.md`; `examples/reference-certified-world-v1/certification/final-report.txt` |
| RustRig safety invariant declarations | `crates/rustrigs/combat/INVARIANTS.md`; `crates/rustrigs/inventory/INVARIANTS.md`; `crates/rustrigs/market/INVARIANTS.md`; `crates/rustrigs/governance/INVARIANTS.md`; matching `invariants.toml` files |
| Open source release candidate | `README.md`; `OPEN_SOURCE_READINESS.md`; `MATURITY.md`; `docs/open-source-launch/`; `docs/open-source/v0.1-public-release-readiness.md`; `scripts/validate_open_source_readiness.sh` |
