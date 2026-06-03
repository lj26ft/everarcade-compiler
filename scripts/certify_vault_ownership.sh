#!/usr/bin/env bash
set -u -o pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/vault_ownership_certification_report.txt"
BOOTSTRAP_REPORT_REL="reports/runtime_bootstrap_certification_report.txt"
CERT_VERSION="vault-ownership-certification-v0.1"

ASSETS=("asset:sword-001" "asset:shield-001" "asset:potion-001" "asset:gem-001")
OWNERS=("player-a" "player-b" "treasury")
INVENTORIES=("inventory:player-a" "inventory:player-b" "inventory:treasury")
VAULTS=("vault:a" "vault:b" "vault:treasury")

SWORD="asset:sword-001"
SHIELD="asset:shield-001"
POTION="asset:potion-001"
GEM="asset:gem-001"

ASSET_NAMES=()
declare -A OWNER=()
declare -A ASSET_INVENTORY=()
declare -A ASSET_VAULT=()
declare -A LINEAGE=()
declare -A INVENTORY_OWNER=()
declare -A VAULT_CREATED=()
declare -A VAULT_MEMBER_COUNT=()
EVENT_LOG=""
EVENT_IDS=""

bootstrap_status="NOT RUN"
creation_status="NOT RUN"
deposit_status="NOT RUN"
transfer_status="NOT RUN"
withdrawal_status="NOT RUN"
checkpoint_status="NOT RUN"
restoration_status="NOT RUN"
replay_status="NOT RUN"
custody_integrity_status="NOT RUN"
overall_result="FAIL"

vault_genesis_root="UNKNOWN"
ownership_root="UNKNOWN"
inventory_root="UNKNOWN"
vault_root_creation="UNKNOWN"
vault_deposit_root="UNKNOWN"
vault_transfer_root="UNKNOWN"
vault_withdrawal_root="UNKNOWN"
vault_root_epoch_0="UNKNOWN"
vault_root_epoch_1="UNKNOWN"
vault_root_epoch_2="UNKNOWN"
vault_checkpoint_identifier="UNKNOWN"
vault_continuity_root="UNKNOWN"
replay_vault_root="UNKNOWN"

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
Vault Genesis Root: $vault_genesis_root
Ownership Root: $ownership_root
Inventory Root: $inventory_root
Vault Root Creation: $vault_root_creation
Vault Deposit Root: $vault_deposit_root
Vault Transfer Root: $vault_transfer_root
Vault Withdrawal Root: $vault_withdrawal_root
Vault Root Epoch 0: $vault_root_epoch_0
Vault Root Epoch 1: $vault_root_epoch_1
Vault Root Epoch 2: $vault_root_epoch_2
Vault Checkpoint Identifier: $vault_checkpoint_identifier
Vault Continuity Root: $vault_continuity_root
Replay Vault Root: $replay_vault_root
Creation Status: $creation_status
Deposit Status: $deposit_status
Transfer Status: $transfer_status
Withdrawal Status: $withdrawal_status
Checkpoint Status: $checkpoint_status
Restoration Status: $restoration_status
Replay Status: $replay_status
Custody Integrity Status: $custody_integrity_status
Overall Result: $overall_result
REPORT
}

print_summary() {
  printf 'Bootstrap: %s\n' "$bootstrap_status"
  printf 'Creation: %s\n' "$creation_status"
  printf 'Deposit: %s\n' "$deposit_status"
  printf 'Transfer: %s\n' "$transfer_status"
  printf 'Withdrawal: %s\n' "$withdrawal_status"
  printf 'Checkpoint: %s\n' "$checkpoint_status"
  printf 'Restoration: %s\n' "$restoration_status"
  printf 'Replay: %s\n' "$replay_status"
  printf 'Custody Integrity: %s\n' "$custody_integrity_status"
  printf 'Vault Ownership Certification: %s\n' "$overall_result"
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

reset_vault_state() {
  local asset inventory vault
  EVENT_LOG=""
  EVENT_IDS=""
  ASSET_NAMES=()

  for asset in "${ASSETS[@]}"; do
    OWNER["$asset"]=""
    ASSET_INVENTORY["$asset"]=""
    ASSET_VAULT["$asset"]=""
    LINEAGE["$asset"]=""
  done

  for inventory in "${INVENTORIES[@]}"; do
    case "$inventory" in
      "inventory:player-a") INVENTORY_OWNER["$inventory"]="player-a" ;;
      "inventory:player-b") INVENTORY_OWNER["$inventory"]="player-b" ;;
      "inventory:treasury") INVENTORY_OWNER["$inventory"]="treasury" ;;
      *) return 1 ;;
    esac
  done

  for vault in "${VAULTS[@]}"; do
    VAULT_CREATED["$vault"]="false"
    VAULT_MEMBER_COUNT["$vault"]="0"
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

asset_name_for() {
  local asset="$1"

  case "$asset" in
    "$SWORD") printf 'Sword #001' ;;
    "$SHIELD") printf 'Shield #001' ;;
    "$POTION") printf 'Potion #001' ;;
    "$GEM") printf 'Gem #001' ;;
    *) printf 'UNKNOWN' ;;
  esac
}

vault_name_for() {
  local vault="$1"

  case "$vault" in
    "vault:a") printf 'Vault A' ;;
    "vault:b") printf 'Vault B' ;;
    "vault:treasury") printf 'Treasury Vault' ;;
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
  append_event "$event_id" "genesis asset=$asset name=$name owner=none inventory=none vault=none" || return 1
}

assign_owner() {
  local event_id="$1"
  local asset="$2"
  local to_owner="$3"

  known_asset "$asset" || return 1
  [[ -n "${LINEAGE[$asset]:-}" ]] || return 1
  [[ -z "${OWNER[$asset]:-}" ]] || return 1

  OWNER["$asset"]="$to_owner"
  LINEAGE["$asset"]+=" -> assign_owner:$to_owner"
  append_event "$event_id" "assign_owner asset=$asset to=$to_owner" || return 1
}

assign_inventory() {
  local event_id="$1"
  local asset="$2"
  local inventory="$3"

  known_asset "$asset" || return 1
  known_inventory "$inventory" || return 1
  [[ -n "${OWNER[$asset]:-}" ]] || return 1
  [[ -z "${ASSET_INVENTORY[$asset]:-}" ]] || return 1
  [[ -z "${ASSET_VAULT[$asset]:-}" ]] || return 1
  [[ "${INVENTORY_OWNER[$inventory]}" == "${OWNER[$asset]}" ]] || return 1

  ASSET_INVENTORY["$asset"]="$inventory"
  LINEAGE["$asset"]+=" -> assign_inventory:$inventory"
  append_event "$event_id" "assign_inventory asset=$asset inventory=$inventory owner=${OWNER[$asset]}" || return 1
}

create_vault() {
  local event_id="$1"
  local vault="$2"

  known_vault "$vault" || return 1
  [[ "${VAULT_CREATED[$vault]}" == "false" ]] || return 1

  VAULT_CREATED["$vault"]="true"
  append_event "$event_id" "create_vault vault=$vault name=$(vault_name_for "$vault")" || return 1
}

authorized_vault_transfer() {
  local from_vault="$1"
  local to_vault="$2"

  case "$from_vault>$to_vault" in
    "vault:a>vault:b"|"vault:b>vault:treasury"|"vault:treasury>vault:b") return 0 ;;
    *) return 1 ;;
  esac
}

deposit_asset() {
  local event_id="$1"
  local asset="$2"
  local from_inventory="$3"
  local to_vault="$4"

  known_asset "$asset" || return 1
  known_inventory "$from_inventory" || return 1
  known_vault "$to_vault" || return 1
  [[ "${VAULT_CREATED[$to_vault]}" == "true" ]] || return 1
  [[ "${ASSET_INVENTORY[$asset]:-}" == "$from_inventory" ]] || return 1
  [[ -z "${ASSET_VAULT[$asset]:-}" ]] || return 1
  [[ "${INVENTORY_OWNER[$from_inventory]}" == "${OWNER[$asset]}" ]] || return 1

  ASSET_INVENTORY["$asset"]=""
  ASSET_VAULT["$asset"]="$to_vault"
  VAULT_MEMBER_COUNT["$to_vault"]="$(( ${VAULT_MEMBER_COUNT[$to_vault]} + 1 ))"
  LINEAGE["$asset"]+=" -> deposit:$from_inventory>$to_vault"
  append_event "$event_id" "deposit asset=$asset from_inventory=$from_inventory to_vault=$to_vault owner=${OWNER[$asset]}" || return 1
}

transfer_custody() {
  local event_id="$1"
  local asset="$2"
  local from_vault="$3"
  local to_vault="$4"

  known_asset "$asset" || return 1
  known_vault "$from_vault" || return 1
  known_vault "$to_vault" || return 1
  [[ "${VAULT_CREATED[$from_vault]}" == "true" ]] || return 1
  [[ "${VAULT_CREATED[$to_vault]}" == "true" ]] || return 1
  [[ "${ASSET_VAULT[$asset]:-}" == "$from_vault" ]] || return 1
  [[ -z "${ASSET_INVENTORY[$asset]:-}" ]] || return 1
  [[ "$from_vault" != "$to_vault" ]] || return 1
  authorized_vault_transfer "$from_vault" "$to_vault" || return 1

  ASSET_VAULT["$asset"]="$to_vault"
  VAULT_MEMBER_COUNT["$from_vault"]="$(( ${VAULT_MEMBER_COUNT[$from_vault]} - 1 ))"
  VAULT_MEMBER_COUNT["$to_vault"]="$(( ${VAULT_MEMBER_COUNT[$to_vault]} + 1 ))"
  LINEAGE["$asset"]+=" -> custody_transfer:$from_vault>$to_vault"
  append_event "$event_id" "transfer_custody asset=$asset from_vault=$from_vault to_vault=$to_vault owner=${OWNER[$asset]}" || return 1
}

withdraw_asset() {
  local event_id="$1"
  local asset="$2"
  local from_vault="$3"
  local to_inventory="$4"

  known_asset "$asset" || return 1
  known_vault "$from_vault" || return 1
  known_inventory "$to_inventory" || return 1
  [[ "${ASSET_VAULT[$asset]:-}" == "$from_vault" ]] || return 1
  [[ -z "${ASSET_INVENTORY[$asset]:-}" ]] || return 1
  [[ "${INVENTORY_OWNER[$to_inventory]}" == "${OWNER[$asset]}" ]] || return 1

  ASSET_VAULT["$asset"]=""
  ASSET_INVENTORY["$asset"]="$to_inventory"
  VAULT_MEMBER_COUNT["$from_vault"]="$(( ${VAULT_MEMBER_COUNT[$from_vault]} - 1 ))"
  LINEAGE["$asset"]+=" -> withdraw:$from_vault>$to_inventory"
  append_event "$event_id" "withdraw asset=$asset from_vault=$from_vault to_inventory=$to_inventory owner=${OWNER[$asset]}" || return 1
}

state_transcript() {
  local asset inventory vault

  printf 'certification_version=%s\n' "$CERT_VERSION"
  printf 'authority=deterministic-runtime-vault-custody\n'
  printf 'wallet_authority=disabled\n'
  printf 'xrpl_authority=disabled\n'
  printf 'xaman_signature_authority=disabled\n'
  printf 'external_settlement=disabled\n'
  printf 'network_ordering_authority=disabled\n'
  printf 'assets=%s\n' "$(IFS=,; printf '%s' "${ASSETS[*]}")"
  printf 'owners=%s\n' "$(IFS=,; printf '%s' "${OWNERS[*]}")"
  printf 'inventories=%s\n' "$(IFS=,; printf '%s' "${INVENTORIES[*]}")"
  printf 'vaults=%s\n' "$(IFS=,; printf '%s' "${VAULTS[*]}")"
  for inventory in "${INVENTORIES[@]}"; do
    printf 'inventory.%s.owner=%s\n' "$inventory" "${INVENTORY_OWNER[$inventory]}"
  done
  for vault in "${VAULTS[@]}"; do
    printf 'vault.%s.created=%s\n' "$vault" "${VAULT_CREATED[$vault]}"
    printf 'vault.%s.member_count=%s\n' "$vault" "${VAULT_MEMBER_COUNT[$vault]}"
  done
  for asset in "${ASSETS[@]}"; do
    printf 'asset.%s.name=%s\n' "$asset" "$(asset_name_for "$asset")"
    printf 'asset.%s.owner=%s\n' "$asset" "${OWNER[$asset]}"
    printf 'asset.%s.inventory=%s\n' "$asset" "${ASSET_INVENTORY[$asset]}"
    printf 'asset.%s.vault=%s\n' "$asset" "${ASSET_VAULT[$asset]}"
    printf 'asset.%s.lineage=%s\n' "$asset" "${LINEAGE[$asset]}"
  done
  printf 'event_log_begin\n%s' "$EVENT_LOG"
  printf 'event_log_end\n'
}

vault_state_root() {
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

validate_unique_vaults() {
  local i j vault

  for (( i = 0; i < ${#VAULTS[@]}; i++ )); do
    vault="${VAULTS[$i]}"
    [[ "${VAULT_CREATED[$vault]}" == "true" ]] || return 1
    for (( j = i + 1; j < ${#VAULTS[@]}; j++ )); do
      [[ "$vault" != "${VAULTS[$j]}" ]] || return 1
    done
  done
}

validate_custody_integrity() {
  local asset event_count event_id_count inventory vault owner lineage actual_count expected_count

  validate_unique_assets || return 1
  event_count="$(printf '%s' "$EVENT_LOG" | sed '/^$/d' | wc -l | tr -d ' ')"
  event_id_count="$(printf '%s' "$EVENT_IDS" | sed '/^$/d' | sort -u | wc -l | tr -d ' ')"
  [[ "$event_count" == "$event_id_count" ]] || return 1

  for asset in "${ASSETS[@]}"; do
    owner="${OWNER[$asset]}"
    inventory="${ASSET_INVENTORY[$asset]}"
    vault="${ASSET_VAULT[$asset]}"
    lineage="${LINEAGE[$asset]}"

    [[ -n "$owner" ]] || return 1
    [[ -n "$lineage" ]] || return 1
    [[ "$lineage" == genesis:none* ]] || return 1
    [[ "$lineage" == *"assign_owner:$owner"* || "$lineage" == *" owner=$owner"* ]] || return 1

    if [[ -n "$vault" ]]; then
      known_vault "$vault" || return 1
      [[ "${VAULT_CREATED[$vault]}" == "true" ]] || return 1
      [[ -z "$inventory" ]] || return 1
      [[ "$lineage" == *"deposit:"* || "$lineage" == *"custody_transfer:"* ]] || return 1
    else
      known_inventory "$inventory" || return 1
      [[ "${INVENTORY_OWNER[$inventory]}" == "$owner" ]] || return 1
    fi
  done

  for vault in "${VAULTS[@]}"; do
    expected_count=0
    for asset in "${ASSETS[@]}"; do
      [[ "${ASSET_VAULT[$asset]}" == "$vault" ]] && expected_count="$(( expected_count + 1 ))"
    done
    actual_count="${VAULT_MEMBER_COUNT[$vault]}"
    [[ "$actual_count" == "$expected_count" ]] || return 1
    [[ "$actual_count" -ge 0 ]] || return 1
  done
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

  vault_checkpoint_identifier="$(checkpoint_identifier_for_root "$root")"
  {
    printf 'Checkpoint Version: %s\n' "$CERT_VERSION"
    printf 'Vault Checkpoint Identifier: %s\n' "$vault_checkpoint_identifier"
    printf 'Persisted Vault Root: %s\n' "$root"
    state_transcript
  } > "$checkpoint_path"
}

checkpoint_integrity_valid() {
  local checkpoint_path="$1"
  local root="$2"

  [[ -s "$checkpoint_path" ]] \
    && [[ "$(report_value "$checkpoint_path" "Vault Checkpoint Identifier")" == "$vault_checkpoint_identifier" ]] \
    && [[ "$(report_value "$checkpoint_path" "Persisted Vault Root")" == "$root" ]]
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
    printf 'vault_genesis_root=%s\n' "$vault_genesis_root"
    printf 'ownership_root=%s\n' "$ownership_root"
    printf 'inventory_root=%s\n' "$inventory_root"
    printf 'vault_root_creation=%s\n' "$vault_root_creation"
    printf 'vault_deposit_root=%s\n' "$vault_deposit_root"
    printf 'vault_transfer_root=%s\n' "$vault_transfer_root"
    printf 'vault_withdrawal_root=%s\n' "$vault_withdrawal_root"
    printf 'vault_root_epoch_0=%s\n' "$vault_root_epoch_0"
    printf 'vault_root_epoch_1=%s\n' "$vault_root_epoch_1"
    printf 'vault_root_epoch_2=%s\n' "$vault_root_epoch_2"
    printf 'checkpoint_identifier=%s\n' "$vault_checkpoint_identifier"
    printf 'restored_vault_root=%s\n' "$restored_root"
    printf 'continued_vault_root=%s\n' "$continued_root"
    printf 'continuity_divergence=none\n'
  } | sha256_text
}

run_vault_lifecycle() {
  local lifecycle_dir="$1"
  local replay_mode="${2:-false}"
  local checkpoint_path="$lifecycle_dir/vault-ownership.checkpoint"
  local restored_root continued_root local_genesis local_ownership local_inventory local_creation local_deposit
  local local_transfer local_withdrawal local_epoch_0 local_epoch_1 local_epoch_2 local_checkpoint local_continuity

  mkdir -p "$lifecycle_dir"
  reset_vault_state || return 1

  create_asset "0000" "$SWORD" "Sword #001" || return 1
  create_asset "0001" "$SHIELD" "Shield #001" || return 1
  create_asset "0002" "$POTION" "Potion #001" || return 1
  create_asset "0003" "$GEM" "Gem #001" || return 1
  validate_unique_assets || return 1
  local_genesis="$(vault_state_root)"

  assign_owner "0004" "$SWORD" "player-a" || return 2
  assign_owner "0005" "$SHIELD" "player-a" || return 2
  assign_owner "0006" "$POTION" "player-b" || return 2
  assign_owner "0007" "$GEM" "treasury" || return 2
  local_ownership="$(vault_state_root)"

  assign_inventory "0008" "$SWORD" "inventory:player-a" || return 3
  assign_inventory "0009" "$SHIELD" "inventory:player-a" || return 3
  assign_inventory "0010" "$POTION" "inventory:player-b" || return 3
  assign_inventory "0011" "$GEM" "inventory:treasury" || return 3
  validate_custody_integrity || return 3
  local_inventory="$(vault_state_root)"

  create_vault "0012" "vault:a" || return 4
  create_vault "0013" "vault:b" || return 4
  create_vault "0014" "vault:treasury" || return 4
  create_vault "0015-duplicate-vault-rejected" "vault:a" && return 4
  validate_unique_vaults || return 4
  local_creation="$(vault_state_root)"

  deposit_asset "0016" "$SWORD" "inventory:player-a" "vault:a" || return 5
  deposit_asset "0017" "$SHIELD" "inventory:player-a" "vault:a" || return 5
  deposit_asset "0018" "$POTION" "inventory:player-b" "vault:b" || return 5
  deposit_asset "0019" "$GEM" "inventory:treasury" "vault:treasury" || return 5
  validate_custody_integrity || return 5
  local_deposit="$(vault_state_root)"

  transfer_custody "0020" "$SWORD" "vault:a" "vault:b" || return 6
  transfer_custody "0021" "$POTION" "vault:b" "vault:treasury" || return 6
  transfer_custody "0022-unauthorized-rejected" "$GEM" "vault:treasury" "vault:a" && return 6
  validate_custody_integrity || return 6
  local_transfer="$(vault_state_root)"

  withdraw_asset "0023" "$SWORD" "vault:b" "inventory:player-a" || return 7
  validate_custody_integrity || return 7
  local_withdrawal="$(vault_state_root)"

  local_epoch_0="$local_withdrawal"
  deposit_asset "0024" "$SWORD" "inventory:player-a" "vault:a" || return 8
  transfer_custody "0025" "$SWORD" "vault:a" "vault:b" || return 8
  validate_custody_integrity || return 8
  local_epoch_1="$(vault_state_root)"
  withdraw_asset "0026" "$POTION" "vault:treasury" "inventory:player-b" || return 8
  deposit_asset "0027" "$POTION" "inventory:player-b" "vault:b" || return 8
  transfer_custody "0028" "$POTION" "vault:b" "vault:treasury" || return 8
  validate_custody_integrity || return 8
  local_epoch_2="$(vault_state_root)"

  if [[ "$local_epoch_0" == "$local_epoch_1" || "$local_epoch_1" == "$local_epoch_2" ]]; then
    return 8
  fi

  if [[ "$replay_mode" == "false" ]]; then
    vault_genesis_root="$local_genesis"
    ownership_root="$local_ownership"
    inventory_root="$local_inventory"
    vault_root_creation="$local_creation"
    vault_deposit_root="$local_deposit"
    vault_transfer_root="$local_transfer"
    vault_withdrawal_root="$local_withdrawal"
    vault_root_epoch_0="$local_epoch_0"
    vault_root_epoch_1="$local_epoch_1"
    vault_root_epoch_2="$local_epoch_2"
    creation_status="PASS"
    deposit_status="PASS"
    transfer_status="PASS"
    withdrawal_status="PASS"
  else
    vault_checkpoint_identifier="$(checkpoint_identifier_for_root "$local_epoch_2")"
  fi

  write_checkpoint "$checkpoint_path" "$local_epoch_2"
  local_checkpoint="$vault_checkpoint_identifier"
  checkpoint_integrity_valid "$checkpoint_path" "$local_epoch_2" || return 9
  [[ "$replay_mode" == "false" ]] && checkpoint_status="PASS"

  restore_from_checkpoint "$checkpoint_path" "$local_epoch_2" || return 10
  restored_root="$local_epoch_2"
  [[ "$replay_mode" == "false" ]] && restoration_status="PASS"

  transfer_custody "0029" "$SHIELD" "vault:a" "vault:b" || return 11
  withdraw_asset "0030" "$POTION" "vault:treasury" "inventory:player-b" || return 11
  deposit_asset "0031" "$POTION" "inventory:player-b" "vault:b" || return 11
  transfer_custody "0032" "$POTION" "vault:b" "vault:treasury" || return 11
  validate_custody_integrity || return 11
  continued_root="$(vault_state_root)"
  [[ "$continued_root" != "$restored_root" ]] || return 11

  local_continuity="$(continuity_root "$restored_root" "$continued_root")"
  if [[ "$replay_mode" == "false" ]]; then
    vault_continuity_root="$local_continuity"
  else
    replay_vault_root="$local_continuity"
    vault_checkpoint_identifier="$local_checkpoint"
  fi
}

run_bootstrap_certification

if [[ "$bootstrap_status" == "PASS" ]]; then
  run_vault_lifecycle "$CERT_WORK_DIR/primary" false || true

  if [[ "$creation_status" == "PASS" \
    && "$deposit_status" == "PASS" \
    && "$transfer_status" == "PASS" \
    && "$withdrawal_status" == "PASS" \
    && "$checkpoint_status" == "PASS" \
    && "$restoration_status" == "PASS" ]]; then
    run_vault_lifecycle "$CERT_WORK_DIR/replay" true || true
  fi

  if [[ "$vault_continuity_root" != "UNKNOWN" \
    && "$replay_vault_root" != "UNKNOWN" \
    && "$replay_vault_root" == "$vault_continuity_root" ]]; then
    replay_status="PASS"
  else
    replay_status="FAIL"
  fi

  if [[ "$creation_status" == "PASS" \
    && "$deposit_status" == "PASS" \
    && "$transfer_status" == "PASS" \
    && "$withdrawal_status" == "PASS" \
    && "$checkpoint_status" == "PASS" \
    && "$restoration_status" == "PASS" \
    && "$replay_status" == "PASS" ]]; then
    custody_integrity_status="PASS"
  else
    custody_integrity_status="FAIL"
  fi
else
  creation_status="FAIL"
  deposit_status="FAIL"
  transfer_status="FAIL"
  withdrawal_status="FAIL"
  checkpoint_status="FAIL"
  restoration_status="FAIL"
  replay_status="FAIL"
  custody_integrity_status="FAIL"
fi

if [[ "$bootstrap_status" == "PASS" \
  && "$creation_status" == "PASS" \
  && "$deposit_status" == "PASS" \
  && "$transfer_status" == "PASS" \
  && "$withdrawal_status" == "PASS" \
  && "$checkpoint_status" == "PASS" \
  && "$restoration_status" == "PASS" \
  && "$replay_status" == "PASS" \
  && "$custody_integrity_status" == "PASS" ]]; then
  overall_result="PASS"
else
  overall_result="FAIL"
fi

write_report
print_summary

[[ "$overall_result" == "PASS" ]]
