# Rustrig Milestone Report

## ABI readiness
ABI v1 is append-only, backward compatible, deterministically serialized, canonically hashed, and replay-safe.

## SDK readiness
The SDK exposes an `everarcade-sdk::rustrigs` workflow that mirrors `cargo new my-rustrig` and `cargo add everarcade-rig-combat` developer ergonomics.

## Marketplace readiness
Marketplace metadata supports Rustrig Package, Rustrig Bundle, and Rustrig Template artifacts with version, hash, author, dependencies, and record types.

## Studio readiness
Studio models Rustrig Browser, Rustrig Library, Rustrig Search, Rustrig Composition, and Rustrig Validation surfaces. Visual logic nodes map to Rustrigs and emit records.

## Replay guarantees
Rustrigs are deterministic pure primitives that consume explicit input and emit protocol records. Runtime authority remains in execution-core/world/replay/orchestrator surfaces.

## Known limitations
This milestone establishes the canonical ABI, primitive library, validation harness, and integration surfaces. Rich UI rendering and package publishing backends remain scaffold-level integrations.
