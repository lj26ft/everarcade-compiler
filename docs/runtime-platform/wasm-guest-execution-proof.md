# WASM Guest Execution Proof v0.1

## Scope

This proof demonstrates that an EverArcade runtime package can carry a real WebAssembly guest, that the runtime loads and instantiates that module, that a guest entrypoint executes, and that the guest output drives the runtime state transition, receipt, journal, and replay evidence.

This proves guest WASM execution.

It does not yet prove:

- interactive gameplay,
- multiplayer networking,
- renderer-driven gameplay,
- or production-grade game hosting.

## Canonical Guest Package Format

The proof package is generated from `contracts/arena-proof-contract/` and contains:

```text
manifest.json
world.wasm
world.json
```

`world.wasm` is built from the canonical guest contract and must begin with the WebAssembly binary magic bytes. `world.json` classifies the package as `wasm-guest-runtime-package`, not as a placeholder package.

## Guest Load Process

The runtime package loader validates:

1. package id and package version,
2. runtime compatibility,
3. `world.wasm` hash,
4. `sha256:<wasm_hash>` signature,
5. package hash.

The guest execution command then validates that `world.wasm` is a WebAssembly binary, compiles it with the existing deterministic Wasmtime configuration, instantiates it, resolves exported `memory` and `alloc`, and invokes `everarcade_guest_execute` with a deterministic JSON invocation.

## Guest Execution Flow

The canonical guest performs the proof sequence:

```text
PlayerJoin
PlayerMove north
ScoreUpdate +1
```

The current guest returns the final deterministic output in one invocation. The runtime does not synthesize the output; it reads the bytes returned by the guest module from guest memory.

## Guest Output Model

Guest output is serialized JSON with these required fields:

```json
{
  "action": "PlayerJoin+PlayerMove+ScoreUpdate",
  "player_id": "player-1",
  "position": { "x": 0, "y": 1 },
  "score": 1
}
```

The runtime parses this output, canonicalizes it through `serde_json`, and computes `guest_output_hash` as SHA-256 over the canonical serialized output.

## State Mutation Model

Runtime state for this proof is a deterministic `GuestRuntimeState` derived directly from guest output:

- `guest_id`
- `guest_hash`
- `action`
- `player_id`
- `position`
- `score`
- `guest_output_hash`

The proof requires `state_root_changed == true` and `state_mutation_origin == "guest_output"`.

## Receipt Model

The receipt contains guest execution references:

- `guest_id`
- `guest_hash`
- `guest_output_hash`
- `state_root`
- `receipt_hash`

The receipt hash is deterministic over receipt id, tick, guest output hash, state root, runtime version, world id, and deterministic epoch.

## Journal Model

The journal entry records:

- guest invocation,
- guest output hash,
- state root,
- receipt hash,
- guest id,
- guest hash.

The journal remains hash chained using deterministic sequence, previous hash, state root, input hash, receipt hash, and timestamp/epoch.

## Replay Model

Replay reconstructs `GuestRuntimeState` from the guest output recorded in the journal, recomputes the replay root, and requires:

```text
replay_root == state_root
Replay Verification: PASS
```

The replay proof is written to:

```text
runtime-root/replay/guest-replay-proof.json
```

## Runtime Evidence

A successful run writes:

```text
runtime-root/
  guest/guest-execution.json
  receipts/receipt-00000000000000000001.json
  journals/journal.jsonl
  replay/guest-replay-proof.json
```

These artifacts include `guest_hash`, `guest_output_hash`, `state_root`, and `replay_root`.

## PASS Criteria

Validation passes only when all sections are present:

```text
Guest Build: PASS
Guest Package: PASS
Guest Load: PASS
Guest Execute: PASS
Guest State Mutation: PASS
Guest Receipt Generation: PASS
Guest Journal Generation: PASS
Guest Replay Generation: PASS
Guest Replay Verification: PASS
```

Certification passes only when the final line is:

```text
WASM Guest Execution Proof v0.1: PASS
```

## Limitations

This is a credibility proof for execution of guest code through the runtime pipeline. It intentionally excludes renderer UI, player input loops, networking, settlement, marketplace, federation, GPU execution, and commercial hosting claims.
