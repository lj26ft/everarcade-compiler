#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
mkdir -p runtime/logs deployment/reports
log="runtime/logs/runtime_activation.log"
: > "$log"
echo "runtime_activation=started jobs=${CARGO_BUILD_JOBS} args=$*" | tee -a "$log"
CARGO_BUILD_JOBS="$CARGO_BUILD_JOBS" cargo test --package execution-core --test runtime_activation_tests "$@" 2>&1 | tee -a "$log"
cat > deployment/reports/runtime_activation_report.md <<RPT
# Runtime Activation Report
- activation_validation: execution-core runtime_activation_tests
- cargo_build_jobs_default: ${CARGO_BUILD_JOBS}
- log: runtime/logs/runtime_activation.log
- renderer_authority: non-authoritative
RPT
echo "runtime_activation=ok" | tee -a "$log"
