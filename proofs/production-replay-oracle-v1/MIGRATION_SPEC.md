# Migration Specification v1

Migration is a source replay/export, destination restore, and continuation proof built on `TRANSITION_SPEC.md` and `RESTORE_SPEC.md`.

## Claim scope

Correct claim: migration is independently reproducible over the supplied fixtures from spec and bundles only. Incorrect claim: migration is formally verified for all possible inputs.

## Migration flow

```text
source operator/world instance
↓
export bundle
↓
destination operator/world instance
↓
restore
↓
continue execution
```

The migration proof shows:

```text
source final root == export source root
export source root == destination restore root
destination continuity root links to source continuity root
post-migration execution preserves the verified history chain
```

## Migration input schema

A migration fixture contains:

- `source_genesis`: genesis document for independent source replay.
- `source_journal`: source accepted-action journal.
- `source_final_roots`: expected roots at the export boundary.
- `export_bundle`: source export bundle conforming to `schemas/export-bundle.schema.json`.
- `destination_restore_bundle`: restore declaration binding destination to the export.
- `destination_continuation_journal`: accepted actions after restore. The first entry round must be source/export tick + 1.
- `migration_link`: continuity link object.
- `expected_destination_roots`: expected final roots after continuation replay from the restored source snapshot.

## Export bundle schema

The formal JSON Schema is `schemas/export-bundle.schema.json`. The v1 production fixture fields are `world_id`, `export_sequence`, `source_operator`, `source_instance`, `export_tick`, `genesis_hash`, `journal_hash`, `checkpoint_hash`, `snapshot_hash`, `receipt_root`, `state_root`, `world_hash`, `continuity_root`, optional `package_identity`, and `export_hash`.

`export_hash = sha256(canonical(export_bundle_without_export_hash))`.

## Destination restore schema

The destination restore bundle uses the restore bundle schema from `RESTORE_SPEC.md`. It must bind to the same `world_id`, `export_hash`, `checkpoint_hash`, `snapshot_hash`, `state_root`, `receipt_root`, and `continuity_root` exported by the source.

## Identity fields

- Operator identity fields: `source_operator`, `source_instance`, `destination_operator`, and `destination_instance` are opaque strings in this proof pack. They are commitment inputs when present.
- Package/runtime identity fields: `package_identity.runtime` and `package_identity.transition_spec` are opaque bundle metadata and are included in `export_hash`.

## Continuity link construction

```json
{
  "schema": "everarcade.hotpocket.arena-wrapper.v0.1.migration-link-v1",
  "world_id": "arena-vanguard",
  "source_continuity_root": "...",
  "destination_restore_continuity_root": "...",
  "destination_final_continuity_root": "...",
  "continuation_journal_hash": "...",
  "migration_hash": "..."
}
```

- `source_continuity_root` must equal `source_final_roots.continuity_root` and `export_bundle.continuity_root`.
- `destination_restore_continuity_root` must equal `destination_restore_bundle.restored_continuity_root`.
- `destination_final_continuity_root` must equal the independently replayed post-migration final continuity root.
- `continuation_journal_hash = sha256(canonical(destination_continuation_journal))`.
- `migration_hash = sha256(canonical(migration_link_without_migration_hash))`.
- `post_migration_continuity_root = sha256(canonical({ state_root, receipt_root, world_hash, tick }))` after replaying continuation actions from the restored state and receipt accumulator.

## Failure cases

A migration verifier must fail if any of these occur:

- source root mismatch between replay, fixture, and export bundle
- destination restore root mismatch against export bundle
- broken continuity link or recomputed `migration_hash`
- wrong operator/world id where bundle identities no longer match
- continuation journal starts from a wrong round/root boundary
- post-migration replay does not produce `expected_destination_roots`
