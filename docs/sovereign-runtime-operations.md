# Sovereign Runtime Operations Layer

This document defines deterministic sovereign runtime operational primitives for deployment, lifecycle, scaling, continuity, diagnostics, archival, recovery, scheduling, XRPL/Evernode semantics, and SDK tooling.

## Scope

This layer provides deterministic protocol objects and hashable manifests for sovereign runtime operations.

This is **not** production cloud orchestration and does not implement Kubernetes or cloud-vendor control planes.

## Operations Modules

- `operations::appliance`: deterministic VM appliance manifests with runtime, topology, replay, checkpoint, orchestration, and persistence roots.
- `operations::lifecycle`: append-only lifecycle journal transitions (bootstrap, activation, upgrade, migration, pause, recovery, archival).
- `operations::sharding`: deterministic partition/topology manifests and shard lineage hash.
- `operations::operator`: replay-safe continuity diagnostics and manifest verification helpers.
- `operations::archive_ops`: archive rotation continuity manifest primitives.
- `operations::recovery_ops`: deterministic recovery manifests across checkpoint/replay/topology/settlement/archive restoration.
- `operations::diagnostics`: replay divergence and continuity diagnostic manifests.
- `operations::scheduler`: deterministic execution windows and tick ordering manifests.
- `operations::evernode`: deterministic XRPL/Evernode deployment integration semantics.
- `operations::sdk`: deterministic project/world authoring and deployment validation manifest primitives.

## Determinism Guarantees

- Canonical serialization via Rust data models + stable hash composition.
- Deterministic roots for equivalent inputs.
- Append-only lifecycle and continuity-oriented recovery lineage.
- Observability and diagnostics are replay-safe and non-mutating.

## Limitations

- Not production cloud orchestration.
- Trust assumption: operators provide honest input manifests and checkpoint material.
- Deployment topology assumption: topology roots are pre-defined deterministic boundaries.
- Recovery automation limitation: bounded to canonical manifest equivalence, not live infrastructure drift remediation.
- Replay diagnostics assumption: divergence detection is root-based and depends on canonical upstream hash inputs.
- XRPL/Evernode integration assumption: semantics only; no live validator or chain participation.
- Scheduling limitation: deterministic ordering is manifest-level and does not model non-deterministic host scheduling.
