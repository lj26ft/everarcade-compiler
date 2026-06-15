# Player Portal Browser Smoke

- Entrypoint fixed: `frontend/player-portal/index.html` now declares `<!DOCTYPE html>` and loads `/src/main.tsx`, so React mounts through the Vite main entrypoint instead of loading `App.tsx` directly.
- Visible UI confirmed: the smoke surface includes `🎮 EverArcade Player Portal`, the Arena Vanguard card, `Play Arena Vanguard`, `Runtime Status`, `Gateway Status`, and `WebSocket Status`.
- Runtime status surfaced: the default state is `🟡 Waiting for runtime`; failed runtime connections show `🔴 Could not connect to runtime` and `Run: everarcade run arena-vanguard`.
- Remaining limitations: this report records code-level and build validation. Manual host-browser validation depends on an available VM IP and live gateway process.
