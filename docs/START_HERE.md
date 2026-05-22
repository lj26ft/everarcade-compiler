# START HERE: Zero-Friction EverArcade Bootstrap

## 1) Run this first

```bash
git clone git@github.com:lj26ft/everarcade-compiler.git
cd everarcade-compiler
./scripts/everarcade_start.sh
```

If cloning over HTTPS and the repository is private, use a Personal Access Token (PAT).

## 2) What artifacts to inspect

After bootstrap completes, inspect `runtime/` artifacts:

- `runtime/world/status.txt`
- `runtime/replay/latest/frame-0001.json`
- `runtime/games/2d-arena/`
- `clients/web-reference/index.html`

## 3) Next commands

```bash
cargo run -p everarcade-cli -- start
cargo run -p everarcade-cli -- start-game 2d-arena
cargo run -p everarcade-cli -- list-games
```

Optional quick diagnostics:

```bash
./scripts/doctor_quick.sh
```
