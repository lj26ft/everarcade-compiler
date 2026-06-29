# Continuum Boundary Benchmark Phase II: Determinism Harness

Determinism is a protocol benchmark. A public claim of replayable forever or verifiable forever requires repeated byte-identical roots across runs, processes, and architectures where available.

## Required invariant

For the same world, inputs, receipts, and checkpoints, every run must reproduce identical:

- `state_root`
- `receipt_root`
- `continuity_root`
- `world_hash`

## Run dimensions

| Dimension | Minimum campaign |
| --- | --- |
| Multiple runs | Re-run the same fixture at least 10 times on one host. |
| Multiple processes | Run independent process invocations over the same fixtures. |
| Multiple architectures | Compare roots across available CPU architectures before broad claims. |
| Checkpoint replay | Restore each checkpoint interval and compare terminal roots. |
| Genesis replay | Replay from genesis and compare against checkpoint-derived roots. |

## Failure policy

Any root mismatch is a protocol failure until explained. Preserve the local raw inputs and outputs under `.everarcade-continuum-phase-ii-review/artifacts/determinism/`, but commit only the summarized finding in `reports/determinism_report.md`.
