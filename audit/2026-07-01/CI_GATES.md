# CI Gates

## Visible gates

- `.github/workflows/deterministic-world-factory.yml` runs the deterministic world factory CI script.
- Rust unit/integration tests exist under `tests/`, `sdk/tests/`, and crate-local test modules for deterministic SDK/runtime behavior.
- JavaScript proof harnesses under `runtime/*-proof/validation/` emit deterministic reports for local proof models.

## Gate classifications

| Gate area | Status | Notes |
| --- | --- | --- |
| Deterministic world factory | Present | Primary visible GitHub Actions gate. |
| Package determinism | Present/partial | Tests and SDK helpers exist; should be tied to canonical verifier CLI. |
| Replay equivalence | Present/partial | Multiple proof/test paths exist; needs minimal required-vs-extended labeling. |
| Root integrity | Present/partial | Local certification harnesses exist; production claims need scope labels. |
| RustRig ABI/contract | Present/partial | Deterministic contract primitives exist; hardening belongs with RustRigs repo. |
| Federation/history/renderer/GPU | Scaffold | Treat as proof/scaffold domains, not production gates. |

## Recommended CI changes

1. Add a small audit-safe verifier smoke test for Core `world.evr` conformance.
2. Publish an explicit CI matrix mapping each gate to Core, Extended, or Scaffold.
3. Keep targeted tests preferred over workspace-wide builds; do not require `cargo test --workspace` for routine audit changes.
