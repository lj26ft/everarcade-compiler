# Documentation Governance

## What Is Authoritative

The authoritative platform documentation set is:

1. `01-executive-overview.md`
2. `02-platform-vision.md`
3. `03-system-architecture.md`
4. `04-runtime-architecture.md`
5. `05-world-runtime.md`
6. `06-federation-runtime.md`
7. `07-renderer-projection.md`
8. `08-sdk-development.md`
9. `09-deployment-evernode.md`
10. `10-release-certification.md`
11. `11-production-readiness.md`
12. `12-gap-analysis.md`
13. `13-runtime-operations-manual.md`

Additional authoritative guides:

- `runtime-capabilities.md`
- `repository-navigation.md`
- `documentation-governance.md`

## What Is Historical

Historical documents include milestone reports, validation reports, readiness reports, architecture book fragments, subsystem notes, deployment reports, and older roadmap documents outside the canonical set. They may be useful evidence, but they are not onboarding requirements and do not override the canonical documents.

## What Can Be Deleted

A historical document can be deleted or replaced by a redirect when:

- its architectural content is fully represented in the canonical set;
- it is not referenced by release certification evidence;
- it does not contain unique implementation instructions;
- owners agree it is not needed for audit traceability.

Reports that are release evidence should be archived, not rewritten as architecture.

## How New Documentation Is Added

New documentation must be one of:

1. an update to the canonical documents;
2. implementation-specific crate or module documentation;
3. operator runbook detail referenced by `13-runtime-operations-manual.md`;
4. validation evidence referenced by release certification;
5. historical report output clearly marked as evidence.

Do not create a new architecture document when an existing canonical document can be updated.

## Ownership Rules

| Area | Owner |
|---|---|
| Platform architecture | `03-system-architecture.md` |
| Runtime behavior | `04-runtime-architecture.md` |
| World domains | `05-world-runtime.md` |
| Federation | `06-federation-runtime.md` |
| Renderer projection | `07-renderer-projection.md` |
| SDK | `08-sdk-development.md` |
| Deployment / Evernode | `09-deployment-evernode.md` |
| Release policy | `10-release-certification.md` |
| Readiness | `11-production-readiness.md` |
| Gaps | `12-gap-analysis.md` |
| Operations | `13-runtime-operations-manual.md` |

## Review Rules

- Architecture changes must update the relevant canonical document in the same change.
- Status claims must use **Implemented**, **Partial**, **Scaffold**, or **Planned**.
- Production readiness must be updated in `11-production-readiness.md`.
- Capability claims must be updated in `runtime-capabilities.md`.
- New gaps must be added to `12-gap-analysis.md`.
- Historical milestone language must not be copied into canonical docs without current validation.

## Architecture Change Process

1. Identify the authority boundary affected.
2. Update the owning canonical document.
3. Update readiness and capability matrices if status changes.
4. Add or update release gates if certification changes.
5. Add operator procedure changes if operations are affected.
6. Link implementation-specific docs rather than duplicating architecture.
7. Review for duplicate determinism, responsibility, and failure-mode sections.
