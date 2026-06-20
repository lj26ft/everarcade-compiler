# Property Targets: governance.vote

## Safety Invariants

- GOV-SAFE-001 One Vote Per Eligible Voter: A voter cannot vote twice on the same proposal. verification_method = "property-test"; proof_status = "Property-Tested".
- GOV-SAFE-002 Eligibility: Only eligible voters count. verification_method = "property-test"; proof_status = "Property-Tested".
- GOV-SAFE-003 Tally Integrity: Final tally equals the sum of valid votes. verification_method = "property-test"; proof_status = "Property-Tested".
- GOV-SAFE-004 Temporal Bound: Votes after close are rejected. verification_method = "property-test"; proof_status = "Property-Tested".
- GOV-SAFE-005 Monotonic Tally: Valid tally updates cannot decrease or inflate incorrectly. verification_method = "property-test"; proof_status = "Property-Tested".

## Integrity Invariants

- INT-DET-001 Determinism: verification_method = "differential"; proof_status = "Differential-Tested".
- INT-REC-001 Receipt Integrity: verification_method = "differential"; proof_status = "Differential-Tested".
- INT-REP-001 Replay Equivalence: verification_method = "replay"; proof_status = "Differential-Tested".
- INT-ROOT-001 Root Equivalence: verification_method = "differential"; proof_status = "Differential-Tested".

Changing the implementation hash voids certification until re-certified.
