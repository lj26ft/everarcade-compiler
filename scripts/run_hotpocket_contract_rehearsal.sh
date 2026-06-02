#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
CARGO_FLAGS=()
for arg in "$@"; do
  case "$arg" in
    --offline|--locked|--frozen) CARGO_FLAGS+=("$arg") ;;
    -h|--help)
      cat <<'USAGE'
Usage: bash scripts/run_hotpocket_contract_rehearsal.sh [--offline] [--locked] [--frozen]

Generates deterministic EverNode packages, stages the HotPocket contract layout,
verifies package hashes, runs the local start.sh rehearsal, and writes a report.
USAGE
      exit 0
      ;;
    *) printf '❌ Unknown argument: %s\n' "$arg" >&2; exit 2 ;;
  esac
done

REPORT="deployment/reports/hotpocket_contract_rehearsal.md"
CONTRACT_DIR="dist/everarcade-hotpocket-contract"
STATUS_FILE="$CONTRACT_DIR/state/operator/status.txt"

printf '🎭 HotPocket Contract Rehearsal\n'
printf '📦 Generating and staging packages...\n'
bash scripts/generate_evernode_packages.sh --stage-contract
printf '✅ Packages generated and staged\n'

printf '🔐 Verifying package hashes...\n'
(cd "$CONTRACT_DIR/packages" && sha256sum -c packages.sha256 >/dev/null)
printf '✅ Package hashes verified\n'

printf '🗂️  Verifying state layout...\n'
test -d "$CONTRACT_DIR/state"
test -d "$CONTRACT_DIR/state/operator"
printf '✅ State layout exists\n'

printf '▶️  Running staged start.sh locally...\n'
bash "$CONTRACT_DIR/start.sh" >/tmp/everarcade-hotpocket-contract-start.log
printf '✅ start.sh completed\n'

test -f "$STATUS_FILE"
grep -q '^ready$' "$STATUS_FILE"
printf '✅ Operator status verified\n'

mkdir -p "$(dirname "$REPORT")"
cat > "$REPORT" <<RPT
# HotPocket Contract Rehearsal

status: passed
cargo_flags: ${CARGO_FLAGS[*]:-none}
default_cargo_build_jobs: ${CARGO_BUILD_JOBS}
package_generation: passed
contract_staging: passed
package_hash_verification: passed
state_layout: passed
start_script: passed
operator_status: $STATUS_FILE
RPT

printf '📝 Rehearsal report written: %s\n' "$REPORT"
printf '🎉 HotPocket contract rehearsal complete\n'
