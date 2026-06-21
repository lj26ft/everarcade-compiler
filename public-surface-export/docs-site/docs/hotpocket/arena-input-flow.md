# Arena Input Flow

```mermaid
sequenceDiagram
  participant P as Player
  participant C as arena-submit
  participant H as HotPocket Wrapper
  participant R as Arena Runtime
  participant J as Journal
  P->>C: join / move / attack / disconnect
  C->>H: canonical JSON input
  H->>H: validate envelope
  H->>R: forward input
  R->>R: deterministic tick
  R->>J: append entry and receipt
  H-->>C: receipt + roots
```

The live CLI supports:

```bash
arena-submit join --player player-1
arena-submit move --player player-1 --direction north
arena-submit attack --player player-1 --target player-2
arena-submit disconnect --player player-1
```

Set `ARENA_HOTPOCKET_URL` to point at a remote running contract wrapper.
