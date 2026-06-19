# EverArcade Compiler Implementation Evidence Map

Export date: 2026-06-19

## Tier 1

Claim: Tier 1 canonical-document consistency gate exists for v0.1.
Evidence:
- `docs/14-v0.1-architecture-freeze.md`
- `docs/10-release-certification.md`
- `MATURITY.md`
Status: PASS

## Tier 2

Claim: Tier 2 adversarial claim-boundary gate exists and prevents unqualified production/commercial/live-settlement claims for v0.1.
Evidence:
- `docs/14-v0.1-architecture-freeze.md`
- `reports/formal-proof-target-v1/report.txt`
- `reports/proof-mapping/report.txt`
- `docs/proofs/proof-mapping-v1.md`
Status: PASS

## GAP-2 Closure

Claim: GAP-2 duplicate-ID validation is closed before canonicalization.
Evidence:
- `reports/tier2-proof-harness/duplicate-id-gap.txt`
- `crates/canonicalizer-kernel/src/lib.rs`
- `crates/canonicalizer-kernel/tests/fixtures.rs`
- `docs/proofs/canonicalizer-spec.md`
Status: PASS

## World Package Certification

Claim: World Package creation and verification evidence exists.
Evidence:
- `reports/world/world-package-report.json`
- `reports/world/world-verify-report.json`
- `reports/world-package-spec/report.txt`
- `docs/world-package/world-package-v0.1.md`
- `docs/world-package-spec-v1.md`
- `docs/world-package-certification.md`
Status: PASS

## RustRig Standard Library

Claim: RustRig Standard Library candidate set exists with registry and proof-target docs.
Evidence:
- `docs/rustrigs/rustrig-standard-library-v1.md`
- `proof-targets/rustrig-standard-library/registry.json`
- `proof-targets/rustrig-standard-library/START_HERE.md`
- `proof-targets/rustrig-standard-library/combat-attack/README.md`
- `proof-targets/rustrig-standard-library/inventory-transfer/README.md`
- `proof-targets/rustrig-standard-library/governance-vote/README.md`
Status: CANDIDATE SET PRESENT

## Arena Vanguard Demo

Claim: Arena Vanguard playable/projection demo artifacts exist.
Evidence:
- `templates/arena-vanguard/world.toml`
- `docs/arena-vanguard/runtime-flow.md`
- `docs/arena-vanguard/replay-verification.md`
- `reports/demo/arena-vanguard-projection-demo.md`
- `reports/demo/arena-vanguard-verify.json`
- `reports/demo/arena-vanguard-final-state.json`
- `scripts/run_arena_vanguard_playable_validation.sh`
Status: PASS WITH MANUAL TEST STILL IMMEDIATE PRIORITY

## Open Source Readiness

Claim: Open-source readiness materials exist and avoid production-readiness overclaiming.
Evidence:
- `docs/open-source/open-source-readiness.md`
- `docs/open-source/v0.1-public-release-readiness.md`
- `reports/open_source_readiness/open_source_readiness_report.md`
- `reports/open_source_readiness_report.txt`
- `MATURITY.md`
Status: PASS FOR READINESS DOCUMENTATION

## Architecture Freeze

Claim: v0.1 architecture is frozen as developer preview with explicit non-goals.
Evidence:
- `docs/14-v0.1-architecture-freeze.md`
- `docs/release/architecture-freeze-v0.1.md`
- `docs/architecture/roadmap/v0.1-roadmap.md`
Status: PASS

## Projection Runtime

Claim: Projection Runtime exists as a non-authoritative visual observer with replay/restore/migration/operator demonstration modes.
Evidence:
- `runtime/projection-runtime/README.md`
- `runtime/projection-runtime/src/runtime.mjs`
- `runtime/projection-runtime/src/certify.mjs`
- `runtime/projection-runtime/src/cli.mjs`
- `runtime/projection-runtime/public/index.html`
- `reports/renderer_runtime_certification_report.txt`
Status: PASS

## Public Demo

Claim: Public demo documentation exists for Arena Vanguard projection/dashboard recording flow.
Evidence:
- `docs/demo/public-demo-runbook.md`
- `docs/demo/public-demo-visual-spec.md`
- `docs/demo/public-demo-script.md`
- `reports/demo/arena-vanguard-projection-demo.md`
Status: DOCS PRESENT; HOSTING METHOD OPEN

## Replay / Restore / Migration

Claim: Replay, restore, and migration proof artifacts exist.
Evidence:
- `runtime/hotpocket-runtime-proof/reports/runtime_restore_report.txt`
- `runtime/hotpocket-migration-proof/reports/hotpocket_migration_validation_report.txt`
- `runtime/hotpocket-migration-proof/reports/equivalence_report.txt`
- `runtime/hotpocket-migration-proof/reports/restore_report.json`
- `runtime/hotpocket-migration-proof/reports/replay_after_migration_report.txt`
Status: PASS

## Federation Boundary

Claim: Federation evidence exists but remains scaffold-level for v0.1 production claims.
Evidence:
- `reports/federated_runtime_certification_report.txt`
- `docs/architecture/federation/federation-runtime.md`
- `docs/14-v0.1-architecture-freeze.md`
Status: SCAFFOLD-LEVEL / NOT PRODUCTION AUTHORITY

## World Contract

Claim: World Contract status is documented as a sovereignty/authority boundary.
Evidence:
- `docs/concepts/world-contract.md`
- `docs/world-contract-certification-framework-v1.md`
- `docs/world-contracts/index.md`
- `docs/architecture/diagrams/world-contract-architecture.md`
Status: SPEC/DOCS PRESENT

## World Registry

Claim: World Registry spec exists as discovery metadata, not runtime authority.
Evidence:
- `docs/world-registry-spec-v1.md`
Status: PASS

## RustRig Certified Set

Claim: Certified RustRig mutation set exists.
Evidence:
- `reports/rustrig-suite/suite-report.txt`
- `reports/rustrig-suite/combat-attack-report.txt`
- `reports/rustrig-suite/inventory-transfer-report.txt`
- `reports/rustrig-suite/market-trade-report.txt`
- `reports/rustrig-suite/governance-vote-report.txt`
Status: PASS
