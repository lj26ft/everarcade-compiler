# Deterministic Profiling Model

## Hard rule
Profiling and observability data must never influence deterministic execution, receipt hashes, state roots, replay roots, or consensus-equivalent outputs.

## Safe measurements
- Operation counts (DAG nodes, receipts, state diffs, ABI encode/decode, WASM calls).
- Fuel and memory-page counters when available from deterministic runtime accounting.
- Replay divergence metadata (operation index, expected vs observed root identifiers).
- Diagnostic duration (`diagnostic_duration_ns`) as operator-only metadata.

## Unsafe inputs (must not be used for decisions)
- Wall clock, local system load, random numbers, PID ordering, network timing.
- Any host measurement in consensus-path branching.

## Emission model
- JSON lines to stdout-first envelopes.
- Stable event names and monotonic sequence indexes per process.
- Optional file sink under `runtime/diagnostics/` or `target/everarcade-profile/`.

## Exclusion from roots
- Profile records are never included in receipt hashing/state-root derivation.
- Execution/replay/commit roots continue to hash only canonical deterministic payloads.

## Replay diagnostics
- Profiling assists with “what diverged where” without changing replay verdicts.
- Divergence metadata maps to operation index and warning list.

## Why wall-clock is diagnostic-only
Wall-clock captures environment behavior (scheduler/CPU/noise), not protocol work. It is therefore useful for operator tuning, but non-authoritative for deterministic equivalence.
