# Invariants: combat.attack

Implementation hash: `sha256:e643650694903152bf46db4c7c7efcf78b31920f8536941d6ad12891eaa213d1`

If the implementation hash changes, certification is void until re-certified.

## Safety Invariants

### COMBAT-SAFE-001 HP Bounds

- category: Safety
- description: Health remains in [0, max_hp].
- severity: Critical
- verification_method: property-test
- proof_status: Property-Tested
- artifact_hash_binding: sha256:e643650694903152bf46db4c7c7efcf78b31920f8536941d6ad12891eaa213d1

### COMBAT-SAFE-002 No Negative Damage

- category: Safety
- description: Damage cannot be negative.
- severity: Critical
- verification_method: property-test
- proof_status: Property-Tested
- artifact_hash_binding: sha256:e643650694903152bf46db4c7c7efcf78b31920f8536941d6ad12891eaa213d1

### COMBAT-SAFE-003 Bounded Damage

- category: Safety
- description: Damage must not exceed configured max damage.
- severity: Critical
- verification_method: property-test
- proof_status: Property-Tested
- artifact_hash_binding: sha256:e643650694903152bf46db4c7c7efcf78b31920f8536941d6ad12891eaa213d1

### COMBAT-SAFE-004 Death Monotonicity

- category: Safety
- description: Dead entities do not return alive through attack.
- severity: Critical
- verification_method: property-test
- proof_status: Property-Tested
- artifact_hash_binding: sha256:e643650694903152bf46db4c7c7efcf78b31920f8536941d6ad12891eaa213d1

### COMBAT-SAFE-005 Target Validity

- category: Safety
- description: Attacker and target must exist and be alive.
- severity: Critical
- verification_method: property-test
- proof_status: Property-Tested
- artifact_hash_binding: sha256:e643650694903152bf46db4c7c7efcf78b31920f8536941d6ad12891eaa213d1

### COMBAT-SAFE-006 Targeting Authority

- category: Safety
- description: Actor cannot mutate unauthorized entities.
- severity: Critical
- verification_method: property-test
- proof_status: Property-Tested
- artifact_hash_binding: sha256:e643650694903152bf46db4c7c7efcf78b31920f8536941d6ad12891eaa213d1

## Integrity Invariants

### INT-DET-001 Determinism

- category: Determinism
- description: Same input and pre-state always produce the same post-state and receipt.
- severity: High
- verification_method: differential
- proof_status: Differential-Tested
- artifact_hash_binding: sha256:e643650694903152bf46db4c7c7efcf78b31920f8536941d6ad12891eaa213d1

### INT-REC-001 Receipt Integrity

- category: Receipt
- description: Receipt commits to accepted/rejected status and mutation effects.
- severity: High
- verification_method: differential
- proof_status: Differential-Tested
- artifact_hash_binding: sha256:e643650694903152bf46db4c7c7efcf78b31920f8536941d6ad12891eaa213d1

### INT-REP-001 Replay Equivalence

- category: Replay
- description: Replay over the same ordered inputs produces equivalent state transitions.
- severity: High
- verification_method: differential
- proof_status: Differential-Tested
- artifact_hash_binding: sha256:e643650694903152bf46db4c7c7efcf78b31920f8536941d6ad12891eaa213d1

### INT-ROOT-001 Root Equivalence

- category: Root
- description: Post-state root equals the canonical root computed from resulting state.
- severity: High
- verification_method: differential
- proof_status: Differential-Tested
- artifact_hash_binding: sha256:e643650694903152bf46db4c7c7efcf78b31920f8536941d6ad12891eaa213d1
