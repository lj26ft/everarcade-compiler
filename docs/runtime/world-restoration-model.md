# World Restoration Model

Restoration semantics cover cold restore, partial restore, multi-era restore, checkpoint rollback, and archive replay restore.

- Cold restore: restore latest checkpoint and replay forward.
- Partial restore: restore nearest checkpoint and replay selected era window.
- Multi-era restore: stitch archives by era order.
- Rollback: deterministic rollback to prior checkpoint root.
- Federation restore: all nodes must converge to equivalent continuity roots with same replay input.
