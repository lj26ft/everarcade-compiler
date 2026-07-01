# Dependencies

## Internal dependencies

- Deterministic runtime primitives depend on stable input ordering, state hashing, replay records, and package validation helpers in `sdk/` and `runtime/`.
- RustRig contract proof depends on `contract-api` deterministic context, output, and ABI policy.
- World package certification depends on documented package layout, manifest integrity, RustRig references, genesis roots, replay roots, federation outputs where applicable, and proof artifacts.
- CI depends on the deterministic world factory script invoked from GitHub Actions.

## Cross-repo dependencies

- `everarcade`: consumes reference package/verifier semantics and owns commercial coordination, registry, certification services, platform trust, and product implementation.
- `everarcade-rustrigs`: consumes deterministic RustRig contract expectations and should provide hardened gameplay rigs and invariants.
- `everarcade-frontend`: consumes registry/proof/certification metadata exposed by the commercial platform and should not define protocol truth.
- `everarcade-hq`: consumes readiness and certification status for internal review, onboarding, operator coordination, and release execution.
