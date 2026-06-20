# Property Targets: inventory.transfer

## Safety Invariants

- INV-SAFE-001 Conservation: Sum of item quantity across all inventories is unchanged. verification_method = "property-test"; proof_status = "Property-Tested".
- INV-SAFE-002 No Overdraw: Source inventory cannot transfer more than it holds. verification_method = "property-test"; proof_status = "Property-Tested".
- INV-SAFE-003 Owner Authorization: Only the holder/controller may transfer owned items. verification_method = "property-test"; proof_status = "Property-Tested".
- INV-SAFE-004 Atomic Reject: Rejected transfers leave world state unchanged. verification_method = "property-test"; proof_status = "Property-Tested".

## Integrity Invariants

- INT-DET-001 Determinism: verification_method = "differential"; proof_status = "Differential-Tested".
- INT-REC-001 Receipt Integrity: verification_method = "differential"; proof_status = "Differential-Tested".
- INT-REP-001 Replay Equivalence: verification_method = "replay"; proof_status = "Differential-Tested".
- INT-ROOT-001 Root Equivalence: verification_method = "differential"; proof_status = "Differential-Tested".

Changing the implementation hash voids certification until re-certified.
