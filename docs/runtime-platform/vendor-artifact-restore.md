# Vendor Artifact Restore

The repository validates the runtime with Cargo in offline mode. Cargo is
configured to replace crates.io with a local `vendor/` directory, but `vendor/`
is intentionally ignored because it is generated dependency material rather than
source owned by this repository.

## Why `vendor/` is ignored

`vendor/` contains third-party crate sources restored or generated for local
release validation. Keeping it out of Git avoids a large dependency snapshot in
normal source diffs and prevents accidental commits of generated artifacts.

## Why offline validation still needs it

`.cargo/config.toml` points Cargo at `vendor/` for offline dependency resolution.
When `vendor/` is absent, `cargo metadata --offline --locked` and the targeted
runtime checks cannot resolve registry dependencies from crates.io.

## Package the local artifact

Use this only when a valid local `vendor/` directory already exists:

```bash
bash scripts/package_vendor_artifact.sh
```

The script does not run `cargo vendor` and does not modify `Cargo.lock`. It
creates:

```text
dist/vendor.tar.gz
dist/vendor.tar.gz.sha256
```

The checksum is printed so operators can compare the artifact they distribute or
store outside Git.

## Restore on a clean clone

Place both artifact files in `dist/` and run:

```bash
bash scripts/restore_vendor_artifact.sh
```

The restore script verifies `dist/vendor.tar.gz.sha256` before deleting any
existing `vendor/` directory. If the artifact is missing or the checksum fails,
it exits without replacing `vendor/`.

## Run the runtime offline gate

After restoring `vendor/`, run:

```bash
bash scripts/check_runtime_offline_gate.sh
```

The gate runs the minimal offline validation set:

```bash
cargo metadata --offline --locked --format-version 1
cargo check -p everarcade-runtime --offline --locked
cargo test -p everarcade-runtime --tests --offline --locked
```

It writes a compact report to:

```text
reports/runtime_offline_gate_report.txt
```

If `vendor/` is missing, the gate prints:

```text
vendor/ is missing. Run: bash scripts/restore_vendor_artifact.sh
```

## Commit policy

Do not commit `vendor/`. Treat it as a restored artifact for offline validation,
not as repository source.
