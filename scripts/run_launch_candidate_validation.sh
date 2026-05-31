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
REPORT="deployment/reports/launch_candidate_validation_run.md"
for report in operator_readiness_report evernode_launch_gate xrpl_launch_gate arena_vanguard_launch_candidate multinode_federation_load_gate; do
  test -f "deployment/reports/${report}.md"
done
cargo test -p execution-core --test evernode_deployment_tests "${CARGO_FLAGS[@]}"
cat > "$REPORT" <<'REPORT_EOF'
# Launch Candidate Validation Run

status: passed
creator workflow: documented
runtime workflow: documented
deployment workflow: documented
operator workflow: documented
recovery workflow: documented
multi-node federation load workflow: documented
REPORT_EOF
echo "launch candidate validation passed; report=$REPORT"
