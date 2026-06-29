# Proof Format V1

This document defines the portable receipt inclusion proof for `world.evr.commitment.v1`.

## Receipt inclusion proof JSON

```json
{
  "version": "world.evr.receipt_proof.v1",
  "commitment_profile": "world.evr.commitment.v1",
  "world_id": "...",
  "receipt_index": 0,
  "receipt_count": 1024,
  "leaf_hash": "...",
  "siblings": [
    { "position": "left", "hash": "..." },
    { "position": "right", "hash": "..." }
  ],
  "peaks": ["..."],
  "receipt_root": "...",
  "checkpoint_hash": "..."
}
```

Hashes are lowercase hex for exactly 32 bytes. `siblings` are ordered from leaf level toward the peak. `position` is relative to the running hash: `left` means the sibling is the left child and the running hash is the right child; `right` means the sibling is the right child and the running hash is the left child. `peaks` are ordered from largest/oldest to smallest/newest, as required by [Receipt MMR V1](./RECEIPT_MMR_V1.md).

## Verifier rules

A verifier MUST:

- reject malformed hex;
- reject unknown versions;
- reject unsupported commitment profiles;
- reject missing fields, duplicated fields, or unexpected field types;
- reject receipt/world/tick/sequence mismatch when receipt bytes or decoded receipt fields are provided;
- reject a proof if the recomputed root does not match claimed `receipt_root`;
- reject checkpoint mismatch when a checkpoint is supplied;
- reject a legacy proof under `world.evr.commitment.v1`;
- reject `world.evr.commitment.v1` proofs under `world.evr.commitment.legacy`;
- reject wrong domain tags, wrong peak ordering, and wrong empty roots.

## Verification algorithm

1. Decode and validate all fixed fields and hashes.
2. Recompute the leaf hash from canonical receipt bytes if receipt bytes are provided; otherwise compare the supplied receipt hash to `leaf_hash`.
3. Fold `siblings` in order using `world.evr.receipt.node.v1` and each sibling `position`.
4. Confirm that the resulting peak hash appears exactly once in `peaks` at the position implied by `receipt_index` and `receipt_count`.
5. Recompute `receipt_root` using `world.evr.receipt.root.v1`, the receipt count encoded as `u64` big-endian, and ordered peaks.
6. Compare the recomputed root to the claimed `receipt_root` and to the checkpoint `receipt_root` when present.
