#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
[[ -d vendor ]] || { echo "Missing vendor/. Run: bash scripts/vendor_deps.sh" >&2; exit 1; }
mkdir -p wasm/reports
cmd=(cargo test --package execution-core --test wasm_runtime_tests "$@")
if "${cmd[@]}"; then s=pass; else s=fail; fi
cat > "wasm/reports/fuel-metering-validation.md" <<R
# FUEL-METERING-VALIDATION
- command: cargo test --package execution-core --test wasm_runtime_tests $*
- timestamp: ${SOURCE_DATE_EPOCH:-$(date -u +"%Y-%m-%dT%H:%M:%SZ")}
- git_commit: $(git rev-parse --short HEAD)
- status: $s
R
[[ "$s" == pass ]]
