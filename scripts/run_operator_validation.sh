#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
CARGO_FLAGS=()
for arg in "$@"; do
  case "$arg" in
    --offline|--locked|--frozen) CARGO_FLAGS+=("$arg") ;;
    *) echo "unknown argument: $arg" >&2; exit 2 ;;
  esac
done
REPORT="deployment/reports/operator_validation_run.md"
for guide in install bootstrap deploy recover upgrade rollback; do
  test -f "docs/operators/${guide}.md"
done
cargo test -p execution-core --test evernode_deployment_tests test_operator_bootstrap "${CARGO_FLAGS[@]}"
cat > "$REPORT" <<'REPORT_EOF'
# Operator Validation Run

status: passed
install workflow: present
bootstrap workflow: present
deployment workflow: present
recovery workflow: present
upgrade workflow: present
rollback workflow: present
REPORT_EOF
echo "operator validation passed; report=$REPORT"
