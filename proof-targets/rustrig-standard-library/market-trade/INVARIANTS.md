# Invariants: market.trade

Implementation hash: `sha256:b058dc38c631eee352906d00ce0fd1b150c3a9e36c49ebfe3daf62b4454301dc`

If the implementation hash changes, certification is void until re-certified.

## Safety Invariants

### MARKET-SAFE-001 Value Conservation

- category: Safety
- description: No value is minted or destroyed by trade.
- severity: Critical
- verification_method: property-test
- proof_status: Property-Tested
- artifact_hash_binding: sha256:b058dc38c631eee352906d00ce0fd1b150c3a9e36c49ebfe3daf62b4454301dc

### MARKET-SAFE-002 No Double Spend

- category: Safety
- description: Listing/item cannot be sold twice.
- severity: Critical
- verification_method: property-test
- proof_status: Property-Tested
- artifact_hash_binding: sha256:b058dc38c631eee352906d00ce0fd1b150c3a9e36c49ebfe3daf62b4454301dc

### MARKET-SAFE-003 Atomic Settlement

- category: Safety
- description: Both legs settle or neither does.
- severity: Critical
- verification_method: property-test
- proof_status: Property-Tested
- artifact_hash_binding: sha256:b058dc38c631eee352906d00ce0fd1b150c3a9e36c49ebfe3daf62b4454301dc

### MARKET-SAFE-004 Seller Ownership

- category: Safety
- description: Seller must own listed item.
- severity: Critical
- verification_method: property-test
- proof_status: Property-Tested
- artifact_hash_binding: sha256:b058dc38c631eee352906d00ce0fd1b150c3a9e36c49ebfe3daf62b4454301dc

### MARKET-SAFE-005 Price Bounds

- category: Safety
- description: Price must be non-negative and valid.
- severity: Critical
- verification_method: property-test
- proof_status: Property-Tested
- artifact_hash_binding: sha256:b058dc38c631eee352906d00ce0fd1b150c3a9e36c49ebfe3daf62b4454301dc

### MARKET-SAFE-006 Fee Bounds

- category: Safety
- description: Fee must remain within configured bounds.
- severity: Critical
- verification_method: property-test
- proof_status: Property-Tested
- artifact_hash_binding: sha256:b058dc38c631eee352906d00ce0fd1b150c3a9e36c49ebfe3daf62b4454301dc

## Integrity Invariants

### INT-DET-001 Determinism

- category: Determinism
- description: Same input and pre-state always produce the same post-state and receipt.
- severity: High
- verification_method: differential
- proof_status: Differential-Tested
- artifact_hash_binding: sha256:b058dc38c631eee352906d00ce0fd1b150c3a9e36c49ebfe3daf62b4454301dc

### INT-REC-001 Receipt Integrity

- category: Receipt
- description: Receipt commits to accepted/rejected status and mutation effects.
- severity: High
- verification_method: differential
- proof_status: Differential-Tested
- artifact_hash_binding: sha256:b058dc38c631eee352906d00ce0fd1b150c3a9e36c49ebfe3daf62b4454301dc

### INT-REP-001 Replay Equivalence

- category: Replay
- description: Replay over the same ordered inputs produces equivalent state transitions.
- severity: High
- verification_method: differential
- proof_status: Differential-Tested
- artifact_hash_binding: sha256:b058dc38c631eee352906d00ce0fd1b150c3a9e36c49ebfe3daf62b4454301dc

### INT-ROOT-001 Root Equivalence

- category: Root
- description: Post-state root equals the canonical root computed from resulting state.
- severity: High
- verification_method: differential
- proof_status: Differential-Tested
- artifact_hash_binding: sha256:b058dc38c631eee352906d00ce0fd1b150c3a9e36c49ebfe3daf62b4454301dc
