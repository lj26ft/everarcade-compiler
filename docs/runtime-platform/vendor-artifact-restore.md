# Vendor Artifact Restore

Runtime offline validation resolves crates through a generated `vendor/`
directory. That dependency snapshot is required for offline checks, but it is
not repository source and must not be committed.

## Git policy

The following generated paths are ignored:

```text
vendor/
dist/vendor.tar.gz
dist/vendor.tar.gz.sha256
```

Vendor archives are release attachments or local operator artifacts, not normal
source files. Codex and contributors should not modify, stage, force-add, or
commit `vendor/`, `dist/vendor.tar.gz`, or `dist/vendor.tar.gz.sha256`.

## Why the artifact exists

`.cargo/config.toml` replaces crates.io with `vendor/` so validation can run
without registry access. A fresh clone does not include `vendor/`, so offline
validation cannot resolve dependencies until the directory is restored from an
external artifact.

## Package a local artifact

Use this only when a valid local `vendor/` directory already exists:

```bash
bash scripts/package_vendor_artifact.sh
```

The script does not run `cargo vendor` and does not modify `Cargo.lock`. It
creates ignored local files:

```text
dist/vendor.tar.gz
dist/vendor.tar.gz.sha256
```

To distribute them, attach both files to the release. Do not include them in a
source commit or PR.

## Restore on a fresh clone

Before offline validation on a fresh clone, obtain both files from one of these
sources:

- a local build
- a release attachment
- a copied `dist/` directory

Place them in `dist/` and run:

```bash
bash scripts/restore_vendor_artifact.sh
```

The restore script verifies `dist/vendor.tar.gz.sha256` before deleting any
existing `vendor/` directory. If the artifact is missing or checksum
verification fails, it exits without replacing `vendor/`.

## Run offline validation

After restoring `vendor/`, run:

```bash
bash scripts/check_runtime_offline_gate.sh
```

The gate runs the minimal offline validation set and writes:

```text
reports/runtime_offline_gate_report.txt
```

If `vendor/` is missing, restore it from a release artifact before running the
offline gate.
