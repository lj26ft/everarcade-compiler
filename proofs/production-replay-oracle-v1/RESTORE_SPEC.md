# Restore Specification v1

This document extends `TRANSITION_SPEC.md` with fixture-witnessed restore proofs for `everarcade-tier2-proof`. Restore is intentionally specified without EverArcade runtime imports: an oracle consumes genesis, journal actions, a snapshot/checkpoint, receipt accumulator data, and continuity metadata, then recomputes all commitments using the replay transition rules.

## Claim scope

Correct claim: restore is independently reproducible over the supplied fixtures from spec and bundles only. Incorrect claim: restore is formally verified for all possible inputs.

## Canonicalization and hash rule

Use the canonical JSON encoder and SHA-256 rules from `TRANSITION_SPEC.md`. Every new commitment in this spec is:

```text
hash = sha256(canonical(object))
```

When a self-referential hash field is present, compute the hash over the same object with that hash field set to JSON `undefined`, matching the replay oracle convention.

## Restore operation

Given:

- exported world snapshot
- prior verified root set
- checkpoint state
- receipt accumulator state
- continuity metadata

Prove:

- replayed `state_root` equals exported `state_root`
- replayed `receipt_root` equals exported `receipt_root`
- replayed `continuity_root` equals exported `continuity_root`
- restored state root, receipt root, world hash, and continuity root equal the exported roots
- the restored world resumes from the same verified history chain

## Restore input schema

A restore fixture contains:

- `genesis`: production replay genesis document; the oracle uses `genesis.state` as the initial ArenaState.
- `journal`: production replay journal document; the oracle uses accepted `journal.entries[*].action` and `journal.entries[*].round` only.
- `checkpoint`: checkpoint object described below.
- `snapshot`: exported snapshot object described below.
- `expected_pre_export_roots`: expected roots after independent replay.
- `export_bundle`: export bundle conforming to `schemas/export-bundle.schema.json`.
- `restore_bundle`: destination restore declaration.
- `expected_restored_roots`: expected roots after restore.

## Snapshot schema

```json
{
  "schema": "everarcade.hotpocket.arena-wrapper.v0.1.restore.snapshot-v1",
  "world_id": "arena-vanguard",
  "tick": 6,
  "state": { "tick": 6, "players": {}, "combat_events": [], "last_sequence": {}, "commitments": [] },
  "roots": {
    "tick": 6,
    "state_root": "...",
    "receipt_root": "...",
    "world_hash": "...",
    "continuity_root": "..."
  },
  "snapshot_hash": "..."
}
```

`state` is the full exported ArenaState, including its commitment chain. `roots` must equal the last commitment in `state.commitments` and the independent replay roots. `snapshot_hash = sha256(canonical(snapshot_without_snapshot_hash))`.

## Checkpoint schema

```json
{
  "schema": "everarcade.hotpocket.arena-wrapper.v0.1.restore.checkpoint-v1",
  "world_id": "arena-vanguard",
  "tick": 6,
  "genesis_hash": "...",
  "journal_hash": "...",
  "snapshot_hash": "...",
  "receipt_accumulator": {
    "final_receipt_hashes": ["..."],
    "last_temp_receipt_hash": "..."
  },
  "roots": { "state_root": "...", "receipt_root": "...", "world_hash": "...", "continuity_root": "...", "tick": 6 },
  "checkpoint_hash": "..."
}
```

`checkpoint_hash = sha256(canonical(checkpoint_without_checkpoint_hash))`.

### Receipt Accumulator

`receipt_accumulator` is authoritative recomputation state, not merely evidence. A restore verifier must derive `receipt_root` from this accumulator and must not consume a stored `receipt_root` as trusted input.

Structure:

```json
{
  "final_receipt_hashes": ["..."],
  "last_temp_receipt_hash": "..."
}
```

Field definitions:

- `final_receipt_hashes`: canonical-order array of finalized receipt hashes for receipts strictly before the receipt currently being committed.
- `last_temp_receipt_hash`: SHA-256 hash of the last receipt before its final `receipt_root`, `world_hash`, `continuity_root`, `output`, and final `receipt_hash` fields are attached.

Canonical ordering:

- `final_receipt_hashes` preserves replay sequence order.
- `last_temp_receipt_hash` is paired with that ordered prefix as the final element under construction.

Receipt root construction:

```text
receipt_root = sha256(canonical([final_receipt_hashes, last_temp_receipt_hash]))
```

Replay relationship:

- During replay, each accepted action first produces a temporary receipt hash.
- The receipt root for that action is computed from the prior finalized receipt hashes and the current temporary receipt hash.
- The receipt is then finalized with the derived roots and final receipt hash for use as prior state by the next action.

## Restore bundle schema

```json
{
  "schema": "everarcade.hotpocket.arena-wrapper.v0.1.restore-bundle-v1",
  "world_id": "arena-vanguard",
  "restore_sequence": 1,
  "destination_operator": "operator-b",
  "destination_instance": "instance-b",
  "export_hash": "...",
  "snapshot_hash": "...",
  "checkpoint_hash": "...",
  "restored_roots": { "state_root": "...", "receipt_root": "...", "world_hash": "...", "continuity_root": "...", "tick": 6 },
  "restored_state_root": "...",
  "restored_receipt_root": "...",
  "restored_continuity_root": "..."
}
```

## Root fields and required hashes

- `world_hash = sha256(canonical({ tick, players, combat_events }))` from `TRANSITION_SPEC.md`.
- `receipt_root = sha256(canonical([final_receipt_hashes, last_temp_receipt_hash]))` from `receipt_accumulator`.
- `state_root = sha256(canonical(full ArenaState including commitments through the restored tick))`.
- `continuity_root = sha256(canonical({ state_root, receipt_root, world_hash, tick }))`.
- `snapshot_hash = sha256(canonical(snapshot_without_snapshot_hash))`.
- `checkpoint_hash = sha256(canonical(checkpoint_without_checkpoint_hash))`.
- `export_hash = sha256(canonical(export_bundle_without_export_hash))`.
- `restored_state_root = export_bundle.state_root`.
- `restored_continuity_root = export_bundle.continuity_root`.

## Failure cases

A restore verifier must fail if any of these occur:

- snapshot `state_root` does not match independent replay or snapshot state
- exported `receipt_root` does not match the root derived from `receipt_accumulator`
- restored `continuity_root` differs from exported continuity root
- checkpoint binds to a different journal than the supplied journal
- checkpoint or snapshot content is tampered without updating its declared hash
- `last_temp_receipt_hash` is missing or incorrect
- `receipt_accumulator` is tampered
- `world_id`, tick, genesis hash, journal hash, checkpoint hash, or snapshot hash disagree across the fixture, checkpoint, snapshot, export bundle, and restore bundle
