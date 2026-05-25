# Runtime Threat Model

This document defines runtime adversarial classes for deterministic execution.

## Threat Classes

For each class: attack surface, mitigations, residual risk, future hardening.

- **Malicious WASM modules**: surface is guest bytecode and host ABI invocation; mitigation includes deterministic host ABI boundaries and fuel metering; residual risk is logic-level abuse; hardening: formal bytecode policy + sandbox fuzzing.
- **Infinite loops / fuel abuse**: surface is untrusted execution loops; mitigation is bounded fuel and scheduler limits; residual risk is denial-of-service under low budgets; hardening: adaptive per-lane quotas.
- **Malformed ABI attacks**: surface is host call encoding; mitigation is canonical ABI decoding and validation; residual risk is parser complexity bugs; hardening: ABI differential fuzzing.
- **Deterministic divergence attacks**: surface is serialization/hash boundaries; mitigation is canonical hashing and deterministic state roots; residual risk is implementation drift; hardening: cross-version replay matrix.
- **Malicious runtime forks**: surface is binary distribution and config mutation; mitigation is release manifests and hash verification; residual risk is operator override; hardening: signature transparency logs.
- **Nondeterministic serialization attacks**: surface is map ordering/encoding; mitigation is canonical structures and ordered serialization; residual risk is future format regressions; hardening: serialization freeze tests.
