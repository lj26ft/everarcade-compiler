# world.evr Commitment Architecture V1

## Status

This document defines the default commitment architecture for `world.evr` packages using commitment profile `world.evr.commitment.v1`. The design goal is that anyone can independently re-check a world with ordinary hardware, open code, and standard hash functions. V1 requires SHA-256 and introduces no trusted setup or exotic cryptographic dependency.

## Commitment profiles

Manifests and packages that use this architecture MUST include:

```json
{
  "commitment_profile": "world.evr.commitment.v1"
}
```

Legacy packages that predate split roots MUST be identified as:

```json
{
  "commitment_profile": "world.evr.commitment.legacy"
}
```

A verifier MUST distinguish `world.evr.commitment.legacy` from `world.evr.commitment.v1`. A v1 proof or checkpoint MUST NOT be accepted under the legacy profile, and a legacy proof MUST NOT be accepted under `world.evr.commitment.v1`.

## Root families

V1 separates commitments into independent root families because current state, append-only history, and continuity have different access patterns.

```text
state_root   = current world state commitment
receipt_root = append-only world history commitment
continuity_root = replay/restore/migration/checkpoint/federation continuity commitment
```

### `state_root`

`state_root` answers:

> What is the current state of the world?

For v1, the current deterministic canonical state hashing MAY remain the baseline when an authenticated map is not yet available. Implementations MUST document the exact canonical state bytes or canonical state hashing profile used to derive `state_root`.

Future-compatible direction:

```text
state_root -> authenticated map / sparse Merkle / IAVL-style tree
```

The state commitment should optimize for long-lived world state, efficient independent re-derivation, and future portable state proofs without changing the receipt history commitment.

### `receipt_root`

`receipt_root` answers:

> Did this event/receipt occur in this world history, and can it be proven into a known checkpoint?

The default v1 structure is a Merkle Mountain Range (MMR), specified in [Receipt MMR V1](./RECEIPT_MMR_V1.md):

```text
receipt_root -> Merkle Mountain Range
```

The receipt MMR is append-only, supports historical inclusion proofs, avoids full root rebuild on every tick, and uses only SHA-256.

### `continuity_root`

`continuity_root` links world history across replay, restore, migration, checkpoint, and federation boundaries. It remains separate from both `state_root` and `receipt_root` so continuity policies can evolve without changing state-map or receipt-MMR proof semantics.

Implementations SHOULD include replay lineage, restoration parentage, migration identifiers, federation boundary commitments, and prior finalized checkpoints in the continuity material when applicable. The exact continuity canonicalization profile MUST be documented by the package or runtime profile.

## World hash

The v1 world hash combines the three independent root families with an explicit domain tag:

```text
world_hash =
  SHA256(
    "world.evr.world_hash.v1" ||
    state_root ||
    receipt_root ||
    continuity_root
  )
```

`state_root`, `receipt_root`, and `continuity_root` MUST be exactly 32 raw bytes when hashed. Hex encodings in JSON MUST decode to exactly 32 bytes before recomputation.

## Verification rules

A v1 verifier MUST:

- reject unknown commitment profiles;
- reject legacy commitments when a v1 proof/checkpoint is required;
- recompute `world_hash` from decoded 32-byte roots;
- reject malformed hex and incorrect root lengths;
- treat `state_root`, `receipt_root`, and `continuity_root` as separate domains.
