# world.evr Package Spec RC2 Handoff

This directory is the RC2 handoff bundle for independent Tier2 review of the `world.evr` package format. RC2 is **not** the frozen v1 artifact format; it is the review candidate Dane should inspect, implement against, and try to break before v1 is frozen.

## Included source specs

- `WORLD_EVR_PACKAGE_SPEC_RC2.md` — RC2 package spec with explicit mandatory binding predicates.
- `WORLD_EVR_PACKAGE_SPEC_RC1.md` — prior review candidate retained for comparison only.
- `CANONICAL_PACKAGE_FORMAT.md` — canonical package layout, serialization, and RC2 hash-manifest authority notes.
- `GAME_PACKAGE_FORMAT.md` — game-package background format.
- `WORLD_PACKAGE_CERTIFICATION.md` and `WORLD_PACKAGE_CERTIFICATION_FRAMEWORK_V1.md` — proof/certification material.
- `CANONICAL_WORLD_CREATION_FLOW_V1.md` — canonical world creation flow.
- `DERIVED_ARTIFACTS.md` — generated artifact expectations.
- `VERIFICATION_MATRIX.md` — review and validation matrix.
- `PACKAGE_FIXTURES.md` — RC2 fixture index and expected verifier behavior.
- `verify-package-rc2.mjs` — cold spec-level verifier for the directory fixtures.

## RC2 fixes

RC2 addresses the two RC1 binding gaps found by independent verification:

1. Runtime identity is explicitly bound: `manifest.runtime.runtime_id == runtime/runtime.json.runtime_id` and `manifest.runtime.version == runtime/runtime.json.version`.
2. Restore package identity is explicitly bound: `restore/checkpoint.json.root_package == manifest.package_name` and restore continuity roots are recomputed from included accumulator data.

RC2 also declares the `hash-manifest.json` stream as the authoritative package hash construction and replaces “mutually bound” prose with mandatory binding predicates.

## Review scope for Dane

Please verify whether the specs and fixtures fully define:

- package archive layout
- required files and optional files
- manifest schema
- canonical serialization
- hash construction and `package_hash`
- `world_id`, runtime, genesis, journal, checkpoint, restore, and migration bindings
- proof/certification fields
- versioning rules
- invalid package detection

## Handoff message

> RC2 is synced.
>
> It fixes the two RC1 binding gaps:
>
> 1. runtime identity is explicitly bound:
> manifest.runtime == runtime/runtime.json
>
> 2. restore checkpoint is explicitly bound:
> checkpoint.root_package == package identity
>
> RC2 also declares the hash-manifest recipe as the authoritative package_hash construction and replaces “mutually bound” prose with mandatory binding predicates.
>
> Please rerun the Python + TS package verifiers and the adversarial repair pass. If either implementation disagrees, or if a repaired failure fixture survives, RC2 is not v1.

## Freeze rule

Do not declare `world.evr` package format v1 frozen until Dane reports that RC2 passes cold independent verification and adversarial repair, with no hidden dependency, ambiguity, unhashed artifact, ordering problem, or restore/migration binding gap.
