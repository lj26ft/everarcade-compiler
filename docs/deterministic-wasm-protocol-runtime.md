# Deterministic WASM Protocol Runtime

This document defines the deterministic WASM protocol boundary.

- Engine hardening: fuel enabled; threads/SIMD/relaxed SIMD/reference types/tail calls/multi-memory disabled.
- Host ABI: explicit typed calls (`host_read_input`, `host_write_output`, `host_emit_state_diff`, `host_emit_log`, `host_abort`).
- Memory bridge: bounds checked read/write with deterministic error messages.
- Typed ABI: `WasmExecutionInput`, `WasmExecutionOutput`, `WasmStateDiff`, `WasmExecutionError`, `WasmHostCallTrace`, and `WasmExecutionManifest`.
- Fuel accounting: protocol receipts must include `fuel_limit`, `fuel_consumed`, `fuel_remaining`, and `fuel_exhausted`.
- State diffs: ordered `BTreeMap` updates and canonical diff hashing.
- DAG scheduling: stable topological ordering using ordered collections only.
- Replay artifacts: runtime outputs under `runtime/replay/wasm/latest/` and `runtime/replay/wasm-dag/`.
- Validation scripts: `verify_wasm_execution_equivalence.sh`, `verify_wasm_dag_equivalence.sh`, `validate_wasm_protocol_runtime.sh`.

## Determinism limitations

- Cross-machine reproducibility requires pinned rustc/wasmtime versions.
- Unsupported guest features: threads, SIMD variants, reference types, tail calls, multi-memory.
- Floating-point behavior is not part of canonical state unless normalized by protocol tests.
- Host ABI versions must remain compatible with historical replay artifacts.
- Fuel metering is wasmtime-version dependent and must be version-locked for equivalence.
- Replay artifact schema changes require explicit migration/versioning.
