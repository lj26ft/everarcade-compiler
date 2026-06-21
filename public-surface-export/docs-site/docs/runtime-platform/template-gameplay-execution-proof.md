# Template Gameplay Execution Proof v0.1

## Scope

This proof exercises the official Creator SDK `arena` template through the deterministic EverArcade runtime package path. It upgrades the evidence class from synthetic audit input execution to template gameplay execution.

This proves template gameplay execution.

It does not yet prove:
multiplayer networking,
renderer-driven gameplay,
real WASM guest execution,
or player-facing playability.

## Arena actions

The Arena template declares canonical deterministic gameplay records in `creator-sdk/templates/arena/src/game.js`:

- `PlayerJoin` records `{ "player_id": "player-1", "action": "join" }`.
- `PlayerMove` records `{ "player_id": "player-1", "action": "move", "direction": "north" }`.
- `PlayerAttack` records `{ "player_id": "player-1", "action": "attack", "target": "dummy" }`.
- `ScoreUpdate` records `{ "player_id": "player-1", "action": "score_update", "score_delta": 5 }`.

The records are serialized by the runtime as stable JSON structs with explicit sequence numbers so replay can apply them in deterministic order.

## Gameplay state

The runtime Arena state contains these required fields:

- `players`
- `positions`
- `health`
- `scores`
- `tick`

The initial state contains a deterministic `dummy` target. The proof joins `player-1`, moves that player north, attacks `dummy`, and applies a score update. The runtime writes the final state to:

```text
runtime-root/gameplay/arena-state.json
```

## Tick flow

The Creator SDK command is:

```bash
node creator-sdk/cli/everarcade.mjs execute-template \
    --template arena \
    --runtime-root <path>
```

The command packages the Arena template, starts the runtime against the package, submits the canonical gameplay inputs, executes one deterministic runtime tick per input, and writes runtime evidence.

The runtime command path is:

```text
Creator SDK execute-template
→ runtime execute-template-proof
→ Arena package load
→ gameplay input submission
→ deterministic tick execution
→ Arena state mutation
→ receipt emission
→ journal append
→ replay verification
```

## Receipts

Each gameplay action emits a runtime receipt under:

```text
runtime-root/receipts/receipt-*.json
```

Gameplay receipts include:

- `action`
- `player_id`
- `state_root`
- `receipt_hash`
- `tick`
- `world_id`

## Journal

The proof writes journal entries to:

```text
runtime-root/journals/journal.jsonl
```

Gameplay journal entries include:

- player id
- action
- tick
- state root
- receipt hash
- canonical gameplay input

Validation proves that join, move, and attack entries were recorded.

## Replay

Replay reconstructs Arena state from the journal gameplay inputs and compares the replay root to the execution root. The proof artifact is:

```text
runtime-root/replay/gameplay-replay-proof.json
```

PASS requires:

```text
replay root == gameplay execution root
Gameplay Replay Verification: PASS
```

## PASS criteria

`bash scripts/validate_template_gameplay_execution.sh` must report:

```text
Arena Package: PASS
Arena Join: PASS
Arena Move: PASS
Arena Attack: PASS
Arena State Mutation: PASS
Arena Receipt Generation: PASS
Arena Journal Generation: PASS
Arena Replay Generation: PASS
Arena Replay Verification: PASS
```

`bash scripts/certify_template_gameplay_execution.sh` must end with:

```text
Template Gameplay Execution Proof v0.1: PASS
```

## Limitations

This is a deterministic record-based gameplay proof for the official Arena template. It proves the template can drive runtime package loading, gameplay input sequencing, gameplay state mutation, receipts, journal entries, and replay verification.

It is not a playable game proof because there is no player-facing interactive loop, no renderer-driven gameplay, no multiplayer networking, and no real WASM guest execution in this milestone.
