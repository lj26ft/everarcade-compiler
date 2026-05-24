#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT/deployment/reports"
LOG_DIR="$ROOT/deployment/reports/logs"
REPORT="$REPORT_DIR/deployment_validation_report.md"
mkdir -p "$REPORT_DIR" "$LOG_DIR"

START="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
LOG_FILE="$LOG_DIR/deployment_validation.log"
: > "$LOG_FILE"

run_step() {
  local name="$1"; shift
  echo "[$(date -u +"%Y-%m-%dT%H:%M:%SZ")] STEP: $name" | tee -a "$LOG_FILE"
  if "$@" >>"$LOG_FILE" 2>&1; then
    echo "- ✅ $name" >> "$REPORT"
  else
    echo "- ❌ $name" >> "$REPORT"
    return 1
  fi
}

cat > "$REPORT" <<REPORT
# Deployment Validation Report

- Generated: $START
- Mode: offline vendored Cargo

## Findings
REPORT

run_step "Vendor preflight" "$ROOT/scripts/preflight_vendor.sh"
run_step "Offline workspace check" cargo check --workspace --locked --frozen --offline
run_step "Release validation wrapper" "$ROOT/scripts/run_release_validation.sh"
run_step "Distribution validation wrapper" "$ROOT/scripts/run_distribution_validation.sh"
run_step "Runtime bootstrap validation wrapper" "$ROOT/scripts/run_runtime_bootstrap.sh"

cat >> "$REPORT" <<REPORT

## Artifacts
- Log: deployment/reports/logs/deployment_validation.log
REPORT

echo "deployment_validation=ok"
