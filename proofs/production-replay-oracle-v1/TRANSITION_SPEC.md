# Production Replay Oracle Transition Spec v1

This pack uses the live `ArenaHotPocketRuntime.process` path from `hotpocket-arena-wrapper/src/runtime.mjs`. State roots, receipt roots, world hashes, continuity roots, action hashes, receipt hashes, journal hashes, and genesis hashes are SHA-256 hashes over the runtime canonical JSON encoding: objects are sorted by UTF-8 key order, arrays retain order, primitive values use JSON encoding, and no whitespace is inserted before hashing.

## Common state and root model

The ArenaState shape is:

```json
{
  "tick": 0,
  "players": {},
  "combat_events": [],
  "last_sequence": {},
  "commitments": []
}
```

A successful transition appends exactly one journal entry and one receipt. The resulting commitment is appended to `state.commitments` and contains:

- `state_root = sha256(canonicalize(full ArenaState after appending the commitment))`
- `receipt_root = sha256(canonicalize(array of receipt_hash values in order))`
- `world_hash = sha256(canonicalize({ tick, players, combat_events }))`
- `continuity_root = sha256(canonicalize({ state_root, receipt_root, world_hash, tick }))`

Rejected inputs throw before receipt or journal append. The state and all roots remain equal to the pre-rejection latest commitment.

## `join`

- Input shape: `{ "action": "join", "player": "<non-empty string>" }`.
- Validation rules: action must be `join`; `player` must be a non-empty string.
- State fields changed: creates missing `players[player]` as `{ id, x: 0, y: 0, health: 100, connected: false, score: 0 }`, sets `connected` to `true`, increments `tick`, and writes `last_sequence[player] = tick`.
- Receipt emitted: accepted receipt with `status: "accepted"`, `mutation: "player_joined"` for first join or `"player_rejoined"` if the player already existed, canonical action hash, state-before hash, output snapshot, and live roots.
- Rejection behavior: invalid input shape rejects before mutation with no receipt, no journal entry, and no root update.
- Root update behavior: successful join recomputes `world_hash`, `receipt_root`, `state_root`, and `continuity_root`, then appends the commitment to state.

## `move`

- Input shape: `{ "action": "move", "player": "<non-empty string>", "direction": "north|south|east|west" }`.
- Validation rules: `player` must be a non-empty string; `direction` must be exactly one of `north`, `south`, `east`, or `west`; the player must be connected at execution time.
- State fields changed: updates `players[player].x/y` by direction (`north`: y - 1, `south`: y + 1, `east`: x + 1, `west`: x - 1), increments `tick`, and writes `last_sequence[player] = tick`.
- Receipt emitted: accepted receipt with `mutation: "player_moved"`, the canonical action hash, state-before hash, output snapshot, and live roots.
- Rejection behavior: unsupported direction or disconnected player rejects before mutation with no receipt, no journal entry, and no root update.
- Root update behavior: successful move recomputes all roots from the post-move ArenaState and cumulative receipt list.

## `attack`

- Input shape: `{ "action": "attack", "player": "<non-empty string>", "target": "<non-empty string>" }`.
- Validation rules: attacker `player` and `target` must be non-empty strings; attacker must be connected. A missing target player is materialized by the production runtime with default player fields before damage is applied.
- State fields changed: target `health` is reduced by 25 with a floor of 0, attacker `score` increases by 10, one combat event is appended, `tick` increments, and `last_sequence[player] = tick`.
- Receipt emitted: accepted receipt with `mutation: "player_attacked"`, canonical action hash, state-before hash, output snapshot containing updated players and combat events, and live roots.
- Rejection behavior: malformed action or disconnected attacker rejects before mutation with no receipt, no journal entry, and no root update.
- Root update behavior: successful attack recomputes all roots from the post-attack ArenaState, including the appended combat event and cumulative receipt list.

## `score`

- Input shape: `{ "action": "score", "player": "<non-empty string>", "delta": <safe integer> }`.
- Validation rules: `player` must be a non-empty string; `delta` must be a JSON number that is a safe integer; player must be connected.
- State fields changed: adds `delta` to `players[player].score`, increments `tick`, and writes `last_sequence[player] = tick`.
- Receipt emitted: accepted receipt with `mutation: "player_scored"`, canonical action hash, state-before hash, output snapshot, and live roots.
- Rejection behavior: missing/non-integer delta or disconnected player rejects before mutation with no receipt, no journal entry, and no root update.
- Root update behavior: successful score recomputes all roots from the post-score ArenaState and cumulative receipt list.

## `disconnect`

- Input shape: `{ "action": "disconnect", "player": "<non-empty string>" }`.
- Validation rules: action must be `disconnect`; `player` must be a non-empty string. The runtime materializes a missing player with default fields, then marks it disconnected.
- State fields changed: sets `players[player].connected = false`, increments `tick`, and writes `last_sequence[player] = tick`.
- Receipt emitted: accepted receipt with `mutation: "player_disconnected"`, canonical action hash, state-before hash, output snapshot, and live roots.
- Rejection behavior: malformed input rejects before mutation with no receipt, no journal entry, and no root update.
- Root update behavior: successful disconnect recomputes all roots from the post-disconnect ArenaState and cumulative receipt list.
