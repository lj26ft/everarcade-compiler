#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

REPORT_DIR="$ROOT/reports"
VALIDATION_REPORT="$REPORT_DIR/hotpocket_integration_validation_report.txt"
CERT_REPORT="$REPORT_DIR/hotpocket_integration_certification_report.txt"
mkdir -p "$REPORT_DIR"

validation_status="FAIL"
adapter_status="FAIL"
input_status="FAIL"
output_status="FAIL"
checkpoint_status="FAIL"
replay_status="FAIL"
settlement_status="FAIL"
status_status="FAIL"
overall="FAIL"

if bash scripts/validate_hotpocket_integration.sh >/dev/null; then
  validation_status="PASS"
fi

read_report_value() {
  local key="$1"
  awk -F': ' -v key="$key" '$1 == key { print $2; found = 1; exit } END { if (!found) print "FAIL" }' "$VALIDATION_REPORT"
}

if [[ -f "$VALIDATION_REPORT" ]]; then
  adapter_status="$(read_report_value Adapter)"
  input_status="$(read_report_value 'Input Export')"
  output_status="$(read_report_value 'Output Export')"
  checkpoint_status="$(read_report_value 'Checkpoint Export')"
  replay_status="$(read_report_value 'Replay Export')"
  settlement_status="$(read_report_value 'Settlement Export')"
  status_status="$(read_report_value 'Status Export')"
fi

if [[ "$validation_status" == PASS \
  && "$adapter_status" == PASS \
  && "$input_status" == PASS \
  && "$output_status" == PASS \
  && "$checkpoint_status" == PASS \
  && "$replay_status" == PASS \
  && "$settlement_status" == PASS \
  && "$status_status" == PASS ]]; then
  overall="PASS"
fi

cat > "$CERT_REPORT" <<REPORT_BODY
HotPocket Integration Certification Report
Adapter: $adapter_status
Inputs: $input_status
Outputs: $output_status
Checkpoint Export: $checkpoint_status
Replay Export: $replay_status
Settlement Export: $settlement_status
Status Export: $status_status
Validation: $validation_status
HotPocket Integration: $overall
REPORT_BODY

cat "$CERT_REPORT"
[[ "$overall" == PASS ]]
