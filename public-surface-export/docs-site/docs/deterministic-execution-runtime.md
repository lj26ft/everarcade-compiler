# Deterministic Execution Runtime Hardening

This document defines the hardened deterministic execution boundary in `execution-core`.

## Model

- Canonical execution state uses `CanonicalExecutionState` with `BTreeMap` ordering and stable bincode/JSON encoding.
- State roots are computed from canonical ordered key/value traversal (`canonical_state_root`).
- Canonical receipts are deterministic protocol objects (`CanonicalExecutionReceipt`) with stable hashing scope.
- Execution journals are append-only and hashable (`ExecutionJournal`).

## Determinism guarantees

- No wall-clock inputs in canonical objects.
- Deterministic ordering via `BTreeMap` and monotonic journal sequence.
- Stable SHA-256 hashing over canonical binary serialization.

## Validation flow

1. `scripts/verify_execution_equivalence.sh`
2. `scripts/verify_execution_replay.sh`
3. `scripts/validate_deterministic_execution_runtime.sh`

## Current limitations

- Cross-machine verification currently relies on matching Rust toolchain and dependency versions.
- Runtime host boundary hardening is scoped to existing deterministic Wasmtime configuration and fuel accounting.
