#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
CARGO_FLAGS=()
for arg in "$@"; do
  case "$arg" in
    --offline|--locked|--frozen) CARGO_FLAGS+=("$arg") ;;
    -h|--help) echo "Usage: bash scripts/run_product_command_validation.sh [--offline] [--locked] [--frozen]"; exit 0 ;;
    *) printf '❌ Unknown argument: %s\n' "$arg" >&2; exit 2 ;;
  esac
done
REPORT="deployment/reports/product_command_validation_run.md"
mkdir -p "$(dirname "$REPORT")"
{
  echo "# Product Command Validation Run"
  echo
  echo "cargo_flags: ${CARGO_FLAGS[*]:-none}"
  echo "default_cargo_build_jobs: ${CARGO_BUILD_JOBS}"
} > "$REPORT"
cargo build -p everarcade-cli "${CARGO_FLAGS[@]}"
./target/debug/everarcade doctor --json >/tmp/everarcade-doctor.json
./target/debug/everarcade status --json >/tmp/everarcade-status.json
./target/debug/everarcade validate --profile quick --json >/tmp/everarcade-validate-quick.json
./target/debug/everarcade artifacts-check --json >/tmp/everarcade-artifacts.json
CARGO_BUILD_JOBS=1 cargo test -p execution-core --test product_facade_tests "${CARGO_FLAGS[@]}"
cat >> "$REPORT" <<'RPT'
status: passed
doctor_json: passed
status_json: passed
validate_quick_json: passed
artifact_policy: passed
product_facade_tests: passed
RPT
echo "product command validation complete"
