# Renderer Runtime v0.1

## Purpose

Renderer Runtime v0.1 is EverArcade's first operational player-facing projection runtime. It converts deterministic civilization state into visible projections while remaining strictly non-authoritative.

The runtime reads projection artifacts, checkpoints, and replay streams. It never mutates protocol state, checkpoints, replay data, settlements, leases, or authority records.

```text
Civilization Runtime -> World State -> Projection -> Renderer Runtime -> Player Experience
```

## Projection Model

Projection is canonical. Renderer Runtime v0.1 consumes deterministic projection artifacts rather than raw mutation events.

The projection model is defined in `renderer/projection/projection_model.sh` and emits roots for:

- World Projection
- Entity Projection
- Physics Projection
- Inventory Projection
- Event Projection
- Replay Projection

Projection roots are generated from canonical transcripts using deterministic ordering. Domain lists are sorted before hashing so replay and live rendering resolve to identical roots.

## World Renderer

The world renderer represents:

- Zones
- Regions
- Civilizations
- Settlements
- World objects

It generates a World Projection Root from read-only world projection artifacts. The world renderer does not create or update zones, regions, settlements, or objects.

## Entity Renderer

The entity renderer represents:

- Players
- NPCs
- Assets
- Structures
- Creatures

It generates an Entity Projection Root from canonical entity descriptors. It does not spawn, destroy, or mutate authoritative entities.

## Physics Renderer

The physics renderer represents:

- Bodies
- Transforms
- Collisions
- Movement
- Interactions

Physics rendering is read-only. The renderer may display transforms, collision facts, and movement facts, but it has no simulation authority and does not perform client prediction.

## Inventory Renderer

The inventory renderer represents:

- Ownership
- Containers
- Equipment
- Vault assets

It generates an Inventory Projection Root from projection artifacts derived from authoritative ownership and vault state. It never transfers, equips, mints, burns, or settles assets.

## Event Renderer

The event renderer represents:

- Combat
- Trades
- Marketplace events
- Governance events
- Civilization events

It generates an Event Projection Root from ordered visible event artifacts. Event rendering is observational and cannot append to protocol event streams.

## Replay Renderer

The replay renderer consumes:

```text
Checkpoint + Replay Stream
```

It regenerates the same projection root as live projection:

```text
Replay Projection Root == Projection Root
```

Replay rendering is therefore deterministic and safe for observer clients, historical clients, and federation lease handoff review.

## PASS Criteria

Renderer Runtime v0.1 passes when:

- Projection root generation succeeds.
- World projection root generation succeeds.
- Entity projection root generation succeeds.
- Physics projection root generation succeeds and reports no simulation authority.
- Inventory projection root generation succeeds.
- Event projection root generation succeeds.
- Replay projection root matches the live projection root.
- Validation reports `Renderer Runtime Validation: PASS`.
- Certification reports `Renderer Runtime: PASS`.

## FAIL Criteria

Renderer Runtime v0.1 fails if:

- Any projection root is missing or non-deterministic.
- Replay projection differs from live projection.
- Renderer code mutates protocol state, checkpoints, replay streams, settlements, or authority records.
- Physics rendering claims simulation authority.
- Canonical ordering is not preserved.
- Validation or certification reports any domain as `FAIL`.

## Relationship To Multi-Lease Federation Runtime

The Multi-Lease Federation Runtime synchronizes civilization continuity across leases. Renderer Runtime v0.1 sits downstream from that federation layer. It can read federated checkpoints, replay streams, and projection artifacts, but it cannot influence lease membership, synchronization, settlement exchange, or recovery.

This preserves protocol sovereignty across lease handoff while allowing each observer or client to produce identical player-visible projections.

## Relationship To Future GPU Runtime

Renderer Runtime v0.1 defines deterministic projection semantics only. Future GPU runtime work may accelerate rasterization, lighting, ray traversal, or post-processing, but GPU execution must remain downstream of these projection roots.

GPU acceleration must not introduce authority, client prediction, nondeterministic projection ordering, or protocol state mutation.
