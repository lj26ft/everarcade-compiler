# Migration Certification Summary

## Migration package

A migration package contains the source canonical state or checkpoint, input/replay commitments, receipt commitments, continuity commitments, and target-environment metadata that is outside the canonical state unless explicitly represented in `metadata.extensions`.

## Replay validation

Both source and destination must replay or validate the migrated state against the same canonical specification. Validation compares canonical bytes and roots after migration.

## Continuity preservation

Migration must preserve `previous_state_root`, `replay_root`, `migration_root`, and `epoch` according to the migration profile. Migration is `PASS` when `restore_elsewhere(state)` yields the same canonical bytes and same root, with continuity commitments preserved or explicitly committed by `migration_root`.
