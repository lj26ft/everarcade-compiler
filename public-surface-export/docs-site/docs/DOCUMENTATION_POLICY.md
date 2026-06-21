# Documentation Policy

## Canonical documentation location

`docs/` is the canonical source for EverArcade documentation. Public-facing explanations, onboarding, architecture, operations, and reference material should be created or consolidated under `docs/` before being linked from the repository root.

Root-level documents are limited to repository entry points: `README.md`, `CONTRIBUTING.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md`, `MATURITY.md`, `REPOSITORY_MAP.md`, and release-readiness audits.

## Subsystem documentation rules

Subsystem `README.md` files may remain next to code when they help a contributor work in that directory. They should:

- describe the local purpose of the directory;
- link back to the appropriate canonical page under `docs/`;
- avoid duplicating full architecture, maturity, roadmap, or operations guidance;
- state when the subsystem is scaffold, experimental, alpha, or production according to `MATURITY.md`.

Detailed design notes that are useful but not authoritative should live under `docs/reference/` or the relevant subsystem section as implementation notes.

## README responsibilities

The root `README.md` answers only: what EverArcade is, why it exists, what can be done today, how to start, and where to find docs. It should remain short enough to scan quickly and should link to canonical docs for details.

Directory README files are navigational aids, not competing documentation portals.

## Documentation ownership expectations

Every material code or workflow change should update one of:

- the relevant canonical page under `docs/`;
- the local subsystem README when local instructions changed;
- `MATURITY.md` when subsystem status changed;
- `REPOSITORY_MAP.md` when ownership boundaries or dependency relationships changed.

Documentation changes should prefer moving, linking, and consolidating existing validated material over rewriting history. Historical reports, validation outputs, and certification artifacts belong in `archive/` unless they are active release evidence.
