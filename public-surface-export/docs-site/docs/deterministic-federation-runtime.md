# Deterministic Federation Runtime

This runtime phase adds deterministic federation continuity primitives without adding Byzantine consensus.

## Model
- Federation continuity is replay-verifiable, canonical, and append-only.
- Deterministic objects: execution manifest, checkpoint, continuity proof, envelope, settlement journal, quorum proof, identity continuity, snapshot, recovery state.

## Limitations
- Not Byzantine consensus.
- Requires trusted identity roots/signature assumptions.
- Requires deterministic toolchain equivalence between nodes.
- Requires compatible checkpoint/replay formats.
- Recovery assumes canonical artifacts are available and untampered.
