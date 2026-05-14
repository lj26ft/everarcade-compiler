# Derived Runtime Artifacts

EverArcade runtime fixture binaries are **derived artifacts**, not canonical repository truth.

## Canonical truth model

Authoritative protocol truth is defined by deterministic generation and validation:

- canonical deterministic generation logic
- canonical package encoding rules
- canonical replay/checkpoint root derivation

In short:

- canonical deterministic generation = protocol truth
- generated runtime artifacts = derived rebuildable outputs

## Authoritative vs rebuildable data

### Authoritative truth (must be preserved)

- packages
- receipts
- checkpoints
- replay roots

### Rebuildable helpers (safe to regenerate)

- indexes
- manifests
- caches
- temporary fixtures

Helper artifacts exist for performance and operator ergonomics, but are not protocol authority.

## Repository policy

Generated runtime fixture binaries (including ad-hoc `*.bin` outputs from smoke/stress/recovery flows) must remain untracked.

Fixtures should be generated dynamically during tests and operator workflows, written into temporary paths, and deleted after use.

## Operator workflow expectations

Operator scripts and tests should:

- allocate temporary fixture output paths (for example via `mktemp`)
- avoid assumptions about committed runtime binaries
- clean up temporary state directories and fixture files
- validate replay/checkpoint continuity from deterministic outputs

This keeps CI, local development, and long-running runtime validation reproducible without committing derived runtime artifacts.
