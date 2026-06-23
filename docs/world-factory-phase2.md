# World Factory MVP Phase 2: Runtime Execution and Replay Verification

World Factory Phase 2 extends the Phase 1 artifact pipeline into a minimal deterministic runtime proof.

```text
Prompt
↓
Blueprint
↓
Contract Plan
↓
Generated world.evr
↓
Runtime Boot
↓
Execution
↓
Replay
↓
Verification
```

Phase 1 demonstrated that a Frontier Settlement blueprint and contract plan can generate a valid `world.evr` package. Phase 2 demonstrates that the generated world can also boot, execute deterministic ticks, emit receipts, replay from genesis, and verify matching commitment roots.

## Input world

Phase 2 uses the existing generated Frontier Settlement world:

```text
examples/world-factory/frontier-settlement/out/world.evr
```

It does not introduce a new template. The source project remains:

```text
examples/world-factory/frontier-settlement
├── world-blueprint.json
└── world-contract-plan.json
```

## Runtime profile mapping

The Phase 2 runtime supports the existing blueprint setting:

```text
runtime_profile = small
```

For this MVP, `small` maps to the following deterministic local assumptions:

| Assumption | Phase 2 mapping |
| --- | --- |
| World instances | Single world instance |
| Operators | Single local operator |
| Tick rate | Fixed deterministic tick advancement |
| Execution | Deterministic, timestamp-free, randomness-free local execution |
| Persistence | Local JSON files under `out/runtime/` |

These assumptions are intentionally narrow. They prove generated-world runtime execution and replay verification only; they do not claim EverNode deployment, multiplayer federation, AI simulation, or production-scale MMO operation.

## CLI flow

From the repository root, the default Frontier Settlement project can be generated, booted, run, and replayed with:

```bash
node creator-sdk/cli/everarcade.mjs world factory generate
node creator-sdk/cli/everarcade.mjs world factory boot
node creator-sdk/cli/everarcade.mjs world factory run --ticks 100
node creator-sdk/cli/everarcade.mjs world factory replay
```

The commands also accept an explicit project path:

```bash
node creator-sdk/cli/everarcade.mjs world factory boot \
  --project examples/world-factory/frontier-settlement
```

When the `everarcade` package binary is linked, the same commands can be run as:

```bash
everarcade world factory generate
everarcade world factory boot
everarcade world factory run --ticks 100
everarcade world factory replay
```

Replay prints `PASS` when replayed roots match runtime roots.

## Runtime boot

`world factory boot` loads the generated `world.evr` package, checks that its runtime profile is `small`, and writes a deterministic runtime package at:

```text
examples/world-factory/frontier-settlement/out/runtime/
```

The bootstrap world state is minimal:

```json
{
  "world_id": "frontier-settlement-demo",
  "tick": 0,
  "settlements": [],
  "market": {},
  "governance": {}
}
```

Boot creates these runtime files:

```text
out/runtime/world-state.json
out/runtime/journal.json
out/runtime/receipts.json
out/runtime/runtime-status.json
out/runtime/runtime-profile.json
```

## Runtime execution

`world factory run --ticks 100` advances the deterministic runtime by 100 ticks. Each tick performs one minimal action:

```text
world.tick
```

For each tick, the runtime appends:

- a journal entry in `out/runtime/journal.json`
- a deterministic receipt in `out/runtime/receipts.json`
- updated commitment roots in `world-state.json` and `runtime-status.json`

Journal entries contain:

```json
{
  "tick": 1,
  "action": "world.tick",
  "receipt_hash": "sha256:...",
  "world_hash": "sha256:..."
}
```

Receipts contain tick-indexed deterministic data only. They include no wall-clock time and no randomness.

## Commitments

Phase 2 reuses the repository's existing World Factory SHA-256 commitment style: canonical JSON content is hashed and represented as `sha256:<hex>`.

The runtime computes:

- `state_root` from the minimal world state
- `receipt_root` from deterministic receipts
- `world_hash` from the generated package identity, runtime identity, contract hash, and world id
- `continuity_root` from the state root, receipt root, world hash, and journal

These roots are surfaced in:

```text
out/runtime/world-state.json
out/runtime/runtime-status.json
out/runtime/world-factory-runtime-report.json
```

## Replay verification

`world factory replay` replays the journal from genesis, regenerates receipts, and recomputes:

```text
state_root
receipt_root
world_hash
continuity_root
```

Replay verifies:

```text
replayed state_root      == runtime state_root
replayed receipt_root    == runtime receipt_root
replayed world_hash      == runtime world_hash
replayed continuity_root == runtime continuity_root
```

If all roots match, replay writes:

```json
{
  "world_id": "frontier-settlement-demo",
  "ticks_executed": 100,
  "state_root": "sha256:...",
  "receipt_root": "sha256:...",
  "world_hash": "sha256:...",
  "continuity_root": "sha256:...",
  "replay_status": "PASS"
}
```

to:

```text
out/runtime/world-factory-runtime-report.json
```

and prints:

```text
PASS
```

## Proof claim boundaries

Phase 2 supports claims about:

- Generated world runtime boot
- Deterministic execution
- Receipt production
- Replay verification
- World Factory execution flow

Phase 2 does not claim:

- EverNode deployment
- Multiplayer federation
- AI simulation
- Production-scale MMO behavior

## Future phases

Future phases are intentionally documented but not implemented here:

```text
Phase 3: Generated world → EverNode deployment
Phase 4: Generated world → Operator attestation
Phase 5: Generated world → One-click launch
```
