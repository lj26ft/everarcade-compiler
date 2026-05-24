#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
mkdir -p "$ROOT/deployment/reports" "$ROOT/deployment/reports/logs"
REPORT="$ROOT/deployment/reports/distribution_validation_report.md"
LOG="$ROOT/deployment/reports/logs/distribution_validation.log"
START="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
if "$ROOT/scripts/preflight_vendor.sh" >"$LOG" 2>&1 && cargo build --locked --frozen --offline -p everarcade-host >>"$LOG" 2>&1; then STATUS="✅ pass"; else STATUS="❌ fail"; fi
cat > "$REPORT" <<REPORT
# Distribution Validation Report

- Generated: $START
- Status: $STATUS
- Commands:
  - scripts/preflight_vendor.sh
  - cargo build --locked --frozen --offline -p everarcade-host
- Log: deployment/reports/logs/distribution_validation.log
REPORT
[[ "$STATUS" == "✅ pass" ]]
