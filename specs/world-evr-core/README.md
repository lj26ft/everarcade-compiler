# world.evr Core and Extended Specification Freeze

This directory freezes the boundary between the minimum scientifically verifiable `world.evr` Core and optional Extended capabilities.

## Files

- `WORLD_EVR_CORE_SPEC_V1.md` — mandatory Core directories, hashes, compatibility, migration, verifier, and preservation rules.
- `WORLD_EVR_EXTENDED_SPEC_V1.md` — optional assets, projections, AI, and metadata namespaces.
- `schemas/world-evr-core-v1.schema.json` — starter validation schema for Core manifests.
- `schemas/world-evr-extended-v1.schema.json` — starter validation schema for Extended descriptors.
- `examples/minimal-core-world/` — Core-only package skeleton.
- `examples/extended-world/` — Core package skeleton with optional Extended namespaces.

## Freeze answer

The smallest thing that constitutes a world is a `world.evr/` package with the ten mandatory Core directories and their required metadata. Assets, projections, AI, and platform metadata are optional extensions and do not affect Core replay determinism.
