# Restore Certification Summary

## Restore workflow

Restore begins from a certified checkpoint or exported state package, hydrates the runtime elsewhere, and resumes deterministic verification from the restored state.

## Root preservation

The restored state must canonicalize to the same canonical bytes and same `state_root` as the source checkpoint. The restore process must not introduce timestamps, host IDs, map-order changes, or generated values into consensus state.

## Continuity preservation

`continuity.previous_state_root`, `continuity.replay_root`, `continuity.migration_root`, and `continuity.epoch` must remain consistent with the source continuity chain. Restore is `PASS` when root preservation and continuity preservation both hold.
