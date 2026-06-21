# Playable Local Game Proof v0.1

Playable Local Game Proof v0.1 proves that a single local Arena session can stay alive across multiple gameplay ticks while accepting player actions, evolving authoritative runtime state, writing receipts, appending a journal, and verifying replay back to the final session root.

## Command

```bash
node creator-sdk/cli/everarcade.mjs play-local \
  --template arena \
  --project <arena-project> \
  --runtime-root <runtime-root>
```

The command packages the Arena template, launches the local runtime session command, submits canonical gameplay actions, advances at least five ticks, writes evidence under the runtime root, and prints:

```text
Playable Local Game: PASS
```

## Session lifecycle

1. The Creator SDK builds and packages the Arena template into a runtime package.
2. The runtime boots the package and creates `session-0001`.
3. The local session processes ordered gameplay inputs instead of a one-shot proof input.
4. Runtime state persists between actions as `gameplay/arena-state.json`.
5. The final session record is written at `sessions/session-0001.json` with `session_id`, `tick`, `player_count`, and `state_root`.

## Player actions

The proof submits these canonical player actions in order:

```json
{ "player_id": "player-1", "action": "join" }
{ "player_id": "player-1", "action": "move", "direction": "north" }
{ "player_id": "player-1", "action": "attack", "target": "dummy" }
{ "player_id": "player-1", "action": "score_update", "score_delta": 5 }
```

A fifth heartbeat tick proves continued progression after the score update.

## Arena session state

Arena state includes:

- `session_id`
- `tick`
- `players`
- `positions`
- `health`
- `scores`
- `events`

The join action adds `player-1`; movement changes the player's position; attack reduces dummy health and adds gameplay score; score update applies an additional gameplay score delta.

## Tick progression and state evolution

The local session executes ticks 1 through 5. Every gameplay input produces a deterministic state root, and the session transcript records state roots by tick so validation can prove that state evolved over time instead of producing one isolated proof tick.

## Receipts

Each accepted gameplay action writes a receipt containing runtime and gameplay evidence, including:

- `session_id`
- `tick`
- `player_count`
- `state_root`
- `receipt_hash`
- `action`
- `player_id`

Receipts are copied into the top-level `receipts/` evidence directory for audit convenience.

## Journal

Each gameplay action appends a journal entry to `journals/journal.jsonl`. The journal records ordered ticks, input hashes, receipt hashes, state roots, player IDs, actions, and the canonical gameplay input used for replay.

## Replay

Replay rebuilds Arena state from journaled gameplay inputs and compares the replay root to the final session state root. Passing validation requires:

```text
Gameplay Replay Verification: PASS
```

and:

```text
replay root == final session root
```

## Evidence layout

A passing proof writes local evidence under the runtime root:

```text
runtime-root/
  sessions/
    session-0001.json
  gameplay/
    arena-state.json
    session-transcript.json
  receipts/
    receipt-*.json
  journals/
    journal.jsonl
  replay/
    gameplay-replay-proof.json
```

## Validation and certification

Run:

```bash
bash scripts/validate_playable_local_game.sh
bash scripts/certify_playable_local_game.sh
```

The validation report includes PASS sections for session start, join, move, attack, score update, multi-tick progression, receipt stream, journal stream, replay root generation, and replay verification. The certification report ends with:

```text
Playable Local Game Proof v0.1: PASS
```

## Limitations

This proves local gameplay interaction.

It does not yet prove:
multiplayer networking,
public testnet operation,
renderer-driven gameplay,
or production hosting.
