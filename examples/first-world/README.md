# First World (Deterministic Onboarding Demo)

`first-world` is the onboarding world designed to prove the runtime works before reading deep architecture docs.

## What this demonstrates

- **Runtime start flow** using `cargo run -p everarcade-cli -- start`.
- **Replay artifacts** produced at `runtime/replay/latest/frame-0001.json`.
- **World state output** produced at `runtime/world/status.txt`.
- **Game manifest materialization** under `runtime/games/2d-arena/`.

## Fast path

Run:

```bash
./scripts/everarcade_start.sh
```

Then inspect:

- `runtime/world/status.txt`
- `runtime/replay/latest/frame-0001.json`
- `runtime/games/2d-arena/game.toml`
- `clients/web-reference/index.html`

## Useful commands

```bash
cargo run -p everarcade-cli -- start
cargo run -p everarcade-cli -- start-game 2d-arena
cargo run -p everarcade-cli -- list-games
```
