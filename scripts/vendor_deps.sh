#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
mkdir -p .cargo deployment/reports

if [[ ! -f Cargo.lock ]]; then
  cargo generate-lockfile
fi

if [[ ! -d vendor ]] || [[ -z "$(find vendor -mindepth 1 -maxdepth 1 -print -quit 2>/dev/null)" ]]; then
  rm -rf vendor
  tmp_cfg=".cargo/config.toml"
  cp "$tmp_cfg" "$tmp_cfg.bak" 2>/dev/null || true
  cat > "$tmp_cfg" <<'CFG'
[net]
offline = false
CFG
  cargo vendor --locked vendor >/tmp/everarcade-vendor-config.toml
fi

cat > .cargo/config.toml <<'CFG'
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"

[net]
offline = true
CFG

if ! cargo metadata --offline --locked >/tmp/runtime_vendor_metadata.json 2>/tmp/runtime_vendor_err.log; then
  echo "Vendor validation failed. Ensure vendor/ exists and rerun: bash scripts/vendor_deps.sh" >&2
  cat /tmp/runtime_vendor_err.log >&2
  exit 1
fi
vendor_hash="$(tar -cf - vendor 2>/dev/null | sha256sum | awk '{print $1}')"
cat > deployment/reports/vendor_validation_report.md <<RPT
# Vendor Validation Report
- cargo_metadata: ok
- cargo_lock: present
- cargo_config: .cargo/config.toml
- vendor_dir: vendor
- vendor_hash: ${vendor_hash}
RPT

echo "dependency_vendor=ok"
