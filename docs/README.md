# EverArcade Documentation

## Purpose

This directory is the onboarding and architecture entry point for EverArcade. It points new engineers to the canonical architecture book, focused subsystem chapters, roadmap status, and legacy evidence.

## Responsibilities

- Make `docs/architecture/everarcade-architecture-book.md` the primary architecture reference.
- Direct readers to subsystem chapters for runtime, federation, world, renderer, deployment, SDK, and roadmap topics.
- Preserve legacy documentation as supporting evidence while clarifying that the architecture book is canonical.
- Help contributors distinguish implemented, partially implemented, scaffold, and planned systems.

## Non-Responsibilities

- This index does not replace crate-level API documentation or source review.
- It does not certify production readiness for every historical milestone report.
- It does not make renderer, observer, dashboard, or analytics artifacts authoritative.

## Internal Components

- `architecture/everarcade-architecture-book.md`: master technical book.
- `architecture/00-executive-summary.md`: fastest onboarding path.
- `architecture/03-repository-map.md`: crate and repository inventory.
- `architecture/implemented-features.md`: canonical implementation status matrix.
- `architecture/runtime/`, `architecture/federation/`, `architecture/world/`, `architecture/renderer/`, `architecture/deployment/`, `architecture/sdk/`, and `architecture/roadmap/`: focused subsystem chapters.
- Existing quickstarts, runbooks, release docs, security docs, SDK docs, and historical reports: supporting material.

## Data Flow

Reader question → architecture book → subsystem chapter → repository map → source module/test/report/runbook → contribution or operation.

## Determinism Guarantees

The documentation system preserves the same rule as the runtime: authoritative claims must be tied to canonical evidence. Current source modules, tests, test vectors, validation reports, and runbooks take precedence over outdated milestone language.

## Failure Modes

- Corruption: a historical report overstates current maturity.
- Divergence: docs and source disagree.
- Recovery: update the subsystem chapter and status matrix in the same change as architecture-affecting code.
- Reconciliation: prefer current implementation and targeted tests over older readiness language.

## Future Evolution

Keep this index small. Add new architecture chapters only when a subsystem needs a stable onboarding surface, and keep scaffold-level renderer/history/federation domains explicitly labeled until validated.

## Start Here

1. Read `architecture/00-executive-summary.md`.
2. Read `architecture/everarcade-architecture-book.md`.
3. Use `architecture/03-repository-map.md` to locate crates and modules.
4. Use `architecture/implemented-features.md` and `architecture/roadmap/implemented-features.md` to distinguish implemented, partial, scaffold, and planned work.
5. Use existing runbooks and quickstarts for commands after you understand the architecture.

## Legacy Documentation

Older milestone reports, validation outputs, and readiness documents remain useful evidence. They are no longer the primary source of truth when they conflict with the architecture book or current source modules.
