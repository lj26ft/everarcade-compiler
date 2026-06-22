# world.evr Package Spec RC1 Handoff

This directory is the RC1 handoff bundle for independent Tier2 review of the `world.evr` package format. RC1 is **not** the frozen v1 artifact format; it is the review candidate Dane should inspect, implement against, and try to break before v1 is frozen.

## Included source specs

- `WORLD_EVR_PACKAGE_SPEC_RC1.md` — RC1 copy of the package spec.
- `CANONICAL_PACKAGE_FORMAT.md` — canonical package layout, serialization, and hash rules.
- `GAME_PACKAGE_FORMAT.md` — game-package background format.
- `WORLD_PACKAGE_CERTIFICATION.md` and `WORLD_PACKAGE_CERTIFICATION_FRAMEWORK_V1.md` — proof/certification material.
- `CANONICAL_WORLD_CREATION_FLOW_V1.md` — canonical world creation flow.
- `DERIVED_ARTIFACTS.md` — generated artifact expectations.
- `VERIFICATION_MATRIX.md` — review and validation matrix.
- `PACKAGE_FIXTURES.md` — fixture index and expected verifier behavior.

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

## Attack questions

Try to answer these adversarially:

- Can two different packages produce the same interpreted world?
- Can file ordering change `package_hash`?
- Can optional fields hide consensus data?
- Can manifest fields disagree with bundled artifacts?
- Can runtime identity be swapped?
- Can `world_id` be spoofed?
- Can genesis hash mismatch package hash?
- Can restore/migration bundles bind to the wrong package?
- Can verifier reproduce `package_hash` without implementation code?
- Can a package be accepted with unknown fields?
- Can a package omit fields required for restore?
- Can a package include extra files that affect runtime but not hash?

## Handoff message

> I want to treat the world.evr artifact format as RC1, not final v1 yet.
>
> I’m syncing the package spec, canonical package format, certification docs, fixtures, and failure cases into tier2-proof under specs/world-evr-package/.
>
> Goal: see if an independent verifier can recompute package_hash and confirm the package binds every load-bearing artifact without using runtime code.
>
> Please try to break it the same way you did with replay/restore/migration.
>
> If the spec leaves any hidden dependency, ambiguous field, unhashed artifact, ordering problem, or restore/migration binding issue, call it out before we freeze v1.

## Freeze rule

Do not declare `world.evr` package format v1 frozen until Dane reports either:

- `RC1 passes cold independent verification.`
- `RC1 has these hidden dependencies / ambiguity gaps.`

If gaps are found, update the spec and cut `WORLD_EVR_PACKAGE_SPEC_RC2`.
