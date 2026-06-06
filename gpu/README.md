# GPU Runtime Layout

The GPU Runtime is EverArcade's deterministic, non-authoritative acceleration layer for renderer workloads. It consumes renderer projection artifacts, deterministic replay streams, and checkpoint identifiers, then produces verifiable render artifacts and roots without mutating protocol state.

## Canonical directories

- `jobs/` - GPU job model, supported job types, deterministic job-root generation.
- `workers/` - deterministic worker records with device references, capacity, availability, and worker-root generation.
- `devices/` - deterministic device capability profiles; no live hardware inspection.
- `queues/` - pending, assigned, running, completed, and failed queue state with canonical ordering.
- `artifacts/` - render artifact records and render-root generation.
- `verification/` - projection, job, worker, and artifact integrity checks.
- `replay/` - deterministic replay render-root regeneration and render-root equivalence checks.

The GPU Runtime never writes protocol state, inventory, economy, governance, settlements, or authority data. All files in this tree are projection consumers or projection-output descriptors.

## Pseudocode authority boundary

- **Input:** renderer projections, GPU job descriptors, worker/device declarations, replay identifiers, and checkpoint references.
- **Output:** render artifact records, job roots, worker roots, and verification transcripts.
- **Authority:** none over protocol state, payment eligibility, or marketplace settlement; provider claims require external verification.
- **EverArcade fit:** a non-authoritative acceleration/marketplace scaffold for future verifiable rendering work.
