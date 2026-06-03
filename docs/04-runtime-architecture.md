# 04. Runtime Architecture

This document teaches the EverArcade runtime from lifecycle to recovery. It is the authoritative runtime architecture document.

## Runtime Lifecycle

The runtime lifecycle is:

1. read configuration;
2. create or open runtime storage;
3. load package metadata and code;
4. initialize health and lifecycle records;
5. admit inputs;
6. execute deterministic ticks or commands;
7. persist journal entries and receipts;
8. create checkpoints;
9. verify replay and health;
10. recover, upgrade, stop, or restart as commanded.

## Package Loading

Package loading resolves the world identifier, package path, metadata, compatibility constraints, and execution material. A package is not authoritative by itself. It becomes part of authoritative history only after the runtime accepts it and binds execution evidence to runtime state.

## WASM Execution

WASM execution is the deterministic contract boundary. The runtime must provide controlled inputs, deterministic host functions, resource limits, canonical ABI behavior, and reproducible output handling. Any behavior that depends on local wall-clock time, host filesystem order, nondeterministic randomness, or renderer state must stay outside authoritative execution.

## State Mutation

State mutation occurs only after deterministic execution produces an accepted transition. The state engine applies the transition, updates canonical state bytes or structured state, and computes roots used by receipts, checkpoints, replay, and federation comparisons.

## Receipt Generation

Receipts are execution evidence. A receipt should identify the package, sequence, input root or hash, output root or hash, prior state root, resulting state root, execution status, and any lineage information required by replay and federation.

Receipts are not logs for convenience. They are protocol evidence and must remain stable across machines.

## Journal Persistence

The journal records ordered runtime history. A valid journal supports replay verification, recovery decisions, and operator audit. Journal persistence must be append-safe and versioned enough for upgrades. A corrupted journal is a runtime incident, not a display issue.

## Checkpoint Creation

A checkpoint is a durable restore point binding sequence, world identity, runtime version, state material, checkpoint hash, and package metadata. Checkpoints reduce recovery cost and bound replay windows. The runtime may create checkpoints on schedule, on operator command, or before upgrade.

## Recovery

Recovery chooses the smallest safe repair:

1. verify journal and checkpoint hashes;
2. select the latest valid checkpoint;
3. replay journal entries after that checkpoint;
4. compare resulting roots;
5. restore missing artifacts when available;
6. quarantine invalid or unverifiable material;
7. resume only after health is valid.

Recovery must never invent canonical state. It restores from verified checkpoints or recomputes from canonical inputs.

## Replay Verification

Replay verification recomputes execution history and compares expected roots. It answers whether a world history can be reproduced from its inputs, package, prior state, and protocol context. Replay failure indicates divergence or corruption and must block certification for the affected release or artifact.

## Upgrade Process

A runtime upgrade must preserve continuity:

1. stop input admission or enter maintenance;
2. create and verify a pre-upgrade checkpoint;
3. verify package and runtime artifacts;
4. apply migration only through declared upgrade paths;
5. restart with the new runtime version;
6. replay or verify the upgrade window;
7. retain rollback material until post-upgrade validation passes.

## Runtime Operator Commands

The runtime operator surface currently includes these command intents:

| Command | Purpose |
|---|---|
| `start` | Boot runtime storage, package, health, and loop. |
| `stop` | Stop the runtime safely. |
| `restart` | Stop and boot the runtime again. |
| `status` | Print persisted runtime health. |
| `verify` | Verify journal integrity. |
| `backup` | Create a checkpoint-backed backup manifest. |
| `restore` | Prepare restore flow; full production restore remains gated. |
| `replay-verify` | Verify replay/journal consistency. |
| `replay-report` | Produce a replay report. |
| `replay-root` | Print computed replay root. |
| `checkpoint` | Create an operator checkpoint. |
| `recover` | Run recovery manager and report result. |
| `doctor` | Run basic runtime health checks. |

The operations manual defines how operators use these commands.
