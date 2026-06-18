# Proof Mapping Framework v1

Status: **PASS**

This document is the canonical bridge from EverArcade protocol invariants to certification evidence, canonicalizer-kernel code targets, and future formal proofs. It exists so a verifier can trace every proof target without reading the entire repository.

## 1. Verification Boundary

The EverArcade verification boundary is:

```text
ArenaState
  ↓ canonicalize()
Canonical Bytes
  ↓ SHA256(canonical bytes)
State Root
  ↓ SHA256(state_root || receipt_root || continuity_root)
World Hash
```

This is the protocol trust boundary.

All certifications and future proofs derive from this boundary. Runtime, replay, federation, restore, migration, and auditor workflows are protocol-valid only when they preserve this boundary and produce the same canonical bytes, state roots, and world hashes for equivalent worlds.

## 2. Invariant Registry

### INV-001 — Canonicalizer Determinism

**Statement:**

```text
canonical(state) = canonical(state)
```

for all valid `ArenaState` values.

**Meaning:** Identical valid state values must serialize to identical canonical bytes every time, independent of process, platform, input map insertion order, or caller.

### INV-002 — State Root Integrity

**Statement:**

```text
state_root = SHA256(canonical(state))
```

**Meaning:** A state root is valid only when it is the SHA-256 digest of the canonical bytes for the corresponding `ArenaState`.

### INV-003 — World Hash Integrity

**Statement:**

```text
world_hash = SHA256(state_root || receipt_root || continuity_root)
```

**Meaning:** A world hash is valid only when it commits, in order, to the state root, receipt root, and continuity root.

### INV-004 — Replay Equivalence

**Statement:**

```text
live execution = replayed execution
```

**Meaning:** Replaying the same accepted inputs from the same genesis/checkpoint material must produce roots equivalent to live execution.

### INV-005 — Federation Equivalence

**Statement:**

```text
all honest operators produce identical roots
```

**Meaning:** Honest operators running the same protocol inputs and certification material must converge on identical roots for the same world state.

### INV-006 — Restore Equivalence

**Statement:**

```text
restored world = original world
```

**Meaning:** Restoring from checkpoint and continuity evidence must recreate the same committed world state as the original source.

### INV-007 — Migration Equivalence

**Statement:**

```text
migrated world = source world
```

**Meaning:** Migration must preserve the committed world state across runtime, operator, or hosting boundary changes.

### INV-008 — JS ↔ Kernel Equivalence

**Statement:**

```text
JS canonicalizer = kernel canonicalizer
```

**Meaning:** JavaScript/runtime canonicalization and the standalone canonicalizer kernel must agree on canonical bytes, state roots, and world hashes for shared fixtures and valid boundary states.

## 3. Certification Mapping

| Invariant | Certification evidence | Status | Artifact |
| --- | --- | --- | --- |
| INV-001 Canonicalizer Determinism | Canonicalizer kernel fixture certification; deterministic canonical JSON serialization checks. | PASS | `crates/canonicalizer-kernel/tests/fixtures.rs`; `reports/root-integrity/canonical-bytes.hex` |
| INV-002 State Root Integrity | Root integrity certification for canonical bytes to state root. | PASS | `reports/root-integrity/state-root.json`; `crates/canonicalizer-kernel/tests/fixtures.rs` |
| INV-003 World Hash Integrity | Ordered world hash certification over state, receipt, and continuity roots. | PASS | `reports/root-integrity/world-hash.json`; `crates/canonicalizer-kernel/tests/fixtures.rs` |
| INV-004 Replay Equivalence | Replay certification comparing live and replay roots. | PASS | `reports/replay/`; `reports/root-integrity/replay-verification.json`; `reports/hotpocket-live/replay-report.json` |
| INV-005 Federation Equivalence | HotPocket live multi-node root comparison and federation scaffold certification. | PARTIALLY CERTIFIED | `reports/hotpocket-live/root-comparison.json`; `reports/hotpocket-live/cluster-report.md`; `docs/06-federation-runtime.md` |
| INV-006 Restore Equivalence | Checkpoint integrity, restoration integrity, and operator recovery evidence. | PARTIALLY CERTIFIED | `reports/runtime_checkpoint_report.json`; `docs/security/checkpoint_integrity.md`; `docs/security/restoration_integrity.md`; `docs/operator-recovery.md` |
| INV-007 Migration Equivalence | Migration and operator recovery roadmap evidence; scaffold-level migration equivalence target. | NOT STARTED | `docs/repo-reorganization-plan.md`; `docs/operator-recovery.md`; `docs/sovereign-world-persistence.md` |
| INV-008 JS ↔ Kernel Equivalence | Canonicalizer kernel handoff fixture certification and kernel CLI equivalence targets. | CERTIFIED | `crates/canonicalizer-kernel/src/lib.rs`; `crates/canonicalizer-kernel/src/bin/canonicalizer-kernel-cli.rs`; `crates/canonicalizer-kernel/tests/fixtures.rs` |

## 4. Proof Target Mapping

| Invariant | Future proof | Kernel target | Proof status |
| --- | --- | --- | --- |
| INV-001 | Canonicalizer Determinism Proof | `canonicalize()` | Not Started |
| INV-002 | Commitment Integrity Proof | `state_root()` | Ready |
| INV-003 | Ordered World Commitment Proof | `world_hash()` | Ready |
| INV-004 | Replay Equivalence Proof | `canonicalize()`, `state_root()`, replay transcript verifier | Partially Certified |
| INV-005 | Federation Equivalence Proof | `state_root()`, `world_hash()`, federation root comparator | Not Started |
| INV-006 | Restore Equivalence Proof | `canonicalize()`, `state_root()`, checkpoint restore verifier | Partially Certified |
| INV-007 | Migration Equivalence Proof | `canonicalize()`, `state_root()`, migration verifier | Not Started |
| INV-008 | Kernel Equivalence Proof | `canonicalize()`, `state_root()`, `world_hash()` | Certified |

## 5. Dependency Graph

```text
INV-001 Canonicalizer Determinism
  ↓
INV-002 State Root Integrity
  ↓
INV-003 World Hash Integrity
  ↓
INV-004 Replay Equivalence
  ↓
INV-005 Federation Equivalence
  ├─↓
  │ INV-006 Restore Equivalence
  └─↓
    INV-007 Migration Equivalence

INV-008 JS ↔ Kernel Equivalence cross-checks INV-001, INV-002, and INV-003,
then supplies implementation equivalence evidence for INV-004 through INV-007.
```

Exact dependency chain:

1. `INV-001` must hold before any root can be trusted, because roots are commitments to canonical bytes.
2. `INV-002` depends on `INV-001`, because state-root integrity is meaningful only for deterministic canonical bytes.
3. `INV-003` depends on `INV-002`, because the world hash includes the state root as its first ordered component.
4. `INV-004` depends on `INV-001` through `INV-003`, because replay equivalence is root equivalence between live and replayed executions.
5. `INV-005` depends on `INV-004`, because honest operators can compare roots only if replayed/live execution produces stable commitments.
6. `INV-006` depends on `INV-003` and `INV-004`, because restore equivalence requires matching world commitments after checkpoint reconstruction and replay validation.
7. `INV-007` depends on `INV-003`, `INV-004`, and `INV-006`, because migration must preserve world commitments and must be recoverable through restore evidence.
8. `INV-008` supports the full chain by ensuring the runtime-facing JavaScript boundary and formal-verification-facing kernel boundary agree on the same commitments.

## 6. Proof Readiness Assessment

| Invariant | Readiness | Rationale |
| --- | --- | --- |
| INV-001 | PROOF READY | Kernel target is small and isolated; fixture evidence exists, but a formal determinism proof is still future work. |
| INV-002 | PROOF READY | `state_root()` is a direct hash over `canonicalize()` bytes. |
| INV-003 | PROOF READY | `world_hash()` is an ordered hash over three roots and has a focused kernel target. |
| INV-004 | PARTIALLY CERTIFIED | Replay evidence exists; full proof needs a transcript semantics model. |
| INV-005 | PARTIALLY CERTIFIED | Multi-node evidence exists; full proof needs an honest-operator model and federation semantics. |
| INV-006 | PARTIALLY CERTIFIED | Checkpoint/restore evidence exists; full proof needs a restore semantics model. |
| INV-007 | NOT STARTED | Migration is currently a roadmap proof target and needs canonical migration fixtures. |
| INV-008 | CERTIFIED | Kernel fixtures and CLI expose canonical bytes, state roots, and world hashes for equivalence checking. |

## 7. External Proof Targets

| External consumer | Intended invariants | Expected use |
| --- | --- | --- |
| HugeGreenCandle | INV-001, INV-002, INV-003, INV-008 | Consume the canonicalizer kernel boundary, fixture evidence, and root formulas as the first formal proof package. |
| Future auditors | INV-001 through INV-008 | Trace certification artifacts to code targets and determine which claims are certified, partially certified, proof ready, or not started. |
| Formal verification systems | INV-001, INV-002, INV-003, INV-004, INV-008 | Model canonical serialization, hash commitments, replay equivalence, and kernel/runtime equivalence. |
| Recursive proof systems | INV-002, INV-003, INV-004, INV-005, INV-006, INV-007 | Prove incremental state commitments, world-hash chaining, replay windows, federation convergence, restore continuity, and migration preservation. |

## Verifier Checklist

A verifier can answer the success criteria as follows:

- **Protocol invariants:** Section 2 lists `INV-001` through `INV-008`.
- **Supporting certifications:** Section 3 maps each invariant to status and artifacts.
- **Implementing code:** Section 4 names kernel targets; Section 3 names code artifacts.
- **Future proofs:** Section 4 maps each invariant to a future proof obligation and proof status.

**Result:** `PROOF MAPPING FRAMEWORK V1: PASS`

### INV-009 — Duplicate Identity Rejection

No invalid duplicate-identity ArenaState may produce canonical bytes or roots. Identity-bearing arrays must contain unique IDs for `players.player_id`, `entities.entity_id`, `positions.entity_id`, and `health.entity_id`; duplicate identifiers are invalid state and must be rejected before canonicalization.

| Invariant | Evidence | Status | Artifact |
| --- | --- | --- | --- |
| INV-009 Duplicate Identity Rejection | Kernel validation rejects duplicate player, entity, position, and health IDs before canonicalization; JS equivalence validation applies the same rule. | CERTIFIED | `crates/canonicalizer-kernel/tests/fixtures.rs`; `runtime/arena_hotpocket/validation/certify-js-kernel-equivalence.js`; `reports/tier2-proof-harness/duplicate-id-gap.txt` |
