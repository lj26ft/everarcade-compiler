# Adapter Demo: Top-down Survival

This example describes a small top-down survival game connected to an EverArcade world through the Engine Adapter SDK v1.

## Scenario

- **Player movement**: Engine captures WASD, stick, or touch input and submits normalized movement intent.
- **Inventory**: World Contract owns authoritative item grants, drops, equipment, and consumption.
- **Combat**: Engine submits attack intent; World Contract resolves hit timing, damage, cooldowns, and loot.
- **Projection**: Engine reads projection frames containing player transforms, enemy states, inventory snapshots, health, and combat events.

## Flow

```text
Browser, Unity, Godot, Bevy, or Unreal client
↓ submitInput(move, attack, use_item)
Adapter
↓
World Contract API
↓ authoritative simulation + RustRigs + package rules
EverArcade Runtime
↓ readProjection(frame)
Adapter
↓
Existing engine renderer
```

## Minimal browser sketch

```javascript
const world = await connectWorld({ endpoint, worldId: 'adapter-demo', playerId });
await submitInput(world, { action: 'move', payload: { x: 1, y: 0 } });
await submitInput(world, { action: 'attack', payload: { slot: 0, target: 'enemy-7' } });
const frame = await readProjection(world, { stream: 'main' });
render(frame);
```
