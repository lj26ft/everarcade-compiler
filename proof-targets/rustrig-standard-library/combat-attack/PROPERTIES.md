# Property Targets: combat.attack

## Safety Invariants

- COMBAT-SAFE-001 HP Bounds: Health remains in [0, max_hp]. verification_method = "property-test"; proof_status = "Property-Tested".
- COMBAT-SAFE-002 No Negative Damage: Damage cannot be negative. verification_method = "property-test"; proof_status = "Property-Tested".
- COMBAT-SAFE-003 Bounded Damage: Damage must not exceed configured max damage. verification_method = "property-test"; proof_status = "Property-Tested".
- COMBAT-SAFE-004 Death Monotonicity: Dead entities do not return alive through attack. verification_method = "property-test"; proof_status = "Property-Tested".
- COMBAT-SAFE-005 Target Validity: Attacker and target must exist and be alive. verification_method = "property-test"; proof_status = "Property-Tested".
- COMBAT-SAFE-006 Targeting Authority: Actor cannot mutate unauthorized entities. verification_method = "property-test"; proof_status = "Property-Tested".

## Integrity Invariants

- INT-DET-001 Determinism: verification_method = "differential"; proof_status = "Differential-Tested".
- INT-REC-001 Receipt Integrity: verification_method = "differential"; proof_status = "Differential-Tested".
- INT-REP-001 Replay Equivalence: verification_method = "replay"; proof_status = "Differential-Tested".
- INT-ROOT-001 Root Equivalence: verification_method = "differential"; proof_status = "Differential-Tested".

Changing the implementation hash voids certification until re-certified.
