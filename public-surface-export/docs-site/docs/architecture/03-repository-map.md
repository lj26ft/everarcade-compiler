# Repository Map

## Purpose

Inventory repository ownership so contributors know where to read and change code.

## Responsibilities

- Own the canonical description of repository map within the architecture book.
- Identify the Rust modules, reports, tests, and operational artifacts that support the subsystem.
- Explain how the subsystem participates in deterministic execution, receipt production, replay, recovery, and federation when applicable.
- Distinguish implemented foundations from scaffold-level or planned work.

## Non-Responsibilities

- It does not make renderer, observer, dashboard, or analytics data authoritative.
- It does not bypass canonical serialization, root comparison, receipt validation, or replay verification.
- It does not replace crate-level API documentation or source code review for implementation details.
- It does not claim production maturity for modules explicitly marked partial, scaffold, or planned.

## Internal Components

- Workspace crates.
- Runtime module trees.
- Deployment, world, wasm, and security reports.
- Test suites and test vectors.

## Data Flow

Input
→ validation at the subsystem boundary
→ deterministic processing by the owning runtime module
→ state mutation only through canonical state or subsystem-specific ledgers
→ receipt, root, checkpoint, projection, or synchronization artifact generation
→ verification by replay, hash comparison, signature checks, continuity validation, or reconciliation.

```text
Deterministic Execution Pipeline

User Input
↓
Host Runtime
↓
WASM Guest or Native Fixture Executor
↓
State Diff
↓
Canonical State
↓
State Root
↓
Receipt
↓
Journal / Store
↓
Replay Verification
```

Where federation or projection participates, the deterministic receipt remains the source of truth and the downstream artifact is verified against the authoritative roots.

## Determinism Guarantees

- Hashing strategy: state roots, execution roots, receipt hashes, checkpoint roots, and continuity roots are compared as stable digests rather than inferred from wall-clock behavior.
- Canonical serialization: protocol objects are serialized through repository-owned ABI/codec/canonical boundaries before hashing or replay comparison.
- Replay guarantees: a verifier can rerun the same input, prior state, package, and protocol epoch to recompute roots and receipts.
- Validation rules: invalid signatures, malformed bundles, root mismatches, missing checkpoint ancestry, or non-contiguous replay windows are rejected or quarantined.

## Failure Modes

- Corruption: detected by hash, signature, manifest, checkpoint, archive, or proof mismatch.
- Divergence: detected by comparing state roots, execution roots, receipt hashes, checkpoint roots, projection roots, or continuity records.
- Recovery: uses checkpoints, receipt ranges, replay windows, archive hydration, rollback plans, and peer resynchronization.
- Reconciliation: selects canonical material, suspends unsafe advancement, repairs gaps, and resumes continuity only after validation.

## Future Evolution

Regenerate crate and module sections as Cargo workspace membership changes.


### Architecture Notes

- Map workspace crates and major non-crate trees.
- Record maturity levels and validation surfaces.
- Show how reports, tests, and scripts support architecture claims.
- It is not a generated file index.
- It does not classify vendored or dependency code.

## Complete Workspace Crate Inventory

| Crate | Purpose | Status / Maturity | Major modules | Tests | Validation scripts / reports |
|---|---|---|---|---:|---|
| `everarcade-abi` | Shared ABI types for execution plans, receipts, VM input/output, state, and serialization. | Implemented foundation. | `lib` | 0 crate integration tests | ABI docs and protocol consumers validate it indirectly. |
| `execution-core` | Deterministic protocol runtime and canonical execution foundation. | Production foundation with partial higher-level domains. | `state_engine`, `receipt_runtime`, `replay_runtime`, `checkpoint`, `lineage`, `federation_runtime`, `world_runtime`, `world_scheduler`, `economy_runtime`, `governance_runtime`, `inventory_runtime`, `world_partition`, `wasm_abi` | 276 | `wasm/reports`, `world/reports`, `benchmarks/reports`, `test_vectors` |
| `contract-api` | Contract-facing API and protocol records for deterministic packages/rustrigs. | Implemented foundation; registry validation partial. | `abi_v1`, `registry_validation`, `protocol_records`, `rustrig` | 0 | Contract examples and JSON/ABI docs. |
| `control-plane` | Provider/control-plane abstractions for deployment, registry, alerts, topology, leases, and API. | Partial / scaffold. | `deployment`, `provider`, `registry`, `api`, `alerts`, `topology`, `leases` | 0 | Deployment readiness reports. |
| `provider-evernode` | Evernode provider adapter, deployment, runtime, lease, topology, upgrade, rollback. | Partial. | `deployment`, `adapter`, `rollback`, `runtime`, `lease`, `topology`, `upgrade` | 0 | Evernode readiness/deployment reports. |
| `rustrigs` | Reusable gameplay/domain packages for AI, economy, inventory, world, quests, XRPL, and related domains. | Partial content scaffold. | `ai`, `diplomacy`, `movement`, `crafting`, `interaction`, `dialogue`, `deployment`, `quests`, plus domain folders | 0 | Rustrig reports and package validation reports. |
| `contracts/set-contract` | Minimal deterministic set-state example contract. | Implemented example. | `lib` | 0 | Contract hash/test vectors indirectly. |
| `contracts/increment-contract` | Minimal deterministic increment example contract. | Implemented example. | `lib` | 0 | Contract examples. |
| `contracts/counter-world` | Counter world example. | Implemented example / tutorial. | `lib` | 0 | Example docs. |
| `contracts/counter-contract` | Counter contract example. | Implemented example. | `lib` | 0 | Example docs. |
| `everarcade-host` | Host runtime for packages, receipts, persistence, federation, recovery, Evernode, signing, query, archive, distributed execution, and operator flows. | Production foundation with partial operational domains. | `runner`, `package_loader`, `receipt_store`, `checkpoint_store`, `federation_network`, `federation_transport`, `distributed_sync`, `recovery`, `evernode`, `signing`, `security`, `node` | 150 | Host integration tests, deployment reports, runbooks. |
| `sdk/everarcade-sdk` | Primary developer SDK for game/session/input/state/runtime abstractions. | Partial but usable for onboarding. | `game`, `input`, `session`, `state`, `runtime`, `validation` | 1 | SDK docs and validation report. |
| `sdk/everarcade-world-sdk` | World authoring SDK. | Partial scaffold. | `lib` | 0 | SDK docs. |
| `sdk/everarcade-entity-sdk` | Entity authoring SDK. | Partial scaffold. | `lib` | 0 | Entity reports. |
| `sdk/everarcade-simulation-sdk` | Simulation authoring SDK. | Partial scaffold. | `lib` | 0 | Simulation workflow reports. |
| `sdk/everarcade-economy-sdk` | Economy SDK. | Partial scaffold. | `lib` | 0 | Economy reports. |
| `sdk/everarcade-governance-sdk` | Governance SDK. | Partial scaffold. | `lib` | 0 | Governance reports. |
| `sdk/client-bridge` | Client bridge API for runtime/client integration. | Partial. | `lib` | 0 | Client bridge docs. |
| `src-bin-everarcade` | Product CLI entry point and commands. | Partial / operator-facing. | `main`, `config`, `product`, `runtime_snapshot`, `commands` | 0 | CLI quickstart and product command reports. |
| `runtime/client` | Local sovereign runtime client shell with projection service, playback, console, world/inventory views. | Scaffold-level client runtime. | `projection_service`, `render_tick`, `world_view`, `renderer`, `playback`, `console`, `event_view` | 0 | Client README and projection docs. |
| `runtime/renderer-client` | Renderer/projection/history/federation client runtime. | Scaffold-level runtime domain. | `runtime`, `world_renderer`, `playback_renderer`, `event_renderer`, `hud`, `federation`, `persistence`, `history`, `transport_runtime` | 0 | Projection, observer, replay transport, and history reports. |
| `tools` | Creator/operator tooling: package certification, creator pipeline, vertical slice certification, plus tool subtrees. | Partial toolchain. | `package_certification`, `creator_pipeline`, `creator_productization`, `vertical_slice_certification` | 1 | Tools READMEs and creator reports. |
| `runtime/content-registry` | Content registry package/manifest/signature validation runtime. | Partial. | `package`, `manifest`, `signature`, `validation`, `runtime` | 0 | Content registry/package reports. |
| `studio-gui` | GUI/editor surface for gameplay authoring, viewport, marketplace, terrain, assets, replay, and workspace. | Scaffold-level product UI. | `app`, `window`, `layout`, `workspace`, `gameplay_authoring`, `world_authoring`, `viewport`, `marketplace`, `terrain`, `assets` | 0 | Studio/editor/authoring reports. |

## Non-Crate Inventory

- `runtime/`: operational runtime modules for nodes, routing, WAN recovery, storage fabric, autoscaling, security, simulation/civilization federation, observer civilization, and packages. These are partial/scaffold operational domains supporting deployment and recovery research.
- `deployment/reports/`: historical milestone, validation, launch, readiness, runtime, federation, projection, release, Evernode, SDK, creator, and operations reports. They are evidence, not independent authority.
- `world/reports/`: world simulation, projection, replay, restoration, scheduling, partition, performance, and continuity reports.
- `wasm/reports/`: WASM runtime, fuel metering, isolation, checkpoint restore, replay, and stateful execution validation reports.
- `security/reports/`: capability, isolation, resource governance, quarantine, abuse, and runtime overflow validation reports.
- `docs/runbooks/`: machine startup/recovery/rejoin, checkpoint restore, rollback, and transport failure procedures.
- `test_vectors/`: binary fixtures for receipts, snapshots, replay consensus, distributed execution, verifier sync, proof markets, archival continuity, entity identity, and lineage.
- `examples/` and `templates/`: onboarding games/worlds for deterministic arena, RTS, persistent world, cooperative session, governance world, and simulation worlds.

## Repository Evidence Commands

The crate inventory was generated from `Cargo.toml`, crate `src` folders, and crate `tests` folders using targeted filesystem inspection rather than a workspace rebuild.
