# Continuum Boundary Benchmark Phase II: GPU Exploration

GPU work is exploratory and is not required for correctness. The benchmark should determine whether a 24 GB AMD GPU can accelerate verification workloads without transfer overhead dominating the result.

## Experiment queue

- Hash acceleration.
- Proof generation.
- Parallel replay.
- Parallel state hashing.
- Parallel archive verification.

## Required measurements

For each experiment, record CPU baseline time, GPU compute time, host-device transfer time, total wall time, memory pressure, correctness comparison, and whether the result is a practical win. Summaries belong in `reports/gpu_report.md`; raw captures remain local under `.everarcade-continuum-phase-ii-review/artifacts/gpu/`.
