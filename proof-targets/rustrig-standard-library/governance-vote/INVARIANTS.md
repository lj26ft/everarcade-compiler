# Invariants: governance.vote

Implementation hash: `sha256:82ff6ff8d4e9758bca138c04a3ec17ca7812b13cc72c6d5ba4a79ae4269dca67`

If the implementation hash changes, certification is void until re-certified.

## Safety Invariants

### GOV-SAFE-001 One Vote Per Eligible Voter

- category: Safety
- description: A voter cannot vote twice on the same proposal.
- severity: Critical
- verification_method: property-test
- proof_status: Property-Tested
- artifact_hash_binding: sha256:82ff6ff8d4e9758bca138c04a3ec17ca7812b13cc72c6d5ba4a79ae4269dca67

### GOV-SAFE-002 Eligibility

- category: Safety
- description: Only eligible voters count.
- severity: Critical
- verification_method: property-test
- proof_status: Property-Tested
- artifact_hash_binding: sha256:82ff6ff8d4e9758bca138c04a3ec17ca7812b13cc72c6d5ba4a79ae4269dca67

### GOV-SAFE-003 Tally Integrity

- category: Safety
- description: Final tally equals the sum of valid votes.
- severity: Critical
- verification_method: property-test
- proof_status: Property-Tested
- artifact_hash_binding: sha256:82ff6ff8d4e9758bca138c04a3ec17ca7812b13cc72c6d5ba4a79ae4269dca67

### GOV-SAFE-004 Temporal Bound

- category: Safety
- description: Votes after close are rejected.
- severity: Critical
- verification_method: property-test
- proof_status: Property-Tested
- artifact_hash_binding: sha256:82ff6ff8d4e9758bca138c04a3ec17ca7812b13cc72c6d5ba4a79ae4269dca67

### GOV-SAFE-005 Monotonic Tally

- category: Safety
- description: Valid tally updates cannot decrease or inflate incorrectly.
- severity: Critical
- verification_method: property-test
- proof_status: Property-Tested
- artifact_hash_binding: sha256:82ff6ff8d4e9758bca138c04a3ec17ca7812b13cc72c6d5ba4a79ae4269dca67

## Integrity Invariants

### INT-DET-001 Determinism

- category: Determinism
- description: Same input and pre-state always produce the same post-state and receipt.
- severity: High
- verification_method: differential
- proof_status: Differential-Tested
- artifact_hash_binding: sha256:82ff6ff8d4e9758bca138c04a3ec17ca7812b13cc72c6d5ba4a79ae4269dca67

### INT-REC-001 Receipt Integrity

- category: Receipt
- description: Receipt commits to accepted/rejected status and mutation effects.
- severity: High
- verification_method: differential
- proof_status: Differential-Tested
- artifact_hash_binding: sha256:82ff6ff8d4e9758bca138c04a3ec17ca7812b13cc72c6d5ba4a79ae4269dca67

### INT-REP-001 Replay Equivalence

- category: Replay
- description: Replay over the same ordered inputs produces equivalent state transitions.
- severity: High
- verification_method: differential
- proof_status: Differential-Tested
- artifact_hash_binding: sha256:82ff6ff8d4e9758bca138c04a3ec17ca7812b13cc72c6d5ba4a79ae4269dca67

### INT-ROOT-001 Root Equivalence

- category: Root
- description: Post-state root equals the canonical root computed from resulting state.
- severity: High
- verification_method: differential
- proof_status: Differential-Tested
- artifact_hash_binding: sha256:82ff6ff8d4e9758bca138c04a3ec17ca7812b13cc72c6d5ba4a79ae4269dca67
