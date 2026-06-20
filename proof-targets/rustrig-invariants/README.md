# RustRig Invariant Proof Targets

Proof harnesses in this directory consume three inputs for each RustRig:

1. RustRig source code under `crates/rustrigs/<domain>/src/`.
2. Canonical declarations from `crates/rustrigs/<domain>/invariants.toml`.
3. Property targets from `crates/rustrigs/<domain>/properties.md`.

Harnesses should bind generated tests to invariant IDs, report proof status transitions, and avoid inferring verifier intent from implementation code alone.
