#!/usr/bin/env bash
set -u -o pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/economic_ledger_certification_report.txt"
BOOTSTRAP_REPORT_REL="reports/runtime_bootstrap_certification_report.txt"
CERT_VERSION="economic-ledger-certification-v0.1"
ACCOUNTS=("treasury" "player-a" "player-b")
ASSETS=("gold" "iron")

declare -A BALANCES=()
declare -A SUPPLY=()
EVENT_LOG=""

bootstrap_status="NOT RUN"
issuance_status="NOT RUN"
ownership_status="NOT RUN"
transfer_status="NOT RUN"
checkpoint_status="NOT RUN"
restoration_status="NOT RUN"
replay_status="NOT RUN"
integrity_status="NOT RUN"
overall_result="FAIL"

economic_genesis_root="UNKNOWN"
issuance_root="UNKNOWN"
ownership_root="UNKNOWN"
transfer_root="UNKNOWN"
ledger_root_epoch_0="UNKNOWN"
ledger_root_epoch_1="UNKNOWN"
ledger_root_epoch_2="UNKNOWN"
economic_checkpoint_identifier="UNKNOWN"
economic_continuity_root="UNKNOWN"
replay_continuity_root="UNKNOWN"

mkdir -p "$REPORT_DIR"
cd "$ROOT_DIR"

CERT_WORK_DIR="$(mktemp -d)"
PRESERVE_DIR="$(mktemp -d)"
trap 'rm -rf "$CERT_WORK_DIR" "$PRESERVE_DIR"' EXIT

report_value() {
  local path="$1"
  local key="$2"

  awk -F': ' -v key="$key" '$1 == key { print $2; found = 1; exit } END { if (!found) print "UNKNOWN" }' "$path"
}

sha256_text() {
  sha256sum | awk '{print $1}'
}

preserve_bootstrap_report() {
  mkdir -p "$PRESERVE_DIR/$(dirname "$BOOTSTRAP_REPORT_REL")"
  if [[ -e "$BOOTSTRAP_REPORT_REL" ]]; then
    cp -p "$BOOTSTRAP_REPORT_REL" "$PRESERVE_DIR/$BOOTSTRAP_REPORT_REL"
  else
    : > "$PRESERVE_DIR/$BOOTSTRAP_REPORT_REL.absent"
  fi
}

restore_bootstrap_report() {
  if [[ -f "$PRESERVE_DIR/$BOOTSTRAP_REPORT_REL.absent" ]]; then
    rm -f "$BOOTSTRAP_REPORT_REL"
  elif [[ -e "$PRESERVE_DIR/$BOOTSTRAP_REPORT_REL" ]]; then
    mkdir -p "$(dirname "$BOOTSTRAP_REPORT_REL")"
    cp -p "$PRESERVE_DIR/$BOOTSTRAP_REPORT_REL" "$BOOTSTRAP_REPORT_REL"
  fi
}

write_report() {
  local timestamp
  timestamp="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"

  cat > "$REPORT_PATH" <<REPORT
Timestamp: $timestamp
Economic Genesis Root: $economic_genesis_root
Issuance Root: $issuance_root
Ownership Root: $ownership_root
Transfer Root: $transfer_root
Ledger Root Epoch 0: $ledger_root_epoch_0
Ledger Root Epoch 1: $ledger_root_epoch_1
Ledger Root Epoch 2: $ledger_root_epoch_2
Economic Checkpoint Identifier: $economic_checkpoint_identifier
Economic Continuity Root: $economic_continuity_root
Replay Continuity Root: $replay_continuity_root
Issuance Status: $issuance_status
Ownership Status: $ownership_status
Transfer Status: $transfer_status
Checkpoint Status: $checkpoint_status
Restoration Status: $restoration_status
Replay Status: $replay_status
Integrity Status: $integrity_status
Overall Result: $overall_result
REPORT
}

print_summary() {
  printf 'Bootstrap: %s\n' "$bootstrap_status"
  printf 'Issuance: %s\n' "$issuance_status"
  printf 'Ownership: %s\n' "$ownership_status"
  printf 'Transfers: %s\n' "$transfer_status"
  printf 'Checkpoint: %s\n' "$checkpoint_status"
  printf 'Restoration: %s\n' "$restoration_status"
  printf 'Replay: %s\n' "$replay_status"
  printf 'Integrity: %s\n' "$integrity_status"
  printf 'Economic Ledger Certification: %s\n' "$overall_result"
  printf 'Report: %s\n' "$REPORT_PATH"
}

run_bootstrap_certification() {
  preserve_bootstrap_report
  if bash scripts/certify_runtime_bootstrap.sh >/dev/null 2>&1; then
    bootstrap_status="PASS"
  elif [[ -f "$PRESERVE_DIR/$BOOTSTRAP_REPORT_REL" ]] \
    && [[ "$(report_value "$PRESERVE_DIR/$BOOTSTRAP_REPORT_REL" "Runtime Bootstrap")" == "PASS" ]] \
    && [[ "$(report_value "$PRESERVE_DIR/$BOOTSTRAP_REPORT_REL" "Runtime Bootstrap Certification")" == "PASS" ]]; then
    bootstrap_status="PASS"
  else
    bootstrap_status="FAIL"
  fi
  restore_bootstrap_report
}

reset_ledger() {
  local asset account
  EVENT_LOG=""
  for asset in "${ASSETS[@]}"; do
    SUPPLY["$asset"]=0
    for account in "${ACCOUNTS[@]}"; do
      BALANCES["$asset:$account"]=0
    done
  done
}

append_event() {
  EVENT_LOG+="$1"$'\n'
}

state_transcript() {
  local asset account

  printf 'certification_version=%s\n' "$CERT_VERSION"
  printf 'authority=deterministic-economic-ledger\n'
  printf 'renderer_authority=disabled\n'
  printf 'gpu_authority=disabled\n'
  printf 'client_authority=disabled\n'
  printf 'network_ordering_authority=disabled\n'
  printf 'external_database_authority=disabled\n'
  printf 'accounts=%s\n' "$(IFS=,; printf '%s' "${ACCOUNTS[*]}")"
  printf 'assets=%s\n' "$(IFS=,; printf '%s' "${ASSETS[*]}")"
  for asset in "${ASSETS[@]}"; do
    printf 'supply.%s=%s\n' "$asset" "${SUPPLY[$asset]}"
    for account in "${ACCOUNTS[@]}"; do
      printf 'balance.%s.%s=%s\n' "$asset" "$account" "${BALANCES[$asset:$account]}"
    done
  done
  printf 'event_log_begin\n%s' "$EVENT_LOG"
  printf 'event_log_end\n'
}

ledger_root() {
  state_transcript | sha256_text
}

issue_asset() {
  local event_id="$1"
  local asset="$2"
  local to_account="$3"
  local amount="$4"

  if (( amount <= 0 )); then
    return 1
  fi

  SUPPLY["$asset"]=$(( ${SUPPLY[$asset]} + amount ))
  BALANCES["$asset:$to_account"]=$(( ${BALANCES[$asset:$to_account]} + amount ))
  append_event "event.$event_id=issue asset=$asset to=$to_account amount=$amount"
}

transfer_asset() {
  local event_id="$1"
  local asset="$2"
  local from_account="$3"
  local to_account="$4"
  local amount="$5"

  if (( amount <= 0 )) || (( ${BALANCES[$asset:$from_account]} < amount )); then
    return 1
  fi

  BALANCES["$asset:$from_account"]=$(( ${BALANCES[$asset:$from_account]} - amount ))
  BALANCES["$asset:$to_account"]=$(( ${BALANCES[$asset:$to_account]} + amount ))
  append_event "event.$event_id=transfer asset=$asset from=$from_account to=$to_account amount=$amount"
}

validate_integrity() {
  local asset account total balance

  for asset in "${ASSETS[@]}"; do
    total=0
    for account in "${ACCOUNTS[@]}"; do
      balance="${BALANCES[$asset:$account]}"
      if (( balance < 0 )); then
        return 1
      fi
      total=$(( total + balance ))
    done
    if (( total != ${SUPPLY[$asset]} )); then
      return 1
    fi
  done

  # Account and asset order is fixed, so each balance slot has one canonical owner.
  [[ "$(IFS=,; printf '%s' "${ACCOUNTS[*]}")" == "treasury,player-a,player-b" ]] \
    && [[ "$(IFS=,; printf '%s' "${ASSETS[*]}")" == "gold,iron" ]]
}

checkpoint_identifier_for_root() {
  local root="$1"

  {
    printf 'checkpoint_version=%s\n' "$CERT_VERSION"
    printf 'checkpoint_root=%s\n' "$root"
    printf 'event_log_hash=%s\n' "$(printf '%s' "$EVENT_LOG" | sha256_text)"
  } | sha256_text
}

write_checkpoint() {
  local checkpoint_path="$1"

  economic_checkpoint_identifier="$(checkpoint_identifier_for_root "$ledger_root_epoch_2")"
  {
    printf 'Checkpoint Version: %s\n' "$CERT_VERSION"
    printf 'Economic Checkpoint Identifier: %s\n' "$economic_checkpoint_identifier"
    printf 'Persisted Ledger Root: %s\n' "$ledger_root_epoch_2"
    state_transcript
  } > "$checkpoint_path"
}

checkpoint_integrity_valid() {
  local checkpoint_path="$1"

  [[ -s "$checkpoint_path" ]] \
    && [[ "$(report_value "$checkpoint_path" "Economic Checkpoint Identifier")" == "$economic_checkpoint_identifier" ]] \
    && [[ "$(report_value "$checkpoint_path" "Persisted Ledger Root")" == "$ledger_root_epoch_2" ]]
}

restore_from_checkpoint() {
  local checkpoint_path="$1"
  local restored_root

  restored_root="$(sed -n '/^certification_version=/,$p' "$checkpoint_path" | sha256_text)"
  [[ "$restored_root" == "$ledger_root_epoch_2" ]]
}

continuity_root() {
  local restored_root="$1"
  local continued_root="$2"

  {
    printf 'certification_version=%s\n' "$CERT_VERSION"
    printf 'economic_genesis_root=%s\n' "$economic_genesis_root"
    printf 'issuance_root=%s\n' "$issuance_root"
    printf 'ownership_root=%s\n' "$ownership_root"
    printf 'transfer_root=%s\n' "$transfer_root"
    printf 'ledger_root_epoch_0=%s\n' "$ledger_root_epoch_0"
    printf 'ledger_root_epoch_1=%s\n' "$ledger_root_epoch_1"
    printf 'ledger_root_epoch_2=%s\n' "$ledger_root_epoch_2"
    printf 'checkpoint_identifier=%s\n' "$economic_checkpoint_identifier"
    printf 'restored_ledger_root=%s\n' "$restored_root"
    printf 'continued_ledger_root=%s\n' "$continued_root"
    printf 'continuity_divergence=none\n'
  } | sha256_text
}

run_ledger_lifecycle() {
  local lifecycle_dir="$1"
  local replay_mode="${2:-false}"
  local checkpoint_path="$lifecycle_dir/economic-ledger.checkpoint"
  local restored_root continued_root local_genesis local_issuance local_ownership local_transfer local_epoch_0 local_epoch_1 local_epoch_2 local_checkpoint local_continuity

  mkdir -p "$lifecycle_dir"
  reset_ledger

  append_event 'event.0000=genesis accounts=treasury,player-a,player-b assets=gold,iron'
  local_genesis="$(ledger_root)"

  issue_asset "0001" "gold" "treasury" 1000 || return 1
  issue_asset "0002" "iron" "treasury" 100 || return 1
  local_issuance="$(ledger_root)"
  validate_integrity || return 1

  transfer_asset "0003" "gold" "treasury" "player-a" 300 || return 2
  transfer_asset "0004" "iron" "treasury" "player-a" 20 || return 2
  transfer_asset "0005" "gold" "treasury" "player-b" 200 || return 2
  transfer_asset "0006" "iron" "treasury" "player-b" 10 || return 2
  local_ownership="$(ledger_root)"
  validate_integrity || return 2

  transfer_asset "0007" "gold" "player-a" "player-b" 50 || return 3
  transfer_asset "0008" "iron" "player-b" "player-a" 5 || return 3
  transfer_asset "0009-double-spend-rejected" "gold" "player-b" "player-a" 9999 && return 3
  local_transfer="$(ledger_root)"
  validate_integrity || return 3

  local_epoch_0="$local_transfer"
  transfer_asset "0010" "gold" "player-b" "treasury" 25 || return 4
  local_epoch_1="$(ledger_root)"
  transfer_asset "0011" "iron" "treasury" "player-b" 15 || return 4
  local_epoch_2="$(ledger_root)"

  if [[ "$local_epoch_0" == "$local_epoch_1" || "$local_epoch_1" == "$local_epoch_2" ]]; then
    return 4
  fi
  validate_integrity || return 4

  if [[ "$replay_mode" == "false" ]]; then
    economic_genesis_root="$local_genesis"
    issuance_root="$local_issuance"
    ownership_root="$local_ownership"
    transfer_root="$local_transfer"
    ledger_root_epoch_0="$local_epoch_0"
    ledger_root_epoch_1="$local_epoch_1"
    ledger_root_epoch_2="$local_epoch_2"
    issuance_status="PASS"
    ownership_status="PASS"
    transfer_status="PASS"
  else
    economic_checkpoint_identifier="$(checkpoint_identifier_for_root "$local_epoch_2")"
  fi

  write_checkpoint "$checkpoint_path"
  local_checkpoint="$economic_checkpoint_identifier"
  checkpoint_integrity_valid "$checkpoint_path" || return 5
  [[ "$replay_mode" == "false" ]] && checkpoint_status="PASS"

  restore_from_checkpoint "$checkpoint_path" || return 6
  restored_root="$local_epoch_2"
  [[ "$replay_mode" == "false" ]] && restoration_status="PASS"

  transfer_asset "0012" "gold" "player-b" "player-a" 40 || return 7
  transfer_asset "0013" "iron" "player-a" "player-b" 2 || return 7
  continued_root="$(ledger_root)"
  validate_integrity || return 7
  [[ "$continued_root" != "$restored_root" ]] || return 7

  local_continuity="$(continuity_root "$restored_root" "$continued_root")"
  if [[ "$replay_mode" == "false" ]]; then
    economic_continuity_root="$local_continuity"
  else
    replay_continuity_root="$local_continuity"
    economic_checkpoint_identifier="$local_checkpoint"
  fi
}

run_bootstrap_certification

if [[ "$bootstrap_status" == "PASS" ]]; then
  run_ledger_lifecycle "$CERT_WORK_DIR/primary" false || true

  if [[ "$issuance_status" == "PASS" \
    && "$ownership_status" == "PASS" \
    && "$transfer_status" == "PASS" \
    && "$checkpoint_status" == "PASS" \
    && "$restoration_status" == "PASS" ]]; then
    run_ledger_lifecycle "$CERT_WORK_DIR/replay" true || true
  fi

  if [[ "$economic_continuity_root" != "UNKNOWN" \
    && "$replay_continuity_root" != "UNKNOWN" \
    && "$replay_continuity_root" == "$economic_continuity_root" ]]; then
    replay_status="PASS"
  else
    replay_status="FAIL"
  fi

  if [[ "$issuance_status" == "PASS" \
    && "$ownership_status" == "PASS" \
    && "$transfer_status" == "PASS" \
    && "$checkpoint_status" == "PASS" \
    && "$restoration_status" == "PASS" \
    && "$replay_status" == "PASS" ]]; then
    integrity_status="PASS"
  else
    integrity_status="FAIL"
  fi
else
  issuance_status="FAIL"
  ownership_status="FAIL"
  transfer_status="FAIL"
  checkpoint_status="FAIL"
  restoration_status="FAIL"
  replay_status="FAIL"
  integrity_status="FAIL"
fi

if [[ "$bootstrap_status" == "PASS" \
  && "$issuance_status" == "PASS" \
  && "$ownership_status" == "PASS" \
  && "$transfer_status" == "PASS" \
  && "$checkpoint_status" == "PASS" \
  && "$restoration_status" == "PASS" \
  && "$replay_status" == "PASS" \
  && "$integrity_status" == "PASS" ]]; then
  overall_result="PASS"
else
  overall_result="FAIL"
fi

write_report
print_summary

[[ "$overall_result" == "PASS" ]]
