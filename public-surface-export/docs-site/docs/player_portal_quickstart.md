# Player Portal Quickstart

Use this browser-first flow when testing Arena Vanguard from a host browser against a local VM or Multipass guest.

```bash
everarcade run arena-vanguard
cd frontend/player-portal
npm run dev
```

Open:

```text
http://<vm-ip>:5173
```

The Player Portal resolves the Arena Vanguard runtime feed from the browser host instead of hardcoding `localhost`:

```text
ws://<current-host>:8791/runtime-feed
```

Set `VITE_ARENA_VANGUARD_GATEWAY_PORT` if the Arena Vanguard gateway is exposed on another port.

If the runtime is not running, the Player Portal shows `🟡 Waiting for runtime`. If the first connection attempt fails, it shows `🔴 Could not connect to runtime` with the command to run.
