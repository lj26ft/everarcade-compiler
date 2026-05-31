#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

for manifest in deployment runtime package world; do
  test -f "deployment/evernode/${manifest}_manifest.toml"
done

for op in deploy start stop restart recover verify; do
  deployment/evernode/operations.sh "$op" >/dev/null
done

bash scripts/build_evernode_packages.sh
(cd deployment/evernode/runtime && sha256sum -c packages.sha256 >/dev/null)

CARGO_BUILD_JOBS=1 cargo test -p execution-core --test arena_vanguard_certification_tests --offline --locked

echo "evernode deployment validation complete"
