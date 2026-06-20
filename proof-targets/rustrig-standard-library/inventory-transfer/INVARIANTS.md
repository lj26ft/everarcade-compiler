# Invariants: inventory.transfer

Implementation hash: `sha256:c10d328a689095e84dc088be8ce04b0b6892390f09d277f38b4154d473e52d35`

If the implementation hash changes, certification is void until re-certified.

## Safety Invariants

### INV-SAFE-001 Conservation

- category: Safety
- description: Sum of item quantity across all inventories is unchanged.
- severity: Critical
- verification_method: property-test
- proof_status: Property-Tested
- artifact_hash_binding: sha256:c10d328a689095e84dc088be8ce04b0b6892390f09d277f38b4154d473e52d35

### INV-SAFE-002 No Overdraw

- category: Safety
- description: Source inventory cannot transfer more than it holds.
- severity: Critical
- verification_method: property-test
- proof_status: Property-Tested
- artifact_hash_binding: sha256:c10d328a689095e84dc088be8ce04b0b6892390f09d277f38b4154d473e52d35

### INV-SAFE-003 Owner Authorization

- category: Safety
- description: Only the holder/controller may transfer owned items.
- severity: Critical
- verification_method: property-test
- proof_status: Property-Tested
- artifact_hash_binding: sha256:c10d328a689095e84dc088be8ce04b0b6892390f09d277f38b4154d473e52d35

### INV-SAFE-004 Atomic Reject

- category: Safety
- description: Rejected transfers leave world state unchanged.
- severity: Critical
- verification_method: property-test
- proof_status: Property-Tested
- artifact_hash_binding: sha256:c10d328a689095e84dc088be8ce04b0b6892390f09d277f38b4154d473e52d35

## Integrity Invariants

### INT-DET-001 Determinism

- category: Determinism
- description: Same input and pre-state always produce the same post-state and receipt.
- severity: High
- verification_method: differential
- proof_status: Differential-Tested
- artifact_hash_binding: sha256:c10d328a689095e84dc088be8ce04b0b6892390f09d277f38b4154d473e52d35

### INT-REC-001 Receipt Integrity

- category: Receipt
- description: Receipt commits to accepted/rejected status and mutation effects.
- severity: High
- verification_method: differential
- proof_status: Differential-Tested
- artifact_hash_binding: sha256:c10d328a689095e84dc088be8ce04b0b6892390f09d277f38b4154d473e52d35

### INT-REP-001 Replay Equivalence

- category: Replay
- description: Replay over the same ordered inputs produces equivalent state transitions.
- severity: High
- verification_method: differential
- proof_status: Differential-Tested
- artifact_hash_binding: sha256:c10d328a689095e84dc088be8ce04b0b6892390f09d277f38b4154d473e52d35

### INT-ROOT-001 Root Equivalence

- category: Root
- description: Post-state root equals the canonical root computed from resulting state.
- severity: High
- verification_method: differential
- proof_status: Differential-Tested
- artifact_hash_binding: sha256:c10d328a689095e84dc088be8ce04b0b6892390f09d277f38b4154d473e52d35
