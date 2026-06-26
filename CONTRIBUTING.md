# Contributing to EverArcade

Thank you for helping make EverArcade understandable and usable by external developers. Start with `README.md`, use `REPOSITORY_MAP.md` to find subsystems, read `ARCHITECTURE.md` for the system flow, and check `MATURITY.md` before making capability claims.

## Support boundary

EverArcade v0.1 is an open-source candidate focused on local deterministic runtime proofs. Do not describe local PASS reports as production, public-testnet, or commercial readiness.

## Protected gates

Every PR must pass both gates documented in [`OPEN_SOURCE_READINESS.md`](OPEN_SOURCE_READINESS.md):

| Gate | Question | Workflow | Local check | Expected |
|------|----------|----------|-------------|----------|
| **Contributor Gate** | Can you reproduce EverArcade? | `.github/workflows/onboarding.yml` | `bash scripts/validate_open_source_readiness.sh` | `READY` |
| **World Artifact Gate** | Can the repo still produce a valid world? | `.github/workflows/deterministic-world-factory.yml` | `bash scripts/ci/run-deterministic-world-factory.sh` | `PASS` |

**Contributor Level 0** (before your first code change):

```bash
bash scripts/ensure_vendor_offline.sh
bash scripts/validate_open_source_readiness.sh
bash scripts/ci/run-deterministic-world-factory.sh
```

## Before opening a PR

```bash
bash scripts/ensure_vendor_offline.sh
cargo fmt --check
cargo build -p everarcade-cli
node creator-sdk/cli/everarcade.mjs world factory generate --project examples/world-factory/frontier-settlement
node creator-sdk/cli/everarcade.mjs world factory verify --project examples/world-factory/frontier-settlement
node creator-sdk/cli/everarcade.mjs world factory replay --project examples/world-factory/frontier-settlement
```

Full World Artifact Gate (determinism + release bundle + attestation):

```bash
bash scripts/ci/run-deterministic-world-factory.sh
```

See [`OPEN_SOURCE_READINESS.md`](OPEN_SOURCE_READINESS.md), [`docs/reproducible-builds.md`](docs/reproducible-builds.md).

## Canonical contributor gate (run these first)

Every PR should pass the **3-command onboarding gate** after prerequisites:

```bash
bash scripts/check_prerequisites.sh
CARGO_BUILD_JOBS=1 bash scripts/validate_developer_onboarding.sh
bash examples/reference-certified-world-v1/operator/verify.sh examples/reference-certified-world-v1
```

Optional but recommended before opening a PR:

```bash
bash scripts/validate_open_source_readiness.sh
git diff --check
```

### What PASS means

- `WORLD VERIFY: PASS` / onboarding PASS — local Creator SDK + runtime proof succeeded.
- `REFERENCE CERTIFIED WORLD V1: PASS` — reference world certification checks passed.
- `Prerequisites: PASS` — offline vendor restored and `everarcade-runtime` checks offline.
- Reports under `reports/` for scaffold areas (federation, GPU, marketplace, etc.) certify **models**, not production services.

CI runs the same gate on Ubuntu and macOS via `.github/workflows/onboarding.yml` with **no network Cargo fetches**.

## Offline vendor

`vendor/` is restored from the committed `dist/vendor.tar.gz` bundle. You do not need network access after clone. Maintainers regenerate the bundle with `bash scripts/vendor_deps.sh` (network required). See `docs/build/offline-build-policy.md`.

### Fixing offline vendor issues (ELI5)

The `vendor/` folder is the full box of dependency bricks. If you see `no matching package named 'anyhow'` (or similar) during `play-local` or `cargo check --offline`:

```bash
bash scripts/ensure_vendor_offline.sh
bash scripts/check_prerequisites.sh
```

If that still fails (maintainers only, network required once):

```bash
bash scripts/vendor_deps.sh
git add dist/vendor.tar.gz dist/vendor.tar.gz.sha256 Cargo.lock
```

Creator SDK runtime commands run `cargo` from the **repository root** so offline vendor policy applies. Temporary `/tmp` cargo workspaces are not used for play-local.

## Repository navigation

- `ARCHITECTURE.md` explains the Developer → World Factory → `world.evr` → Verification → Registry → Vault → Operator → Player flow.
- `REPOSITORY_MAP.md` maps directories to audiences and maturity boundaries.
- `REPOSITORY_POLICY.md` explains what belongs in git versus GitHub Releases.
- `scripts/MANIFEST.md` explains every script before you run or modify it.
- Examples live under `examples/`, `creator-examples/`, and template paths referenced from the Creator SDK docs.

## How to test targeted changes

Match validation to your change. Examples:

```bash
# Developer experience / docs
bash scripts/certify_developer_experience.sh

# Runtime or Creator SDK
CARGO_BUILD_JOBS=1 bash scripts/validate_playable_local_game.sh
CARGO_BUILD_JOBS=1 bash scripts/validate_creator_sdk.sh
```

Avoid broad `cargo test --workspace` unless a maintainer explicitly requests it. Use `CARGO_BUILD_JOBS=1` for large validation runs.

## Coding standards

- Keep scripts deterministic, non-interactive, and safe to run from the repository root.
- Make generated report paths explicit.
- Label scaffold, prototype, and production-candidate areas honestly.
- Do not add try/catch blocks around imports.
- Do not commit `vendor/`, `node_modules/`, runtime roots, or local build outputs unless explicitly approved fixtures.
- Do commit updated `dist/vendor.tar.gz` + checksum when `Cargo.lock` dependency sets change.
- Keep documentation claims aligned with `README.md`, `MATURITY.md`, and `docs/runtime-platform/proof-chain.md`.

## Submitting changes

1. Create a focused branch.
2. Update documentation for behavior changes.
3. Run the canonical gate (and targeted scripts as needed).
4. Ensure `git diff --check` passes.
5. Open a pull request using the template — include maturity-impact notes.

## Security-sensitive changes

Do not include secrets, credentials, wallet keys, or private operator configuration. If you find a vulnerability, follow `SECURITY.md` rather than opening a public exploit report.
## Gate artifact boundaries

The **Contributor Gate** proves a first-time contributor can restore the canonical vendor bundle, validate the pinned toolchain, and run the local development/onboarding proof path. It produces a runnable local development environment, runtime package checks, and readiness reports. It does **not** produce or certify `world.evr`.

The **World Artifact Gate** is the only protected gate that produces the canonical Frontier Settlement `world.evr`, verifies it, replays it, creates the deployment bundle, and verifies the release attestation. Use this gate when the artifact under review is `world.evr`, an attestation, or a release bundle.

Determinism claims are limited to the pinned Rust toolchain in `rust-toolchain.toml`, the pinned Node runtime in `.nvmrc`, the committed vendor bundle, and isolated CI/local gate runs. Cross-machine reproducibility outside those pins remains a future hardening goal.
