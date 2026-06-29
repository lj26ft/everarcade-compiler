# Continuum Boundary Benchmark Phase II Independent Review Addendum

This addendum gives independent reviewers a stable map for reproducing Phase II boundary benchmarks without committing generated artifacts.

## Local reproduction path

Reviewers should run Phase II campaigns from the repository root and write raw outputs to:

```text
.everarcade-continuum-phase-ii-review/artifacts/
```

That path is git-ignored. It is the expected local bundle location for raw receipts, checkpoints, snapshots, archives, profiler traces, GPU captures, synthetic worlds, corruption products, and crash outputs.

## Committed reviewer materials

- `benches/hardware/README.md`
- `benches/determinism/README.md`
- `benches/gpu/README.md`
- `benches/adversarial/README.md`
- `fixtures/README.md`
- `synthetic-worlds/README.md`
- `failure-fixtures/README.md`
- `reports/*_report.md`

## Review boundary

The pull request should contain benchmark definitions and concise report templates only. Produced artifacts should be regenerated locally or distributed through an explicit external artifact channel, never silently added to the PR.
