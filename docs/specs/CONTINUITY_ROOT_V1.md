# Continuity Root V1

`continuity_root` commits a bounded replay slice to its predecessor and terminal receipts. It is a verification root, not a gameplay rule.

## Domain separation

Use the UTF-8 domain tag `everarcade.continuity_root.v1`, followed by one `0x00` byte, followed by canonical JSON bytes of the continuity input object. Hash with SHA-256 and encode the 32-byte digest as lowercase hex.

## Canonical serialization

Canonical JSON is UTF-8 JSON with object keys sorted lexicographically by Unicode code point, no insignificant whitespace, arrays kept in listed order, integers encoded as JSON numbers, strings escaped as JSON requires, and absent optional values encoded explicitly as `null` when they are part of this spec.

## Inputs and ordering

The canonical input object contains exactly these fields before key sorting:

1. `schema_version`: integer `1`.
2. `world_id`: stable world identifier string.
3. `epoch`: non-negative integer protocol epoch.
4. `previous_state_root`: lowercase 64-hex state root for the state before this slice, or `null` for genesis.
5. `receipt_root`: lowercase 64-hex receipt accumulator root for receipts in this slice.
6. `replay_root`: lowercase 64-hex root over replay entries used to rederive the slice.
7. `migration_root`: lowercase 64-hex migration root, or `null` if no migration is included.
8. `terminal_tick`: non-negative integer terminal tick after the slice.

The logical field order above is normative for review. The serialized byte order is sorted-key canonical JSON.

## Empty root

The empty continuity root is `0000000000000000000000000000000000000000000000000000000000000000`. It may appear only before any continuity slice exists. Once a slice is claimed, verifiers MUST compute the tagged SHA-256 root instead of accepting the zero root.

## What exactly is committed?

The root commits to the world identity, protocol epoch, predecessor state root, receipt root, replay root, optional migration root, and terminal tick for one bounded slice. It does not directly commit full state bytes; that is the job of `state_root`.

## How does a verifier recompute it?

A verifier replays the same ordered inputs from the predecessor state, recomputes every receipt and replay entry, derives `receipt_root` and `replay_root`, constructs the continuity input object exactly as specified, canonicalizes it, and hashes `domain_tag || 0x00 || canonical_json` with SHA-256.

## What invalidates it?

A continuity root is invalid if any required input is missing, if hex roots are not lowercase 32-byte SHA-256 digests, if receipt or replay ordering differs, if the predecessor state root differs, if migration material differs, if the terminal tick differs, if canonical serialization differs, or if the computed digest does not equal the claimed `continuity_root`.

## Example

```json
{
  "schema_version": 1,
  "world_id": "determinism-fixture-v1",
  "epoch": 0,
  "previous_state_root": "e64407dda4eba756bdf466719c4ccaf039c87983fea852bba0a83c4513b90e0e",
  "receipt_root": "5078a4030cb405b5a70e9605952d614b97b61ce3c455b33c87b5bdb888201c92",
  "replay_root": "d8bd290afdecae7563b6f79a3abbd7f9dbb7454b3955ba813592615a259757de",
  "migration_root": null,
  "terminal_tick": 3
}
```

## Test vector

For the example above, canonicalize with sorted keys and hash with tag `everarcade.continuity_root.v1` plus `0x00` separator. Expected root:

```text
0b23e15fac98c7d73e6e30c021e410336936eeef18e2128b36504298f8657129
```
