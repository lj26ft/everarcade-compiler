# EverArcade Production Readiness Assessment

Canonical assessment date: 2026-06-03  
Assessment basis: repository state only; no credit was given for intended plans, TODOs, empty interfaces, or reports unsupported by implemented code/tests. Renderer, history, and federation are intentionally treated as scaffold-level runtime domains unless concrete runtime behavior exists.

## Executive production readiness score

```text
Deterministic Execution Core ..... 86
State Persistence ................ 72
Federation Runtime ............... 45
World Runtime .................... 62
Renderer Runtime ................. 38
Deployment Infrastructure ........ 54
Evernode Readiness ............... 42
XRPL/Xahau Integration ........... 24
Developer Experience ............. 46
Operational Readiness ............ 34

Overall Production Readiness ..... 50 / 100
Classification ................... Experimental Deployment Candidate
Commercial Readiness ............. Not Yet Commercially Ready
External Developer Readiness ..... Developer Preview Only
```

## Classification rubric

| Classification | Applied meaning in this assessment |
| --- | --- |
| Production Ready | Implemented, tested, recoverable, and validated through reproducible operational evidence. |
| Deployment Ready | Functional and recoverable, but still requires operational hardening and live-environment proof. |
| Experimental | Functional slices exist, but validation is incomplete, synthetic, narrow, or not yet tied to an operator workflow. |
| Scaffold | Architecture, data structures, commands, manifests, or reports exist, but production runtime behavior is absent or mostly simulated. |
| Planned | Design-only material with no implemented runtime behavior found. |

## Evidence inventory reviewed

- Core runtime crates and modules: `execution-core/`, `everarcade-host/`, `runtime/`, `contract-api/`, `contracts/`, `sdk/`, and `src-bin-everarcade/`.
- Operator/deployment assets: `scripts/`, `deploy/`, `deployment/evernode/`, `deployment/reports/`, `docs/operators/`, and `docs/runbooks/`.
- Architecture and planning docs: `docs/architecture/`, `docs/runtime/`, `docs/release/`, and roadmap/gap docs.
- Test inventory found by file count: 281 execution-core test files, 150 everarcade-host test files, 2 SDK test files, no renderer-client test files, and no CLI test files under the reviewed paths.
- Report/script inventory found by file count: 355 deployment report files and 251 `run_*validation*.sh` scripts. These were treated as evidence only where scripts execute concrete checks or tests; plain generated reports and placeholder scripts did not increase readiness scores.

## Category 1: Deterministic Execution Core

**Score: 86 / 100**  
**Classification: Deployment Ready**

### What is implemented

- WASM execution via Wasmtime with deterministic configuration, fuel accounting, disabled nondeterministic-ish feature classes such as SIMD, relaxed SIMD, tail calls, multi-memory, and threads.
- ABI encode/decode boundary for VM input/output and an `everarcade_execute` entrypoint path in the WASM runtime.
- Execution receipts containing pre/post state roots, input/output hashes, diff hash, events hash, fuel used, execution id, and WASM hash.
- Canonical hashing modules, ABI validation modules, checkpoint modules, replay modules, receipt modules, journal modules, and extensive targeted test inventory.
- Checkpoint restoration validates snapshot root before returning encoded state.
- Release and Evernode gate scripts run targeted execution-core tests for deployment paths.

### Partially implemented

- Cross-machine reproducibility evidence exists as certification artifacts and tests, but the core does not yet provide a single public command that reproduces a fresh-machine proof from a released package without repository context.
- Receipts and roots are strong, but independent verifier ergonomics are still fragmented across modules and scripts.
- ABI stability is represented by versioned modules and tests, but a published compatibility policy/gate is not yet a standalone release artifact.

### Scaffolding or missing

- A minimal, canonical `everarcade verify <package> <replay>` operator/developer flow is not yet the authoritative path.
- The current core surface is very broad, making it hard to separate stable production API from internal modules.

### Answers

- **Can execution be reproduced on a fresh machine?** Mostly, if the repository, vendored dependencies, and targeted scripts are available. Not yet as a polished binary/package-only proof.
- **Can execution be verified independently?** Partially. Receipts, roots, hashes, and replay tests exist; external verifier packaging and documentation remain gaps.
- **Can execution survive recovery?** Yes for tested checkpoint/replay paths; still needs longer-duration and real deployment recovery proof.

### Blocking issues

1. Publish a canonical verifier CLI and artifact format for independent verification.
2. Reduce public runtime API to a stable facade and mark internal modules.
3. Add release gates that prove replay/checkpoint equivalence from built artifacts, not just source-tree tests.

## Category 2: State Persistence

**Score: 72 / 100**  
**Classification: Experimental**

### What is implemented

- Host persistence writes receipts, journal entries, and checkpoints into separate directories under a world root.
- Writes use temp-file plus rename semantics for JSON records.
- Journal verification checks sequence continuity and previous-entry hashes.
- World verification checks journal consistency and receipt presence.
- Replay restoration returns the latest checkpoint state root.
- Dedicated persistence, checkpoint, snapshot, recovery, receipt-store, and state-folder tests exist in `everarcade-host/tests/` and `execution-core/tests/`.

### Partially implemented

- Corruption detection exists for missing receipts and broken journal continuity, but durability is limited to filesystem JSON records without fsync, lock discipline, compaction, migration, or crash-consistency proof across interrupted writes.
- Checkpoint durability is represented by records and validation tests, but backup/restore and archive integrity are not integrated into a single operator workflow.
- Archive integrity and historical replay validation are represented by renderer/history modules and reports; those domains remain scaffold-level for production scoring.

### Scaffolding or missing

- No production store abstraction with versioned schema migration, atomic multi-record transactions, retention/compaction, encrypted-at-rest option, or cloud/off-host backup target.
- No continuously scheduled restore rehearsal with a machine-readable pass/fail status used by deployment gates.

### Answers

- **Can a world survive host restart?** For local tested state-folder/checkpoint scenarios, yes. For production host lifecycle with service manager and real crash timing, not yet proven.
- **Can corruption be detected?** Partially: journal sequence/hash and missing receipt checks exist; broad corruption matrix coverage is incomplete for production storage.
- **Can state be restored from checkpoints?** Yes in targeted code/tests; not yet operationalized as a full operator recovery runbook with automated verification.

### Blocking issues

1. Add durable persistence guarantees: fsync, file locking, schema versioning, migration, and crash-interruption tests.
2. Make restore rehearsal a deployment gate.
3. Define backup/archive retention and integrity verification.

## Category 3: Federation Runtime

**Score: 45 / 100**  
**Classification: Scaffold**

### What is implemented

- Federation modules exist for topology state, peer continuity state, sync engine, transport framing, divergence comparison, checkpoint sync, replay verification, reconciliation, and leases.
- Topology state can upsert sorted peers and track active/stale/diverged status.
- Sync code can compare continuity bundles, advance a peer checkpoint, request/apply journal ranges, and reject non-contiguous journal entries.
- Host modules exist for network messages, TCP transport, peer registry, convergence, checkpoint sync, replay sync, partition recovery, and distributed receipts.
- Many host/federation tests and validation scripts exist, including local/multinode/federation certification scripts.

### Partially implemented

- Local deterministic convergence and message-shape validation appear exercised, but federation behavior is still not proven as a live, multi-host, long-duration runtime with process restarts and storage faults.
- Topology management is data-structure-level and test-harness-level more than operator-deployed runtime orchestration.
- Failure and recovery are represented by modules and reports, but evidence is mostly local/synthetic.

### Scaffolding or missing

- Production peer discovery, authentication policy, rate limits, live transport soak testing, split-brain handling, and operator-controlled topology changes are not production-proven.
- Federation should not be marketed as production-ready for public or paid customers yet.

### Answers

- **Can multiple runtimes coordinate?** Locally and in test harnesses, partially. Production multi-node coordination remains unproven.
- **Can divergence be detected?** Yes at root/journal/continuity comparison level; live operational response remains incomplete.
- **Can failed peers recover?** Partially in tests and scaffolds; not yet proven on real distributed leases.

### Blocking issues

1. Run and publish a repeatable two-/four-node cross-machine federation certification.
2. Add live peer auth, partition, restart, replay catch-up, and stale-peer recovery gates.
3. Tie federation health to operator dashboards and incident workflows.

## Category 4: World Runtime

**Score: 62 / 100**  
**Classification: Experimental**

### What is implemented

- Persistent world runtime can create genesis state, tick deterministically from an input root, append replay history, validate continuity, and restore from checkpoint/replay-restored state.
- Scheduler modules implement ordered ticks, queued events, event continuity checks, state-root derivation, timeline replay, convergence comparison, and entity lineage reconstruction.
- Economy runtime provides deterministic append-only ledger transfers and continuity roots.
- Inventory runtime provides ownership records, transfers, and continuity roots.
- Governance, civilization, faction, society, ecology, procedural world, and autonomous recovery modules exist with targeted validation reports/tests.

### Partially implemented

- Scheduler/economy/inventory are deterministic kernels, not a full hosted persistent world service with real player sessions, tick loop supervision, backpressure, live input queues, and operator-controlled maintenance.
- Governance/economy/world modules contain useful domain models but do not yet form a commercial live-game operation surface.
- Partitioning and migration have deterministic primitives, but not proven high-load runtime behavior.

### Scaffolding or missing

- No canonical continuously running world daemon is proven as the v0.1 product path.
- No paid-customer-grade game lifecycle: tenant provisioning, player identity/session auth, quota enforcement, live operations, moderation/abuse tooling, or billing-safe settlement.

### Answers

- **Can a persistent world operate continuously?** Not proven. Deterministic tick and recovery primitives exist, but continuous operation must be validated.
- **Can world state evolve deterministically?** Yes for implemented primitives and tests.

### Blocking issues

1. Build a stateful runtime host around the deterministic world loop.
2. Connect session input, tick scheduling, replay, checkpoints, and persistence in one recoverable service.
3. Add long-running soak, restart, and restore validation.

## Category 5: Renderer Runtime

**Score: 38 / 100**  
**Classification: Scaffold**

### What is implemented

- `runtime/renderer-client` includes deterministic projection helpers, textual world/HUD/event/inventory renderers, replay transport wire messages, historical replay structs, archive/provenance validators, and observer-style recovery primitives.
- Replay window wire messages can validate reconstruction-only mode, chunk order, bounds, and deterministic wire hashes.
- Renderer code consistently represents renderer/observer as non-authoritative.

### Partially implemented

- Historical replay, projection archive, projection stream, and observer recovery models exist, but there are no renderer-client test files in the reviewed test inventory.
- Projection validation is mostly library/data-structure-level, not an end-to-end renderer reconnect/recover proof.

### Scaffolding or missing

- No production renderer service, browser/websocket frame stream, missed-frame catch-up, subscription protocol, asset pipeline integration, or observer recovery runbook is proven.
- Renderer/history/federation should remain explicitly marked scaffold-level runtime domains.

### Answers

- **Can renderers recover from missed frames?** Not production-proven. Some replay window/catch-up primitives exist.
- **Can projections be validated?** Partially at data-structure/hash level, not as an operational renderer pipeline.

### Blocking issues

1. Add renderer-client test suite for replay window, catch-up, archive corruption, and observer recovery.
2. Define and implement one production projection stream protocol.
3. Validate missed-frame recovery against persisted replay/checkpoint data.

## Category 6: Deployment Infrastructure

**Score: 54 / 100**  
**Classification: Experimental**

### What is implemented

- Scripts exist for install, bootstrap, release packaging, release validation, fresh-VM validation, Evernode package generation, local clusters, recovery, Linux VM smoke/stress, and many domain-specific validations.
- Systemd service template and operator config templates exist.
- Release validation script performs offline cargo check/build for host release path.
- Evernode gate script generates deterministic packages, verifies tarball hashes/signatures/receipts, and runs targeted execution-core Evernode deployment tests.
- Operator docs exist for install, bootstrap, deploy, recover, rollback, and upgrade.

### Partially implemented

- There is a large deployment script surface, but some scripts are placeholders or synthetic report generators.
- Fresh VM bootstrap exists as a script/documentation concept, but current assessment did not find a single verified, self-contained operator workflow that starts from an empty VM and results in a running recoverable world.
- Runtime update and rollback docs exist, but upgrade safety is not proven by a complete migration/restart/restore gate.

### Scaffolding or missing

- No signed public release artifact consumption path with `doctor`, `install`, `start`, `verify`, `backup`, `restore`, and `upgrade` all validated on a clean host.
- No container image, service health endpoint standard, package registry, or automatic rollback gate is production-certified.

### Answers

- **Can a new operator deploy EverArcade?** A repository-aware operator can run scripts locally; a new external operator cannot yet follow a polished production deployment path.
- **Can they upgrade safely?** Not proven.
- **Can they recover safely?** Partially, through targeted recovery scripts/docs; not yet a complete drill gate.

### Blocking issues

1. Define one canonical install/deploy/recover workflow and remove/label placeholder scripts.
2. Validate on a clean VM with no repository assumptions except signed artifacts.
3. Add service lifecycle, health, backup, restore, and rollback gates.

## Category 7: Evernode Readiness

**Score: 42 / 100**  
**Classification: Scaffold**

### What is implemented

- Evernode manifests exist for runtime, world, deployment, package, and operator metadata.
- Package-generation and gate scripts verify generated package hashes/signatures/receipts locally.
- Operations shell supports deploy/start/stop/restart/recover/verify as local deterministic simulation output.
- Existing readiness report explicitly states single/two-node local readiness is partial and live Evernode deployment is not included.

### Partially implemented

- Resource and capacity notes exist, along with Evernode sizing/config documents.
- Local package/deployment gate is useful as a precursor to lease deployment.

### Scaffolding or missing

- No evidence of a live Evernode lease deployment, lease lifecycle management, production persistence behavior on lease storage, resource exhaustion drills, or lease recovery proof.
- Runtime lifecycle commands are simulation-level, not a provider-integrated deployment client.

### Answers

- **Can EverArcade run reliably on an Evernode lease today?** Not proven by this repository.
- **What is still required?** Live lease deployment, persistence mapping, service lifecycle, resource limits, backup/restore, restart recovery, and operator documentation tied to real lease behavior.

### Blocking issues

1. Run a real Evernode lease trial and capture evidence.
2. Replace simulation operations with provider-backed deployment commands.
3. Validate lease restart, disk pressure, package upgrade, checkpoint restore, and replay verification.

## Category 8: XRPL/Xahau Integration

**Score: 24 / 100**  
**Classification: Scaffold**

### What is implemented

- Data structures exist for XRPL accounts, anchors, hooks, inventory anchors, manifests, markets, ownership, payments, settlement receipts, vaults, world identity, and witnesses.
- Xahau settlement intent, hook invocation, proof anchor, settlement receipt, and deterministic proof-hash construction exist.
- Host Xahau gateway builds hook invocation parameters and settlement receipts from intents.
- XRPL publisher payloads can hash and verify record payloads.
- Tests exist for XRPL adapter/root anchor/anchor intent/testnet paths.

### Partially implemented

- Dry-run submission returns a deterministic `dry-run-tx` and non-dry-run returns a placeholder transaction string.
- `xrpl-live` feature-gated client/testnet hooks exist but are stubs.
- Ownership and asset lineage are represented as manifests/records, not live ledger-verified state transitions.

### Scaffolding or missing

- No live XRPL/Xahau transaction submission, key management, reliable idempotent settlement, ledger confirmation tracking, reorg/finality policy, or production reconciliation.
- No commercial-grade settlement, marketplace, or asset ownership verification workflow.

### Answers

- **What is implemented?** Deterministic anchoring payloads, settlement intent models, Xahau invocation assembly, dry-run/placeholder submitters, and test coverage around records.
- **What remains architecture-only?** Live settlement, ownership verification against ledger state, asset lineage enforcement, key/secret handling, retry/finality, and marketplace operation.

### Blocking issues

1. Keep live settlement disabled for v0.1 unless explicitly scoped as experimental.
2. Implement live submission adapter with secure key management and idempotent queueing.
3. Add ledger verification, retry/finality, and settlement recovery tests.

## Category 9: Developer Experience

**Score: 46 / 100**  
**Classification: Experimental**

### What is implemented

- SDK crates expose game, input, runtime, replay, session, state, validation, economy, entity, governance, simulation, and world hooks.
- SDK can load manifests, hash packages/assets, verify deterministic package equivalence, register assets, create local federation directories, replay local world logs, and expose developer runtime hooks.
- CLI supports many developer/creator commands including new-game, run-dev, replay tools, validate/deploy game, package content, publish package, and runtime diagnostics.
- Developer docs and creator docs exist: quickstarts, SDK guide, game manifest guide, local federation guide, deployment guide, replay debugging, content packaging, and examples.

### Partially implemented

- Many CLI developer/creator commands emit JSON diagnostics or seed local files rather than performing full workflows.
- SDK hooks are useful but thin; templates/examples overlap and do not yet present one canonical external-developer path.
- CLI tests were not found in the reviewed inventory.

### Scaffolding or missing

- No single polished `everarcade new && everarcade run && everarcade replay-verify && everarcade package` path proven by tests and docs.
- No stable plugin/template registry, error catalog, public API stability promise, or onboarding smoke test from a clean user environment.

### Answers

- **Can a developer build a game without reading source code?** A motivated developer can start with docs/examples, but external onboarding is not yet self-service production quality.
- **Can they package and run locally?** Partially; package/hash utilities and CLI commands exist, but the workflow needs consolidation and tests.

### Blocking issues

1. Choose one canonical starter template and one canonical quickstart path.
2. Add CLI integration tests for create/run/replay/package.
3. Publish stable SDK facade and mark experimental modules.

## Category 10: Operational Readiness

**Score: 34 / 100**  
**Classification: Scaffold**

### What is implemented

- Operator docs exist for installation, bootstrap, deployment, recovery, rollback, and upgrade.
- Runbooks exist for checkpoint restore, machine recovery, machine rejoin, startup, and transport failure.
- Runtime/node modules include health, metrics, dashboard, watchdog, supervisor, daemon, lifecycle, recovery, and service state concepts.
- Observability reports identify metrics, logs, alerts, health states, recovery signals, replay growth, checkpoint age, lease capacity, and cost model requirements.

### Partially implemented

- Operational vocabulary and docs are ahead of production wiring.
- Some metrics/health tests and observability validation scripts exist, but real telemetry collection/export/alert routing is missing.
- Incident recovery workflows are documented but not tied to machine-readable state and deployment gates.

### Scaffolding or missing

- No production monitoring stack, log rotation/export, alert destinations, dashboards, SLOs, incident process, support bundle, backup automation, or disaster recovery drill evidence.
- No small-team operations checklist that proves unattended runtime operation.

### Answers

- **Could a small operations team run this platform?** Not yet without direct engineering support. They can follow draft docs for local/operator scenarios but not production operations.

### Blocking issues

1. Implement runtime health and metrics endpoints outside consensus authority.
2. Add backup/restore automation and scheduled restore drills.
3. Build incident runbooks linked to alerts and release gates.

## What is genuinely production-capable today?

- Deterministic core primitives: WASM execution, ABI serialization, receipts, roots, fuel-limited deterministic configuration, checkpoint validation, and replay/checkpoint test harnesses.
- Local filesystem persistence primitives for receipts/journal/checkpoints with basic integrity checks.
- Deterministic package/hash/signature/receipt generation for local Evernode-style artifacts.
- Developer-preview SDK primitives and docs for deterministic local experimentation.

## What is experimental?

- Persistent world runtime as an integrated service.
- Host restart recovery as a complete operator workflow.
- Local federation/convergence/recovery simulations and test harnesses.
- Deployment scripts and release gates.
- Developer CLI workflows.

## What is scaffold?

- Renderer/history runtime.
- Production federation.
- Evernode live lease deployment.
- XRPL/Xahau live integration and settlement.
- Operations/observability.
- Commercial marketplace/economy operations.

## What must be built before v0.1?

1. One stateful runtime host that runs a deterministic world continuously.
2. Durable persistence with restart and restore verification.
3. One canonical CLI/operator deployment path.
4. Clean-VM or clean-lease validation from signed/generated artifacts.
5. Recovery gate proving checkpoint restore and replay equivalence.
6. Developer quickstart that creates, runs, verifies, and packages one game.
7. Explicit labels for scaffold/experimental commands and docs.

## What must be built before paid customers?

1. Production operations: monitoring, logs, alerts, backups, restore drills, incident response.
2. Tenant/game lifecycle management, quotas, auth, and abuse controls.
3. Supportable upgrade/rollback process.
4. Security hardening for untrusted packages, package limits, sandbox policy, and dependency/release provenance.
5. Commercial settlement must remain disabled or dry-run until XRPL/Xahau live integration is production proven.

## What must be built before public launch?

1. Public installer/release artifacts with checksums/signatures and offline verification.
2. Stable SDK/API facade with compatibility policy.
3. Canonical documentation path and examples cleanup.
4. Renderer and replay observer path with tests.
5. Security disclosures, threat model, and public limitations.
6. Clear product promise: developer preview/local deterministic world first; federation/settlement/renderer advanced domains marked experimental or roadmap.

## Shortest path to a deployable Evernode-hosted deterministic world runtime

1. Freeze v0.1 scope to a **single deterministic world runtime** with local player/session input, replay, checkpoint, and restore.
2. Implement a service wrapper that owns: package load, world tick loop, append-only replay, periodic checkpoint, receipt/journal persistence, health status, and graceful shutdown.
3. Add a storage contract for Evernode lease paths and validate restart from persisted world root.
4. Convert `deployment/evernode/operations.sh` from simulation output to real package install/start/stop/restart/recover/verify actions.
5. Run a live Evernode lease trial: deploy, tick, checkpoint, kill/restart, restore, verify replay, package upgrade, rollback.
6. Publish the evidence as a release gate and block v0.1 until it passes.
