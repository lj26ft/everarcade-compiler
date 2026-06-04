#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/xaman_signing_certification_report.txt"
VALIDATION_REPORT="$REPORT_DIR/xaman_signing_validation_report.txt"

cd "$ROOT_DIR"
# shellcheck source=../xrpl/live_settlement_model.sh
source "$ROOT_DIR/xrpl/live_settlement_model.sh"
# shellcheck source=../xaman/signing_model.sh
source "$ROOT_DIR/xaman/signing_model.sh"
mkdir -p "$REPORT_DIR"
xaman_initialize_roots

bash "$ROOT_DIR/scripts/validate_xaman_signing.sh" >/dev/null

report_value() {
  local key="$1"
  awk -F': ' -v key="$key" '$1 == key { print $2; found = 1; exit } END { if (!found) print "UNKNOWN" }' "$VALIDATION_REPORT"
}

payload_status="$(report_value "Payload")"
deep_link_status="$(report_value "Deep Link")"
qr_status="$(report_value "QR Metadata")"
tracking_status="$(report_value "Tracking")"
receipt_status="$(report_value "Receipt Import")"
continuity_status="$(report_value "Continuity Update")"
replay_status="$(report_value "Replay")"
validation_status="$(report_value "Xaman Signing Validation")"
continuity_root_value="$(report_value "Settlement Continuity Root")"
replay_root_value="$(report_value "Replay Root")"
overall_status="FAIL"

if [[ "$validation_status" == "PASS" \
  && "$payload_status" == "PASS" \
  && "$deep_link_status" == "PASS" \
  && "$qr_status" == "PASS" \
  && "$tracking_status" == "PASS" \
  && "$receipt_status" == "PASS" \
  && "$continuity_status" == "PASS" \
  && "$replay_status" == "PASS" \
  && "$continuity_root_value" == "$replay_root_value" ]]; then
  overall_status="PASS"
fi

cat > "$REPORT_PATH" <<REPORT
Xaman Signing Certification Report
Version: $XAMAN_SIGNING_VERSION

Payload: $payload_status
Deep Link: $deep_link_status
QR Metadata: $qr_status
Tracking: $tracking_status
Receipt Import: $receipt_status
Continuity Update: $continuity_status
Replay: $replay_status

Payload Root: $(xaman_payload_root)
Deep Link Root: $(xaman_deep_link_root)
QR Root: $(xaman_qr_root)
Tracking Root: $(xaman_tracking_root)
Signed Receipt Root: $(xaman_signed_receipt_root)
Settlement Continuity Root: $continuity_root_value
Replay Root: $replay_root_value

Xaman Signing Layer: $overall_status
REPORT

cat "$REPORT_PATH"
[[ "$overall_status" == "PASS" ]]
