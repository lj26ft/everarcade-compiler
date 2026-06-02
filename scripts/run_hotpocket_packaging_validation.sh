#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
CARGO_FLAGS=()
for arg in "$@"; do
  case "$arg" in
    --offline|--locked|--frozen) CARGO_FLAGS+=("$arg") ;;
    -h|--help) echo "Usage: bash scripts/run_hotpocket_packaging_validation.sh [--offline] [--locked] [--frozen]"; exit 0 ;;
    *) printf '❌ Unknown argument: %s\n' "$arg" >&2; exit 2 ;;
  esac
done
REPORT="deployment/reports/hotpocket_packaging_validation_run.md"
mkdir -p "$(dirname "$REPORT")"
cargo build -p everarcade-cli "${CARGO_FLAGS[@]}"
./target/debug/everarcade package --json >/tmp/everarcade-package.json
./target/debug/everarcade stage-contract --json >/tmp/everarcade-stage-contract.json
./target/debug/everarcade rehearse --json >/tmp/everarcade-rehearse.json
cat > "$REPORT" <<RPT
# HotPocket Packaging Validation Run

status: passed
cargo_flags: ${CARGO_FLAGS[*]:-none}
default_cargo_build_jobs: ${CARGO_BUILD_JOBS}
package: passed
stage_contract: passed
rehearsal: passed
RPT
echo "hotpocket packaging validation complete"
