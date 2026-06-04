#!/usr/bin/env bash
# Deterministic Xaman signing-layer model for EverArcade Runtime v0.1.
# This file models signable payloads, authorization tracking, signed receipts,
# and replayable continuity evidence only. It never signs or stores keys.

XAMAN_SIGNING_VERSION="xaman-signing-layer-v0.1"
XAMAN_WALLET="${SENDER:-rEverArcadeSender111111111111111111111}"
XAMAN_REFERENCE_DATA="everarcade:xaman:settlement:001"
XAMAN_PAYLOAD_TIMESTAMP="2026-06-04T00:00:10Z"
XAMAN_REQUEST_TIMESTAMP="2026-06-04T00:00:11Z"
XAMAN_TRACKING_TIMESTAMP="2026-06-04T00:00:12Z"
XAMAN_RECEIPT_TIMESTAMP="2026-06-04T00:00:20Z"
XAMAN_RECEIPT_OUTCOME="approved"
XAMAN_TRACKING_STATES=("Created" "Pending" "Approved" "Rejected" "Expired")
XAMAN_CURRENT_STATUS="Approved"

xaman_sha256_text() {
  sha256sum | awk '{print $1}'
}

xaman_short_hash() {
  cut -c1-16
}

xaman_payload_id_seed() {
  cat <<EOF_SEED
version=$XAMAN_SIGNING_VERSION
wallet=$XAMAN_WALLET
intent.root=$(intent_root)
transaction.root=$(transaction_root)
settlement.root=$(settlement_root)
reference=$XAMAN_REFERENCE_DATA
timestamp=$XAMAN_PAYLOAD_TIMESTAMP
EOF_SEED
}

xaman_payload_id() {
  [[ -n "${XAMAN_PAYLOAD_ID_CACHE:-}" ]] || xaman_initialize_roots
  printf '%s\n' "$XAMAN_PAYLOAD_ID_CACHE"
}

xaman_request_id() {
  [[ -n "${XAMAN_REQUEST_ID_CACHE:-}" ]] || xaman_initialize_roots
  printf '%s\n' "$XAMAN_REQUEST_ID_CACHE"
}

canonical_xaman_payload() {
  cat <<EOF_PAYLOAD
version=$XAMAN_SIGNING_VERSION
PayloadID=$(xaman_payload_id)
Wallet=$XAMAN_WALLET
TransactionRoot=$(transaction_root)
SettlementRoot=$(settlement_root)
ReferenceData=$XAMAN_REFERENCE_DATA
Timestamp=$XAMAN_PAYLOAD_TIMESTAMP
SigningAuthority=human:xaman
RuntimeAuthority=everarcade
Custody=none
SigningStatus=unsigned-request
EOF_PAYLOAD
}

xaman_payload_root() {
  [[ -n "${XAMAN_PAYLOAD_ROOT_CACHE:-}" ]] || xaman_initialize_roots
  printf '%s\n' "$XAMAN_PAYLOAD_ROOT_CACHE"
}

canonical_xaman_deep_link() {
  cat <<EOF_LINK
version=$XAMAN_SIGNING_VERSION
PayloadIdentifier=$(xaman_payload_id)
RequestIdentifier=$(xaman_request_id)
TransactionReference=$(transaction_hash)
TransactionRoot=$(transaction_root)
RequestURL=xaman://payload/$(xaman_payload_id)?request=$(xaman_request_id)&tx=$(transaction_hash)
Timestamp=$XAMAN_REQUEST_TIMESTAMP
EOF_LINK
}

xaman_deep_link_root() {
  [[ -n "${XAMAN_DEEP_LINK_ROOT_CACHE:-}" ]] || xaman_initialize_roots
  printf '%s\n' "$XAMAN_DEEP_LINK_ROOT_CACHE"
}

xaman_request_hash() {
  [[ -n "${XAMAN_REQUEST_HASH_CACHE:-}" ]] || xaman_initialize_roots
  printf '%s\n' "$XAMAN_REQUEST_HASH_CACHE"
}

canonical_xaman_qr_metadata() {
  cat <<EOF_QR
version=$XAMAN_SIGNING_VERSION
PayloadID=$(xaman_payload_id)
ReferenceData=$XAMAN_REFERENCE_DATA
RequestHash=$(xaman_request_hash)
Encoding=qr-metadata-only
NetworkCalls=disabled
EOF_QR
}

xaman_qr_root() {
  [[ -n "${XAMAN_QR_ROOT_CACHE:-}" ]] || xaman_initialize_roots
  printf '%s\n' "$XAMAN_QR_ROOT_CACHE"
}

canonical_xaman_tracking_state() {
  cat <<EOF_TRACKING
version=$XAMAN_SIGNING_VERSION
PayloadID=$(xaman_payload_id)
SupportedStates=${XAMAN_TRACKING_STATES[*]}
Created=2026-06-04T00:00:12Z
Pending=2026-06-04T00:00:13Z
Approved=2026-06-04T00:00:20Z
Rejected=not-entered
Expired=not-entered
CurrentStatus=$XAMAN_CURRENT_STATUS
Timestamp=$XAMAN_TRACKING_TIMESTAMP
EOF_TRACKING
}

xaman_tracking_root() {
  [[ -n "${XAMAN_TRACKING_ROOT_CACHE:-}" ]] || xaman_initialize_roots
  printf '%s\n' "$XAMAN_TRACKING_ROOT_CACHE"
}

canonical_xaman_signed_receipt() {
  cat <<EOF_RECEIPT
version=$XAMAN_SIGNING_VERSION
PayloadID=$(xaman_payload_id)
TransactionHash=$(transaction_hash)
Wallet=$XAMAN_WALLET
Outcome=$XAMAN_RECEIPT_OUTCOME
Timestamp=$XAMAN_RECEIPT_TIMESTAMP
PayloadRoot=$(xaman_payload_root)
TransactionRoot=$(transaction_root)
SettlementRoot=$(settlement_root)
EOF_RECEIPT
}

xaman_signed_receipt_root() {
  [[ -n "${XAMAN_SIGNED_RECEIPT_ROOT_CACHE:-}" ]] || xaman_initialize_roots
  printf '%s\n' "$XAMAN_SIGNED_RECEIPT_ROOT_CACHE"
}

canonical_xaman_continuity_update() {
  cat <<EOF_CONTINUITY
version=$XAMAN_SIGNING_VERSION
IntentRoot=$(intent_root)
TransactionRoot=$(transaction_root)
PayloadRoot=$(xaman_payload_root)
SignedReceiptRoot=$(xaman_signed_receipt_root)
ReceiptMatchesPayload=PASS
PayloadMatchesTransaction=PASS
TransactionMatchesIntent=PASS
RuntimeAuthority=everarcade
SigningAuthority=xaman-human-approval
SettlementContinuityRoot=$(xaman_settlement_continuity_root)
EOF_CONTINUITY
}

xaman_settlement_continuity_root() {
  [[ -n "${XAMAN_CONTINUITY_ROOT_CACHE:-}" ]] || xaman_initialize_roots
  printf '%s\n' "$XAMAN_CONTINUITY_ROOT_CACHE"
}

canonical_xaman_replay() {
  cat <<EOF_REPLAY
version=$XAMAN_SIGNING_VERSION
intent.root=$(intent_root)
transaction.root=$(transaction_root)
payload.root=$(xaman_payload_root)
signed_receipt.root=$(xaman_signed_receipt_root)
settlement_continuity.root=$(xaman_settlement_continuity_root)
replay.root=$(xaman_settlement_continuity_root)
replay.equivalence=PASS
EOF_REPLAY
}

xaman_replay_root() {
  awk -F= '$1 == "replay.root" { print $2 }' < <(canonical_xaman_replay)
}

xaman_initialize_roots() {
  if [[ -n "${XAMAN_ROOTS_INITIALIZED:-}" ]]; then
    return
  fi
  initialize_roots
  XAMAN_ROOTS_INITIALIZED=1
  XAMAN_PAYLOAD_ID_CACHE="xaman-payload-$(xaman_payload_id_seed | xaman_sha256_text | xaman_short_hash)"
  XAMAN_REQUEST_ID_CACHE="xaman-request-$(printf 'request|%s|%s\n' "$XAMAN_PAYLOAD_ID_CACHE" "$(transaction_hash)" | xaman_sha256_text | xaman_short_hash)"
  XAMAN_PAYLOAD_ROOT_CACHE="$(canonical_xaman_payload | xaman_sha256_text)"
  XAMAN_DEEP_LINK_ROOT_CACHE="$(canonical_xaman_deep_link | xaman_sha256_text)"
  XAMAN_REQUEST_HASH_CACHE="$(canonical_xaman_deep_link | xaman_sha256_text)"
  XAMAN_QR_ROOT_CACHE="$(canonical_xaman_qr_metadata | xaman_sha256_text)"
  XAMAN_TRACKING_ROOT_CACHE="$(canonical_xaman_tracking_state | xaman_sha256_text)"
  XAMAN_SIGNED_RECEIPT_ROOT_CACHE="$(canonical_xaman_signed_receipt | xaman_sha256_text)"
  XAMAN_CONTINUITY_ROOT_CACHE="$(printf 'xaman-continuity|%s|%s|%s|%s\n' "$(intent_root)" "$(transaction_root)" "$XAMAN_PAYLOAD_ROOT_CACHE" "$XAMAN_SIGNED_RECEIPT_ROOT_CACHE" | xaman_sha256_text)"
}

xaman_validate_payload() {
  [[ "$(xaman_payload_id)" == xaman-payload-* ]] \
    && [[ "$XAMAN_WALLET" == "$SENDER" ]] \
    && [[ "$(xaman_payload_root)" =~ ^[0-9a-f]{64}$ ]] \
    && [[ "$(transaction_root)" =~ ^[0-9a-f]{64}$ ]] \
    && [[ "$(settlement_root)" =~ ^[0-9a-f]{64}$ ]]
}

xaman_validate_deep_link() {
  [[ "$(xaman_request_id)" == xaman-request-* ]] \
    && [[ "$(transaction_hash)" =~ ^[0-9a-f]{64}$ ]] \
    && [[ "$(xaman_deep_link_root)" =~ ^[0-9a-f]{64}$ ]] \
    && canonical_xaman_deep_link | grep -q '^RequestURL=xaman://payload/'
}

xaman_validate_qr_metadata() {
  xaman_validate_deep_link \
    && [[ "$(xaman_request_hash)" == "$(xaman_deep_link_root)" ]] \
    && [[ "$(xaman_qr_root)" =~ ^[0-9a-f]{64}$ ]]
}

xaman_validate_tracking() {
  local state
  for state in Created Pending Approved Rejected Expired; do
    [[ " ${XAMAN_TRACKING_STATES[*]} " == *" $state "* ]] || return 1
  done
  [[ "$XAMAN_CURRENT_STATUS" == "Approved" ]] && [[ "$(xaman_tracking_root)" =~ ^[0-9a-f]{64}$ ]]
}

xaman_validate_receipt_import() {
  xaman_validate_payload \
    && [[ "$XAMAN_RECEIPT_OUTCOME" == "approved" ]] \
    && [[ "$XAMAN_WALLET" == "$SENDER" ]] \
    && [[ "$(xaman_signed_receipt_root)" =~ ^[0-9a-f]{64}$ ]]
}

xaman_validate_continuity_update() {
  xaman_validate_receipt_import \
    && validate_transaction \
    && [[ "$(intent_root)" =~ ^[0-9a-f]{64}$ ]] \
    && [[ "$(transaction_root)" =~ ^[0-9a-f]{64}$ ]] \
    && [[ "$(xaman_settlement_continuity_root)" =~ ^[0-9a-f]{64}$ ]]
}

xaman_validate_replay() {
  xaman_validate_continuity_update && [[ "$(xaman_replay_root)" == "$(xaman_settlement_continuity_root)" ]]
}

write_xaman_signing_artifacts() {
  xaman_initialize_roots
  mkdir -p xaman/payloads xaman/requests xaman/receipts xaman/status xaman/tracking
  {
    canonical_xaman_payload
    printf 'PayloadRoot=%s\n' "$(xaman_payload_root)"
  } > xaman/payloads/settlement_payload.txt
  {
    canonical_xaman_deep_link
    printf 'DeepLinkRoot=%s\n' "$(xaman_deep_link_root)"
  } > xaman/requests/deep_link.txt
  {
    canonical_xaman_qr_metadata
    printf 'QRRoot=%s\n' "$(xaman_qr_root)"
  } > xaman/requests/qr_metadata.txt
  {
    canonical_xaman_tracking_state
    printf 'TrackingRoot=%s\n' "$(xaman_tracking_root)"
  } > xaman/tracking/payload_tracking.txt
  {
    printf 'PayloadID=%s\n' "$(xaman_payload_id)"
    printf 'CurrentStatus=%s\n' "$XAMAN_CURRENT_STATUS"
    printf 'TrackingRoot=%s\n' "$(xaman_tracking_root)"
  } > xaman/status/payload_status.txt
  {
    canonical_xaman_signed_receipt
    printf 'SignedReceiptRoot=%s\n' "$(xaman_signed_receipt_root)"
  } > xaman/receipts/signed_receipt.txt
  {
    canonical_xaman_continuity_update
  } > xaman/receipts/settlement_continuity.txt
  {
    canonical_xaman_replay
  } > xaman/status/replay_validation.txt
}
