#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

for path in \
  arena-vanguard/manifest.toml \
  arena-vanguard-world/world_manifest.toml \
  arena-vanguard-assets/asset_manifest.toml \
  arena-vanguard-rustrigs/marketplace_manifest.toml \
  arena-vanguard-package/runtime_package.toml \
  arena-vanguard-package/world_package.toml \
  arena-vanguard-package/deployment_package.toml \
  arena-vanguard-package/asset_package.toml; do
  test -f "$path"
done

CARGO_BUILD_JOBS=1 cargo test -p execution-core --test arena_vanguard_certification_tests --offline --locked

echo "arena vanguard validation complete"
