# governance.vote Invariants

- **RR-GOV-001 Vote Determinism**
- **RR-GOV-002 Vote Uniqueness**
- **RR-GOV-003 Tally Integrity**
- **RR-GOV-004 Quorum Integrity**
- **RR-GOV-005 Proposal Finality**
- **RR-GOV-006 Tick Window Integrity**
- **RR-GOV-007 Receipt Integrity**
- **RR-GOV-008 Replay Equivalence**
- **RR-GOV-009 Root Equivalence**

## Proof target
Verifiers prove deterministic state transition, receipt integrity, replay equivalence, root equivalence, and mutation-specific conservation/safety properties from `src/lib.rs`.
