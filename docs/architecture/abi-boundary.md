# ABI Boundary

- Canonical serialization uses stable JSON encoding semantics and fixed field meaning per ABI structs.
- Deterministic ordering requirements apply to ordered collections before hashing/receipt generation.
- Host↔WASM ownership: host manages linear-memory transfer; core manages ABI decode/validate/execute semantics.
- Memory bridge assumptions: caller provides complete byte buffers and lengths; malformed buffers are invalid input.
- Replay divergence is invalid when canonical ABI bytes decode to non-equivalent semantic plans/results.

## WASM boundary profiling
Optional deterministic-safe profiling is supported at the host↔WASM boundary via count/fuel/memory metrics. `diagnostic_duration_ns` is emitted as diagnostic metadata only and MUST NOT affect execution decisions or receipt/state roots.
