#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
mkdir -p dist
required=(manifest/world-manifest.toml metadata/world-metadata.json world-contract/world-contract.toml genesis/frontier-state.json genesis/continuity-state.json continuity/policies.toml projection/projection.toml registry/frontier-validation-registry-entry.json proofs/proof-mapping.toml)
for file in "${required[@]}"; do test -f "$file" || { echo "missing required file: $file" >&2; exit 1; }; done
printf '%s\n' "${required[@]}" | sort > dist/package-manifest.txt
tar -cf dist/world.evr "${required[@]}"
echo "built dist/world.evr"
