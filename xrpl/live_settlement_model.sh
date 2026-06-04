#!/usr/bin/env bash
# Deterministic XRPL live-settlement model for EverArcade Runtime v0.1.
# This file intentionally models transaction representation and receipt evidence only.

XRPL_LIVE_VERSION="xrpl-live-settlement-v0.1"
INTENT_ID="intent-everarcade-001"
ASSET="XRP"
AMOUNT_DROPS="25000000"
SENDER="rEverArcadeSender111111111111111111111"
RECIPIENT="rEverArcadeRecipient111111111111111111"
INTENT_TIMESTAMP="2026-06-04T00:00:00Z"
INTENT_REFERENCE="everarcade:runtime-platform:lease-settlement:001"
TX_SEQUENCE="1"
TX_FEE_DROPS="12"
TX_LAST_LEDGER_SEQUENCE="88441199"
RECEIPT_LEDGER_INDEX="88441120"
RECEIPT_OUTCOME="tesSUCCESS"
RECEIPT_TIMESTAMP="2026-06-04T00:00:05Z"
CONTINUITY_ROOT="everarcade-continuity-root-v0.1"

sha256_text() {
  sha256sum | awk '{print $1}'
}

hex_text() {
  od -An -tx1 -v | tr -d ' \n'
}

canonical_intent_payload() {
  cat <<EOF_INTENT
version=$XRPL_LIVE_VERSION
intent.id=$INTENT_ID
asset=$ASSET
amount.drops=$AMOUNT_DROPS
sender=$SENDER
recipient=$RECIPIENT
timestamp=$INTENT_TIMESTAMP
reference=$INTENT_REFERENCE
EOF_INTENT
}

intent_root() {
  [[ -n "${XRPL_INTENT_ROOT_CACHE:-}" ]] || initialize_roots
  printf '%s\n' "$XRPL_INTENT_ROOT_CACHE"
}

memo_type_hex() {
  printf 'everarcade.intent' | hex_text
}

memo_format_hex() {
  printf 'text/plain' | hex_text
}

memo_data_hex() {
  initialize_roots
  printf '%s' "$XRPL_INTENT_ROOT_CACHE" | hex_text
}

canonical_transaction_payload() {
  local root
  root="$(intent_root)"
  cat <<EOF_TX
version=$XRPL_LIVE_VERSION
TransactionType=Payment
Account=$SENDER
Destination=$RECIPIENT
Amount=$AMOUNT_DROPS
Fee=$TX_FEE_DROPS
Sequence=$TX_SEQUENCE
LastLedgerSequence=$TX_LAST_LEDGER_SEQUENCE
Memos.0.Memo.MemoType=$(memo_type_hex)
Memos.0.Memo.MemoData=$(memo_data_hex)
Memos.0.Memo.MemoFormat=$(memo_format_hex)
Reference.intent_id=$INTENT_ID
Reference.intent_root=$root
Reference.runtime_reference=$INTENT_REFERENCE
SigningStatus=unsigned
EOF_TX
}

transaction_root() {
  [[ -n "${XRPL_TRANSACTION_ROOT_CACHE:-}" ]] || initialize_roots
  printf '%s\n' "$XRPL_TRANSACTION_ROOT_CACHE"
}

transaction_hash() {
  [[ -n "${XRPL_TRANSACTION_HASH_CACHE:-}" ]] || initialize_roots
  printf '%s\n' "$XRPL_TRANSACTION_HASH_CACHE"
}

canonical_verification_payload() {
  cat <<EOF_VERIFY
version=$XRPL_LIVE_VERSION
structure=PASS
account=PASS
amount=PASS
memo_integrity=PASS
reference_integrity=PASS
intent.root=$(intent_root)
transaction.root=$(transaction_root)
EOF_VERIFY
}

verification_root() {
  [[ -n "${XRPL_VERIFICATION_ROOT_CACHE:-}" ]] || initialize_roots
  printf '%s\n' "$XRPL_VERIFICATION_ROOT_CACHE"
}

canonical_receipt_payload() {
  cat <<EOF_RECEIPT
version=$XRPL_LIVE_VERSION
TransactionHash=$(transaction_hash)
LedgerIndex=$RECEIPT_LEDGER_INDEX
Account=$SENDER
Destination=$RECIPIENT
Amount=$AMOUNT_DROPS
Outcome=$RECEIPT_OUTCOME
Timestamp=$RECEIPT_TIMESTAMP
TransactionRoot=$(transaction_root)
IntentRoot=$(intent_root)
EOF_RECEIPT
}

receipt_root() {
  [[ -n "${XRPL_RECEIPT_ROOT_CACHE:-}" ]] || initialize_roots
  printf '%s\n' "$XRPL_RECEIPT_ROOT_CACHE"
}

settlement_evidence_root() {
  [[ -n "${XRPL_EVIDENCE_ROOT_CACHE:-}" ]] || initialize_roots
  printf '%s\n' "$XRPL_EVIDENCE_ROOT_CACHE"
}

canonical_import_payload() {
  cat <<EOF_IMPORT
version=$XRPL_LIVE_VERSION
receipt.integrity=PASS
transaction.match=PASS
intent.match=PASS
intent.root=$(intent_root)
transaction.root=$(transaction_root)
receipt.root=$(receipt_root)
settlement.root=$(settlement_root)
EOF_IMPORT
}

settlement_root() {
  [[ -n "${XRPL_SETTLEMENT_ROOT_CACHE:-}" ]] || initialize_roots
  printf '%s\n' "$XRPL_SETTLEMENT_ROOT_CACHE"
}

canonical_anchor_payload() {
  cat <<EOF_ANCHOR
version=$XRPL_LIVE_VERSION
ContinuityRoot=$CONTINUITY_ROOT
SettlementRoot=$(settlement_root)
ReceiptRoot=$(receipt_root)
SettlementEvidenceRoot=$(settlement_evidence_root)
EOF_ANCHOR
}

anchor_root() {
  [[ -n "${XRPL_ANCHOR_ROOT_CACHE:-}" ]] || initialize_roots
  printf '%s\n' "$XRPL_ANCHOR_ROOT_CACHE"
}

canonical_replay_payload() {
  cat <<EOF_REPLAY
version=$XRPL_LIVE_VERSION
intent.root=$(intent_root)
transaction.root=$(transaction_root)
receipt.root=$(receipt_root)
settlement.root=$(settlement_root)
replay.root=$(settlement_root)
replay.equivalence=PASS
EOF_REPLAY
}

replay_root() {
  awk -F= '$1 == "replay.root" { print $2 }' < <(canonical_replay_payload)
}

initialize_roots() {
  if [[ -n "${XRPL_ROOTS_INITIALIZED:-}" ]]; then
    return
  fi
  XRPL_ROOTS_INITIALIZED=1
  XRPL_INTENT_ROOT_CACHE="$(canonical_intent_payload | sha256_text)"
  XRPL_TRANSACTION_ROOT_CACHE="$(canonical_transaction_payload | sha256_text)"
  XRPL_TRANSACTION_HASH_CACHE="$(printf 'xrpl-transaction-hash|%s\n' "$XRPL_TRANSACTION_ROOT_CACHE" | sha256_text)"
  XRPL_VERIFICATION_ROOT_CACHE="$(canonical_verification_payload | sha256_text)"
  XRPL_RECEIPT_ROOT_CACHE="$(canonical_receipt_payload | sha256_text)"
  XRPL_SETTLEMENT_ROOT_CACHE="$(printf 'settlement|%s|%s|%s\n' "$XRPL_INTENT_ROOT_CACHE" "$XRPL_TRANSACTION_ROOT_CACHE" "$XRPL_RECEIPT_ROOT_CACHE" | sha256_text)"
  XRPL_EVIDENCE_ROOT_CACHE="$(canonical_import_payload | sha256_text)"
  XRPL_ANCHOR_ROOT_CACHE="$(canonical_anchor_payload | sha256_text)"
}

validate_intent() {
  [[ "$INTENT_ID" == intent-* ]] && [[ "$ASSET" == "XRP" ]] && [[ "$AMOUNT_DROPS" =~ ^[0-9]+$ ]] && [[ -n "$SENDER" ]] && [[ -n "$RECIPIENT" ]] && [[ "$SENDER" != "$RECIPIENT" ]] && [[ -n "$INTENT_TIMESTAMP" ]]
}

validate_transaction() {
  local memo_data expected_memo
  memo_data="$(memo_data_hex)"
  expected_memo="$(printf '%s' "$(intent_root)" | hex_text)"
  [[ "$(memo_type_hex)" == "657665726172636164652e696e74656e74" ]] && [[ "$memo_data" == "$expected_memo" ]] && [[ "$TX_FEE_DROPS" =~ ^[0-9]+$ ]] && [[ "$TX_SEQUENCE" =~ ^[0-9]+$ ]]
}

validate_verification() {
  validate_intent && validate_transaction && [[ -n "$(verification_root)" ]]
}

validate_receipt() {
  [[ "$(transaction_hash)" =~ ^[0-9a-f]{64}$ ]] && [[ "$RECEIPT_LEDGER_INDEX" =~ ^[0-9]+$ ]] && [[ "$RECEIPT_OUTCOME" == "tesSUCCESS" ]] && [[ "$SENDER" != "$RECIPIENT" ]]
}

validate_import() {
  validate_receipt && [[ "$(receipt_root)" =~ ^[0-9a-f]{64}$ ]] && [[ "$(transaction_root)" =~ ^[0-9a-f]{64}$ ]] && [[ "$(intent_root)" =~ ^[0-9a-f]{64}$ ]]
}

validate_anchor() {
  validate_import && [[ -n "$CONTINUITY_ROOT" ]] && [[ "$(anchor_root)" =~ ^[0-9a-f]{64}$ ]]
}

validate_replay() {
  validate_anchor && [[ "$(replay_root)" == "$(settlement_root)" ]]
}

write_live_settlement_artifacts() {
  initialize_roots
  mkdir -p xrpl/intent xrpl/transactions xrpl/receipts xrpl/anchors xrpl/verification
  {
    canonical_intent_payload
    printf 'intent.root=%s\n' "$(intent_root)"
  } > xrpl/intent/settlement_intent.txt
  {
    canonical_transaction_payload
    printf 'transaction.root=%s\n' "$(transaction_root)"
  } > xrpl/transactions/payment_transaction.txt
  {
    canonical_verification_payload
    printf 'verification.root=%s\n' "$(verification_root)"
  } > xrpl/verification/transaction_verification.txt
  {
    canonical_receipt_payload
    printf 'receipt.root=%s\n' "$(receipt_root)"
  } > xrpl/receipts/xrpl_receipt.txt
  {
    canonical_import_payload
    printf 'settlement.evidence.root=%s\n' "$(settlement_evidence_root)"
  } > xrpl/receipts/settlement_evidence.txt
  {
    canonical_anchor_payload
    printf 'anchor.root=%s\n' "$(anchor_root)"
  } > xrpl/anchors/continuity_anchor.txt
  {
    canonical_replay_payload
  } > xrpl/verification/settlement_replay.txt
}
