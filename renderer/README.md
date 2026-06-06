# Renderer Runtime Layout

The Renderer Runtime is the non-authoritative player-facing projection layer for EverArcade. It consumes deterministic projection artifacts emitted from civilization/world runtime state and renders them without mutating protocol state, checkpoints, replay streams, or settlements.

## Canonical directories

- `projection/` - canonical projection model, deterministic ordering, projection root generation.
- `world/` - zones, regions, civilizations, settlements, and world objects.
- `entities/` - players, NPCs, assets, structures, and creatures.
- `physics/` - read-only bodies, transforms, collisions, movement, and interactions.
- `inventory/` - ownership, containers, equipment, and vault assets.
- `events/` - combat, trades, marketplace, governance, and civilization events.
- `ui/` - player-visible UI projection descriptors derived from projection artifacts.
- `replay/` - checkpoint plus replay-stream equivalence projection checks.

All renderer artifacts are deterministic, read-only projections. Runtime authority remains outside this tree.

## Pseudocode authority boundary

- **Input:** runtime projection records, replay/checkpoint references, and read-only world/entity/inventory/event descriptions.
- **Output:** deterministic projection roots and render-facing descriptors.
- **Authority:** none over gameplay, economy, settlement, identity, or replay truth; renderer output is evidence to compare, not state to trust.
- **EverArcade fit:** lets a player-facing client visualize authoritative state produced elsewhere.
