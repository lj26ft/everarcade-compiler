# Federation Recovery

## Purpose

Detects divergence, selects recovery material, and restores continuity after peer or network failure.

## Responsibilities

- Own the canonical description of federation recovery within the architecture book.
- Identify the Rust modules, reports, tests, and operational artifacts that support the subsystem.
- Explain how the subsystem participates in deterministic execution, receipt production, replay, recovery, and federation when applicable.
- Distinguish implemented foundations from scaffold-level or planned work.

## Non-Responsibilities

- It does not make renderer, observer, dashboard, or analytics data authoritative.
- It does not bypass canonical serialization, root comparison, receipt validation, or replay verification.
- It does not replace crate-level API documentation or source code review for implementation details.
- It does not claim production maturity for modules explicitly marked partial, scaffold, or planned.

## Internal Components

- reconciliation_engine and divergence modules.
- host recovery, runtime_recovery, partition_recovery, peer_recovery.
- rollback, checkpoint_restore, replay_reconstruction.
- network partition and stale node recovery tests.

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

Partial; production runbooks exist but automated orchestration is still maturing.


```text
Federation Pipeline

Runtime A
↓
Receipt / Checkpoint / Replay Bundle
↓
Synchronization Transport
↓
Signature and Hash Verification
↓
Consensus Window
↓
Reconciliation Plan
↓
Continuity Verification
```
