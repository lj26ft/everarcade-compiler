# EverArcade Compiler (v0.1.x)

EverArcade is a deterministic runtime/appliance toolchain for packaging and validating reproducible game/world execution artifacts.

## Maturity level

- **Current status:** v0.1.x pre-production hardening.
- **Production-useful today:** deterministic runtime packaging/validation flows, offline vendored Rust builds, release validation scripts.
- **Prototype/stub areas:** several SDK and higher-level gameplay/docs surfaces still evolving; use `docs/stub-vs-usable-matrix.md` as the current source of truth.

## Quickstart (fresh clone)

```bash
git clone <repo>
cd everarcade-compiler
bash scripts/vendor_deps.sh
cargo fmt --all --check
cargo test --workspace
bash scripts/build_runtime_release.sh
bash scripts/validate_clean_vm_bootstrap.sh
```

## Offline/vendor requirement

This repository is configured for vendored Cargo dependencies via `.cargo/config.toml`. You must generate `vendor/` first:

```bash
bash scripts/vendor_deps.sh
```

Build/test/release scripts intentionally fail fast if `vendor/` is missing and do not silently fall back to network mode.

## Runtime packaging flow

1. Generate vendored dependencies (`scripts/vendor_deps.sh`).
2. Build runtime release bundle (`scripts/build_runtime_release.sh`).
3. Validate a clean bootstrap from packaged runtime (`scripts/validate_clean_vm_bootstrap.sh`).
4. Optionally run release gate checks (`scripts/release_validate.sh`).

## Deterministic appliance model

EverArcade packages binaries, runtime config, and a manifest into a deterministic runtime appliance (`dist/everarcade-runtime...`). Validation scripts exercise bootstrap/start/validate/shutdown in a reproducible path intended for clean Linux VM onboarding.

## Where to start as a new developer

- Start with `docs/START_HERE.md` for quickest operator path.
- Use `docs/README.md` for the docs map.
- Review `docs/stub-vs-usable-matrix.md` before building new features.
- For local reproducibility and audit context, read `docs/onboarding-audit.md` and `docs/v0.1.0-runtime-audit.md`.
