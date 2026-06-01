#!/usr/bin/env bash
set -euo pipefail
export CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS:-1}
mkdir -p deployment/reports
cargo test -p rustrigs "$@"
cat > deployment/reports/executable_rustrigs_validation_report.md <<'RPT'
# Executable Rustrigs Validation

Status: Executable

Primitive Rustrig crate builds/tests with deterministic record-emitting modules.
RPT
