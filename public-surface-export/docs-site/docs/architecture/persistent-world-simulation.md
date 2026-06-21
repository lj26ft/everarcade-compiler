# Persistent World Simulation Architecture

Persistent world continuity is derived from deterministic replay and archival restoration,
not hidden mutable server state.

This model defines a civilization runtime where all continuity is reconstructed from canonical tick-ordered operations, checkpoints, and archives. Deterministic ordering, append-only economy/inventory ledgers, and restoration manifests are the only continuity sources.

## Principles
- Deterministic world evolution by canonical tick progression.
- Entity lifecycle continuity across spawn, mutation, migration, upgrade, archival, restoration, and retirement.
- Inventory and economy continuity through append-only mutation streams.
- Replay/archive continuity via multi-era checkpoint chains.
- Scheduler responsibility: stable ordering, deterministic restoration, replay-safe sequencing.
- Restoration assumption: cold/partial/multi-era restore must reconstruct equivalent continuity roots.
- Operations assumption: diagnostics are stdout-first and excluded from state roots and receipt hashes.
