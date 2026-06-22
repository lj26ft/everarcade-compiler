# world.evr Package Spec V1

V1 frozen after RC2 independent review. `WORLD_EVR_PACKAGE_SPEC_V1.md` is the frozen v1 package artifact format for `world.evr` packages.

Independent review status:

- Python verifier: PASS
- TypeScript verifier: PASS
- Cross-implementation agreement: PASS
- Adversarial repair pass: PASS

The spec is the trust root. Verifiers are replaceable. The artifact is what is tested.

## Files

- `WORLD_EVR_PACKAGE_SPEC_V1.md` — frozen V1 package artifact specification.
- `WORLD_EVR_PACKAGE_SPEC_RC2.md` — preserved historical RC2 review material.
- `fixtures/world-package-valid-001/` — valid V1 package fixture.
- `failure-fixtures/` — negative fixtures that must be rejected by V1 verifiers.
- `verify-package-v1.mjs` — standalone cold spec-level verifier for the directory fixtures.
- `PACKAGE_FIXTURES.md` — fixture index and expected verifier behavior.
- `review/V1_FREEZE_NOTE.md` — freeze handoff note covering RC1 findings, RC2 fixes, independent verdict, and V1 decision.

## V1 binding predicates

V1 freezes the RC2 fixes as mandatory semantic predicates:

1. Runtime identity is explicitly bound: `manifest.runtime.runtime_id == runtime/runtime.json.runtime_id` and `manifest.runtime.runtime_version == runtime/runtime.json.runtime_version`.
2. Restore material is package-bound: `checkpoint.root_package == manifest.package_name` and `journal.root_package == manifest.package_name`.
3. Restore checkpoint continuity is root-bound: `checkpoint.roots.continuity_root` must equal the recomputed continuity root for `restore/journal.json`.
4. The `hash-manifest.json` stream is the authoritative package hash construction.

`runtime_version` is the canonical runtime version field. Aliases are non-canonical and should not appear in V1 fixtures.

## Verification

Run:

```bash
node specs/world-evr-package/verify-package-v1.mjs
```

A conforming V1 verifier must pass the valid fixture and reject every fixture under `failure-fixtures/`, including runtime identity mismatch, restore root package mismatch, non-canonical hash-manifest ordering, and continuity root mismatch.
