#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VALIDATION_REPORT="$ROOT_DIR/reports/local_game_launch_validation_report.txt"
CERT_REPORT="$ROOT_DIR/reports/local_game_launch_certification_report.txt"

if [[ ! -f "$VALIDATION_REPORT" ]]; then
  echo "Validation report missing: $VALIDATION_REPORT"
  exit 1
fi

require_line() {
  local label="$1"
  local pattern="$2"
  if rg -q "$pattern" "$VALIDATION_REPORT"; then
    echo "$label: PASS"
    echo "$label: PASS" >> "$CERT_REPORT.tmp"
  else
    echo "$label: FAIL"
    echo "$label: FAIL" >> "$CERT_REPORT.tmp"
    exit 1
  fi
}

: > "$CERT_REPORT.tmp"
{
  echo "Local Game Launch Certification Report"
  echo "Classification: Runtime Boot Proven"
  echo
} >> "$CERT_REPORT.tmp"

require_line "Creator Create" '^Creator Create: PASS'
require_line "Creator Build" '^Creator Build: PASS'
require_line "Creator Test" '^Creator Test: PASS'
require_line "Runtime Package" '^Runtime Package: PASS'
require_line "Runtime Start" '^Runtime Start: PASS'
require_line "Session Evidence" '^Session Evidence: PASS'
require_line "Projection Evidence" '^Projection Evidence: PASS'

echo "Local Game Launch: RUNTIME BOOT PROVEN" | tee -a "$CERT_REPORT.tmp"
mv "$CERT_REPORT.tmp" "$CERT_REPORT"
