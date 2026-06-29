# Continuum Boundary Benchmark Phase II: Catastrophe and Adversarial Harness

This harness validates recovery and failure detection under crash, corruption, and adversarial history manipulation.

## Crash tests

Simulate process kill, power loss, out-of-memory, and disk-full conditions. Measure recovery time, checkpoint recovery, and journal recovery.

## Corruption tests

Corrupt receipts, checkpoints, proofs, snapshots, and archives. Verification must fail correctly and isolate the corrupted interval or artifact where possible.

## Adversarial tests

Attempt receipt reordering, checkpoint reordering, missing history, duplicate receipts, forged proofs, forged checkpoints, and partial archives. Record detection rate, failure mode, and time to detect.

## Reporting

Summaries belong in `reports/catastrophe_report.md`; generated corruptions and crash outputs are local artifacts under `.everarcade-continuum-phase-ii-review/artifacts/adversarial/`.
