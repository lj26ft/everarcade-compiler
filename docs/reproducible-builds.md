# Reproducible Builds

EverArcade compiler builds and World Factory outputs are designed to be **recoverable, repeatable, and verifiable offline**.

## Canonical vendor

The committed vendor bundle consists of:

| Artifact | Purpose |
|----------|---------|
| `dist/vendor.tar.gz` | Compressed vendor tree (canonical transport) |
| `dist/vendor.tar.gz.sha256` | Archive checksum |
| `vendor.sha256` | Vendor tree hash (all files under `vendor/`) |
| `vendor-manifest.json` | Toolchain, Cargo.lock hash, restore/regenerate commands |

`vendor/` itself is gitignored locally but restored on every clone:

```bash
bash scripts/ensure_vendor_offline.sh
```

Maintainers regenerate (network required once):

```bash
bash scripts/vendor_deps.sh
git add dist/vendor.tar.gz dist/vendor.tar.gz.sha256 vendor.sha256 vendor-manifest.json Cargo.lock
```

### Verify vendor

Tree hash uses `LC_ALL=C` byte-lexicographic ordering so Linux/macOS/CI runners agree:

```bash
bash scripts/verify_vendor_tree_hash.sh
bash scripts/check_prerequisites.sh
CARGO_NET_OFFLINE=true cargo metadata --offline --locked --format-version 1 >/dev/null
CARGO_NET_OFFLINE=true cargo check --offline --locked -p everarcade-cli
```

Expected: `Prerequisites: PASS` and no crates.io lookups.

## Run CI locally

```bash
bash scripts/ci/run-deterministic-world-factory.sh
```

This runs:

1. Vendor restore + prerequisites
2. `cargo fmt --check`
3. `cargo build` / `cargo test` for `everarcade-cli` (offline)
4. World Factory generate → verify → boot → run → replay → deploy → attest
5. Determinism check (two isolated project copies)
6. Release bundle build, inspect, smoke-test (100 MB gate)

## Regenerate world.evr (World Factory)

```bash
node creator-sdk/cli/everarcade.mjs world factory generate \
  --project examples/world-factory/frontier-settlement

node creator-sdk/cli/everarcade.mjs world factory verify \
  --project examples/world-factory/frontier-settlement
```

## Confirm deterministic package hash

```bash
RUN_A="$(mktemp -d)"
RUN_B="$(mktemp -d)"
cp examples/world-factory/frontier-settlement/world-blueprint.json "$RUN_A/"
cp examples/world-factory/frontier-settlement/world-contract-plan.json "$RUN_A/"
cp examples/world-factory/frontier-settlement/world-blueprint.json "$RUN_B/"
cp examples/world-factory/frontier-settlement/world-contract-plan.json "$RUN_B/"

node creator-sdk/cli/everarcade.mjs world factory generate --project "$RUN_A"
node creator-sdk/cli/everarcade.mjs world factory generate --project "$RUN_B"

diff "$RUN_A/out/world.evr/expected-package-hash.txt" "$RUN_B/out/world.evr/expected-package-hash.txt"
```

Matching hashes mean package generation is deterministic for the same blueprint inputs.

## Known non-deterministic artifacts

| Artifact | Field | Notes |
|----------|-------|-------|
| `world-release-attestation.json` | `timestamp` | Set at attest create time; excluded from signature bytes |
| `attester-ed25519-private.pem` | — | Generated once per project dir if missing |
| Attestation hash | — | Changes if timestamp or new key is generated |

Deterministic gates compare package hash, factory reports, deployment manifest, and runtime replay report — not attestation timestamps.

## Offline Cargo configuration

`.cargo/config.toml` points `crates-io` at `vendor/` with `offline = true`. Do not remove this for contributor or CI builds.

## Contributor quick gate

```bash
cargo fmt --check
cargo build -p everarcade-cli
node creator-sdk/cli/everarcade.mjs world factory generate
node creator-sdk/cli/everarcade.mjs world factory verify
node creator-sdk/cli/everarcade.mjs world factory replay
```