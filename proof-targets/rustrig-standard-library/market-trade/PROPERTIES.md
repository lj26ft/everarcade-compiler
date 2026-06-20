# Property Targets: market.trade

## Safety Invariants

- MARKET-SAFE-001 Value Conservation: No value is minted or destroyed by trade. verification_method = "property-test"; proof_status = "Property-Tested".
- MARKET-SAFE-002 No Double Spend: Listing/item cannot be sold twice. verification_method = "property-test"; proof_status = "Property-Tested".
- MARKET-SAFE-003 Atomic Settlement: Both legs settle or neither does. verification_method = "property-test"; proof_status = "Property-Tested".
- MARKET-SAFE-004 Seller Ownership: Seller must own listed item. verification_method = "property-test"; proof_status = "Property-Tested".
- MARKET-SAFE-005 Price Bounds: Price must be non-negative and valid. verification_method = "property-test"; proof_status = "Property-Tested".
- MARKET-SAFE-006 Fee Bounds: Fee must remain within configured bounds. verification_method = "property-test"; proof_status = "Property-Tested".

## Integrity Invariants

- INT-DET-001 Determinism: verification_method = "differential"; proof_status = "Differential-Tested".
- INT-REC-001 Receipt Integrity: verification_method = "differential"; proof_status = "Differential-Tested".
- INT-REP-001 Replay Equivalence: verification_method = "replay"; proof_status = "Differential-Tested".
- INT-ROOT-001 Root Equivalence: verification_method = "differential"; proof_status = "Differential-Tested".

Changing the implementation hash voids certification until re-certified.
