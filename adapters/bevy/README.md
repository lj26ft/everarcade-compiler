# Bevy Adapter

The Bevy adapter is the Rust-native path for games that want EverArcade authority while retaining Bevy ECS, rendering, assets, schedules, and plugins.

## Design direction

- Provide a Bevy plugin that owns EverArcade connection resources.
- Convert Bevy input events or gameplay intents into `submitInput()` calls.
- Publish projection frames into ECS resources or events for rendering systems.
- Reuse Rust-native verification and receipt types where available.

## SDK surface

```text
connectWorld(config)
submitInput(connection, input)
readProjection(connection, query)
verifyWorld(connection, proof)
disconnect(connection)
```
