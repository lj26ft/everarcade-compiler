#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
# shellcheck source=lib/common.sh
source "$ROOT/scripts/lib/common.sh"
cd "$ROOT"
mkdir -p .cargo deployment/reports dist

if [[ "${CARGO_NET_OFFLINE:-}" == "true" ]]; then
  echo "vendor_deps.sh requires network access to refresh vendor/; unset CARGO_NET_OFFLINE." >&2
  exit 1
fi

if [[ ! -f Cargo.lock ]]; then
  cargo generate-lockfile
fi

refresh_vendor() {
  rm -rf vendor
  cat > .cargo/config.toml <<'CFG'
[net]
offline = false
CFG
  if ! cargo vendor --locked vendor >/tmp/everarcade-vendor-config.toml 2>/tmp/everarcade-vendor-locked.err; then
    echo "cargo vendor --locked failed; regenerating lockfile and retrying" >&2
    cat /tmp/everarcade-vendor-locked.err >&2 || true
    cargo generate-lockfile
    cargo vendor vendor >/tmp/everarcade-vendor-config.toml
  fi
}

if ! vendor_offline_ok "$ROOT"; then
  refresh_vendor
fi

cat > .cargo/config.toml <<'CFG'
# EverArcade locked offline vendor policy (Phase 0).
# vendor/ is restored from dist/vendor.tar.gz on clean clones via scripts/ensure_vendor_offline.sh
# Regenerate with: bash scripts/vendor_deps.sh (maintainers, networked environment only)

[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"

[net]
offline = true
CFG

if ! vendor_metadata_offline_ok "$ROOT"; then
  echo "Vendor validation failed after refresh; retrying once" >&2
  refresh_vendor
  cat > .cargo/config.toml <<'CFG'
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"

[net]
offline = true
CFG
  vendor_metadata_offline_ok "$ROOT" || {
    echo "Vendor validation failed; offline metadata still unavailable" >&2
    exit 1
  }
fi

tar -czf dist/vendor.tar.gz vendor
(
  cd dist
  sha256sum vendor.tar.gz > vendor.tar.gz.sha256
)

vendor_hash="$(vendor_tree_sha256 "$ROOT")"
vendor_crate_count="$(find vendor -maxdepth 1 -mindepth 1 -type d | wc -l | tr -d ' ')"
cargo_lock_hash="$(sha256sum Cargo.lock | awk '{print $1}')"
artifact_hash="$(awk '{print $1}' dist/vendor.tar.gz.sha256)"
cargo_version="$(cargo --version 2>/dev/null || echo 'unknown')"
generated_at="$(date -u +%Y-%m-%dT%H:%M:%SZ)"

printf '%s\n' "$vendor_hash" > vendor.sha256

cat > vendor-manifest.json <<JSON
{
  "schema": "everarcade-vendor-manifest-v1",
  "generated_at": "${generated_at}",
  "toolchain": {
    "cargo": "${cargo_version}"
  },
  "cargo_lock_sha256": "${cargo_lock_hash}",
  "vendor_tree_sha256": "${vendor_hash}",
  "vendor_crate_count": ${vendor_crate_count},
  "vendor_directory": "vendor",
  "artifact_path": "dist/vendor.tar.gz",
  "artifact_sha256": "${artifact_hash}",
  "restore_command": "bash scripts/ensure_vendor_offline.sh",
  "regenerate_command": "bash scripts/vendor_deps.sh",
  "verify_commands": [
    "CARGO_NET_OFFLINE=true cargo metadata --offline --locked --format-version 1",
    "CARGO_NET_OFFLINE=true cargo check --offline --locked -p everarcade-cli"
  ]
}
JSON

cat > deployment/reports/vendor_validation_report.md <<RPT
# Vendor Validation Report
- cargo_metadata_offline: ok
- cargo_lock: present
- cargo_config: .cargo/config.toml
- vendor_dir: vendor
- vendor_tree_hash: ${vendor_hash}
- vendor_artifact: dist/vendor.tar.gz
- vendor_artifact_sha256: $(awk '{print $1}' dist/vendor.tar.gz.sha256)
RPT

if ! offline_cargo_check_workspace "$ROOT" /tmp/everarcade-vendor-deps-check.log; then
  echo "workspace cargo check --offline failed after vendor refresh" >&2
  tail -20 /tmp/everarcade-vendor-deps-check.log >&2 || true
  exit 1
fi

echo "dependency_vendor=ok"
echo "vendor_artifact=dist/vendor.tar.gz"
echo "vendor_manifest=vendor-manifest.json"
echo "vendor_sha256=vendor.sha256"
echo "offline_workspace_check=ok"