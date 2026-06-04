#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/xrpl_live_settlement_certification_report.txt"
VALIDATION_REPORT="$REPORT_DIR/xrpl_live_settlement_validation_report.txt"

cd "$ROOT_DIR"
# shellcheck source=../xrpl/live_settlement_model.sh
source "$ROOT_DIR/xrpl/live_settlement_model.sh"
mkdir -p "$REPORT_DIR"
initialize_roots

bash "$ROOT_DIR/scripts/validate_xrpl_live_settlement.sh" >/dev/null

report_value() {
  local key="$1"
  awk -F': ' -v key="$key" '$1 == key { print $2; found = 1; exit } END { if (!found) print "UNKNOWN" }' "$VALIDATION_REPORT"
}

intent_status="$(report_value "Intent")"
transaction_status="$(report_value "Transaction")"
verification_status="$(report_value "Verification")"
receipt_status="$(report_value "Receipt")"
import_status="$(report_value "Import")"
anchor_status="$(report_value "Anchor")"
replay_status="$(report_value "Replay")"
validation_status="$(report_value "XRPL Live Settlement Validation")"
settlement_root_value="$(report_value "Settlement Root")"
replay_root_value="$(report_value "Replay Root")"
overall_status="FAIL"

if [[ "$validation_status" == "PASS" \
  && "$intent_status" == "PASS" \
  && "$transaction_status" == "PASS" \
  && "$verification_status" == "PASS" \
  && "$receipt_status" == "PASS" \
  && "$import_status" == "PASS" \
  && "$anchor_status" == "PASS" \
  && "$replay_status" == "PASS" \
  && "$settlement_root_value" == "$replay_root_value" ]]; then
  overall_status="PASS"
fi

cat > "$REPORT_PATH" <<REPORT
XRPL Live Settlement Certification Report
Version: $XRPL_LIVE_VERSION

Intent: $intent_status
Transaction: $transaction_status
Verification: $verification_status
Receipt: $receipt_status
Import: $import_status
Anchor: $anchor_status
Replay: $replay_status

Intent Root: $(intent_root)
Transaction Root: $(transaction_root)
Verification Root: $(verification_root)
Receipt Root: $(receipt_root)
Settlement Evidence Root: $(settlement_evidence_root)
Anchor Root: $(anchor_root)
Settlement Root: $settlement_root_value
Replay Root: $replay_root_value

XRPL Live Settlement Layer: $overall_status
REPORT

cat "$REPORT_PATH"
[[ "$overall_status" == "PASS" ]]
