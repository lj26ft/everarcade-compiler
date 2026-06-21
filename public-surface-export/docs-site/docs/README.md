---
slug: /docs/readme
---

# EverArcade Documentation

EverArcade documentation is now organized around a canonical platform documentation set. Historical reports and subsystem notes remain evidence, but new engineers should not need them for onboarding.

## Canonical Reading Order

1. `01-executive-overview.md`
2. `02-platform-vision.md`
3. `03-system-architecture.md`
4. `04-runtime-architecture.md`
5. `11-production-readiness.md`
6. `12-gap-analysis.md`
7. `runtime-capabilities.md`
8. `13-runtime-operations-manual.md`
9. `14-v0.1-architecture-freeze.md`
10. `repository-navigation.md`
11. `documentation-governance.md`

## Authoritative Documents

The authoritative architecture and operations documents are the numbered `01` through `14` documents in this directory, plus `runtime-capabilities.md`, `repository-navigation.md`, and `documentation-governance.md`.

## Historical Evidence

Older documents under `docs/architecture/`, `docs/runtime/`, `docs/security/`, `docs/operators/`, `docs/runbooks/`, `docs/release/`, and deployment reports may contain useful evidence or implementation-specific details. They are not the source of truth when they conflict with the canonical documents.

## Documentation Rule

Do not add new architecture documents unless the concept cannot fit into the canonical set. Update the owning canonical document instead.
