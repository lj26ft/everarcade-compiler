# EverArcade Client Command Rundown + End-to-End Bootstrap

This repo has **two primary CLIs**:

- `everarcade` (developer client flow: initialize/build/package/run local federation)
- `everarcade-host` (host/operator runtime flow and deep operational commands)

## 1) Bootstrap prerequisites

From repo root:

```bash
# 1) Vendor Rust dependencies locally (required in this repo's default setup)
./scripts/vendor_deps.sh

# 2) Optional fast sanity check for a basic host loop
./scripts/linux_vm_smoke.sh
```

Why vendoring first: the workspace expects crates to resolve through a local `vendor/` directory via `.cargo/config.toml` after running `scripts/vendor_deps.sh`.

## 2) Developer client (`everarcade`) commands

Build and run:

```bash
cargo run -p everarcade-cli -- <command>
```

Available commands:

- `init-game [name]`
  - Creates `.everarcade-dev/<name>`.
- `build-game`
  - Writes `.everarcade-dev/build.json` with deterministic build metadata.
- `package-game`
  - Hashes `build.json` into `.everarcade-dev/package.hash`.
- `run-local-federation`
  - Creates local node folders (`node-a/node-b/node-c`) plus timeline/inspector stubs.
- `replay-world`
  - Emits `.everarcade-dev/replay.log`.
- `inspect-simulation`
  - Emits `.everarcade-dev/simulation.inspect`.
- `xahau-build-hooks`
- `xahau-install-hooks`
- `xahau-verify-hooks`
- `xahau-submit-settlement`
- `xahau-anchor-checkpoint`
- `xahau-vault-status`
  - Xahau-prefixed commands write status artifacts under `.everarcade-dev/xahau/`.

### Minimal end-to-end dev client flow

```bash
cargo run -p everarcade-cli -- init-game my-first-world
cargo run -p everarcade-cli -- build-game
cargo run -p everarcade-cli -- package-game
cargo run -p everarcade-cli -- run-local-federation
cargo run -p everarcade-cli -- replay-world
cargo run -p everarcade-cli -- inspect-simulation
```

## 3) Host/operator CLI (`everarcade-host`) quick flow

Build and run:

```bash
cargo run -p everarcade-host -- <command>
```

Core start-here commands:

- `init --state <path>`
- `generate-fixture --output <path>`
- `run --package <path> --state <path>`
- `verify --state <path>`
- `status --state <path>`
- `doctor --state <path>`

### Minimal end-to-end host flow

```bash
STATE_DIR=$(mktemp -d)
PKG_FILE=$(mktemp)

cargo run -p everarcade-host -- init --state "$STATE_DIR"
cargo run -p everarcade-host -- generate-fixture --output "$PKG_FILE"
cargo run -p everarcade-host -- run --package "$PKG_FILE" --state "$STATE_DIR"
cargo run -p everarcade-host -- verify --state "$STATE_DIR"
cargo run -p everarcade-host -- status --state "$STATE_DIR"
```

## 4) Useful script shortcuts for end-to-end confidence

- `./scripts/linux_vm_smoke.sh`
  - Runs fixture generation + run + verify loop via `everarcade-host`.
- `./scripts/install_smoke.sh`
  - Exercises release packaging and install/uninstall lifecycle.
- `./scripts/release_smoke.sh`
  - End-to-end release package integrity + deployment smoke.
- `./scripts/bootstrap_e2e.sh`
  - One-command happy path: vendors deps, runs `everarcade` dev flow, then runs `everarcade-host` init/generate/run/verify/status.
- `./scripts/local_cluster.sh`
  - Runs selected multi-node/local-cluster integration tests.
- `./scripts/networked_cluster.sh`
  - Lightweight simulated networked cluster confirmation.

## 5) Practical notes

- If `cargo` fails with missing `vendor/` or crates index errors, run `./scripts/vendor_deps.sh` first.
- Use temp state directories (`mktemp -d`) for clean local loops.
- Start with `bootstrap_e2e.sh` for one-command bring-up, then use `linux_vm_smoke.sh`/`local_cluster.sh` for deeper confidence.
