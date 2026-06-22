# WORLD_EVR_PACKAGE_SPEC_V1 Freeze Note

## RC1 findings

RC1 exposed two binding gaps during review:

1. Runtime identity was present as data but was not semantically bound between the package manifest and `runtime/runtime.json`.
2. Restore checkpoints were present as data but were not package-bound, allowing restore material to be transplanted across packages without a mandatory verifier predicate catching it.

## RC2 fixes

RC2 closed both gaps with mandatory semantic verifier predicates:

- `manifest.runtime.runtime_id == runtime/runtime.json.runtime_id`
- `manifest.runtime.runtime_version == runtime/runtime.json.runtime_version`
- `checkpoint.root_package == manifest.package_name`
- `journal.root_package == manifest.package_name`

RC2 also made the hash-manifest package hash recipe authoritative and replaced vague mutual-binding prose with explicit verifier predicates.

## RC2 independent verdict

Independent cold verification passed across two independent implementations:

- Python verifier: 9/9
- TypeScript verifier: 9/9
- Cross-implementation agreement: PASS
- Adversarial repair pass: PASS

Dane's RC2 verdict: RC2 passes; both implementations and both passes agree with no field ambiguity.

## V1 freeze decision

`WORLD_EVR_PACKAGE_SPEC_V1` is frozen as the v1 package artifact format for `world.evr` packages. The package hash remains independently reproducible from `.evr` package contents plus the public spec only, with no EverArcade runtime code.

## Remaining non-blocking future work

- Add additional language-specific verifier examples that consume only the public artifact and spec.
- Add more negative fixtures for canonical JSON edge cases and optional proof metadata.
- Define a future migration policy for any post-v1 package format without changing V1 semantics.
