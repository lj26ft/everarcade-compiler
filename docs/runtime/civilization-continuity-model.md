# Civilization Continuity Model

Continuity is encoded as deterministic lifecycle/economy/inventory mutations plus periodic checkpoints.

- Identity continuity: entity_id + generation chain.
- Evolution safety: stage transitions are replay-safe and monotonic.
- Economy continuity: append-only ledger checkpoints with deterministic roots.
- Inventory continuity: deterministic ownership/mutation ordering.
- Federation continuity: shared replay input yields equivalent continuity roots across nodes.
