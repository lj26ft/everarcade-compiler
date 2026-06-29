# Bottleneck Report

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

The lightweight local run did not reach catastrophic saturation. Disk fsync write throughput is the lowest sampled throughput class and should be treated as the first boundary to expand with larger Phase II campaigns; CPU receipt hashing remained repeatable at the sampled scales.

## Raw artifact references

- `.everarcade-continuum-phase-ii-review/artifacts/phase_ii_raw_results.json`

## Reproduction notes

Run the Phase II probes from the repository root and preserve raw outputs under `.everarcade-continuum-phase-ii-review/artifacts/`. Commit concise reports only; do not commit generated raw artifacts or regenerated archives.
