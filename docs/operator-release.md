
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
- `scripts/vendor_deps.sh` refreshes `Cargo.lock` and vendors dependencies into `vendor/`.
- `scripts/release_validate.sh` now fails fast if `Cargo.lock` or `vendor/` is missing, and validates offline build+test with `--locked --frozen --offline`.
