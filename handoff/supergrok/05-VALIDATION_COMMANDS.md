# Validation Commands — EverArcade Compiler

## Prerequisites

| Tool | Required for | Install |
|------|--------------|---------|
| `bash` | All scripts | system |
| `cargo` / `rustc` | Rust builds | rustup |
| `node` 18+ | Creator SDK | nvm/system |
| `rg` (ripgrep) | `validate_open_source_readiness.sh` | `apt install ripgrep` |
| `openssl` | Manifest signing scripts | system |
| `tar`, `sha256sum` | Reference world packaging | system |
| Network | Cargo deps (vendor incomplete) | — |

**Vendor note:** `.cargo/config.toml` sets `offline = true` and `vendor/` as source. Vendor snapshot is **incomplete** (missing `bincode`). Full offline workspace build may fail. Use network Cargo or restore vendor per `docs/build/offline-build-policy.md`.

---

## Canonical gate (required for most PRs)

```bash
# From repo root
bash scripts/check_prerequisites.sh
CARGO_BUILD_JOBS=1 bash scripts/validate_developer_onboarding.sh
bash examples/reference-certified-world-v1/operator/verify.sh examples/reference-certified-world-v1
```

Creates temp project, builds via Creator SDK, packages, runs local session, verifies replay.

Expected: onboarding completes without error; look for `WORLD VERIFY: PASS` in flow.

```bash
bash examples/reference-certified-world-v1/operator/verify.sh examples/reference-certified-world-v1
```

Expected: `REFERENCE CERTIFIED WORLD V1: PASS`

```bash
bash scripts/validate_open_source_readiness.sh
```

Expected: classification READY or CONDITIONAL READY; no FAIL lines.

---

## Manual First World flow

```bash
TMPDIR="$(mktemp -d)"
PROJECT="$TMPDIR/frontier-world"

node creator-sdk/cli/everarcade.mjs world templates
node creator-sdk/cli/everarcade.mjs world rustrigs
node creator-sdk/cli/everarcade.mjs world init --template frontier --name frontier-world --dir "$PROJECT"
CARGO_BUILD_JOBS=1 node creator-sdk/cli/everarcade.mjs world run --project "$PROJECT"
node creator-sdk/cli/everarcade.mjs world package --project "$PROJECT"
node creator-sdk/cli/everarcade.mjs world verify --project "$PROJECT"
```

---

## Reference world build + verify

```bash
cd examples/reference-certified-world-v1
./operator/build-world-evr.sh .
./operator/verify.sh .
```

Build artifact: `/tmp/everarcade-v0.1-world.evr` (when using full v0.1 flow from docs).

Certified hash: `sha256:ef5409866bb75211145a0da901611621c57237bc79dad0c0c2cdde1dc3873883`

---

## Open source / contributor checks

```bash
git diff --check
bash scripts/validate_open_source_readiness.sh
bash scripts/certify_developer_experience.sh
```

---

## Runtime-targeted (only when touching runtime/SDK)

```bash
CARGO_BUILD_JOBS=1 bash scripts/validate_playable_local_game.sh
CARGO_BUILD_JOBS=1 bash scripts/validate_creator_sdk.sh
CARGO_BUILD_JOBS=1 bash scripts/validate_deterministic_execution.sh
```

---

## Scaffold area scripts (DO NOT use as production proof)

These validate **deterministic models**, not live services:

```bash
bash scripts/validate_multi_lease_federation.sh      # federation shell model
bash scripts/validate_renderer_runtime.sh            # projection model
bash scripts/validate_gpu_marketplace.sh             # GPU model
bash scripts/validate_player_gateway.sh              # record hashes
bash scripts/validate_creator_marketplace.sh         # marketplace model
bash scripts/validate_commercial_revenue.sh          # revenue model
bash scripts/validate_public_testnet.sh              # BROKEN: missing public-testnet/ at root
bash scripts/validate_xrpl_live_settlement.sh        # XRPL model
```

---

## Avoid unless maintainer requests

```bash
cargo test --workspace          # slow; broad; may hit vendor issues
bash scripts/validate.sh        # umbrella; unclear scope
# Running all 65 validate_*.sh scripts sequentially
```

Use `CARGO_BUILD_JOBS=1` for large Cargo builds to reduce memory pressure.

---

## Key script locations

| Script | Purpose |
|--------|---------|
| `scripts/validate_developer_onboarding.sh` | Primary contributor gate |
| `scripts/validate_open_source_readiness.sh` | Doc/policy presence check |
| `scripts/validate_playable_local_game.sh` | Local game launch |
| `scripts/validate_creator_sdk.sh` | SDK smoke |
| `scripts/validate_repo_reality_audit.sh` | Regenerate reality audit |
| `scripts/vendor_deps.sh` | Restore vendor directory |
| `scripts/restore_vendor_artifact.sh` | Restore from release artifact |
| `examples/reference-certified-world-v1/operator/verify.sh` | Reference world gate |

---

## Expected PASS semantics

| Output | Meaning |
|--------|---------|
| `WORLD VERIFY: PASS` | Creator SDK local verification succeeded |
| `REFERENCE CERTIFIED WORLD V1: PASS` | Reference world certification |
| `Open Source Audit: PASS` | Readiness script completed (may be CONDITIONAL) |
| `EVERARCADE V0.1 OPEN SOURCE READY: PASS` | Security audit result |
| `reports/*: PASS` | Model certification — **not production** |

**Never equate PASS with production, testnet, or commercial readiness.**