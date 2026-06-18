# ArenaState Model

`ArenaState` is the complete persisted consensus state at one arena tick. The canonical top-level field order is:

```text
schema_version
world_id
arena_id
tick
players
entities
positions
health
receipts
continuity
metadata
```

Unknown fields are rejected. Missing required fields are rejected. Optional consensus values must appear explicitly as `null` or an empty collection where specified.

## Required fields and allowed value types

| Field | Type | Deterministic constraint |
| --- | --- | --- |
| `schema_version` | unsigned integer | Must be `1` for this handoff. |
| `world_id` | UTF-8 string | Stable world identifier. |
| `arena_id` | UTF-8 string | Stable arena/shard identifier. |
| `tick` | unsigned integer | Completed simulation tick; genesis is `0`. |
| `players` | array of `PlayerState` | Sorted by UTF-8 bytes of `player_id`; unique IDs. |
| `entities` | array of `EntityState` | Sorted by UTF-8 bytes of `entity_id`; unique IDs. |
| `positions` | array of `PositionState` | Sorted by UTF-8 bytes of `entity_id`; unique component per entity. |
| `health` | array of `HealthState` | Sorted by UTF-8 bytes of `entity_id`; unique component per entity. |
| `receipts` | `ReceiptState` object | Fixed field order. |
| `continuity` | `ContinuityState` object | Fixed field order. |
| `metadata` | `MetadataState` object | Fixed field order; nested maps sorted by UTF-8 bytes. |

Allowed consensus values are JSON objects, arrays, UTF-8 strings, integers, booleans where specified, and `null` where specified. Floating-point values, exponent notation, `NaN`, `Infinity`, timestamps, host pointers, randomized map seeds, and platform-native binary blobs are not valid consensus state.

## `players`: `PlayerState`

Field order:

```text
player_id
controller_id
join_tick
status
score
metadata
```

* `player_id`: string; stable unique player identifier.
* `controller_id`: string; stable input/account authority.
* `join_tick`: unsigned integer.
* `status`: string enum: `active`, `inactive`, `eliminated`, or `left`.
* `score`: signed integer.
* `metadata`: object; recursive dynamic keys sorted by UTF-8 byte order.

## `entities`: `EntityState`

Field order:

```text
entity_id
entity_type
owner_player_id
spawn_tick
despawn_tick
attributes
```

* `entity_id`: string; stable unique entity identifier.
* `entity_type`: string.
* `owner_player_id`: string or `null`.
* `spawn_tick`: unsigned integer.
* `despawn_tick`: unsigned integer or `null`.
* `attributes`: object containing deterministic JSON values only; recursive keys sorted by UTF-8 byte order.

## `positions`: `PositionState`

Field order:

```text
entity_id
x
y
z
rotation
```

* `entity_id`: string.
* `x`, `y`, `z`, `rotation`: signed integers. Fractional positions must be pre-scaled to integers before canonicalization.

## `health`: `HealthState`

Field order:

```text
entity_id
current
maximum
```

* `entity_id`: string.
* `current`: signed integer.
* `maximum`: signed integer.

## `receipts`: `ReceiptState`

Field order:

```text
receipt_root
receipt_count
last_receipt_hash
```

* `receipt_root`: lowercase 64-character hex string. Empty tree uses 64 zero characters.
* `receipt_count`: unsigned integer.
* `last_receipt_hash`: lowercase 64-character hex string or `null` when `receipt_count` is `0`.

## `continuity`: `ContinuityState`

Field order:

```text
continuity_root
previous_state_root
replay_root
migration_root
epoch
```

* `continuity_root`: lowercase 64-character hex string.
* `previous_state_root`: lowercase 64-character hex string or `null` at genesis.
* `replay_root`: lowercase 64-character hex string.
* `migration_root`: lowercase 64-character hex string or `null`.
* `epoch`: unsigned integer.

## `metadata`: `MetadataState`

Field order:

```text
ruleset_id
ruleset_version
created_by
labels
extensions
```

* `ruleset_id`: string.
* `ruleset_version`: unsigned integer.
* `created_by`: string or `null`.
* `labels`: sorted unique array of strings, compared by UTF-8 bytes.
* `extensions`: object containing deterministic JSON values only; recursive keys sorted by UTF-8 byte order.
