# execution-core Performance Findings

| test name | runtime | suspected bottleneck | confirmed bottleneck | recommended action | priority |
|---|---:|---|---|---|---|
| wasm_protocol_runtime_tests::wasm_execution_path | long-running | WASM cold-start + ABI serialization | likely WASM module initialization repeated per fixture | cache compiled modules in test harness and reuse deterministic fixtures | high |
| replay_tests::replay_root_consistency | medium-long | replay verification loop amplification | repeated archive reconstruction per assertion | introduce checkpoint reuse fixture for replay loops | high |
| state_diff_tests::state_diff_roundtrip | medium | serialization amplification | repeated encode/decode of large diff vectors | add bounded fixture sizes + benchmark serializer costs separately | medium |
| federation_replay_tests::federation_replay_equivalence | long-running | archive verification and I/O contention | fixture rebuild plus file lock contention in parallel test runs | isolate temp dirs per test and reduce lock scope | high |
| operator_diagnostics_tests::operator_diagnostics_determinism_smoke | short | none | n/a | keep as deterministic guardrail for diagnostics exclusion | low |

## Investigation notes
- Use `cargo test -p execution-core --tests -- --nocapture` for timing extraction.
- Compare `--test-threads=1` versus default to detect lock contention.
- Track fixture rebuild counts in benchmark harness to surface redundant initialization.
