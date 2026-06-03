# Deterministic Physics Certification v0.1

## Purpose

Deterministic Physics Certification proves that the EverArcade Runtime Platform can execute an authoritative physics lifecycle with repeatable state roots across genesis, fixed-step simulation, checkpointing, restart, restoration, continued simulation, and replay.

This certification validates physics state evolution only. It does not validate rendering, GPU execution, client prediction, networking, federation, XRPL integration, or economic behavior.

## Deterministic Physics Requirements

Authoritative physics must remain:

- CPU deterministic.
- Replayable from the same genesis state and input stream.
- Checkpointable at a sealed physics root.
- Restorable without hidden live process state.
- Hashable into stable physics roots.

Authoritative physics must not depend on:

- GPU execution.
- Renderer state.
- Client prediction.
- Floating runtime ordering.
- Parallel nondeterminism.

If a future implementation uses Rapier for authoritative physics, authoritative builds must use enhanced determinism, fixed timesteps, stable insertion ordering, disabled parallel execution, and disabled SIMD where required for cross-run equivalence.

## Physics Root Model

The certification script constructs a deterministic scene containing:

- Static Ground.
- Rigid Body A.
- Rigid Body B.
- Rigid Body C.

The root material includes the certification version, CPU authority declaration, fixed timestep, disabled GPU/renderer authority, stable insertion order, and fixed-point body state. Each tick root is a SHA-256 hash of the canonical scene transcript plus canonical state for that tick.

The validated tick range is:

- Physics Genesis Root.
- Physics Root Tick 0.
- Physics Root Tick 1.
- Physics Root Tick 2.
- Physics Root Tick 3.
- Physics Root Tick 4.

Tick roots must evolve over time and an independent re-execution of the same fixed-step simulation must reproduce `Physics Root Tick 4` exactly.

## Checkpoint Model

After Tick 4, the certification creates a checkpoint record containing:

- Checkpoint version.
- Physics Checkpoint Identifier.
- Checkpoint tick.
- Persisted physics root.
- Genesis and tick roots needed to verify lineage.

The checkpoint identifier is derived from the canonical physics scene, checkpoint version, checkpoint tick, and persisted Tick 4 root. A checkpoint passes only if the identifier, tick, and persisted physics root verify exactly.

## Replay Model

Replay executes the complete lifecycle again:

```text
Genesis -> Simulation -> Checkpoint -> Restart -> Restore -> Continue Simulation
```

The replay lifecycle must reproduce the same continuity root as the primary lifecycle. Replay failure indicates hidden nondeterminism, incorrect checkpoint material, restoration drift, or unstable ordering.

## Authoritative vs Non-Authoritative Physics

Authoritative physics is the CPU deterministic state used for persistence, restoration, replay, and world continuity decisions.

Non-authoritative physics may exist for visual smoothing, client-side presentation, prediction experiments, or GPU acceleration, but it cannot seal canonical roots, create checkpoints, validate restoration, or decide persistent world state.

## GPU Boundary

GPU work is outside the authoritative boundary for this certification. GPU state, renderer state, and client presentation state are not included in physics roots. A GPU-accelerated path can be useful for non-authoritative visualization, but the canonical physics root must be produced from CPU deterministic state.

## PASS Criteria

The certification passes when:

1. Runtime bootstrap certification is available and passing.
2. Physics world genesis produces a stable root.
3. Fixed timestep simulation produces evolving Tick 0 through Tick 4 roots.
4. Independent re-execution reproduces the Tick 4 root.
5. Checkpoint creation and verification pass.
6. Restart and restoration recover the persisted physics root.
7. Continued simulation advances from the restored root.
8. Full lifecycle replay reproduces the same continuity root.
9. Ordering validation confirms identical insertion order, timestep, and input stream produce identical roots.

## FAIL Criteria

The certification fails if any required status is not `PASS`, including:

- Runtime bootstrap failure.
- Missing or unstable physics genesis.
- Tick roots that do not evolve.
- Independent simulation divergence.
- Checkpoint creation or verification failure.
- Restoration mismatch.
- Continuity root mismatch.
- Replay mismatch.
- Nondeterministic ordering or altered timestep/input stream.

## Relationship To Persistent World Certification

Persistent World Certification proves deterministic world continuity across long-lived world genesis, evolution, checkpointing, restoration, epoch transition, and replay.

Deterministic Physics Certification applies that same persistence and replay discipline to physics state. It narrows the authority boundary to CPU deterministic physics roots and proves the subsystem can become a candidate authoritative component for persistent worlds and future Rustrigs-based game execution.
