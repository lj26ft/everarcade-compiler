
## Install / Debug / Doctor / Uninstall

From extracted tarball directory:

```bash
./install.sh --prefix "$HOME/.local/everarcade" --bin-dir "$HOME/.local/bin"
everarcade-host init --state ~/.everarcade
everarcade-host generate-fixture --output /tmp/everarcade-package.bin
everarcade-host run --package /tmp/everarcade-package.bin --state ~/.everarcade
everarcade-host verify --state ~/.everarcade
everarcade-host debug --state ~/.everarcade
everarcade-host doctor --state ~/.everarcade
./uninstall.sh --prefix "$HOME/.local/everarcade" --bin-dir "$HOME/.local/bin"
```

Troubleshooting installer path issue: installer resolves its own script directory and always reads `./bin/everarcade-host` relative to `install.sh`.

## Offline-capable reproducible build flow

From repository root:

```bash
scripts/vendor_deps.sh
cargo build --locked --frozen --offline
cargo test --locked --frozen --offline
scripts/release_validate.sh
```

Notes:
- `scripts/vendor_deps.sh` refreshes `Cargo.lock`, vendors dependencies into `vendor/`, and writes offline cargo source config at `.cargo/config.toml`.
- Keep the repository lean: do **not** commit `vendor/` (it is intentionally gitignored).
- Local offline builds are supported by running `scripts/vendor_deps.sh` before `cargo build/test --offline`.
- Optional release artifact: package vendored dependencies separately with `VENDOR_ARCHIVE=dist/vendor.tar.gz scripts/vendor_deps.sh`.
- `scripts/release_validate.sh` fails fast if `Cargo.lock` or `.cargo/config.toml` is missing, and accepts `VENDOR_ARCHIVE` to hydrate `vendor/` when validating offline without a checked-in vendor directory.
