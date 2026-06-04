#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/xrpl_live_settlement_validation_report.txt"

cd "$ROOT_DIR"
# shellcheck source=../xrpl/live_settlement_model.sh
source "$ROOT_DIR/xrpl/live_settlement_model.sh"
mkdir -p "$REPORT_DIR"
initialize_roots

status_for() {
  local name="$1"
  local fn="$2"
  if "$fn"; then
    printf '%s: PASS\n' "$name"
  else
    printf '%s: FAIL\n' "$name"
    return 1
  fi
}

intent_status="FAIL"
transaction_status="FAIL"
verification_status="FAIL"
receipt_status="FAIL"
import_status="FAIL"
anchor_status="FAIL"
replay_status="FAIL"
overall_status="FAIL"

write_live_settlement_artifacts

validate_intent && intent_status="PASS"
validate_transaction && transaction_status="PASS"
validate_verification && verification_status="PASS"
validate_receipt && receipt_status="PASS"
validate_import && import_status="PASS"
validate_anchor && anchor_status="PASS"
validate_replay && replay_status="PASS"

if [[ "$intent_status" == "PASS" \
  && "$transaction_status" == "PASS" \
  && "$verification_status" == "PASS" \
  && "$receipt_status" == "PASS" \
  && "$import_status" == "PASS" \
  && "$anchor_status" == "PASS" \
  && "$replay_status" == "PASS" ]]; then
  overall_status="PASS"
fi

cat > "$REPORT_PATH" <<REPORT
XRPL Live Settlement Validation Report
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
Settlement Root: $(settlement_root)
Replay Root: $(replay_root)

XRPL Live Settlement Validation: $overall_status
REPORT

cat "$REPORT_PATH"
[[ "$overall_status" == "PASS" ]]
