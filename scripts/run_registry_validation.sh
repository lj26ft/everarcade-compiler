#!/usr/bin/env bash
set -euo pipefail
required=(manifest.toml metadata.json package.bin hash.sha256 signature.bin)
for package_dir in registry/rustrigs/*; do
  [ -d "$package_dir" ] || continue
  for artifact in "${required[@]}"; do
    test -s "$package_dir/$artifact"
  done
done
for category in Combat Inventory Quests Dialogue Economy Crafting World AI Factions Civilizations UI Utilities XRPL Deployment; do
  grep -q "name = \"$category\"" registry/community/categories.toml
done
CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS:-1} cargo test -p contract-api --offline --locked
