# EverArcade Compiler Current State Export

Export date: 2026-06-19

## Runtime Status

- v0.1 is a developer-preview architecture centered on a deterministic single-world runtime authority path: package loading, deterministic execution, state roots, receipts, journals, checkpoints, replay verification, recovery, and operator-visible status.
- Renderer, history, and federation are scaffold-level runtime domains for v0.1, not production authority domains.
- Evidence: `docs/14-v0.1-architecture-freeze.md`, `docs/10-release-certification.md`, `MATURITY.md`.

## Replay / Federation / Restore / Migration Certification

- Replay, restore, and migration evidence exists through HotPocket/runtime proof packages and generated reports.
- Federation is treated as scaffold-level/runtime validation evidence, not a production split-brain-safe authority claim.
- Evidence: `runtime/hotpocket-runtime-proof/reports/runtime_restore_report.txt`, `runtime/hotpocket-migration-proof/reports/hotpocket_migration_validation_report.txt`, `runtime/hotpocket-migration-proof/reports/equivalence_report.txt`, `reports/federated_runtime_certification_report.txt`, `docs/14-v0.1-architecture-freeze.md`.

## Canonicalizer Status

- Canonicalizer kernel exists and GAP-2 duplicate identity validation is closed before canonicalization.
- Invalid duplicate-identity ArenaState inputs produce deterministic validation errors and no canonical bytes, state root, or world hash.
- Evidence: `crates/canonicalizer-kernel/src/lib.rs`, `crates/canonicalizer-kernel/tests/fixtures.rs`, `docs/proofs/canonicalizer-spec.md`, `reports/tier2-proof-harness/duplicate-id-gap.txt`.

## Tier 2 Proof Status

- Tier 2 adversarial claim-boundary certification is documented in the architecture freeze and reported as part of the proof target package.
- Evidence: `docs/14-v0.1-architecture-freeze.md`, `reports/formal-proof-target-v1/report.txt`, `reports/proof-mapping/report.txt`, `docs/proofs/proof-mapping-v1.md`.

## GAP-2 Duplicate-ID Closure

- GAP-2 is closed with duplicate identity checks for players, entities, positions, and health before canonicalization.
- Evidence: `reports/tier2-proof-harness/duplicate-id-gap.txt`, `crates/canonicalizer-kernel/src/lib.rs`.

## World Contract Status

- World Contract is specified and documented as a sovereignty/authority boundary, with certification framework docs present.
- Evidence: `docs/concepts/world-contract.md`, `docs/world-contract-certification-framework-v1.md`, `docs/world-contracts/index.md`, `docs/architecture/diagrams/world-contract-architecture.md`.

## World Package Status

- World Package v0.1/v1 specifications and certification reports exist; deterministic package creation and verification reports pass for the example world.
- Evidence: `docs/world-package/world-package-v0.1.md`, `docs/world-package-spec-v1.md`, `docs/world-package-certification.md`, `reports/world/world-package-report.json`, `reports/world/world-verify-report.json`, `reports/world-package-spec/report.txt`.

## World Registry Status

- World Registry specification exists and explicitly scopes registry as discovery metadata, not runtime authority, marketplace, or host.
- Evidence: `docs/world-registry-spec-v1.md`.

## RustRig Certified Set

- Certified RustRig mutation suite includes inventory.transfer, combat.attack, market.trade, and governance.vote.
- Evidence: `reports/rustrig-suite/suite-report.txt`, `proof-targets/rustrigs/inventory-transfer/`, `proof-targets/rustrigs/combat-attack/`, `proof-targets/rustrigs/market-trade/`, `proof-targets/rustrigs/governance-vote/`.

## RustRig Standard Library Candidate Set

- Candidate RustRig Standard Library proof targets exist under `proof-targets/rustrig-standard-library/`, with registry and per-action invariant/property docs.
- Evidence: `docs/rustrigs/rustrig-standard-library-v1.md`, `proof-targets/rustrig-standard-library/registry.json`, `proof-targets/rustrig-standard-library/START_HERE.md`.

## Projection Runtime

- Projection Runtime v0.1 exists as a non-authoritative downstream visual observer with live, replay, restore, migration, and operator demonstration modes.
- Evidence: `runtime/projection-runtime/README.md`, `runtime/projection-runtime/src/runtime.mjs`, `runtime/projection-runtime/src/certify.mjs`, `runtime/projection-runtime/public/index.html`, `reports/renderer_runtime_certification_report.txt`.

## Arena Vanguard Playable Demo

- Arena Vanguard demo has runtime/projection artifacts, replay verification docs, and public demo runbook/reporting.
- Manual playable demo test and rough recording remain immediate priorities.
- Evidence: `templates/arena-vanguard/world.toml`, `docs/arena-vanguard/runtime-flow.md`, `docs/arena-vanguard/replay-verification.md`, `reports/demo/arena-vanguard-projection-demo.md`, `reports/demo/arena-vanguard-verify.json`, `scripts/run_arena_vanguard_playable_validation.sh`.

## Open-Source Readiness

- Open-source readiness docs and reports exist, including maturity classification and public release readiness. Current readiness is documentation/release-prep oriented, not a production readiness claim.
- Evidence: `docs/open-source/open-source-readiness.md`, `docs/open-source/v0.1-public-release-readiness.md`, `reports/open_source_readiness/open_source_readiness_report.md`, `reports/open_source_readiness_report.txt`, `MATURITY.md`.

## Explicit v0.1 Non-Goals

v0.1 does not claim:

- production federation or split-brain-safe multi-host authority;
- production renderer/history/federation authority;
- live XRPL/Xahau settlement or wallet-mediated production settlement;
- production wallet/vault custody;
- production marketplace, revenue, billing, trust, or moderation guarantees;
- public production testnet or hosted-service commitments;
- a 3D engine rewrite.

Evidence: `docs/14-v0.1-architecture-freeze.md`, `docs/architecture/roadmap/v0.1-roadmap.md`, `MATURITY.md`.
