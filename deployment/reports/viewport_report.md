# Viewport Report

## GUI readiness

EverArcade Studio now has a native Rust desktop GUI crate (`studio-gui`) using the egui/eframe application model. The shell includes the required panels: Hierarchy, Inspector, Viewport, Assets, Replay, Simulation, Diagnostics, Deployment, and Console.

## Creator workflow readiness

The visual workflow covers Create Project, Import Assets, Build World, Place Entities, Run Simulation, Inspect Replay, Inspect Runtime, Package Content, and Deploy Runtime. Project management, asset browsing, package publishing, and deployment surfaces are integrated into the Studio app model.

## Replay safety guarantees

Studio is projection-only. Viewport selection consumes runtime projection data, replay UI is reconstruction-only, inspector edits are routed through deterministic editor actions, and direct authority mutation requests are rejected.

## Deployment readiness

The deployment workspace displays runtime status, node status, deployment lineage, package lineage, and federation topology, with deploy, validate, restore, and rollback actions represented in the GUI model.

## Remaining limitations

The GUI is an initial native scaffold with deterministic in-memory models and validation tests. Docking persistence is represented by a deterministic serialized layout; future work can replace the internal docking model with a richer third-party dock widget while preserving authority boundaries.
