# Browser Adapter

The browser adapter is the reference implementation for SDK v1 because it has the lowest onboarding friction, works in demos without native tooling, and maps directly onto web transports.

## Provided surface

- `connectWorld(config)`
- `submitInput(connection, input)`
- `readProjection(connection, query)`
- `verifyWorld(connection, proof)`
- `disconnect(connection)`

## Usage

```javascript
import { connectWorld, submitInput, readProjection, disconnect } from './index.js';

const world = await connectWorld({
  endpoint: 'https://world.example.everarcade.dev',
  worldId: 'top-down-survival',
  playerId: 'player-1',
});

await submitInput(world, {
  tick: 42,
  action: 'move',
  payload: { x: 1, y: 0 },
});

const projection = await readProjection(world, { stream: 'main', sinceTick: 42 });
await disconnect(world);
```
