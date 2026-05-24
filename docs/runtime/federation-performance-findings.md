# Federation Performance Findings

## Measured
Simulation scripts record replay sync, archive verification, restoration, divergence analysis, federation recovery, and topology serialization costs in `federation/reports/`.

## Inferred
Costs scale mostly with node count and archive size scenario (`small`, `medium`, `large`, `multi-era`).

## Speculative
Future real-network overheads (latency, bandwidth contention) may increase absolute timings, but deterministic ordering constraints should still hold.
