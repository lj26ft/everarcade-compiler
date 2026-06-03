# Implemented Features

## Purpose

Provide the verified inventory of implemented, partially implemented, scaffold, and planned subsystems. This is the canonical status matrix for contributor onboarding.

## Responsibilities

- Normalize older milestone reports into one status model.
- Identify duplicate or obsolete assumptions.
- Keep implementation claims tied to source modules, tests, reports, or runbooks.

## Non-Responsibilities

- It does not certify production readiness by itself.
- It does not replace release gates, operator runbooks, or security audits.

## Internal Components

- Implemented: code paths and tests exist for the foundation.
- Partially Implemented: code and reports exist, but production automation or full integration is incomplete.
- Stub: named modules or reports exist primarily as scaffolding.
- Planned: required architecture with no complete implementation.

## Data Flow

Input → repository evidence → status classification → roadmap action → release gate verification.

## Determinism Guarantees

Status is based on deterministic architecture evidence: source modules, test names, validation reports, fixtures, and runbooks. Claims should be downgraded when evidence is only narrative.

## Failure Modes

- Corruption: stale milestone language overstates maturity.
- Divergence: docs and source disagree.
- Recovery: update this matrix before release notes.
- Reconciliation: prefer current source and tests over historical reports.

## Future Evolution

Regenerate this matrix for each release candidate and link release gates to the rows below.

## Verified Inventory

| Subsystem | Status | Evidence | Missing / Gap |
|---|---|---|---|
| Execution core | Implemented | `execution-core/src/lib.rs`, `execute.rs`, ABI exports, test vectors | Harden package registry and persistence policies |
| WASM ABI | Implemented foundation | `alloc`, `execute`, `output_len`, wasm validation reports | Production sandbox policy and compatibility matrix |
| State engine | Implemented | `state_engine/{apply,store,merkle,proof,snapshot,history}` | Durable backend integration |
| Receipt system | Implemented foundation | ABI receipt, `receipt_runtime`, signed/distributed receipt tests | Operational publication and retention |
| Replay engine | Implemented foundation | `replay`, `replay_runtime`, host replay tests, replay reports | Long-horizon UX and archive scaling |
| Checkpoints | Implemented foundation | checkpoint modules, checkpoint sync, transfer/restore tests | Production storage and automated repair |
| Lineage / continuity | Implemented foundation | lineage, continuity, archive, compression modules | Governance of archival pruning |
| Federation runtime | Partially Implemented | federation runtime, host network/security/transport tests | WAN hardening, control-plane automation |
| Synchronization | Partially Implemented | distributed sync, receipt transfer, resumable sync tests | Production backpressure and peer policy |
| Recovery / reconciliation | Partially Implemented | recovery, reconciliation, partition recovery, runbooks | Automated operator-safe remediation |
| Topology | Partially Implemented | topology modules, peer registry, discovery tests | Dynamic production membership control |
| World runtime | Partially Implemented | world runtime, reports, examples | Stable public API and scale testing |
| Simulation | Partially Implemented | simulation modules, debugger, SDK | Full gameplay/product integration |
| Scheduler | Partially Implemented | world_scheduler, simulation_scheduler, reports | Load-adaptive production scheduling |
| Economy | Partially Implemented | economy runtime, economic tests, SDK | Real settlement integration and audit |
| Governance | Partially Implemented | governance runtime/security/sync, tests | Real-world operations and dispute policy |
| Inventory | Stub / partial | inventory runtime, renderer, SDK entries | Complete game-facing APIs |
| Partitioning | Partially Implemented | world_partition, distributed execution, failover tests | Production placement and migration policy |
| Renderer projection | Scaffold | runtime client and renderer-client modules, projection reports | Production frontend and transport |
| Historical replay UI | Scaffold / partial | renderer history modules, tools | Query UX and storage retention |
| Renderer federation | Scaffold | renderer federation modules and validation reports | Non-authoritative CDN/observer fabric |
| Runtime distribution | Partially Implemented | package modules, content registry, docs | Signed registry and upgrade policy |
| Release certification | Partially Implemented | many release reports and validation docs | Enforced CI gates |
| Evernode deployment | Partially Implemented | evernode modules, provider crate, docs | Provider-grade automation |
| Operational recovery | Partially Implemented | runbooks, recovery modules, tests | Fully automated safe workflows |
| SDK overview | Partially Implemented | SDK crates and docs | Versioned API stability |
| Game development | Partially Implemented | templates, examples, creator tools | Complete polished creator flow |
| Contract development | Implemented foundation | contract-api, ABI, example contracts | Registry, compatibility, and audit policy |

## Duplicate and Obsolete Material

- Multiple milestone reports describe the same concepts: replay validation, runtime federation, projection streams, release gates, and Evernode readiness. This book treats them as evidence, not separate sources of truth.
- Earlier names such as Arena Vanguard and sovereign runtime remain historical context; EverArcade architecture terms are canonical here.
- Renderer/history/federation client modules are intentionally classified as scaffold-level unless tied to authoritative host execution tests.
- Reports that state readiness without enforced tests are treated as roadmap evidence, not proof of production maturity.
