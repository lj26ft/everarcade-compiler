#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/xaman_signing_validation_report.txt"

cd "$ROOT_DIR"
# shellcheck source=../xrpl/live_settlement_model.sh
source "$ROOT_DIR/xrpl/live_settlement_model.sh"
# shellcheck source=../xaman/signing_model.sh
source "$ROOT_DIR/xaman/signing_model.sh"
mkdir -p "$REPORT_DIR"
xaman_initialize_roots

payload_status="FAIL"
deep_link_status="FAIL"
qr_status="FAIL"
tracking_status="FAIL"
receipt_status="FAIL"
continuity_status="FAIL"
replay_status="FAIL"
overall_status="FAIL"

write_live_settlement_artifacts
write_xaman_signing_artifacts

xaman_validate_payload && payload_status="PASS"
xaman_validate_deep_link && deep_link_status="PASS"
xaman_validate_qr_metadata && qr_status="PASS"
xaman_validate_tracking && tracking_status="PASS"
xaman_validate_receipt_import && receipt_status="PASS"
xaman_validate_continuity_update && continuity_status="PASS"
xaman_validate_replay && replay_status="PASS"

if [[ "$payload_status" == "PASS" \
  && "$deep_link_status" == "PASS" \
  && "$qr_status" == "PASS" \
  && "$tracking_status" == "PASS" \
  && "$receipt_status" == "PASS" \
  && "$continuity_status" == "PASS" \
  && "$replay_status" == "PASS" ]]; then
  overall_status="PASS"
fi

cat > "$REPORT_PATH" <<REPORT
Xaman Signing Validation Report
Version: $XAMAN_SIGNING_VERSION

Payload: $payload_status
Deep Link: $deep_link_status
QR Metadata: $qr_status
Tracking: $tracking_status
Receipt Import: $receipt_status
Continuity Update: $continuity_status
Replay: $replay_status

Payload ID: $(xaman_payload_id)
Request ID: $(xaman_request_id)
Intent Root: $(intent_root)
Transaction Root: $(transaction_root)
Transaction Hash: $(transaction_hash)
Settlement Root: $(settlement_root)
Payload Root: $(xaman_payload_root)
Deep Link Root: $(xaman_deep_link_root)
Request Hash: $(xaman_request_hash)
QR Root: $(xaman_qr_root)
Tracking Root: $(xaman_tracking_root)
Signed Receipt Root: $(xaman_signed_receipt_root)
Settlement Continuity Root: $(xaman_settlement_continuity_root)
Replay Root: $(xaman_replay_root)

Xaman Signing Validation: $overall_status
REPORT

cat "$REPORT_PATH"
[[ "$overall_status" == "PASS" ]]
