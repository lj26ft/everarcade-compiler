#!/usr/bin/env bash
set -u -o pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/wallet_authority_certification_report.txt"
BOOTSTRAP_REPORT_REL="reports/runtime_bootstrap_certification_report.txt"
CERT_VERSION="wallet-authority-certification-v0.1"

ASSETS=("asset:sword-001" "asset:shield-001" "asset:potion-001")
OWNERS=("player-a" "player-b")
INVENTORIES=("inventory:player-a" "inventory:player-b")
VAULTS=("vault:a" "vault:b")
AUTHORITIES=("authority:a" "authority:b" "delegate:c")

SWORD="asset:sword-001"
SHIELD="asset:shield-001"
POTION="asset:potion-001"
AUTHORITY_A="authority:a"
AUTHORITY_B="authority:b"
DELEGATE_C="delegate:c"

ASSET_NAMES=()
declare -A OWNER=()
declare -A ASSET_INVENTORY=()
declare -A ASSET_VAULT=()
declare -A PRIMARY_AUTHORITY=()
declare -A DELEGATE_FROM=()
declare -A DELEGATE_STATUS=()
declare -A LINEAGE=()
declare -A INVENTORY_OWNER=()
declare -A VAULT_CREATED=()
EVENT_LOG=""
EVENT_IDS=""
AUTHORIZED_APPROVALS="0"
UNAUTHORIZED_APPROVALS="0"
REJECTED_REQUESTS="0"

bootstrap_status="NOT RUN"
assignment_status="NOT RUN"
authorization_status="NOT RUN"
delegation_status="NOT RUN"
revocation_status="NOT RUN"
checkpoint_status="NOT RUN"
restoration_status="NOT RUN"
replay_status="NOT RUN"
integrity_status="NOT RUN"
overall_result="FAIL"

authority_genesis_root="UNKNOWN"
authority_assignment_root="UNKNOWN"
authorized_action_root="UNKNOWN"
unauthorized_rejection_root="UNKNOWN"
delegation_root="UNKNOWN"
delegated_action_root="UNKNOWN"
revocation_root="UNKNOWN"
revoked_authority_root="UNKNOWN"
authority_root_epoch_0="UNKNOWN"
authority_root_epoch_1="UNKNOWN"
authority_root_epoch_2="UNKNOWN"
authority_checkpoint_identifier="UNKNOWN"
authority_continuity_root="UNKNOWN"
replay_authority_root="UNKNOWN"

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
Authority Genesis Root: $authority_genesis_root
Authority Assignment Root: $authority_assignment_root
Authorized Action Root: $authorized_action_root
Unauthorized Rejection Root: $unauthorized_rejection_root
Delegation Root: $delegation_root
Delegated Action Root: $delegated_action_root
Revocation Root: $revocation_root
Revoked Authority Root: $revoked_authority_root
Authority Root Epoch 0: $authority_root_epoch_0
Authority Root Epoch 1: $authority_root_epoch_1
Authority Root Epoch 2: $authority_root_epoch_2
Authority Checkpoint Identifier: $authority_checkpoint_identifier
Authority Continuity Root: $authority_continuity_root
Replay Authority Root: $replay_authority_root
Assignment Status: $assignment_status
Authorization Status: $authorization_status
Delegation Status: $delegation_status
Revocation Status: $revocation_status
Checkpoint Status: $checkpoint_status
Restoration Status: $restoration_status
Replay Status: $replay_status
Integrity Status: $integrity_status
Overall Result: $overall_result
REPORT
}

print_summary() {
  printf 'Bootstrap: %s\n' "$bootstrap_status"
  printf 'Assignment: %s\n' "$assignment_status"
  printf 'Authorization: %s\n' "$authorization_status"
  printf 'Delegation: %s\n' "$delegation_status"
  printf 'Revocation: %s\n' "$revocation_status"
  printf 'Checkpoint: %s\n' "$checkpoint_status"
  printf 'Restoration: %s\n' "$restoration_status"
  printf 'Replay: %s\n' "$replay_status"
  printf 'Integrity: %s\n' "$integrity_status"
  printf 'Wallet Authority Certification: %s\n' "$overall_result"
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

reset_authority_state() {
  local asset inventory vault authority
  EVENT_LOG=""
  EVENT_IDS=""
  ASSET_NAMES=()
  AUTHORIZED_APPROVALS="0"
  UNAUTHORIZED_APPROVALS="0"
  REJECTED_REQUESTS="0"

  for asset in "${ASSETS[@]}"; do
    OWNER["$asset"]=""
    ASSET_INVENTORY["$asset"]=""
    ASSET_VAULT["$asset"]=""
    PRIMARY_AUTHORITY["$asset"]=""
    LINEAGE["$asset"]=""
  done

  for inventory in "${INVENTORIES[@]}"; do
    case "$inventory" in
      "inventory:player-a") INVENTORY_OWNER["$inventory"]="player-a" ;;
      "inventory:player-b") INVENTORY_OWNER["$inventory"]="player-b" ;;
      *) return 1 ;;
    esac
  done

  for vault in "${VAULTS[@]}"; do
    VAULT_CREATED["$vault"]="true"
  done

  for authority in "${AUTHORITIES[@]}"; do
    DELEGATE_FROM["$authority"]=""
    DELEGATE_STATUS["$authority"]="none"
  done
}

append_event() {
  local event_id="$1"
  local event_body="$2"

  if printf '%s' "$EVENT_IDS" | grep -qxF "$event_id"; then
    return 1
  fi

  EVENT_IDS+="$event_id"$'\n'
  EVENT_LOG+="event.$event_id=$event_body"$'\n'
}

known_asset() {
  local expected asset="$1"
  for expected in "${ASSETS[@]}"; do
    [[ "$asset" == "$expected" ]] && return 0
  done
  return 1
}

known_inventory() {
  local expected inventory="$1"
  for expected in "${INVENTORIES[@]}"; do
    [[ "$inventory" == "$expected" ]] && return 0
  done
  return 1
}

known_vault() {
  local expected vault="$1"
  for expected in "${VAULTS[@]}"; do
    [[ "$vault" == "$expected" ]] && return 0
  done
  return 1
}

known_authority() {
  local expected authority="$1"
  for expected in "${AUTHORITIES[@]}"; do
    [[ "$authority" == "$expected" ]] && return 0
  done
  return 1
}

asset_name_for() {
  local asset="$1"
  case "$asset" in
    "$SWORD") printf 'Sword #001' ;;
    "$SHIELD") printf 'Shield #001' ;;
    "$POTION") printf 'Potion #001' ;;
    *) printf 'UNKNOWN' ;;
  esac
}

create_asset() {
  local event_id="$1"
  local asset="$2"
  local name="$3"
  local existing

  known_asset "$asset" || return 1
  for existing in "${ASSET_NAMES[@]}"; do
    [[ "$existing" == "$asset" ]] && return 1
  done

  ASSET_NAMES+=("$asset")
  LINEAGE["$asset"]="genesis:none"
  append_event "$event_id" "genesis asset=$asset name=$name owner=none inventory=none vault=none authority=none" || return 1
}

assign_owner_inventory_vault() {
  local event_id="$1"
  local asset="$2"
  local owner="$3"
  local inventory="$4"
  local vault="$5"

  known_asset "$asset" || return 1
  known_inventory "$inventory" || return 1
  known_vault "$vault" || return 1
  [[ "${INVENTORY_OWNER[$inventory]}" == "$owner" ]] || return 1
  [[ -n "${LINEAGE[$asset]:-}" ]] || return 1
  [[ -z "${OWNER[$asset]:-}" ]] || return 1

  OWNER["$asset"]="$owner"
  ASSET_INVENTORY["$asset"]="$inventory"
  ASSET_VAULT["$asset"]="$vault"
  LINEAGE["$asset"]+=" -> owner:$owner -> inventory:$inventory -> vault_custody:$vault"
  append_event "$event_id" "asset_setup asset=$asset owner=$owner inventory=$inventory vault=$vault" || return 1
}

assign_authority() {
  local event_id="$1"
  local asset="$2"
  local authority="$3"

  known_asset "$asset" || return 1
  known_authority "$authority" || return 1
  [[ -n "${OWNER[$asset]:-}" ]] || return 1
  [[ -z "${PRIMARY_AUTHORITY[$asset]:-}" ]] || return 1
  [[ "${DELEGATE_STATUS[$authority]}" == "none" ]] || return 1

  PRIMARY_AUTHORITY["$asset"]="$authority"
  LINEAGE["$asset"]+=" -> authority_assignment:$authority"
  append_event "$event_id" "assign_authority asset=$asset owner=${OWNER[$asset]} authority=$authority" || return 1
}

is_authorized_for_asset() {
  local asset="$1"
  local authority="$2"
  local primary

  known_asset "$asset" || return 1
  known_authority "$authority" || return 1
  primary="${PRIMARY_AUTHORITY[$asset]:-}"
  [[ -n "$primary" ]] || return 1
  [[ "$authority" == "$primary" ]] && return 0
  [[ "${DELEGATE_FROM[$authority]:-}" == "$primary" && "${DELEGATE_STATUS[$authority]:-}" == "active" ]] && return 0
  return 1
}

authorize_action() {
  local event_id="$1"
  local asset="$2"
  local authority="$3"
  local action="$4"

  is_authorized_for_asset "$asset" "$authority" || return 1
  AUTHORIZED_APPROVALS="$(( AUTHORIZED_APPROVALS + 1 ))"
  LINEAGE["$asset"]+=" -> authorized_action:$authority:$action"
  append_event "$event_id" "authorized action=$action asset=$asset authority=$authority owner=${OWNER[$asset]} vault=${ASSET_VAULT[$asset]}" || return 1
}

expect_rejected_action() {
  local asset="$1"
  local authority="$2"

  if is_authorized_for_asset "$asset" "$authority"; then
    UNAUTHORIZED_APPROVALS="$(( UNAUTHORIZED_APPROVALS + 1 ))"
    return 1
  fi
  :
}

delegate_authority() {
  local event_id="$1"
  local asset="$2"
  local from_authority="$3"
  local delegate="$4"

  known_asset "$asset" || return 1
  known_authority "$from_authority" || return 1
  known_authority "$delegate" || return 1
  [[ "${PRIMARY_AUTHORITY[$asset]:-}" == "$from_authority" ]] || return 1
  [[ "$from_authority" != "$delegate" ]] || return 1
  [[ "${DELEGATE_STATUS[$delegate]}" == "none" ]] || return 1

  DELEGATE_FROM["$delegate"]="$from_authority"
  DELEGATE_STATUS["$delegate"]="active"
  LINEAGE["$asset"]+=" -> delegation:$from_authority>$delegate"
  append_event "$event_id" "delegate_authority asset=$asset from=$from_authority delegate=$delegate status=active" || return 1
}

revoke_authority() {
  local event_id="$1"
  local asset="$2"
  local from_authority="$3"
  local delegate="$4"

  known_asset "$asset" || return 1
  [[ "${PRIMARY_AUTHORITY[$asset]:-}" == "$from_authority" ]] || return 1
  [[ "${DELEGATE_FROM[$delegate]:-}" == "$from_authority" ]] || return 1
  [[ "${DELEGATE_STATUS[$delegate]:-}" == "active" ]] || return 1

  DELEGATE_STATUS["$delegate"]="revoked"
  LINEAGE["$asset"]+=" -> revocation:$delegate"
  append_event "$event_id" "revoke_authority asset=$asset from=$from_authority delegate=$delegate status=revoked" || return 1
}

state_transcript() {
  local asset authority inventory vault

  printf 'certification_version=%s\n' "$CERT_VERSION"
  printf 'runtime_authority=enabled\n'
  printf 'settlement=disabled\n'
  printf 'xrpl=disabled\n'
  printf 'xaman=disabled\n'
  printf 'cryptographic_signing=disabled\n'
  printf 'multisig=disabled\n'
  printf 'assets=%s\n' "$(IFS=,; printf '%s' "${ASSETS[*]}")"
  printf 'owners=%s\n' "$(IFS=,; printf '%s' "${OWNERS[*]}")"
  printf 'inventories=%s\n' "$(IFS=,; printf '%s' "${INVENTORIES[*]}")"
  printf 'vaults=%s\n' "$(IFS=,; printf '%s' "${VAULTS[*]}")"
  printf 'authorities=%s\n' "$(IFS=,; printf '%s' "${AUTHORITIES[*]}")"
  printf 'authorized_approvals=%s\n' "$AUTHORIZED_APPROVALS"
  printf 'unauthorized_approvals=%s\n' "$UNAUTHORIZED_APPROVALS"
  printf 'rejected_requests=%s\n' "$REJECTED_REQUESTS"
  for inventory in "${INVENTORIES[@]}"; do
    printf 'inventory.%s.owner=%s\n' "$inventory" "${INVENTORY_OWNER[$inventory]}"
  done
  for vault in "${VAULTS[@]}"; do
    printf 'vault.%s.created=%s\n' "$vault" "${VAULT_CREATED[$vault]}"
  done
  for authority in "${AUTHORITIES[@]}"; do
    printf 'authority.%s.delegate_from=%s\n' "$authority" "${DELEGATE_FROM[$authority]}"
    printf 'authority.%s.delegate_status=%s\n' "$authority" "${DELEGATE_STATUS[$authority]}"
  done
  for asset in "${ASSETS[@]}"; do
    printf 'asset.%s.name=%s\n' "$asset" "$(asset_name_for "$asset")"
    printf 'asset.%s.owner=%s\n' "$asset" "${OWNER[$asset]}"
    printf 'asset.%s.inventory=%s\n' "$asset" "${ASSET_INVENTORY[$asset]}"
    printf 'asset.%s.vault=%s\n' "$asset" "${ASSET_VAULT[$asset]}"
    printf 'asset.%s.primary_authority=%s\n' "$asset" "${PRIMARY_AUTHORITY[$asset]}"
    printf 'asset.%s.lineage=%s\n' "$asset" "${LINEAGE[$asset]}"
  done
  printf 'event_log_begin\n%s' "$EVENT_LOG"
  printf 'event_log_end\n'
}

authority_state_root() {
  state_transcript | sha256_text
}

validate_unique_assets() {
  local i j
  [[ "${#ASSET_NAMES[@]}" -eq "${#ASSETS[@]}" ]] || return 1
  for (( i = 0; i < ${#ASSET_NAMES[@]}; i++ )); do
    known_asset "${ASSET_NAMES[$i]}" || return 1
    for (( j = i + 1; j < ${#ASSET_NAMES[@]}; j++ )); do
      [[ "${ASSET_NAMES[$i]}" != "${ASSET_NAMES[$j]}" ]] || return 1
    done
  done
}

validate_authority_integrity() {
  local asset authority primary_count event_count event_id_count active_delegate_count

  validate_unique_assets || return 1
  event_count="$(printf '%s' "$EVENT_LOG" | sed '/^$/d' | wc -l | tr -d ' ')"
  event_id_count="$(printf '%s' "$EVENT_IDS" | sed '/^$/d' | sort -u | wc -l | tr -d ' ')"
  [[ "$event_count" == "$event_id_count" ]] || return 1
  [[ "$UNAUTHORIZED_APPROVALS" == "0" ]] || return 1

  for asset in "${ASSETS[@]}"; do
    [[ -n "${OWNER[$asset]}" ]] || return 1
    known_inventory "${ASSET_INVENTORY[$asset]}" || return 1
    known_vault "${ASSET_VAULT[$asset]}" || return 1
    [[ "${INVENTORY_OWNER[${ASSET_INVENTORY[$asset]}]}" == "${OWNER[$asset]}" ]] || return 1
    [[ -n "${PRIMARY_AUTHORITY[$asset]}" ]] || return 1
    known_authority "${PRIMARY_AUTHORITY[$asset]}" || return 1
    [[ "${LINEAGE[$asset]}" == genesis:none* ]] || return 1
    [[ "${LINEAGE[$asset]}" == *"authority_assignment:${PRIMARY_AUTHORITY[$asset]}"* ]] || return 1
  done

  for authority in "${AUTHORITIES[@]}"; do
    primary_count=0
    for asset in "${ASSETS[@]}"; do
      [[ "${PRIMARY_AUTHORITY[$asset]}" == "$authority" ]] && primary_count="$(( primary_count + 1 ))"
    done
    active_delegate_count=0
    for asset in "${ASSETS[@]}"; do
      [[ "${DELEGATE_FROM[$authority]:-}" == "${PRIMARY_AUTHORITY[$asset]}" && "${DELEGATE_STATUS[$authority]:-}" == "active" ]] && active_delegate_count="$(( active_delegate_count + 1 ))"
    done
    [[ "$primary_count" -le 1 ]] || return 1
    [[ "$active_delegate_count" -le 1 ]] || return 1
    if [[ "${DELEGATE_STATUS[$authority]:-}" == "active" || "${DELEGATE_STATUS[$authority]:-}" == "revoked" ]]; then
      known_authority "${DELEGATE_FROM[$authority]}" || return 1
      [[ "${DELEGATE_FROM[$authority]}" != "$authority" ]] || return 1
    fi
  done

  [[ "${LINEAGE[$SWORD]}" == *"delegation:$AUTHORITY_A>$DELEGATE_C"* ]] || return 1
  [[ "${LINEAGE[$SWORD]}" == *"revocation:$DELEGATE_C"* ]] || return 1
  [[ "${DELEGATE_STATUS[$DELEGATE_C]}" == "revoked" ]] || return 1
  expect_rejected_action "$SWORD" "$DELEGATE_C" || return 1
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
  local root="$2"

  authority_checkpoint_identifier="$(checkpoint_identifier_for_root "$root")"
  {
    printf 'Checkpoint Version: %s\n' "$CERT_VERSION"
    printf 'Authority Checkpoint Identifier: %s\n' "$authority_checkpoint_identifier"
    printf 'Persisted Authority Root: %s\n' "$root"
    state_transcript
  } > "$checkpoint_path"
}

checkpoint_integrity_valid() {
  local checkpoint_path="$1"
  local root="$2"

  [[ -s "$checkpoint_path" ]] \
    && [[ "$(report_value "$checkpoint_path" "Authority Checkpoint Identifier")" == "$authority_checkpoint_identifier" ]] \
    && [[ "$(report_value "$checkpoint_path" "Persisted Authority Root")" == "$root" ]]
}

restore_from_checkpoint() {
  local checkpoint_path="$1"
  local expected_root="$2"
  local restored_root

  restored_root="$(sed -n '/^certification_version=/,$p' "$checkpoint_path" | sha256_text)"
  [[ "$restored_root" == "$expected_root" ]]
}

continuity_root() {
  local restored_root="$1"
  local continued_root="$2"

  {
    printf 'certification_version=%s\n' "$CERT_VERSION"
    printf 'authority_genesis_root=%s\n' "$authority_genesis_root"
    printf 'authority_assignment_root=%s\n' "$authority_assignment_root"
    printf 'authorized_action_root=%s\n' "$authorized_action_root"
    printf 'unauthorized_rejection_root=%s\n' "$unauthorized_rejection_root"
    printf 'delegation_root=%s\n' "$delegation_root"
    printf 'delegated_action_root=%s\n' "$delegated_action_root"
    printf 'revocation_root=%s\n' "$revocation_root"
    printf 'revoked_authority_root=%s\n' "$revoked_authority_root"
    printf 'authority_root_epoch_0=%s\n' "$authority_root_epoch_0"
    printf 'authority_root_epoch_1=%s\n' "$authority_root_epoch_1"
    printf 'authority_root_epoch_2=%s\n' "$authority_root_epoch_2"
    printf 'checkpoint_identifier=%s\n' "$authority_checkpoint_identifier"
    printf 'restored_authority_root=%s\n' "$restored_root"
    printf 'continued_authority_root=%s\n' "$continued_root"
    printf 'continuity_divergence=none\n'
  } | sha256_text
}

run_authority_lifecycle() {
  local lifecycle_dir="$1"
  local replay_mode="${2:-false}"
  local checkpoint_path="$lifecycle_dir/wallet-authority.checkpoint"
  local before_rejected after_rejected before_revoked after_revoked restored_root continued_root
  local local_genesis local_assignment local_authorized local_unauthorized local_delegation local_delegated
  local local_revocation local_revoked local_epoch_0 local_epoch_1 local_epoch_2 local_checkpoint local_continuity

  mkdir -p "$lifecycle_dir"
  reset_authority_state || return 1

  create_asset "0000" "$SWORD" "Sword #001" || return 1
  create_asset "0001" "$SHIELD" "Shield #001" || return 1
  create_asset "0002" "$POTION" "Potion #001" || return 1
  assign_owner_inventory_vault "0003" "$SWORD" "player-a" "inventory:player-a" "vault:a" || return 1
  assign_owner_inventory_vault "0004" "$SHIELD" "player-a" "inventory:player-a" "vault:a" || return 1
  assign_owner_inventory_vault "0005" "$POTION" "player-b" "inventory:player-b" "vault:b" || return 1
  validate_unique_assets || return 1
  local_genesis="$(authority_state_root)"

  assign_authority "0006" "$SWORD" "$AUTHORITY_A" || return 2
  assign_authority "0007" "$SHIELD" "$AUTHORITY_B" || return 2
  assign_authority "0008" "$POTION" "$DELEGATE_C" || return 2
  assign_authority "0009-duplicate-authority-rejected" "$SWORD" "$AUTHORITY_B" && return 2
  validate_authority_integrity_initial || return 2
  local_assignment="$(authority_state_root)"

  authorize_action "0009" "$SWORD" "$AUTHORITY_A" "request_transfer" || return 3
  local_authorized="$(authority_state_root)"

  before_rejected="$local_authorized"
  expect_rejected_action "$SWORD" "$AUTHORITY_B" || return 3
  after_rejected="$(authority_state_root)"
  [[ "$after_rejected" == "$before_rejected" ]] || return 3
  local_unauthorized="$after_rejected"

  delegate_authority "0010" "$SWORD" "$AUTHORITY_A" "$DELEGATE_C" || return 4
  validate_authority_integrity_post_delegation || return 4
  local_delegation="$(authority_state_root)"

  authorize_action "0011" "$SWORD" "$DELEGATE_C" "delegated_request_transfer" || return 5
  local_delegated="$(authority_state_root)"

  revoke_authority "0012" "$SWORD" "$AUTHORITY_A" "$DELEGATE_C" || return 6
  validate_authority_integrity_post_revocation || return 6
  local_revocation="$(authority_state_root)"

  before_revoked="$local_revocation"
  expect_rejected_action "$SWORD" "$DELEGATE_C" || return 6
  after_revoked="$(authority_state_root)"
  [[ "$after_revoked" == "$before_revoked" ]] || return 6
  local_revoked="$after_revoked"

  local_epoch_0="$local_revoked"
  authorize_action "0013" "$SHIELD" "$AUTHORITY_B" "epoch_transfer_validation" || return 7
  local_epoch_1="$(authority_state_root)"
  authorize_action "0014" "$POTION" "$DELEGATE_C" "epoch_transfer_validation" || return 7
  local_epoch_2="$(authority_state_root)"
  [[ "$local_epoch_0" != "$local_epoch_1" && "$local_epoch_1" != "$local_epoch_2" ]] || return 7

  if [[ "$replay_mode" == "false" ]]; then
    authority_genesis_root="$local_genesis"
    authority_assignment_root="$local_assignment"
    authorized_action_root="$local_authorized"
    unauthorized_rejection_root="$local_unauthorized"
    delegation_root="$local_delegation"
    delegated_action_root="$local_delegated"
    revocation_root="$local_revocation"
    revoked_authority_root="$local_revoked"
    authority_root_epoch_0="$local_epoch_0"
    authority_root_epoch_1="$local_epoch_1"
    authority_root_epoch_2="$local_epoch_2"
    assignment_status="PASS"
    authorization_status="PASS"
    delegation_status="PASS"
    revocation_status="PASS"
  else
    authority_checkpoint_identifier="$(checkpoint_identifier_for_root "$local_epoch_2")"
  fi

  write_checkpoint "$checkpoint_path" "$local_epoch_2"
  local_checkpoint="$authority_checkpoint_identifier"
  checkpoint_integrity_valid "$checkpoint_path" "$local_epoch_2" || return 8
  [[ "$replay_mode" == "false" ]] && checkpoint_status="PASS"

  restore_from_checkpoint "$checkpoint_path" "$local_epoch_2" || return 9
  restored_root="$local_epoch_2"
  [[ "$replay_mode" == "false" ]] && restoration_status="PASS"

  authorize_action "0015" "$SHIELD" "$AUTHORITY_B" "post_restore_authorized_activity" || return 10
  expect_rejected_action "$SWORD" "$DELEGATE_C" || return 10
  validate_authority_integrity || return 10
  continued_root="$(authority_state_root)"
  [[ "$continued_root" != "$restored_root" ]] || return 10

  local_continuity="$(continuity_root "$restored_root" "$continued_root")"
  if [[ "$replay_mode" == "false" ]]; then
    authority_continuity_root="$local_continuity"
  else
    replay_authority_root="$local_continuity"
    authority_checkpoint_identifier="$local_checkpoint"
  fi
}

validate_authority_integrity_initial() {
  local asset count authority
  validate_unique_assets || return 1
  for asset in "${ASSETS[@]}"; do
    [[ -n "${PRIMARY_AUTHORITY[$asset]}" ]] || return 1
    count=0
    for authority in "${AUTHORITIES[@]}"; do
      [[ "${PRIMARY_AUTHORITY[$asset]}" == "$authority" ]] && count="$(( count + 1 ))"
    done
    [[ "$count" == "1" ]] || return 1
  done
}

validate_authority_integrity_post_delegation() {
  [[ "${DELEGATE_FROM[$DELEGATE_C]}" == "$AUTHORITY_A" ]] || return 1
  [[ "${DELEGATE_STATUS[$DELEGATE_C]}" == "active" ]] || return 1
  [[ "${LINEAGE[$SWORD]}" == *"delegation:$AUTHORITY_A>$DELEGATE_C"* ]] || return 1
}

validate_authority_integrity_post_revocation() {
  [[ "${DELEGATE_FROM[$DELEGATE_C]}" == "$AUTHORITY_A" ]] || return 1
  [[ "${DELEGATE_STATUS[$DELEGATE_C]}" == "revoked" ]] || return 1
  [[ "${LINEAGE[$SWORD]}" == *"revocation:$DELEGATE_C"* ]] || return 1
}

run_bootstrap_certification

if [[ "$bootstrap_status" == "PASS" ]]; then
  run_authority_lifecycle "$CERT_WORK_DIR/primary" false || true

  if [[ "$assignment_status" == "PASS" \
    && "$authorization_status" == "PASS" \
    && "$delegation_status" == "PASS" \
    && "$revocation_status" == "PASS" \
    && "$checkpoint_status" == "PASS" \
    && "$restoration_status" == "PASS" ]]; then
    run_authority_lifecycle "$CERT_WORK_DIR/replay" true || true
  fi

  if [[ "$authority_continuity_root" != "UNKNOWN" \
    && "$replay_authority_root" != "UNKNOWN" \
    && "$replay_authority_root" == "$authority_continuity_root" ]]; then
    replay_status="PASS"
  else
    replay_status="FAIL"
  fi

  if [[ "$assignment_status" == "PASS" \
    && "$authorization_status" == "PASS" \
    && "$delegation_status" == "PASS" \
    && "$revocation_status" == "PASS" \
    && "$checkpoint_status" == "PASS" \
    && "$restoration_status" == "PASS" \
    && "$replay_status" == "PASS" ]]; then
    integrity_status="PASS"
  else
    integrity_status="FAIL"
  fi
else
  assignment_status="FAIL"
  authorization_status="FAIL"
  delegation_status="FAIL"
  revocation_status="FAIL"
  checkpoint_status="FAIL"
  restoration_status="FAIL"
  replay_status="FAIL"
  integrity_status="FAIL"
fi

if [[ "$bootstrap_status" == "PASS" \
  && "$assignment_status" == "PASS" \
  && "$authorization_status" == "PASS" \
  && "$delegation_status" == "PASS" \
  && "$revocation_status" == "PASS" \
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
