# Deterministic Execution Proof v0.1

Deterministic Execution Proof v0.1 proves that a booted EverArcade runtime package can accept a canonical operator input, execute deterministic runtime work after boot, mutate state, emit execution evidence, and replay the resulting journal back to the same execution root.

This proves deterministic runtime execution.

It does not yet prove interactive gameplay, multiplayer networking, real WASM game execution, or renderer-driven gameplay.

## Input flow

The operator-facing runtime command is:

```bash
cargo run -q -p everarcade-runtime --bin runtime -- \
  execute-proof <runtime-root> <world-id> <package-path>
```

The creator-facing command is:

```bash
node creator-sdk/cli/everarcade.mjs execute-local \
  --project <project> \
  --runtime-root <runtime-root>
```

`execute-local` ensures a runtime package exists, starts the local runtime, submits the proof input, executes the proof command, and validates replay.

## Canonical proof input

The canonical proof input is serialized with stable struct field order:

```json
{
  "player_id": "audit-player",
  "action": "move",
  "direction": "north",
  "sequence": 1
}
```

The runtime hashes the canonical JSON bytes with SHA-256. That stable input hash is written into the receipt, journal entry, and replay proof. Runtime ordering is replay-safe because the input queue assigns monotonically increasing sequence numbers and the proof executes exactly one queued input for tick `1` in a fresh proof root.

## Tick execution

The proof command invokes the existing runtime execution engine:

1. `RuntimeLoop::submit_input(...)` enqueues the canonical proof input.
2. `RuntimeLoop::run_ticks(1)` executes exactly one tick.
3. `RuntimeLoop::execute_tick(...)` pops the input, mutates runtime state with the input hash, computes a new state root, writes a receipt, appends a journal entry, and updates runtime health.

PASS requires:

- `ticks_executed == 1`.
- The tick number is `1`.
- The state root after execution is different from the pre-execution root.
- Runtime tick metrics increased.

## State mutation model

For this proof milestone, the deterministic state transition commits to input evidence rather than game-WASM behavior:

```text
state_bytes := state_bytes || input_hash
state_root := sha256(state_bytes)
```

This is intentionally narrow. It proves deterministic runtime work after boot without claiming game semantics.

## Receipt model

The proof produces:

```text
receipts/receipt-00000000000000000001.json
```

Required fields are:

- `receipt_id`
- `tick`
- `input_hash`
- `state_root`
- `receipt_hash`
- `runtime_version`
- `world_id`
- `timestamp_or_epoch`

The receipt hash is deterministic. It hashes only stable fields: receipt id, tick, input hash, state root, runtime version, world id, and deterministic epoch/tick value. Wall-clock time is not part of `receipt_hash`.

## Journal model

The proof appends:

```text
journals/journal.jsonl
```

Each entry contains:

- `sequence`
- `state_root`
- `input_hash`
- `receipt_hash`
- hash-chain material

PASS requires journal length `>= 1` and an intact journal hash chain.

## Checkpoint model

The proof command creates and verifies a checkpoint after the one executed tick:

```text
checkpoints/checkpoint-00000000000000000001.json
```

The checkpoint records:

- checkpoint identifier
- checkpoint hash
- journal position
- state root
- state snapshot

The default runtime checkpoint interval may be larger than one tick. Therefore the proof command explicitly creates a checkpoint after the proof tick so the milestone can prove checkpoint generation without executing unrelated extra ticks.

## Replay model

Replay consumes the generated journal entries and applies the same deterministic state transition from genesis:

```text
replay_state := replay_state || journal.input_hash
replay_root := sha256(replay_state)
```

PASS requires:

```text
replay_root == execution_root
```

The proof output reports:

```text
Replay Verification: PASS
```

## Runtime evidence tree

A successful proof writes evidence under the runtime root:

```text
runtime-root/
  reports/
    runtime_start_report.json
    deterministic-execution-proof.json
  receipts/
    receipt-00000000000000000001.json
  journals/
    journal.jsonl
  checkpoints/
    checkpoint-00000000000000000001.json
  replay/
    replay-proof.json
```

The runtime also maintains its existing world-scoped runtime directories under `worlds/<world-id>/`.

## PASS criteria

The validation report must contain:

```text
Input Submitted: PASS
Tick Executed: PASS
State Mutation: PASS
Receipt Generated: PASS
Journal Generated: PASS
Checkpoint Generated: PASS
Replay Root Generated: PASS
Replay Verified: PASS
```

The certification report must end with:

```text
Deterministic Execution Proof v0.1: PASS
```

## What is proven

- A runtime package can boot.
- The runtime accepts deterministic input after boot.
- The runtime executes one deterministic tick through `RuntimeLoop`.
- Runtime state mutates.
- A new state root is generated.
- A deterministic receipt is generated.
- A journal entry is appended.
- A checkpoint is generated and verified.
- Replay produces the same root as execution.
- Creator SDK users have a local command to run the proof.

## What is not proven

This proof does not yet prove:

- Interactive gameplay.
- Multiplayer networking.
- Real WASM game execution.
- Renderer-driven gameplay.
- Marketplace behavior.
- Federation behavior.
- XRPL settlement.
- GPU execution.
- Player gateway behavior.
- Commercial revenue behavior.
