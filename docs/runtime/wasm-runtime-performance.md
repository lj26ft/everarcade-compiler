# WASM Runtime Performance Baseline

## Scenarios
- small payload: ABI encode/decode and host-call overhead dominates.
- medium payload: mixed ABI and memory page pressure.
- large payload: serialization and memory growth dominate.
- repeated calls: boundary crossing + receipt generation accumulation.

## Metrics captured
- host ↔ WASM call count
- fuel consumed
- memory pages touched
- receipt generation count
- state serialization contribution

## Determinism constraints
All WASM benchmark instrumentation writes to diagnostic reports only. Receipt bytes, replay roots, and state roots remain unchanged across profiling modes.
