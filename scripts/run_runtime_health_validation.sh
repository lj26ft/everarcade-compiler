#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
mkdir -p runtime/logs deployment/reports
log="runtime/logs/runtime_health_validation.log"
: > "$log"
echo "runtime_health_validation=started jobs=${CARGO_BUILD_JOBS} args=$*" | tee -a "$log"
CARGO_BUILD_JOBS="$CARGO_BUILD_JOBS" cargo test --package execution-core --test runtime_node_tests "$@" 2>&1 | tee -a "$log"
cat > deployment/reports/runtime_health_report.md <<RPT
# Runtime Health Report
- health_validation: execution-core runtime_node_tests
- health_gate: daemon readiness requires running, healthy, non-authoritative state
- cargo_build_jobs_default: ${CARGO_BUILD_JOBS}
- log: runtime/logs/runtime_health_validation.log
RPT
echo "runtime_health_validation=ok" | tee -a "$log"
