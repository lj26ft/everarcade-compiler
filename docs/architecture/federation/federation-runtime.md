# Federation Runtime

## Purpose

Coordinates multiple runtimes without making rendering or UI authoritative.

## Responsibilities

- Own the canonical description of federation runtime within the architecture book.
- Identify the Rust modules, reports, tests, and operational artifacts that support the subsystem.
- Explain how the subsystem participates in deterministic execution, receipt production, replay, recovery, and federation when applicable.
- Distinguish implemented foundations from scaffold-level or planned work.

## Non-Responsibilities

- It does not make renderer, observer, dashboard, or analytics data authoritative.
- It does not bypass canonical serialization, root comparison, receipt validation, or replay verification.
- It does not replace crate-level API documentation or source code review for implementation details.
- It does not claim production maturity for modules explicitly marked partial, scaffold, or planned.

## Internal Components

- execution-core federation_runtime.
- everarcade-host federation_network/security/transport.
- distributed_execution and distributed_sync.
- renderer-client federation scaffold modules.

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

Scaffold-to-operational: tests cover many paths, but production WAN operation needs hardening.


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
