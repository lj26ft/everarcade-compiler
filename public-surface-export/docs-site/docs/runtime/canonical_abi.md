# Canonical ABI Semantics (Protocol Law)

- Encoding: UTF-8 JSON bytes from `serde_json::to_vec`.
- Request envelope fields and order: `version`, `payload`.
- Request payload for contract execution is the raw `ContractExecutionRequest.input` bytes.
- Response envelope fields and order: `mutations`, `stdout`, `status`.
- Mutation entry shape: `[key, value_bytes]`.

## Success envelope

`status = "ok"`, deterministic `mutations`, deterministic `stdout` bytes.

## Failure mapping

- ABI decode failure: `ExecutionStatus::MalformedAbi`.
- Duplicate mutation keys after successful decode: `ExecutionStatus::DuplicateMutation`.
- Fuel trap/exhaustion: `ExecutionStatus::FuelExhausted`.
- Memory read bounds/handle violations: `ExecutionStatus::MemoryViolation`.
- Wasm trap not caused by fuel: `ExecutionStatus::ExecutionTrap`.
- Module compile/load failures: `ExecutionStatus::ModuleLoadFailed`.

All failure statuses are terminal, deterministic, and must not commit state.
