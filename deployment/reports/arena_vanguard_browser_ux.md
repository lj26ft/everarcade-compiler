# Arena Vanguard Browser UX

- Entrypoint fixed: all audited frontend shells use standards mode, mount `#root`, and load `/src/main.tsx`.
- Visible UI confirmed: the first Player Portal screen exposes a Game Card, Play Button, Connection Status, Session Status, HUD Preview, and Controls Help.
- Runtime status surfaced: Play attempts display staged feedback for gateway connection, session join, character spawn, world feed activation, and ready state.
- WebSocket behavior: the runtime feed URL is derived as `ws://<current-host>:8791/runtime-feed` by default, with `VITE_ARENA_VANGUARD_GATEWAY_PORT` available for alternate gateway ports.
- Remaining limitations: renderer/history/federation remain scaffold-level runtime domains; full live multiplayer behavior still requires `everarcade run arena-vanguard` plus a browser reachable gateway.
