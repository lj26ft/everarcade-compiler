#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
CARGO_FLAGS=()
for arg in "$@"; do
  case "$arg" in
    --offline|--locked|--frozen) CARGO_FLAGS+=("$arg") ;;
    -h|--help) echo "Usage: bash scripts/run_release_gate_validation.sh [--offline] [--locked] [--frozen]"; exit 0 ;;
    *) printf '❌ Unknown argument: %s\n' "$arg" >&2; exit 2 ;;
  esac
done
REPORT="deployment/reports/release_gate_validation_run.md"
mkdir -p "$(dirname "$REPORT")"
cargo build -p everarcade-cli "${CARGO_FLAGS[@]}"
./target/debug/everarcade validate --profile full --json >/tmp/everarcade-validate-full.json
./target/debug/everarcade release-gate --json >/tmp/everarcade-release-gate.json
test -f validation_logs/release_report.md
test -f validation_logs.tar.gz
cat > "$REPORT" <<RPT
# Release Gate Validation Run

status: passed
cargo_flags: ${CARGO_FLAGS[*]:-none}
default_cargo_build_jobs: ${CARGO_BUILD_JOBS}
validate_full: passed
release_gate: passed
release_report: validation_logs/release_report.md
release_archive: validation_logs.tar.gz
RPT
echo "release gate validation complete"
