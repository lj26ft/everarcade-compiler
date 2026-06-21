# Deterministic Benchmarking Architecture

## Philosophy
EverArcade benchmarks exist to measure runtime behavior under deterministic constraints, not to change runtime semantics. All benchmark fixtures and harnesses must be replayable and produce the same consensus artifacts as non-benchmark execution.

**Benchmarks may measure runtime behavior, but benchmark measurements must never alter deterministic outputs.**

## Deterministic-safe rules
- Benchmarks must not mutate canonical receipt/state/replay encoding.
- Diagnostics are operator-only, non-consensus metadata and must remain excluded from deterministic hashes/state roots.
- Benchmark fixture ordering must be stable and deterministic.
- Any timing/allocator counters must be captured out-of-band from deterministic artifacts.

## Allowed nondeterministic diagnostics
- Wall-clock runtime measurements.
- Process-level memory usage snapshots.
- Allocator/profile samples enabled only in diagnostic mode.

These are allowed because they are stored in benchmark reports only and not consumed by consensus verification.

## Reproducibility expectations
- Pin repeat count and fixture seeds.
- Record host assumptions (CPU model, core count, RAM, storage class, kernel).
- Emit normalized JSON for CI and markdown summaries for humans.

## Measurement scope
- CPU: operation count, replay verification throughput, wall-clock diagnostics.
- Memory: estimated bytes, allocation bursts, memory page growth.
- Storage: replay archive bytes and checkpoint growth.

## Replay-safe constraints
- Replay roots and receipts must match when profiling is on/off.
- Benchmark harness must run deterministic fixture loading and stable ordering.
- Report output is written to `benchmarks/reports/` and must never be fed back into state reconstruction.
