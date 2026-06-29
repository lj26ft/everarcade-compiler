# World Hash V1

`world_hash` is the top-level root that binds state, receipts, and continuity for one checkpointed world view.

## Composition

```text
world_hash = SHA256(
  utf8("everarcade.world_hash.v1") || 0x00 ||
  bytes32(state_root) ||
  bytes32(receipt_root) ||
  bytes32(continuity_root)
)
```

## Field ordering

The byte order is fixed:

1. domain tag `everarcade.world_hash.v1`
2. one `0x00` separator byte
3. `state_root` as 32 raw bytes decoded from lowercase hex
4. `receipt_root` as 32 raw bytes decoded from lowercase hex
5. `continuity_root` as 32 raw bytes decoded from lowercase hex

No JSON, delimiters, length prefixes, or whitespace are included after the separator.

## Byte encoding

All root fields MUST be lowercase 64-character hexadecimal SHA-256 digests before decoding. Verifiers MUST reject uppercase hex, odd-length hex, non-hex characters, missing fields, and roots that do not decode to exactly 32 bytes.

## Domain separation

The domain tag `everarcade.world_hash.v1` prevents collisions with `state_root`, `receipt_root`, `continuity_root`, and checkpoint roots even when the same child roots are reused in another context.

## Empty behavior

There is no special all-zero world hash for a claimed world state. Empty child domains use their own empty behavior, then `world_hash` is still computed with this formula. If all three child roots are zero, the resulting world hash is the tagged SHA-256 digest over those three zero byte arrays.

## What invalidates it?

A world hash is invalid if any child root is malformed, if child roots were computed under a different versioned spec, if field ordering changes, if the domain tag changes, or if the recomputed digest differs from the claim.

## Example and vector

Given:

```text
state_root      = d61d07c40e33d6378729e9e5889045de997a484ebd882ccb5f86ef4fd44ccc87
receipt_root    = 5078a4030cb405b5a70e9605952d614b97b61ce3c455b33c87b5bdb888201c92
continuity_root = 0b23e15fac98c7d73e6e30c021e410336936eeef18e2128b36504298f8657129
```

Expected:

```text
world_hash      = 2d1430d5758b6163e8b82261cbe94539bd52d790974e9f310e501cad5f2ff509
```
