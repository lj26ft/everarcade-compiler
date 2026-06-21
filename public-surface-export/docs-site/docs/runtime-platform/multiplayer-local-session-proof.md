# Multiplayer Local Session Proof v0.1

This proof demonstrates deterministic multiplayer gameplay inside one local EverArcade runtime session.

## Session lifecycle

The Creator SDK command packages the Arena template and launches the runtime command `multiplayer-local-session`. The runtime creates `session-0001`, executes ordered gameplay inputs, writes receipts and journal entries, then replays the journal to verify the final session root.

## Player registration

The proof registers two players in the same session:

```json
{ "player_id": "player-a", "action": "join" }
{ "player_id": "player-b", "action": "join" }
```

The Arena state includes `session_id`, `tick`, `players`, `positions`, `health`, `scores`, `events`, and `player_count`. The proof requires `player_count == 2`.

## Shared state

Both players mutate one shared Arena state object. Player A and Player B have independent entries in `players`, `positions`, `health`, and `scores`. The session evidence is written to `sessions/session-0001.json` with `session_id`, `player_count`, `players`, and `tick`.

## Interaction model

The deterministic multiplayer sequence is:

1. Player A joins.
2. Player B joins.
3. Player A moves north.
4. Player B moves south.
5. Player A attacks Player B.
6. Player A receives a score update.

The attack subtracts deterministic health from Player B and records the interaction in state events.

## Receipt model

Each multiplayer action writes a runtime receipt containing `player_id`, `session_id`, `action`, `state_root`, and `receipt_hash`. Receipts also include guest execution evidence (`guest_hash` and `guest_output_hash`) for the Arena Guest WASM package used by the local proof.

## Journal model

Each gameplay journal entry contains `tick`, `player_id`, `action`, `state_root`, `receipt_hash`, and the canonical gameplay input. The journal records actions from both players in tick order.

## Replay model

Replay reconstructs Arena state from the multiplayer journal entries and compares the replay root with the final session root. The proof writes `replay/multiplayer-replay-proof.json` and passes only when `replay_root == final_session_root`.

Expected result:

```text
Multiplayer Replay Verification: PASS
Multiplayer Local Session: PASS
```

## Limitations

This proves multiplayer gameplay inside a local runtime.

It does not prove:

- network transport,
- federation synchronization,
- Evernode deployment,
- or production multiplayer hosting.
