#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

REPORT_PATH="reports/multiplayer_local_session_certification_report.txt"
VALIDATION_REPORT="reports/multiplayer_local_session_validation_report.txt"
mkdir -p reports

validation_output="$(bash scripts/validate_multiplayer_local_session.sh)"
validation_status="FAIL"
if [[ -f "$VALIDATION_REPORT" ]] && rg -q '^Overall Result: PASS$' "$VALIDATION_REPORT"; then
  validation_status="PASS"
fi

final_status="FAIL"
if [[ "$validation_status" == "PASS" ]]; then
  final_status="PASS"
fi

cat > "$REPORT_PATH" <<REPORT
Multiplayer Local Session Certification Report
Validation Report: $VALIDATION_REPORT
Validation Status: $validation_status

Validation Output:
$validation_output

Multiplayer Local Session Proof v0.1: $final_status
REPORT

cat "$REPORT_PATH"
[[ "$final_status" == "PASS" ]]
