# Repository Walkthrough

This walkthrough maps the main repository areas a first-time contributor should recognize.

## `crates/`

Rust crates for deterministic runtime, verification, package, RustRig, and supporting implementation work. Start here when changing compiled behavior, validators, or reusable Rust components.

## `creator-sdk/`

Creator-facing CLI and SDK surface for creating a First World, listing templates, listing RustRigs, running local worlds, packaging `dist/world.evr`, and verifying generated evidence.

## `runtime/`

Runtime-facing state, packages, or local execution assets. Treat renderer, history, and federation-related runtime areas as scaffold-level unless maturity documents say otherwise.

## `templates/`

World Template Library source folders. Templates provide starter World Package layouts, World Contracts, RustRig selections, genesis state, projections, registry metadata, and certification mappings for archetypes such as arena, frontier, settlement, social, and civilization.

## `examples/`

Reference worlds, demo worlds, package layouts, registries, and First World material. Use examples to understand expected artifact shape before changing implementation paths.

## `proofs/`

Proof targets and verification-facing assets. Use this area when working on invariant, certification, replay, restore, or package proof examples.

## `docs/`

Public documentation for onboarding, concepts, runtime operations, verification, templates, certification, security, and open-source launch material. New contributors should start at `docs/index.md` and `docs/open-source-launch/launch-overview.md`.
