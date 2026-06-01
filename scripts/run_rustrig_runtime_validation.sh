#!/usr/bin/env bash
set -euo pipefail
export CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS:-1}
mkdir -p deployment/reports
cargo test -p execution-core --test rustrig_runtime_tests "$@"
cat > deployment/reports/rustrig_runtime_validation_report.md <<'RPT'
# Rustrig Runtime Validation

Status: Executable

Validated kernel execution, registry lookup, authoritative record application, replay equivalence, checkpoint recovery, and intent-only handling.
RPT
