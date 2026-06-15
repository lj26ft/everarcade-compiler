# Persistent World Report

## Status
Persistent world activation is represented by `LiveWorldPlatform`, which drives world create, boot, host, shutdown, restart, migration, recovery, and checkpoint persistence through deterministic roots.

## Guarantees
- Deterministic recovery is validated by restoring from replay-equivalent world state.
- Replay continuity is preserved through append-only archive entries and checkpoint replay tips.
- Authority preservation is enforced by existing runtime authority mutation rejection tests.

## Validation
Covered by `test_persistent_world_equivalence`, `test_world_recovery_equivalence`, and `test_replay_safe_multiplayer`.
