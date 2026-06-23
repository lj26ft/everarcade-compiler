# Offline Build Policy

## Locked policy (Phase 0)

EverArcade requires **reproducible offline Cargo builds** for clean clones, contributors, auditors, and CI.

- `.cargo/config.toml` sets `offline = true` and uses `vendor/` as the only crates-io source.
- `vendor/` is **not** committed directly (gitignored).
- The **committed** artifact `dist/vendor.tar.gz` + `dist/vendor.tar.gz.sha256` is the reproducible vendor bundle.
- Clean clones restore `vendor/` with:

  ```bash
  bash scripts/ensure_vendor_offline.sh
  ```

- CI and the canonical contributor gate run with `CARGO_NET_OFFLINE=true` and must not fetch crates from the network.

## Day 1 contributor flow

```bash
git clone https://github.com/lj26ft/everarcade-compiler.git
cd everarcade-compiler
bash scripts/check_prerequisites.sh
CARGO_BUILD_JOBS=1 bash scripts/validate_developer_onboarding.sh
bash examples/reference-certified-world-v1/operator/verify.sh examples/reference-certified-world-v1
```

`check_prerequisites.sh` restores `vendor/` from `dist/vendor.tar.gz`, verifies offline `cargo metadata`, and runs a targeted offline `cargo check` on `everarcade-runtime`.

## Why offline vendor matters (ELI5)

We ship the full box of dependency "bricks" inside the repository so anyone can build the same castle without ordering missing pieces online. That is required for sovereign, verifiable, reproducible foundations.

## Fixing "no matching package named anyhow" (or similar)

This usually means `vendor/` was not restored or Creator SDK tried to build outside the repo workspace.

```bash
bash scripts/ensure_vendor_offline.sh
bash scripts/check_prerequisites.sh
CARGO_BUILD_JOBS=1 bash scripts/validate_developer_onboarding.sh
```

Creator SDK `play-local` runs `cargo run --offline --locked -p everarcade-runtime` from the repository root.

## Maintainer regeneration (network required)

Only maintainers refreshing dependencies need network access:

```bash
bash scripts/vendor_deps.sh
```

Equivalent manual steps:

```bash
cargo vendor --locked vendor
CARGO_NET_OFFLINE=true cargo check --offline --locked
```

This will:

1. Run `cargo vendor` against `Cargo.lock`
2. Write `.cargo/config.toml` with offline vendored sources
3. Verify `cargo metadata --offline --locked`
4. Regenerate `dist/vendor.tar.gz` and `dist/vendor.tar.gz.sha256`

Commit the updated `Cargo.lock` (if changed), `dist/vendor.tar.gz`, and `dist/vendor.tar.gz.sha256`. Do not commit `vendor/` itself.

## Verification commands

```bash
bash scripts/ensure_vendor_offline.sh
CARGO_NET_OFFLINE=true cargo metadata --offline --locked --format-version 1 >/dev/null
CARGO_NET_OFFLINE=true CARGO_BUILD_JOBS=1 cargo check -p everarcade-runtime --offline --locked
bash scripts/validate_open_source_readiness.sh
```

## Claim boundary

Offline builds are **READY** when `scripts/check_prerequisites.sh` and `scripts/validate_open_source_readiness.sh` pass without vendor warnings. This does not claim production, public-testnet, or commercial readiness.