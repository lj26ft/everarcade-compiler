# EverArcade Canonicalizer Specification

Status: protocol draft 0.1
Scope: deterministic world-state canonicalization, canonical byte generation, state-root generation, and world-hash composition.

The canonicalizer is the protocol trust boundary between an EverArcade world simulation and every downstream certification artifact. Any verifier that receives the same valid `ArenaState` MUST derive the same canonical bytes, `state_root`, and `world_hash` without depending on host architecture, language runtime, map iteration order, wall-clock time, or serialization-library defaults.

The normative terms **MUST**, **MUST NOT**, **REQUIRED**, **SHOULD**, and **MAY** are used with their RFC 2119 meanings.

## 1. Canonical State Model

### 1.1 `ArenaState`

`ArenaState` is the complete persisted consensus state for one arena at one simulation tick. It is the only input to state-root generation.

An `ArenaState` MUST be represented as a JSON object with exactly the following top-level fields in canonical field order:

1. `schema_version`
2. `world_id`
3. `arena_id`
4. `tick`
5. `players`
6. `entities`
7. `positions`
8. `health`
9. `receipts`
10. `continuity`
11. `metadata`

Unknown top-level fields MUST be rejected. Missing required fields MUST be rejected. Optional fields MUST be present with an explicit empty value or `null` as defined below; omitting optional fields is not canonical.

| Field | Required | Type | Canonical ordering | Description |
| --- | --- | --- | --- | --- |
| `schema_version` | Yes | unsigned integer | scalar | Canonicalizer schema version. This document defines version `1`. |
| `world_id` | Yes | string | scalar | Stable world identifier. |
| `arena_id` | Yes | string | scalar | Stable arena or shard identifier within the world. |
| `tick` | Yes | unsigned integer | scalar | Completed simulation tick represented by this state. Genesis MUST use `0`. |
| `players` | Yes | array of `PlayerState` | sorted by `player_id` ascending | Persisted player records. Empty array is canonical for no players. |
| `entities` | Yes | array of `EntityState` | sorted by `entity_id` ascending | Persisted entity records. Empty array is canonical for no entities. |
| `positions` | Yes | array of `PositionState` | sorted by `entity_id` ascending | Persisted position components. Empty array is canonical for no positions. |
| `health` | Yes | array of `HealthState` | sorted by `entity_id` ascending | Persisted health components. Empty array is canonical for no health components. |
| `receipts` | Yes | `ReceiptState` object | fixed object field order | Receipt commitments required by downstream certification. |
| `continuity` | Yes | `ContinuityState` object | fixed object field order | Replay, migration, and restore continuity commitments. |
| `metadata` | Yes | `MetadataState` object | sorted object keys where maps are allowed | Protocol metadata that is part of consensus. Empty object fields are canonical when no metadata exists. |

### 1.2 Scalar requirements

* Unsigned integers MUST be encoded in base-10 JSON number form with no sign, no leading zeroes except the literal `0`, and no exponent notation.
* Signed integers MAY appear only where explicitly specified and MUST use base-10 JSON number form with a leading `-` for negative values.
* Strings MUST be Unicode scalar values encoded as UTF-8 in the final byte representation.
* Binary commitments, hashes, and roots MUST be lowercase hexadecimal strings unless a field explicitly specifies another representation.
* Booleans MAY appear only where explicitly specified.
* `null` MAY appear only for fields explicitly marked nullable.

### 1.3 `PlayerState`

A `PlayerState` records consensus-relevant player identity and status.

Canonical field order:

1. `player_id`
2. `controller_id`
3. `join_tick`
4. `status`
5. `score`
6. `metadata`

| Field | Required | Type | Description |
| --- | --- | --- | --- |
| `player_id` | Yes | string | Stable unique player identifier within the arena. |
| `controller_id` | Yes | string | Stable input authority or account identifier. |
| `join_tick` | Yes | unsigned integer | Tick at which this player entered consensus state. |
| `status` | Yes | string enum | One of `active`, `inactive`, `eliminated`, or `left`. |
| `score` | Yes | signed integer | Integer-only score. Use `0` when the game has no score. |
| `metadata` | Yes | object | Consensus metadata for the player. Use `{}` when empty. |

Ordering requirements:

* `players` MUST be sorted by the UTF-8 byte order of `player_id`.
* Duplicate `player_id` values MUST be rejected.
* `metadata` keys MUST be sorted by byte-lexicographic order of the keys' UTF-8 encodings.

### 1.4 `EntityState`

An `EntityState` records consensus-relevant entity identity and lifecycle.

Canonical field order:

1. `entity_id`
2. `entity_type`
3. `owner_player_id`
4. `spawn_tick`
5. `despawn_tick`
6. `attributes`

| Field | Required | Type | Description |
| --- | --- | --- | --- |
| `entity_id` | Yes | string | Stable unique entity identifier within the arena. |
| `entity_type` | Yes | string | Stable type identifier, such as `avatar`, `projectile`, or game-defined type. |
| `owner_player_id` | Yes | string or `null` | Owning player, or `null` for world-owned entities. |
| `spawn_tick` | Yes | unsigned integer | Tick at which the entity entered consensus state. |
| `despawn_tick` | Yes | unsigned integer or `null` | Tick at which the entity left active consensus state, or `null` while active. |
| `attributes` | Yes | object | Integer, string, boolean, null, array, or object values only. Use `{}` when empty. |

Ordering requirements:

* `entities` MUST be sorted by the UTF-8 byte order of `entity_id`.
* Duplicate `entity_id` values MUST be rejected.
* `attributes` object keys MUST be sorted recursively by byte-lexicographic order of the keys' UTF-8 encodings.

### 1.5 `PositionState`

A `PositionState` records integer-only spatial state.

Canonical field order:

1. `entity_id`
2. `x`
3. `y`
4. `z`
5. `rotation`

| Field | Required | Type | Description |
| --- | --- | --- | --- |
| `entity_id` | Yes | string | Entity whose position is persisted. |
| `x` | Yes | signed integer | Fixed-unit X coordinate. |
| `y` | Yes | signed integer | Fixed-unit Y coordinate. |
| `z` | Yes | signed integer | Fixed-unit Z coordinate. Use `0` for two-dimensional arenas. |
| `rotation` | Yes | signed integer | Fixed-unit orientation. Use `0` when unused. |

Ordering requirements:

* `positions` MUST be sorted by the UTF-8 byte order of `entity_id`.
* Duplicate position components for the same `entity_id` MUST be rejected.
* Floating-point coordinates MUST NOT appear. Worlds that need fractional positions MUST scale to integers before canonicalization.

### 1.6 `HealthState`

A `HealthState` records integer-only health or durability state.

Canonical field order:

1. `entity_id`
2. `current`
3. `maximum`

| Field | Required | Type | Description |
| --- | --- | --- | --- |
| `entity_id` | Yes | string | Entity whose health component is persisted. |
| `current` | Yes | signed integer | Current health. |
| `maximum` | Yes | signed integer | Maximum health. |

Ordering requirements:

* `health` MUST be sorted by the UTF-8 byte order of `entity_id`.
* Duplicate health components for the same `entity_id` MUST be rejected.
* `maximum` SHOULD be greater than or equal to `0`; game-specific validity rules MAY be stricter before canonicalization.

### 1.7 `ReceiptState`

`ReceiptState` commits to execution receipts that are not themselves expanded inside `ArenaState`.

Canonical field order:

1. `receipt_root`
2. `receipt_count`
3. `last_receipt_hash`

| Field | Required | Type | Description |
| --- | --- | --- | --- |
| `receipt_root` | Yes | lowercase hex string | Root commitment for all receipts included through this tick. Use 64 zero characters for an empty receipt tree. |
| `receipt_count` | Yes | unsigned integer | Number of receipts included in `receipt_root`. |
| `last_receipt_hash` | Yes | lowercase hex string or `null` | Hash of the most recent receipt, or `null` when `receipt_count` is `0`. |

### 1.8 `ContinuityState`

`ContinuityState` commits to replay and restore continuity.

Canonical field order:

1. `continuity_root`
2. `previous_state_root`
3. `replay_root`
4. `migration_root`
5. `epoch`

| Field | Required | Type | Description |
| --- | --- | --- | --- |
| `continuity_root` | Yes | lowercase hex string | Current continuity commitment. Use 64 zero characters at genesis if no prior continuity exists. |
| `previous_state_root` | Yes | lowercase hex string or `null` | State root of tick `N-1`, or `null` for genesis tick `0`. |
| `replay_root` | Yes | lowercase hex string | Commitment to replay inputs through this tick. Use 64 zero characters when empty. |
| `migration_root` | Yes | lowercase hex string or `null` | Migration commitment if this state was produced by migration, otherwise `null`. |
| `epoch` | Yes | unsigned integer | Continuity epoch. Genesis MUST use `0`. |

### 1.9 `MetadataState`

`MetadataState` contains only metadata that is consensus-relevant. Presentation-only metadata MUST NOT be included.

Canonical field order:

1. `ruleset_id`
2. `ruleset_version`
3. `created_by`
4. `labels`
5. `extensions`

| Field | Required | Type | Description |
| --- | --- | --- | --- |
| `ruleset_id` | Yes | string | Stable ruleset identifier. |
| `ruleset_version` | Yes | unsigned integer | Integer ruleset version. |
| `created_by` | Yes | string or `null` | Creator authority, or `null` when not consensus-relevant. |
| `labels` | Yes | array of strings | Sorted unique labels. Use `[]` when empty. |
| `extensions` | Yes | object | Namespaced extension data. Use `{}` when empty. |

Ordering requirements:

* `labels` MUST be sorted by UTF-8 byte order and duplicates MUST be rejected.
* `extensions` keys MUST be sorted recursively by byte-lexicographic order of the keys' UTF-8 encodings.
* Extension values MUST obey all deterministic encoding rules in this specification.

## 2. Deterministic Encoding Rules

### 2.1 Character encoding

* Canonical strings MUST be valid Unicode and MUST be serialized as JSON strings.
* Canonical bytes MUST be the UTF-8 encoding of the canonical JSON text.
* Invalid Unicode, implementation-specific byte strings, and platform-native string encodings MUST be rejected.

### 2.2 Fixed field ordering and sorted keys

Canonical object encoding MUST use this ordering rule:

1. For objects with a schema-defined field order, emit fields exactly in that order.
2. For dynamic maps, metadata maps, attributes maps, extension maps, and any other schema-permitted dynamic object keys, emit keys sorted by byte-lexicographic order of their UTF-8 encodings. Compare the raw UTF-8 byte sequences as unsigned bytes from left to right; if one key is a byte-prefix of the other, the shorter byte sequence sorts first.
3. Recursively apply the same rule to nested objects.

Implementations MUST NOT rely on host-language map iteration order. Implementations MUST NOT use locale-aware sorting, Unicode collation, ICU-dependent comparison, case-folded comparison, natural-language ordering, host-runtime string comparison where it differs from raw UTF-8 byte comparison, or any platform-default sort for canonical key ordering.

### 2.3 Array ordering

Arrays MUST be deterministic:

* `players` sorted by `player_id`.
* `entities` sorted by `entity_id`.
* `positions` sorted by `entity_id`.
* `health` sorted by `entity_id`.
* `labels` sorted by byte-lexicographic order of each label's UTF-8 encoding.
* Extension arrays MUST either preserve explicitly consensus-defined order or be sorted by a documented extension rule.

Arrays whose order is semantically unordered MUST be sorted before canonicalization. Duplicate keys in arrays that declare uniqueness MUST be rejected.

### 2.4 Consensus value domain

Canonical consensus state MUST use only:

* JSON objects.
* JSON arrays.
* UTF-8 strings.
* Integers in the exact range `[-9223372036854775808, 18446744073709551615]` unless a field narrows the range.
* Booleans where explicitly allowed.
* `null` where explicitly allowed.

Canonical consensus state MUST NOT contain:

* Floating-point numbers.
* `NaN`, `Infinity`, or `-Infinity`.
* Exponent notation.
* Timestamps, wall-clock dates, local time zones, or clock-derived values.
* Random values, nonces, UUIDs, or generated identifiers unless the value already exists as persisted consensus state and is deterministically derived by the ruleset.
* Platform-dependent serialized values such as pointers, memory addresses, hash-map seeds, enum discriminants without schema, or native binary blobs.

### 2.5 JSON syntax restrictions

Canonical JSON MUST be emitted without insignificant whitespace.

Required syntax:

* Object delimiters: `{` and `}`.
* Array delimiters: `[` and `]`.
* Name separator: `:`.
* Value separator: `,`.
* No spaces, tabs, carriage returns, or newlines outside JSON strings.
* Strings escaped according to JSON rules. Escapes MUST be deterministic; emit the shortest valid JSON escape for quotation mark, reverse solidus, and control characters. Non-control Unicode characters SHOULD be emitted directly as UTF-8 rather than `\u` escapes.
* Duplicate object keys MUST be rejected before canonicalization.

## 3. Canonical Byte Representation

The version `1` canonical byte pipeline is:

```text
ArenaState
  -> validate schema and value domain
  -> normalize required optional empties and nullable fields
  -> sort arrays and dynamic object keys
  -> emit canonical JSON
  -> UTF-8 encode canonical JSON
  -> canonical_bytes
```

### 3.1 Validation

Before encoding, an implementation MUST validate that:

1. The input is an `ArenaState` object with exactly the fields defined in Section 1.1.
2. All required nested fields are present.
3. All integer values are in range and are represented without floating-point conversion.
4. All string values are valid Unicode.
5. All arrays that require uniqueness contain no duplicates.
6. All root and hash strings are lowercase hexadecimal with exactly 64 characters unless nullable and set to `null`.
7. `tick == 0` if and only if `continuity.previous_state_root == null`, unless a migration profile explicitly defines a non-genesis import state.

Invalid states MUST NOT produce canonical bytes.

### 3.2 Normalization

Normalization MUST NOT alter consensus meaning. It is limited to:

* Sorting arrays according to Section 2.3.
* Sorting dynamic object keys recursively by byte-lexicographic UTF-8 key comparison.
* Converting absent optional collections from parser-level absence into their required explicit canonical empty values only if the schema version being decoded permits that compatibility behavior. Version `1` canonical output MUST always include the explicit field.

Normalization MUST NOT round numbers, coerce strings into numbers, generate identifiers, insert timestamps, or derive missing roots.

### 3.3 Canonical JSON emission

Canonical JSON emission MUST produce exactly one valid UTF-8 JSON text for each valid `ArenaState`.

Example shape with placeholder values:

```json
{"schema_version":1,"world_id":"world-1","arena_id":"arena-1","tick":0,"players":[],"entities":[],"positions":[],"health":[],"receipts":{"receipt_root":"0000000000000000000000000000000000000000000000000000000000000000","receipt_count":0,"last_receipt_hash":null},"continuity":{"continuity_root":"0000000000000000000000000000000000000000000000000000000000000000","previous_state_root":null,"replay_root":"0000000000000000000000000000000000000000000000000000000000000000","migration_root":null,"epoch":0},"metadata":{"ruleset_id":"default","ruleset_version":1,"created_by":null,"labels":[],"extensions":{}}}
```

The line above is illustrative. Implementations MUST derive canonical JSON from the rules in this specification, not by copying example formatting.

### 3.4 Binary canonicalization

Version `1` uses canonical JSON as the canonical byte source. A future binary canonicalizer MAY be introduced only by incrementing `schema_version` or by defining an explicit `canonicalizer_version` envelope. Binary encodings MUST NOT be substituted for version `1` JSON canonical bytes.

### 3.5 UTF-8 byte-order fixture

The fixture `docs/proofs/fixtures/utf8-key-ordering.json` is a conformance test vector for dynamic object key ordering. It contains extension keys whose ordering differs between raw UTF-8 byte comparison and UTF-16 or locale-sensitive comparison. A conforming implementation MUST canonicalize the fixture's dynamic `metadata.extensions` object with this key order:

```json
["A","a","é","","𐀀"]
```

The expected order is derived only from the raw UTF-8 byte sequences: `41`, `61`, `c3 a9`, `ee 80 80`, and `f0 90 80 80`. Locale-aware collation, Unicode collation, ICU-dependent sorting, case folding, and platform-default string sorting MUST NOT be used to reorder these keys.

## 4. State Root Generation

The state root is the SHA-256 digest of the canonical bytes:

```text
state_root = SHA256(canonical_bytes)
```

Requirements:

* Hash algorithm: SHA-256 as specified by FIPS 180-4.
* Input bytes: exactly the UTF-8 bytes emitted by Section 3.
* Byte ordering: the SHA-256 digest is the 32-byte output in standard digest order.
* Output encoding: lowercase hexadecimal string of the 32 digest bytes, exactly 64 characters.
* No prefix, length header, domain tag, salt, or trailing newline is included in `state_root` version `1`.

## 5. World Hash Generation

The world hash combines the state commitment with receipt and continuity commitments:

```text
world_hash = SHA256(state_root_bytes || receipt_root_bytes || continuity_root_bytes)
```

Where:

* `state_root_bytes` is the 32-byte binary digest represented by the `state_root` hex string.
* `receipt_root_bytes` is the 32-byte binary digest represented by `receipts.receipt_root`.
* `continuity_root_bytes` is the 32-byte binary digest represented by `continuity.continuity_root`.
* `||` means byte concatenation.
* The exact concatenation order is `state_root`, then `receipt_root`, then `continuity_root`.
* The input length to SHA-256 is exactly 96 bytes.
* The output encoding is lowercase hexadecimal, exactly 64 characters.

Implementations MUST decode each hex root to bytes before concatenation. Concatenating ASCII hex strings is not canonical for `world_hash` version `1`.

## 6. Canonicalizer Invariants

### Invariant 1: Same `ArenaState`, same canonical bytes

For every valid `ArenaState` `S`, any two conforming implementations MUST emit byte-identical `canonical_bytes(S)`.

### Invariant 2: Same canonical bytes, same state root

For any byte string `B`, `SHA256(B)` is deterministic. Therefore, if two implementations produce the same canonical bytes, they MUST produce the same `state_root`.

### Invariant 3: Different `ArenaState`, different state root

If two valid states differ in any consensus field after canonical normalization, their canonical bytes MUST differ. Their state roots are expected to differ by SHA-256 collision resistance. A cryptographic collision is outside the protocol validity model but MUST be treated as a root-integrity failure if discovered.

### Invariant 4: Replay reproduces canonical bytes

A deterministic replay from genesis or from a certified checkpoint MUST reproduce the same `ArenaState` and therefore the same canonical bytes for each tick. Replay certification MUST compare canonical bytes or roots at deterministic checkpoints, not host-native object layouts.

### Invariant 5: Canonicalization is infrastructure independent

Canonicalization MUST NOT depend on:

* CPU architecture or endianness.
* Operating system.
* Compiler version.
* Standard-library map iteration order.
* Locale or timezone.
* Filesystem ordering.
* Thread scheduling.
* Network timing.
* Hardware random-number generators.

Any state transition that depends on those values MUST resolve them before persistence through deterministic, ruleset-defined inputs.

## 7. Proof Targets

The canonicalizer specification is the foundation for progressive verification systems.

### Tier 1: Replay Certification

Objective: prove or check that replaying a recorded input stream reconstructs the same canonical state sequence.

Required statement:

```text
Replay(genesis_state, inputs[0..N]) -> canonical_bytes[N]
SHA256(canonical_bytes[N]) == state_root[N]
```

Certification artifacts SHOULD include the replay input root, selected tick roots, terminal `state_root`, `receipt_root`, `continuity_root`, and `world_hash`.

### Tier 2: Root Integrity Proof

Objective: prove that a published state root is exactly the SHA-256 hash of the canonical state.

Required statement:

```text
state_root == SHA256(canonical_state)
```

For all valid `ArenaState` values, the proof system MUST enforce:

1. Schema validity.
2. Deterministic ordering, including byte-lexicographic UTF-8 comparison for every dynamic object key.
3. Integer-only value domain.
4. Canonical JSON byte construction.
5. SHA-256 digest correctness.

### Tier 3: Transition Proof

Objective: prove a valid state transition.

Required statement:

```text
State(N) + Input(N) -> State(N+1)
```

A transition proof MUST bind:

* `state_root[N]`.
* Canonical input commitment for `Input(N)`.
* Ruleset or module commitment.
* Canonical bytes for `State(N+1)`.
* `state_root[N+1]`.

The proof MUST show that the ruleset transition function produces `State(N+1)` from `State(N)` and `Input(N)` and that `state_root[N+1] == SHA256(canonical_bytes(State(N+1)))`.

### Tier 4: Recursive Continuity Proofs

Objective: aggregate many transition proofs into compact continuity proofs suitable for federation, restore, migration, and ZK verification.

Required statement:

```text
ContinuityProof(0..N) verifies:
  state_root[0]
  transition_valid[0..N-1]
  state_root[N]
  receipt_root[N]
  continuity_root[N]
  world_hash[N]
```

Recursive proofs SHOULD preserve the exact root ordering from Sections 4 and 5. They MUST also preserve byte-lexicographic UTF-8 key comparison for any dynamic object key admitted into proof witness data. They MUST NOT introduce an alternate canonicalization path.

## Implementation Conformance Checklist

A conforming implementation MUST:

1. Reject invalid `ArenaState` values instead of best-effort serializing them.
2. Emit canonical JSON with schema-defined field order and dynamic keys sorted by byte-lexicographic UTF-8 comparison.
3. Encode canonical JSON as UTF-8 bytes with no trailing newline.
4. Compute `state_root` as SHA-256 over exactly those bytes.
5. Compute `world_hash` as SHA-256 over `state_root_bytes || receipt_root_bytes || continuity_root_bytes`.
6. Produce identical roots across independent implementations for the same valid state.
