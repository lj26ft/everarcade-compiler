#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
CARGO_FLAGS=()
for arg in "$@"; do
  case "$arg" in
    --offline|--locked|--frozen) CARGO_FLAGS+=("$arg") ;;
    -h|--help) echo "Usage: bash scripts/run_frontend_readiness_validation.sh [--offline] [--locked] [--frozen]"; exit 0 ;;
    *) printf '❌ Unknown argument: %s\n' "$arg" >&2; exit 2 ;;
  esac
done
REPORT="deployment/reports/frontend_readiness_validation_run.md"
mkdir -p "$(dirname "$REPORT")"
cargo build -p everarcade-cli "${CARGO_FLAGS[@]}"
for command in doctor status artifacts-check; do
  ./target/debug/everarcade "$command" --json > "/tmp/everarcade-${command}.json"
  python3 -m json.tool "/tmp/everarcade-${command}.json" >/dev/null
done
./target/debug/everarcade validate --profile quick --json >/tmp/everarcade-validate-json.json
python3 -m json.tool /tmp/everarcade-validate-json.json >/dev/null
cat > "$REPORT" <<RPT
# Frontend Readiness Validation Run

status: passed
cargo_flags: ${CARGO_FLAGS[*]:-none}
default_cargo_build_jobs: ${CARGO_BUILD_JOBS}
doctor_json: passed
status_json: passed
artifacts_json: passed
validate_json: passed
RPT
echo "frontend readiness validation complete"
