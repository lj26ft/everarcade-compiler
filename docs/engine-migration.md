# Engine Migration Guide

EverArcade Engine Adapter SDK v1 is for teams that already have a working game client and do not want to rewrite it. The migration replaces the authoritative server while preserving the engine, assets, and rendering pipeline.

## What stays in the engine

- Rendering, cameras, lighting, shaders, particles, animation, audio, UI, and asset loading.
- Local input capture and device-specific control schemes.
- Client-side prediction, interpolation, smoothing, and visual effects.
- Scene organization, prefabs, nodes, actors, ECS presentation components, and editor workflows.
- Non-authoritative presentation physics used for feel and visuals.

## What moves into the World Contract

- Authoritative player intent validation and input ordering.
- Deterministic game-state transitions.
- Inventory grants, consumption, crafting, drops, and equipment rules.
- Combat resolution, cooldowns, hit validation, damage, death, rewards, and respawn rules.
- Economy rules, progression, ownership, match lifecycle, and world-specific permissions.
- Projection definitions that describe what the client may render from authoritative state.

## What becomes a RustRig

- Deterministic gameplay modules that benefit from reusable, auditable Rust logic.
- AI behaviors, encounter directors, loot tables, combat calculators, procedural spawning, and simulation helpers.
- Shared rules that multiple World Contracts or World Packages need to compose.
- Performance-sensitive logic that should run near the EverArcade Runtime instead of inside a client engine.

## Migration checklist

1. Identify the current authoritative server responsibilities.
2. Move authoritative rules into a World Contract.
3. Extract reusable deterministic modules into RustRigs.
4. Package assets, metadata, contract bindings, and adapter expectations into a World Package.
5. Keep the existing engine renderer and connect it through the adapter SDK.
6. Submit input through `submitInput()` instead of directly mutating authoritative state.
7. Render `readProjection()` output instead of trusting local-only state.
8. Add `verifyWorld()` where the client must confirm package, receipt, checkpoint, or projection integrity.

## Migration outcome

```text
I do not need to rewrite my game.
I connect my game to a World.
The World becomes authoritative.
The engine remains the renderer.
```
