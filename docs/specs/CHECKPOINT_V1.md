# Checkpoint V1

A `world.evr.checkpoint.v1` checkpoint commits to current state, append-only receipts, continuity, and prior checkpoint linkage. Checkpoints are periodic anchors and SHOULD NOT be emitted every tick by default.

## JSON shape

```json
{
  "version": "world.evr.checkpoint.v1",
  "world_id": "...",
  "checkpoint_index": 0,
  "start_tick": 0,
  "end_tick": 1024,
  "receipt_count": 1024,
  "state_root": "...",
  "receipt_root": "...",
  "continuity_root": "...",
  "world_hash": "...",
  "previous_checkpoint_hash": "...",
  "created_at_logical_tick": 1024
}
```

`state_root`, `receipt_root`, `continuity_root`, `world_hash`, and `previous_checkpoint_hash` are lowercase hex strings for 32-byte values. The genesis checkpoint MAY use 32 zero bytes for `previous_checkpoint_hash` if no previous checkpoint exists.

## Checkpoint hash

```text
checkpoint_hash =
  SHA256(
    "world.evr.checkpoint.v1" ||
    canonical_checkpoint_bytes
  )
```

`canonical_checkpoint_bytes` MUST be deterministic. V1 conformance tooling uses canonical JSON bytes with sorted keys and no insignificant whitespace.

## Verification

A verifier MUST recompute `world_hash` according to [Commitment Architecture V1](./COMMITMENT_ARCHITECTURE_V1.md), recompute `checkpoint_hash`, verify prior checkpoint linkage when a prior checkpoint is supplied, and reject malformed hex, unknown versions, unsupported commitment profiles, or roots of the wrong length.

The anchoring chain is:

```text
receipt -> MMR proof -> checkpoint root -> external ledger anchor
```
