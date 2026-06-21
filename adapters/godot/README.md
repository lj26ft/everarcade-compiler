# Godot Adapter

The Godot adapter is a minimal-footprint design for GDScript or C# projects that already own scenes, nodes, animation, input maps, and rendering.

## Components

- **World Client**: Opens and closes the EverArcade world connection.
- **Input Submitter**: Converts Godot input actions into language-neutral World Contract inputs.
- **Projection Reader**: Pulls projection frames and applies them to local nodes without making Godot authoritative.

## SDK surface

```text
connectWorld(config)
submitInput(connection, input)
readProjection(connection, query)
verifyWorld(connection, proof)
disconnect(connection)
```

Implementation should prefer Godot-native HTTP/WebSocket APIs and avoid large dependencies so projects can vendor the adapter directly.
