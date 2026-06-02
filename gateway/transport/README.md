# Gateway Transport

Arena Vanguard gateway transport supports:

- HTTP join/action/status submission.
- WebSocket world-state streaming.
- Reconnect and session resume using resume tokens.
- Heartbeat validation, connection timeout, and retry metadata.

The transport is non-authoritative: it never writes world, player, enemy, inventory, XP, or level state directly. It forwards actions to the runtime host and relays runtime state feeds.
