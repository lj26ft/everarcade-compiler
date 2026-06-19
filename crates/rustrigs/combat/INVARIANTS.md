# combat.attack Invariants

- **RR-COMBAT-001 Attack Determinism**
- **RR-COMBAT-002 Damage Integrity**
- **RR-COMBAT-003 Health Floor**
- **RR-COMBAT-004 Death Consistency**
- **RR-COMBAT-005 Receipt Integrity**
- **RR-COMBAT-006 Replay Equivalence**
- **RR-COMBAT-007 Root Equivalence**

## Proof target
Verifiers prove deterministic state transition, receipt integrity, replay equivalence, root equivalence, and mutation-specific conservation/safety properties from `src/lib.rs`.
