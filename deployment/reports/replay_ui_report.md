# Replay Ui Report

## Creator workflow readiness
- Status: scaffold-ready deterministic creator surface.
- Studio workflow covers Create Project, Import Assets, Build World, Place Entities, Configure Runtime, Run Simulation, Inspect Replay, Debug Divergence, Package Content, and Deploy Runtime.

## Replay continuity guarantees
- Replay surfaces are reconstruction-only or read-model projections.
- Creator actions resolve to stable hashes and append-safe lineage records.
- Direct replay mutation requests are rejected.

## Deployment guarantees
- Deployment UX validates EverArcade runtime compatibility.
- Package, node, and federation lineage are represented by deterministic hashes.
- Authority remains inside deterministic runtime execution boundaries.

## Creator ergonomics
- Visual surfaces expose project, asset, world, viewport, hierarchy, inspector, replay, simulation, diagnostics, publishing, and deployment workflows.
- Runtime and renderer displays are projection-only.

## Operational limitations
- Current implementation is scaffold-level creator UX logic for validation and lineage modeling.
- Renderer, history, and federation domains remain scaffold-level runtime domains.
- Full interactive UI rendering and live network deployment are outside this milestone patch.
