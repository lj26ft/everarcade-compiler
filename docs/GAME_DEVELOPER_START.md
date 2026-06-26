# Game Developer Start

## Fresh VM Flow
```bash
git clone https://github.com/lj26ft/everarcade-compiler.git
cd everarcade-compiler
./scripts/everarcade_start.sh
cargo run -p everarcade-cli -- start-game 2d-arena
```

If cloning with HTTPS for a private repo, use a PAT.

## Core artifacts
- `runtime/world/status.txt`
- `runtime/replay/latest/frame-0001.json`
- `runtime/games/2d-arena/`
- `clients/web-reference/index.html`

## Appliance Flow
- `./bin/everarcade start-game 2d-arena`
