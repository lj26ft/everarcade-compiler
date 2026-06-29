# Replay Interval Report

Status: Phase II local benchmark run completed.

## Scope

This report summarizes Continuum Boundary Benchmark Phase II probes for independent review. Raw local artifacts are excluded from git and were written under:

```text
.everarcade-continuum-phase-ii-review/artifacts/
```

## Host

- Commit: `e26abdb5214df040fa78c1dbc9780264d0332c3a`
- Timestamp UTC: `2026-06-29T12:32:24Z`
- Platform: `Linux-6.12.47-x86_64-with-glibc2.39`
- Python: `3.14.4`
- Logical CPUs: `3`

## Findings

Replay interval probes covered 100, 1000, 5000, 10000 synthetic receipts. Best sampled replay throughput was 291879 receipts/sec; longest sampled interval was 0.036172s for 10000 receipts.

## Raw artifact references

- `.everarcade-continuum-phase-ii-review/artifacts/phase_ii_raw_results.json`

## Reproduction notes

Run the Phase II probes from the repository root and preserve raw outputs under `.everarcade-continuum-phase-ii-review/artifacts/`. Commit concise reports only; do not commit generated raw artifacts or regenerated archives.
