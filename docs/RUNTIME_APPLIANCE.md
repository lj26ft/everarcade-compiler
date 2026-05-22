# Runtime Appliance
Use `scripts/build_vm_runtime_appliance.sh` to create `everarcade-runtime-appliance-v0.1.0.tar.gz`.

The runtime appliance is validated with `start-game 2d-arena` and expected to produce:
- `runtime/world/status.txt` (or `world/status.txt` when extracted at runtime root)
- `runtime/replay/latest/frame-0001.json` (or `replay/latest/frame-0001.json`)
- `runtime/games/2d-arena/game.toml`
- `clients/web-reference/index.html`
