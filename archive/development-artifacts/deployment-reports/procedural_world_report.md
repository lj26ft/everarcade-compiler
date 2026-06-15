# Procedural World Report

## Continuity Guarantees
- faction continuity: preserved when applicable
- societal continuity: preserved when applicable
- diplomacy continuity: deterministic and divergence-rejecting
- ecology continuity: deterministic and divergence-rejecting
- replay continuity: append-only canonical history
- social-memory continuity: append-only and replay-restorable
- federation continuity: deterministic replay-lineage synchronization

## Operational Limitations
- Renderer/history/federation domains remain scaffold-level and non-authoritative.
- Replay hydration is reconstruction-only and cannot mutate authority.
- Authority is constrained to deterministic execution runtime boundaries.
