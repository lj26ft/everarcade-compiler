#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
test -f dist/world.evr
test -f dist/package-manifest.txt
tar -tf dist/world.evr >/tmp/frontier-validation-package.txt
grep -q 'world-contract/world-contract.toml' /tmp/frontier-validation-package.txt
grep -q 'genesis/frontier-state.json' /tmp/frontier-validation-package.txt
grep -q 'proofs/proof-mapping.toml' /tmp/frontier-validation-package.txt
grep -q 'frontier-validation' metadata/world-metadata.json
grep -q 'continuity = "required"' world-contract/world-contract.toml
echo "frontier validation verification passed"
