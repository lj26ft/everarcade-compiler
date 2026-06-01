# Runtime Authority Boundary Audit

## Audit result

EverArcade is correctly oriented around a record-first authority model, but live deployment should continue to treat renderer, history, federation, and external settlement domains as scaffold-level unless a specific implementation is wired to deterministic records and recovery tests.

## Canonical authority chain

```text
Rustrigs emit records
Runtime applies records
Replay records history
Checkpoints preserve recovery
HotPocket state stores authority
XRPL/Xahau settlement remains external
GPU/render state is projection-only
```

## Boundary assessment

| Boundary | Expected rule | Current posture | Deployment recommendation |
| --- | --- | --- | --- |
| Rustrigs | Rustrigs emit deterministic records, not direct world mutations. | Rustrig crates and validation tests exist; the safest deployment stance is to accept only records through runtime validation. | Keep rustrig APIs narrow: emit canonical records, reject host I/O and nondeterministic clocks. |
| Runtime | Runtime applies accepted records in deterministic order. | Execution-core tests cover rustrig runtime, operator control plane, and EverNode deployment surfaces. | Treat runtime as the only applier of records; avoid bypass writers into `state/world/`. |
| Replay | Replay is append-only history. | Replay docs and tests emphasize replay roots, replay recovery, and anchors. | Require replay append before or atomically with world root updates. |
| Checkpoints | Checkpoints accelerate restore and preserve recovery continuity. | Checkpoint restore and recovery docs exist; deployment tests assert checkpoint continuity. | Continue checkpoint lineage validation and reject checkpoints not tied to replay cursors. |
| HotPocket state | HotPocket `state/` is the authority folder. | The repo has multiple runtime folders; a canonical `state/` model is now documented. | Deployment scripts should map all runtime authority paths into the canonical HotPocket `state/` layout. |
| XRPL/Xahau | Settlement remains external. | Anchor payload tests mark external settlement as required. | Do not place private keys or live submission queues in HotPocket state. |
| GPU/render | Rendering is derived projection only. | Renderer/projection docs exist; runtime domains remain scaffold-level. | Keep GPU output outside authority and verify it against replay/checkpoint roots. |

## What must be true before live authority

- Only runtime acceptance code may write authoritative world, replay, checkpoint, receipt, package, rustrig, anchor, and operator records.
- Every rustrig-visible side effect must be represented as a canonical record before it changes world state.
- Every checkpoint must identify the replay cursor and roots it summarizes.
- Every external anchor must begin as an intent record and end as a verified receipt record; signing/submission stays outside consensus.
- GPU/projection services must be restartable from replay/checkpoint data and must not write into authority.

## Launch risk

The primary risk is not a missing concept; it is accidental authority leakage from scaffolds. Before live deployment, scripts and operator docs should consistently name the canonical `state/` paths and reject writes from renderer, Studio cache, external settlement workers, and host-local automation into those authority directories.
