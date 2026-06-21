# Unity Adapter

The Unity adapter targets existing Unity clients that keep GameObjects, prefabs, scenes, animations, physics presentation, UI, and asset bundles while delegating authority to an EverArcade world.

## Components

- **World Client**: Manages session lifecycle, endpoint configuration, and protocol negotiation.
- **Input Submitter**: Sends player intent from Unity input systems to the World Contract API.
- **Projection Reader**: Reads authoritative projection frames for rendering and reconciliation.
- **Verification Client**: Verifies world identity, package digests, receipts, and projection commitments.

## SDK surface

```text
connectWorld(config)
submitInput(connection, input)
readProjection(connection, query)
verifyWorld(connection, proof)
disconnect(connection)
```
