# inventory.transfer Invariants

- **RR-INV-001 Transfer Determinism**
- **RR-INV-002 Inventory Conservation**
- **RR-INV-003 No Overdraw**
- **RR-INV-004 Source/Destination Validity**
- **RR-INV-005 Receipt Integrity**
- **RR-INV-006 Replay Equivalence**
- **RR-INV-007 Root Equivalence**

## Proof target
Verifiers prove deterministic state transition, receipt integrity, replay equivalence, root equivalence, and mutation-specific conservation/safety properties from `src/lib.rs`.
