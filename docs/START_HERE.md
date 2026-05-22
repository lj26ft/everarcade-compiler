# START HERE: Zero-Friction EverArcade Bootstrap

## 1) Run this first

```bash
git clone git@github.com:lj26ft/everarcade-compiler.git
cd everarcade-compiler
bash scripts/release_validate_fresh_vm.sh
```

If cloning over HTTPS and the repository is private, use a Personal Access Token (PAT).


## 2) Canonical pre-release validation

Run this before tagging or packaging a release:

```bash
bash scripts/release_validate_fresh_vm.sh
```

This validates the fresh-VM runtime-first path (`runtime/...` as canonical layout), and does **not** require `.everarcade-dev` as a primary bootstrap contract.

## 3) What artifacts to inspect

After bootstrap completes, inspect `runtime/` artifacts:

- `runtime/world/status.txt`
- `runtime/replay/latest/frame-0001.json`
- `runtime/games/2d-arena/`
- `clients/web-reference/index.html`

## 4) Next commands

```bash
cargo run -p everarcade-cli -- start
cargo run -p everarcade-cli -- start-game 2d-arena
cargo run -p everarcade-cli -- list-games
```

Optional quick diagnostics:

```bash
./scripts/doctor_quick.sh
```
