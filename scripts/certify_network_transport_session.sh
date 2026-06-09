#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

REPORT_PATH="reports/network_transport_certification_report.txt"
VALIDATION_REPORT="reports/network_transport_validation_report.txt"
mkdir -p reports

validation_output="$(bash scripts/validate_network_transport_session.sh)"
validation_status="FAIL"
if [[ -f "$VALIDATION_REPORT" ]] && rg -q '^Overall Result: PASS$' "$VALIDATION_REPORT"; then
  validation_status="PASS"
fi

final_status="FAIL"
if [[ "$validation_status" == "PASS" ]]; then
  final_status="PASS"
fi

cat > "$REPORT_PATH" <<REPORT
Network Transport & Session Synchronization Certification Report
Validation Report: $VALIDATION_REPORT
Validation Status: $validation_status

Validation Output:
$validation_output

Network Transport & Session Synchronization Proof v0.1: $final_status
REPORT

cat "$REPORT_PATH"
[[ "$final_status" == "PASS" ]]
