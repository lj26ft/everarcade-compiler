#!/usr/bin/env bash
set -euo pipefail
export CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS:-1}
mkdir -p deployment/reports
cargo test -p execution-core --test arena_vanguard_certification_tests "$@"
cat > deployment/reports/arena_vanguard_rustrig_usage_validation_report.md <<'RPT'
# Arena Vanguard Rustrig Usage Validation

Status: Executable

Arena Vanguard certification includes real Rustrig runtime execution flow.
RPT
