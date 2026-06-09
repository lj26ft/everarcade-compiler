#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

REPORT_PATH="reports/deterministic_execution_certification_report.txt"
VALIDATION_REPORT="reports/deterministic_execution_validation_report.txt"
mkdir -p reports

validation_output="$(bash scripts/validate_deterministic_execution.sh)"
validation_status="FAIL"
if [[ -f "$VALIDATION_REPORT" ]] && rg -q '^Overall Result: PASS$' "$VALIDATION_REPORT"; then
  validation_status="PASS"
fi

final_status="FAIL"
if [[ "$validation_status" == "PASS" ]]; then
  final_status="PASS"
fi

cat > "$REPORT_PATH" <<REPORT
Deterministic Execution Certification Report
Validation Report: $VALIDATION_REPORT
Validation Status: $validation_status

Validation Output:
$validation_output

Deterministic Execution Proof v0.1: $final_status
REPORT

cat "$REPORT_PATH"
[[ "$final_status" == "PASS" ]]
