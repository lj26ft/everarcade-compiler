# Arena Vanguard Host

Authoritative live-session host for Arena Vanguard.

Responsibilities:

- Runtime lifecycle: `start`, `stop`, `restart`, `recover`, `status`.
- Session management and registry updates.
- Checkpoint management and replay persistence.
- Player session persistence under `player_sessions/`.
- Runtime-owned telemetry for gateway and operator feeds.

The host owns state mutation. The gateway only transports join requests, action submissions, heartbeats, status reads, and resume tokens.
