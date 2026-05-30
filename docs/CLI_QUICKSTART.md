# EverArcade CLI Quick Start

This page is the shortest source-first path from a fresh checkout to a local deterministic game run. It separates the currently usable Rust binaries from scaffold/status surfaces so new users do not confuse roadmap commands with a finished game platform.

## 0) What works today

| surface | binary/crate | maturity | use it for |
|---|---|---|---|
| Product CLI | `everarcade` from `src-bin-everarcade` | usable prototype | Create/install/list/inspect/start/package template games and run diagnostics. |
| Host/operator CLI | `everarcade-host` | usable prototype with many operator scaffolds | Initialize host state, generate a fixture, run/verify a package, inspect world/operator state. |
| Terminal runtime demo | `runtime-client` | local demo | Exercise deterministic input, state roots, inventory, and replay ticks without a renderer. |
| Renderer projection demo | `renderer-client` | scaffold-level projection client | Validate non-authoritative replay/history/projection status surfaces. |
| Renderer/history/federation domains | multiple runtime commands | scaffold | Treat as reconstruction/status-only until vertical-slice integration tests prove live authority boundaries. |

## 1) Install from source

```bash
git clone <repo-url> everarcade-compiler
cd everarcade-compiler
bash scripts/vendor_deps.sh
CARGO_BUILD_JOBS=1 cargo build -p everarcade-cli -p everarcade-host -p runtime-client -p renderer-client
```

The repository is configured to use vendored Cargo dependencies. If `vendor/` is missing, Cargo fails before compiling dependencies. Run `bash scripts/vendor_deps.sh` first on a networked machine, then keep `vendor/`, `Cargo.lock`, and `.cargo/config.toml` together for offline release validation.

Optional shell install after the build:

```bash
mkdir -p ~/.local/bin
cp target/debug/everarcade target/debug/everarcade-host target/debug/runtime-client target/debug/renderer-client ~/.local/bin/
export PATH="$HOME/.local/bin:$PATH"
```

## 2) Create and run a game as fast as possible

The product CLI stores local runtime state under `./runtime/` from the current working directory.

```bash
# Create a deterministic starter game from templates/topdown-arena.
cargo run -p everarcade-cli -- new-game my-first-game

# Inspect the generated manifest.
cargo run -p everarcade-cli -- inspect-game my-first-game

# Start the game and seed runtime/world plus runtime/replay/latest.
cargo run -p everarcade-cli -- start-game my-first-game

# Verify the replay frame created by start-game.
cargo run -p everarcade-cli -- replay-world

# Package the manifest by writing runtime/games/my-first-game/package.hash.
cargo run -p everarcade-cli -- package-game my-first-game
```

Expected success markers:

- `developer_game_created=my-first-game deterministic=true replay=append-only`
- `✅ Game running: my-first-game`
- `replay verified: .../runtime/replay/latest/frame-0001.json`

## 3) Run the built-in template path

```bash
cargo run -p everarcade-cli -- start-game topdown-arena
cargo run -p everarcade-cli -- list-games
cargo run -p everarcade-cli -- inspect-game topdown-arena
```

`start-game <id>` will install `templates/<id>` into `runtime/games/<id>` when that template exists and the game has not already been installed.

## 4) Import an existing local game folder

A game folder should contain at least `game.toml`. The template also includes `runtime.toml`, assets metadata, source, and a replay-equivalence marker test.

```bash
cargo run -p everarcade-cli -- install-game ./templates/topdown-arena
cargo run -p everarcade-cli -- list-games
cargo run -p everarcade-cli -- inspect-game topdown-arena
cargo run -p everarcade-cli -- run-game topdown-arena
```

## 5) Asset, replay, and diagnostics commands

```bash
cargo run -p everarcade-cli -- asset-register
cargo run -p everarcade-cli -- asset-build
cargo run -p everarcade-cli -- asset-verify
cargo run -p everarcade-cli -- diagnostics
cargo run -p everarcade-cli -- runtime-public-api-status
cargo run -p everarcade-cli -- runtime-symbol-audit
cargo run -p everarcade-cli -- workspace-validation-status
```

Use these as deterministic smoke checks. `diagnostics` prints JSON summarizing runtime config, release manifest, replay status, topology status, and validation hints.

## 6) Host/operator quick path

Use `everarcade-host` when you want host state, fixture packages, run receipts, and verification rather than the product CLI's template-game convenience flow.

```bash
cargo run -p everarcade-host -- init --state runtime/host-state
cargo run -p everarcade-host -- generate-fixture --output /tmp/everarcade-package.bin
cargo run -p everarcade-host -- run --package /tmp/everarcade-package.bin --state runtime/host-state
cargo run -p everarcade-host -- verify --state runtime/host-state
```

Good first operator inspection commands:

```bash
cargo run -p everarcade-host -- partition-status --world-root runtime/world
cargo run -p everarcade-host -- simulation-status --world-root runtime/world
cargo run -p everarcade-host -- observer-status --world-root runtime/world
cargo run -p everarcade-host -- topology-status --world-root runtime/world
```

## 7) Runtime demo binaries

The terminal client is a deterministic local loop that emits ticks, roots, validation roots, and a final replay count:

```bash
cargo run -p runtime-client
```

The renderer client is a non-authoritative projection/replay scaffold. It is useful for checking that projection commands are wired, not for proving authoritative gameplay:

```bash
cargo run -p renderer-client -- projection-session-status
cargo run -p renderer-client -- replay-network-status
cargo run -p renderer-client -- historical-query
```

## 8) Validation before sharing a release candidate

Prefer targeted checks while the workspace is still pre-v0.1 and scaffold-heavy:

```bash
cargo fmt --all --check
CARGO_BUILD_JOBS=1 cargo check -p everarcade-cli -p everarcade-host -p runtime-client -p renderer-client
bash scripts/test_fresh_bootstrap_paths.sh
bash scripts/validate_clean_vm_bootstrap.sh
```

Do not present renderer/history/federation status commands as public launch-ready gameplay until they have end-to-end vertical-slice tests covering real package install, session start, input submission, replay verification, renderer projection, persistence, and recovery.
