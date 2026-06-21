# Engine Adapter SDK v1

EverArcade engine adapters let an existing client keep its renderer, assets, input devices, camera, audio, and gameplay presentation while replacing the authoritative server with an EverArcade World.

## Adapter architecture

```text
Client
↓
Adapter
↓
World Contract API
↓
EverArcade Runtime
```

- **Client**: Unity, Godot, Unreal, Bevy, or Browser game code that owns rendering and local presentation.
- **Adapter**: Thin SDK layer that authenticates, transports input, reads projections, and exposes verification hooks.
- **World Contract API**: Language-neutral boundary for submitted inputs, world projections, receipts, packages, and RustRig execution.
- **EverArcade Runtime**: Authoritative execution environment for World Contracts, RustRigs, World Packages, receipts, and projection publication.

## Common SDK surface

All adapters expose the same concepts even when naming follows engine conventions:

```text
connectWorld(config) -> WorldConnection
submitInput(connection, input) -> InputReceipt
readProjection(connection, query) -> ProjectionFrame
verifyWorld(connection, proof) -> VerificationResult
disconnect(connection) -> void
```

### Method responsibilities

| Method | Responsibility |
| --- | --- |
| `connectWorld()` | Opens a session to a world endpoint, negotiates protocol version, identifies the world package, and returns a connection handle. |
| `submitInput()` | Sends player or bot intent to the World Contract API; the world remains authoritative over acceptance and ordering. |
| `readProjection()` | Reads the latest or requested projection frame for rendering in the existing engine. |
| `verifyWorld()` | Checks world identity, package digest, receipts, checkpoint proofs, or projection commitments where supported. |
| `disconnect()` | Closes transport and releases local adapter resources without changing world state. |

## Integration principle

```text
Keep the engine.
Keep the assets.
Keep the gameplay.
Replace the authoritative server.
```

The adapter should not dictate render loops, ECS layout, animation systems, physics visualization, shaders, or asset formats. Those stay in the engine. Authority, deterministic state transition, package validation, RustRig execution, and receipts move to EverArcade.
