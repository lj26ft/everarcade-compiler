# EverArcade Client Command Rundown + End-to-End Bootstrap

This repo has **two primary CLIs**:

- `everarcade` (developer client runtime flow)
- `everarcade-host` (host/operator runtime flow and operational commands)

## 1) Bootstrap prerequisites

From repo root:

```bash
./scripts/vendor_deps.sh
```

## 2) Developer client (`everarcade`) commands

Build and run:

```bash
cargo run -p everarcade-cli -- <command>
```

Preferred runtime commands:

- `start`
- `start-game <game-id>`
- `list-games`
- `inspect-game <game-id>`

Legacy commands remain as compatibility aliases:

- `init-game [name]`
- `build-game`
- `package-game`
- `run-local-federation`
- `replay-world`
- `inspect-simulation`

### Minimal end-to-end dev client flow

```bash
git clone https://github.com/lj26ft/everarcade-compiler.git
cd everarcade-compiler
./scripts/everarcade_start.sh
cargo run -p everarcade-cli -- start-game 2d-arena
```

If cloning over HTTPS and the repository is private, use a Personal Access Token (PAT).

Expected artifacts:

- `runtime/world/status.txt`
- `runtime/replay/latest/frame-0001.json`
- `runtime/games/2d-arena/game.toml`
- `clients/web-reference/index.html`
