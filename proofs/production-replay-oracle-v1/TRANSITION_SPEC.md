# Production Replay Oracle Transition Spec v1

This pack specifies the Arena Vanguard / HotPocket arena replay transition rules at commitment level. An independent implementation can reproduce every published commitment from the genesis state and journal actions only; runtime-generated roots, receipts, hashes, checkpoints, and implementation artifacts are validation outputs, not replay inputs.

## Canonicalization and hashing

All hashes in this specification are lowercase hex SHA-256 digests.

`canonical(value)` is the UTF-8 string produced by the production canonical JSON encoder:

- `null`, booleans, numbers, and strings use `JSON.stringify(value)`.
- Arrays preserve element order and encode as `[canonical(item0),canonical(item1),...]` with no whitespace.
- Objects encode own enumerable fields sorted by UTF-8 key order and encode as `{"key":canonical(value),...}` with no whitespace.
- Hash input bytes are the UTF-8 bytes of `canonical(value)`, except `action_hash`, which hashes the UTF-8 bytes of `canonical(action)` directly and is therefore equivalent to `sha256(canonical(action))`.

## Common state and root model

The genesis `ArenaState` shape is:

```json
{
  "tick": 0,
  "players": {},
  "combat_events": [],
  "last_sequence": {},
  "commitments": []
}
```

Player records are objects with these fields and default values:

```json
{
  "id": "player-id",
  "x": 0,
  "y": 0,
  "health": 100,
  "connected": false,
  "score": 0
}
```

A successful transition appends exactly one journal entry and one receipt. The resulting commitment is appended to `state.commitments` and contains:

- `world_hash = sha256(canonical({ tick, players, combat_events }))`
- `receipt_root = sha256(canonical(array of receipt_hash strings in receipt sequence order))`
- `state_root = sha256(canonical(full post-action ArenaState before appending the new commitment))`
- `continuity_root = sha256(canonical({ state_root, receipt_root, world_hash, tick }))`

Rejected inputs throw before mutation, before journal append in the commitment-level oracle, and before receipt creation. The state and all roots remain equal to the pre-rejection latest commitment. Published journal documents may include a `rejected_inputs` evidence section, but those evidence records are not replay inputs and are excluded from receipt/root construction.

## Action transition rules

### `join`

- Input shape: `{ "action": "join", "player": "<non-empty string>" }`.
- Validation rules: action must be `join`; `player` must be a non-empty string.
- State fields changed: creates missing `players[player]` as the default player record, sets `connected` to `true`, increments `tick`, and writes `last_sequence[player] = tick`.
- Receipt emitted: accepted receipt with `mutation: "player_joined"` for first join or `"player_rejoined"` if the player already existed.
- Rejection behavior: invalid input shape rejects before mutation with no receipt, no journal entry, and no root update.

### `move`

- Input shape: `{ "action": "move", "player": "<non-empty string>", "direction": "north|south|east|west" }`.
- Validation rules: `player` must be a non-empty string; `direction` must be exactly one of `north`, `south`, `east`, or `west`; the player must be connected at execution time.
- State fields changed: updates `players[player].x/y` by direction (`north`: y - 1, `south`: y + 1, `east`: x + 1, `west`: x - 1), increments `tick`, and writes `last_sequence[player] = tick`.
- Receipt emitted: accepted receipt with `mutation: "player_moved"`.
- Rejection behavior: unsupported direction or disconnected player rejects before mutation with no receipt, no journal entry, and no root update.

### `attack`

- Input shape: `{ "action": "attack", "player": "<non-empty string>", "target": "<non-empty string>" }`.
- Validation rules: attacker `player` and `target` must be non-empty strings; attacker must be connected. A missing target player is materialized with default player fields before damage is applied.
- State fields changed: target `health` is reduced by 25 with a floor of 0, attacker `score` increases by 10, one combat event is appended, `tick` increments, and `last_sequence[player] = tick`.
- Receipt emitted: accepted receipt with `mutation: "player_attacked"`.
- Rejection behavior: malformed action or disconnected attacker rejects before mutation with no receipt, no journal entry, no combat event, and no root update.

### `score`

- Input shape: `{ "action": "score", "player": "<non-empty string>", "delta": <safe integer> }`.
- Validation rules: `player` must be a non-empty string; `delta` must be a JSON number that is a safe integer; player must be connected.
- State fields changed: adds `delta` to `players[player].score`, increments `tick`, and writes `last_sequence[player] = tick`.
- Receipt emitted: accepted receipt with `mutation: "player_scored"`.
- Rejection behavior: missing/non-integer delta or disconnected player rejects before mutation with no receipt, no journal entry, and no root update.

### `disconnect`

- Input shape: `{ "action": "disconnect", "player": "<non-empty string>" }`.
- Validation rules: action must be `disconnect`; `player` must be a non-empty string. The runtime materializes a missing player with default fields, then marks it disconnected.
- State fields changed: sets `players[player].connected = false`, increments `tick`, and writes `last_sequence[player] = tick`.
- Receipt emitted: accepted receipt with `mutation: "player_disconnected"`.
- Rejection behavior: malformed input rejects before mutation with no receipt, no journal entry, and no root update.

## Combat Event Specification

Combat events are stored only in `state.combat_events`. They are not stored separately and are included in `world_hash`, receipt `output.combat_events`, `state_root`, and `continuity_root` through those structures.

### Object structure

Each successful attack appends exactly one event object with this logical structure:

```json
{
  "tick": 3,
  "attacker": "alice",
  "target": "bob",
  "damage": 25,
  "target_health": 75
}
```

Fields are:

1. `tick` — JSON number, safe integer. This is the post-action tick: `tickOverride` from the journal entry `round` during replay, otherwise `state.tick + 1`.
2. `attacker` — JSON string. Equal to normalized `action.player`.
3. `target` — JSON string. Equal to normalized `action.target`.
4. `damage` — JSON number, fixed integer `25`.
5. `target_health` — JSON number, the target health after damage and floor-at-zero clamping.

The production insertion order is `tick`, `attacker`, `target`, `damage`, `target_health`. Hashing still uses canonical object key sorting, so implementations must not depend on insertion order for hashes; fixture comparisons that inspect JSON objects should use the listed order.

### Event insertion rules

- Successful attack: emit one event after target materialization, damage application, and attacker score increment, and before `after.tick` is assigned. The event `tick` still equals the post-action tick.
- Failed attack: emit no event if validation or execution fails. Examples include malformed target or disconnected attacker.
- Rejected input: emit no event and do not alter `combat_events`.
- Dead target: still emit one event for a successful attack against a target already at `health: 0`; `target_health` remains `0` and `damage` remains `25`.
- Ordering: append events to `combat_events` in accepted attack execution order. Non-attack actions do not insert placeholders.

## Receipt Specification

A receipt is generated for every accepted journal action and for no rejected action. Receipts are ordered by `sequence`, which starts at `1` for the first accepted journal entry and increments by `1` per accepted journal entry. Rejections do not consume a receipt sequence number.

### Receipt schema

The production receipt is the following object. Field order shown below is production insertion order; receipt hashing uses canonical key sorting.

```json
{
  "schema": "everarcade.hotpocket.arena-wrapper.v0.1.receipt",
  "execution_id": "arena-hotpocket-000001",
  "sequence": 1,
  "round": 1,
  "status": "accepted",
  "generated_at": "1970-01-01T00:00:00.000Z",
  "action_hash": "...",
  "state_before_hash": "...",
  "mutation": "player_joined",
  "state_root": "...",
  "receipt_hash": "...",
  "output": { "...": "..." },
  "receipt_root": "...",
  "world_hash": "...",
  "continuity_root": "..."
}
```

Field definitions:

- `schema`: string, always `everarcade.hotpocket.arena-wrapper.v0.1.receipt`.
- `execution_id`: string, `arena-hotpocket-` plus zero-padded six-digit sequence.
- `sequence`: number, accepted action sequence starting at 1.
- `round`: number, equal to the post-action tick. During replay this must equal the journal entry `round`.
- `status`: string, always `accepted` for emitted receipts.
- `generated_at`: string, always `1970-01-01T00:00:00.000Z` for deterministic replay.
- `action_hash`: `sha256(canonical(action))`, where `action` is the normalized action object.
- `state_before_hash`: `sha256(canonical(ArenaState before action))`.
- `mutation`: action-specific mutation string from the transition rules.
- `state_root`: final commitment `state_root`, after appending the commitment to `after.commitments`.
- `receipt_hash`: final receipt hash as specified below.
- `output`: accepted output snapshot.
- `receipt_root`: final cumulative receipt root including this receipt.
- `world_hash`: final post-action world hash.
- `continuity_root`: final post-action continuity root.

The `output` object has this production structure:

```json
{
  "accepted": true,
  "action": "join",
  "mutation": "player_joined",
  "tick": 1,
  "players": {},
  "combat_events": [],
  "state_root": "...",
  "receipt_root": "...",
  "world_hash": "...",
  "continuity_root": "..."
}
```

`players` and `combat_events` are deep copies of the post-action state at the time the commitment is made.

### Receipt generation rules

1. Apply the accepted action to produce `before`, `after`, and `mutation`.
2. Build `baseReceipt` with fields through `mutation`.
3. Build a temporary receipt by adding `state_root = sha256(canonical(after))` before the commitment is appended.
4. Compute a temporary `receipt_hash = sha256(canonical(tempReceipt))`.
5. Compute commitments from `after` and the prior receipts plus this temporary-hash receipt.
6. Append the commitment object to `after.commitments`.
7. Build `output` from the post-action state and commitment values.
8. Replace receipt roots and hashes with final values: set `output`, final `state_root`, `receipt_root`, `world_hash`, and `continuity_root`, then compute the final `receipt_hash` as specified below.

Accepted actions produce `status: "accepted"`. Failed actions and rejected inputs produce no receipt and do not change ordering or receipt roots.

## Receipt Hash Construction

The final receipt hash is:

```text
receipt_hash = sha256(canonical(receipt_without_final_receipt_hash))
```

Construction detail:

- Start with the final receipt object containing `output`, final `state_root`, final `receipt_root`, final `world_hash`, and final `continuity_root`.
- Exclude the final `receipt_hash` value by setting the `receipt_hash` field to JSON `undefined` in the JavaScript production spread object before canonicalization. In canonical object encoding this serializes the key as `"receipt_hash":undefined` because the production canonicalizer recursively returns `undefined` for that value and interpolates it into the object string.
- Hash the UTF-8 bytes of that canonical string.
- Store the result as lowercase hex in `receipt.receipt_hash`.

No fields other than the final value of `receipt_hash` are excluded.

## Receipt Root Construction

`receipt_root` is a deterministic ordered accumulator over receipt hashes:

```text
receipt_root = sha256(canonical([receipt_hash_1, receipt_hash_2, ..., receipt_hash_n]))
```

Rules:

- Empty set: `sha256(canonical([]))`, i.e. SHA-256 over the two bytes `[]`.
- Single receipt: `sha256(canonical([receipt_hash_1]))`.
- Multiple receipts: order by receipt `sequence` ascending, which is journal accepted-action order.
- Receipt hashes are lowercase hex strings in the array.
- No Merkle pairing, duplicate padding, byte reversal, or binary decoding is applied.

During receipt generation the commitment's `receipt_root` is computed using the temporary receipt hash from step 4 above. The final receipt hash then commits to that final receipt root. The next action's receipt root uses the previous receipt's final `receipt_hash` as stored in the receipt list.

## State Root and Continuity Root Construction

For each accepted action:

1. Compute `world_hash` from `{ tick, players, combat_events }` after the transition and before root return.
2. Compute `receipt_root` from cumulative receipt hashes as described above.
3. Compute a provisional `state_root = sha256(canonical(state))` over the post-action state before appending the commitment.
4. Compute `continuity_root = sha256(canonical({ state_root, receipt_root, world_hash, tick }))`.
5. Append `{ tick, state_root, receipt_root, world_hash, continuity_root }` to `state.commitments`.
6. The receipt's final `state_root` is the commitment `state_root` value. The next state's hash includes the appended commitment when it is used as `state_before_hash` for the next action.

## Commitment Construction Pipeline

The replay dependency graph is explicit and acyclic:

```text
Genesis
    ↓
Read journal accepted actions in order
    ↓
Validate and normalize each action
    ↓
Apply action transition
    ↓
Build Combat Events during accepted attack transitions
    ↓
Build World State (tick, players, combat_events, last_sequence, commitments)
    ↓
Build world_hash
    ↓
Generate Receipt object
    ↓
Build receipt_hash values
    ↓
Build receipt_root
    ↓
Build state_root
    ↓
Build continuity_root
    ↓
Append commitment to state.commitments
```

Hidden dependencies are forbidden. Replay must not read published `state_root`, `receipt_root`, `world_hash`, `continuity_root`, `receipt_hash`, checkpoints, receipt files, runtime state snapshots, or implementation-specific artifacts as inputs. Those values may only be used after replay to assert byte-identical validation.

## Independent Replay Verification Fixtures

Permanent conformance fixtures live under `proofs/production-replay-oracle-v1/conformance-fixtures/`:

- Fixture A: `combat-events.json` verifies combat event contents and ordering.
- Fixture B: `receipt-single.json` verifies a single receipt, `receipt_hash`, and `receipt_root`.
- Fixture C: `receipt-multiple.json` verifies multiple receipt ordering and cumulative `receipt_root`.
- Fixture D: `full-replay.json` verifies final `world_hash`, `receipt_root`, `state_root`, and `continuity_root` from genesis and journal only.

The validation command is:

```bash
node proofs/production-replay-oracle-v1/verify-independent-replay-oracle-v1.mjs
```

The verifier implements this specification directly and compares the independently derived outputs to the published expected roots after replay. It does not consume runtime-generated commitments during replay.
