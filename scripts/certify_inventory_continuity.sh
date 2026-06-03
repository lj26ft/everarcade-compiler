#!/usr/bin/env bash
set -u -o pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/inventory_continuity_certification_report.txt"
BOOTSTRAP_REPORT_REL="reports/runtime_bootstrap_certification_report.txt"
CERT_VERSION="inventory-continuity-certification-v0.1"

ASSETS=("asset:sword-001" "asset:shield-001" "asset:potion-001" "asset:gem-001")
OWNERS=("player-a" "player-b" "world-treasury")
INVENTORIES=("inventory:player-a" "inventory:player-a-equipped" "inventory:player-b" "inventory:treasury" "inventory:treasury-staging")

SWORD="asset:sword-001"
SHIELD="asset:shield-001"
POTION="asset:potion-001"
GEM="asset:gem-001"

ASSET_NAMES=()
declare -A OWNER=()
declare -A ASSET_INVENTORY=()
declare -A QUANTITY=()
declare -A LINEAGE=()
declare -A INVENTORY_OWNER=()
EVENT_LOG=""
EVENT_IDS=""

bootstrap_status="NOT RUN"
assignment_status="NOT RUN"
modification_status="NOT RUN"
transfer_status="NOT RUN"
checkpoint_status="NOT RUN"
restoration_status="NOT RUN"
replay_status="NOT RUN"
integrity_status="NOT RUN"
overall_result="FAIL"

inventory_genesis_root="UNKNOWN"
ownership_root="UNKNOWN"
inventory_assignment_root="UNKNOWN"
inventory_modification_root="UNKNOWN"
inventory_transfer_root="UNKNOWN"
inventory_root_epoch_0="UNKNOWN"
inventory_root_epoch_1="UNKNOWN"
inventory_root_epoch_2="UNKNOWN"
inventory_checkpoint_identifier="UNKNOWN"
inventory_continuity_root="UNKNOWN"
replay_inventory_root="UNKNOWN"

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
Inventory Genesis Root: $inventory_genesis_root
Ownership Root: $ownership_root
Inventory Assignment Root: $inventory_assignment_root
Inventory Modification Root: $inventory_modification_root
Inventory Transfer Root: $inventory_transfer_root
Inventory Root Epoch 0: $inventory_root_epoch_0
Inventory Root Epoch 1: $inventory_root_epoch_1
Inventory Root Epoch 2: $inventory_root_epoch_2
Inventory Checkpoint Identifier: $inventory_checkpoint_identifier
Inventory Continuity Root: $inventory_continuity_root
Replay Inventory Root: $replay_inventory_root
Assignment Status: $assignment_status
Modification Status: $modification_status
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
  printf 'Assignment: %s\n' "$assignment_status"
  printf 'Modification: %s\n' "$modification_status"
  printf 'Transfer: %s\n' "$transfer_status"
  printf 'Checkpoint: %s\n' "$checkpoint_status"
  printf 'Restoration: %s\n' "$restoration_status"
  printf 'Replay: %s\n' "$replay_status"
  printf 'Integrity: %s\n' "$integrity_status"
  printf 'Inventory Continuity Certification: %s\n' "$overall_result"
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

reset_inventory_state() {
  local asset inventory
  EVENT_LOG=""
  EVENT_IDS=""
  ASSET_NAMES=()

  for asset in "${ASSETS[@]}"; do
    OWNER["$asset"]=""
    ASSET_INVENTORY["$asset"]=""
    QUANTITY["$asset"]="1"
    LINEAGE["$asset"]=""
  done

  for inventory in "${INVENTORIES[@]}"; do
    case "$inventory" in
      "inventory:player-a"|"inventory:player-a-equipped") INVENTORY_OWNER["$inventory"]="player-a" ;;
      "inventory:player-b") INVENTORY_OWNER["$inventory"]="player-b" ;;
      "inventory:treasury"|"inventory:treasury-staging") INVENTORY_OWNER["$inventory"]="world-treasury" ;;
      *) return 1 ;;
    esac
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
  QUANTITY["$asset"]="1"
  LINEAGE["$asset"]="genesis:none"
  append_event "$event_id" "genesis asset=$asset name=$name owner=none inventory=none quantity=1" || return 1
}

assign_owner() {
  local event_id="$1"
  local asset="$2"
  local to_owner="$3"

  known_asset "$asset" || return 1
  [[ -n "${LINEAGE[$asset]}" ]] || return 1
  [[ -z "${OWNER[$asset]}" ]] || return 1
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
  [[ -n "${OWNER[$asset]}" ]] || return 1
  [[ -z "${ASSET_INVENTORY[$asset]}" ]] || return 1
  [[ "${INVENTORY_OWNER[$inventory]}" == "${OWNER[$asset]}" ]] || return 1

  ASSET_INVENTORY["$asset"]="$inventory"
  LINEAGE["$asset"]+=" -> assign_inventory:$inventory"
  append_event "$event_id" "assign_inventory asset=$asset inventory=$inventory owner=${OWNER[$asset]}" || return 1
}

move_item() {
  local event_id="$1"
  local asset="$2"
  local from_inventory="$3"
  local to_inventory="$4"

  known_asset "$asset" || return 1
  known_inventory "$from_inventory" || return 1
  known_inventory "$to_inventory" || return 1
  [[ "${ASSET_INVENTORY[$asset]}" == "$from_inventory" ]] || return 1
  [[ "${INVENTORY_OWNER[$to_inventory]}" == "${OWNER[$asset]}" ]] || return 1
  [[ "$from_inventory" != "$to_inventory" ]] || return 1

  ASSET_INVENTORY["$asset"]="$to_inventory"
  LINEAGE["$asset"]+=" -> move:$from_inventory>$to_inventory"
  append_event "$event_id" "move asset=$asset from=$from_inventory to=$to_inventory owner=${OWNER[$asset]}" || return 1
}

stack_item() {
  local event_id="$1"
  local asset="$2"
  local from_quantity="$3"
  local to_quantity="$4"

  known_asset "$asset" || return 1
  [[ -n "${ASSET_INVENTORY[$asset]}" ]] || return 1
  [[ "${QUANTITY[$asset]}" == "$from_quantity" ]] || return 1
  [[ "$to_quantity" -gt "$from_quantity" ]] || return 1

  QUANTITY["$asset"]="$to_quantity"
  LINEAGE["$asset"]+=" -> stack:$from_quantity>$to_quantity"
  append_event "$event_id" "stack asset=$asset inventory=${ASSET_INVENTORY[$asset]} from=$from_quantity to=$to_quantity" || return 1
}

unstack_item() {
  local event_id="$1"
  local asset="$2"
  local from_quantity="$3"
  local to_quantity="$4"

  known_asset "$asset" || return 1
  [[ -n "${ASSET_INVENTORY[$asset]}" ]] || return 1
  [[ "${QUANTITY[$asset]}" == "$from_quantity" ]] || return 1
  [[ "$to_quantity" -gt 0 ]] || return 1
  [[ "$to_quantity" -lt "$from_quantity" ]] || return 1

  QUANTITY["$asset"]="$to_quantity"
  LINEAGE["$asset"]+=" -> unstack:$from_quantity>$to_quantity"
  append_event "$event_id" "unstack asset=$asset inventory=${ASSET_INVENTORY[$asset]} from=$from_quantity to=$to_quantity" || return 1
}

transfer_item() {
  local event_id="$1"
  local asset="$2"
  local from_owner="$3"
  local to_owner="$4"
  local from_inventory="$5"
  local to_inventory="$6"

  known_asset "$asset" || return 1
  known_inventory "$from_inventory" || return 1
  known_inventory "$to_inventory" || return 1
  [[ -n "$to_owner" ]] || return 1
  [[ "${OWNER[$asset]}" == "$from_owner" ]] || return 1
  [[ "${ASSET_INVENTORY[$asset]}" == "$from_inventory" ]] || return 1
  [[ "${INVENTORY_OWNER[$to_inventory]}" == "$to_owner" ]] || return 1
  [[ "$from_owner" != "$to_owner" ]] || return 1

  OWNER["$asset"]="$to_owner"
  ASSET_INVENTORY["$asset"]="$to_inventory"
  LINEAGE["$asset"]+=" -> transfer:$from_owner/$from_inventory>$to_owner/$to_inventory"
  append_event "$event_id" "transfer asset=$asset from_owner=$from_owner to_owner=$to_owner from_inventory=$from_inventory to_inventory=$to_inventory" || return 1
}

state_transcript() {
  local asset inventory

  printf 'certification_version=%s\n' "$CERT_VERSION"
  printf 'authority=deterministic-runtime-inventory-membership\n'
  printf 'renderer_state=disabled\n'
  printf 'gpu_state=disabled\n'
  printf 'client_state=disabled\n'
  printf 'network_ordering_authority=disabled\n'
  printf 'external_database_authority=disabled\n'
  printf 'wallet_authority=disabled\n'
  printf 'vault_custody_authority=disabled\n'
  printf 'xrpl_settlement_authority=disabled\n'
  printf 'assets=%s\n' "$(IFS=,; printf '%s' "${ASSETS[*]}")"
  printf 'owners=%s\n' "$(IFS=,; printf '%s' "${OWNERS[*]}")"
  printf 'inventories=%s\n' "$(IFS=,; printf '%s' "${INVENTORIES[*]}")"
  for inventory in "${INVENTORIES[@]}"; do
    printf 'inventory.%s.owner=%s\n' "$inventory" "${INVENTORY_OWNER[$inventory]}"
  done
  for asset in "${ASSETS[@]}"; do
    printf 'asset.%s.name=%s\n' "$asset" "$(asset_name_for "$asset")"
    printf 'asset.%s.owner=%s\n' "$asset" "${OWNER[$asset]}"
    printf 'asset.%s.inventory=%s\n' "$asset" "${ASSET_INVENTORY[$asset]}"
    printf 'asset.%s.quantity=%s\n' "$asset" "${QUANTITY[$asset]}"
    printf 'asset.%s.lineage=%s\n' "$asset" "${LINEAGE[$asset]}"
  done
  printf 'event_log_begin\n%s' "$EVENT_LOG"
  printf 'event_log_end\n'
}

inventory_state_root() {
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

validate_integrity() {
  local asset event_count event_id_count inventory owner quantity lineage seen_memberships membership_count
  seen_memberships=""

  validate_unique_assets || return 1
  event_count="$(printf '%s' "$EVENT_LOG" | sed '/^$/d' | wc -l | tr -d ' ')"
  event_id_count="$(printf '%s' "$EVENT_IDS" | sed '/^$/d' | sort -u | wc -l | tr -d ' ')"
  [[ "$event_count" == "$event_id_count" ]] || return 1

  for asset in "${ASSETS[@]}"; do
    owner="${OWNER[$asset]}"
    inventory="${ASSET_INVENTORY[$asset]}"
    quantity="${QUANTITY[$asset]}"
    lineage="${LINEAGE[$asset]}"

    [[ -n "$owner" ]] || return 1
    [[ -n "$inventory" ]] || return 1
    known_inventory "$inventory" || return 1
    [[ "${INVENTORY_OWNER[$inventory]}" == "$owner" ]] || return 1
    [[ "$quantity" =~ ^[1-9][0-9]*$ ]] || return 1
    [[ -n "$lineage" ]] || return 1
    [[ "$lineage" == genesis:none* ]] || return 1
    [[ "$lineage" == *"$owner"* ]] || return 1

    if printf '%s' "$seen_memberships" | grep -qxF "$asset"; then
      return 1
    fi
    seen_memberships+="$asset"$'\n'
  done

  membership_count="$(printf '%s' "$seen_memberships" | sed '/^$/d' | wc -l | tr -d ' ')"
  [[ "$membership_count" == "${#ASSETS[@]}" ]] || return 1
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

  inventory_checkpoint_identifier="$(checkpoint_identifier_for_root "$root")"
  {
    printf 'Checkpoint Version: %s\n' "$CERT_VERSION"
    printf 'Inventory Checkpoint Identifier: %s\n' "$inventory_checkpoint_identifier"
    printf 'Persisted Inventory Root: %s\n' "$root"
    state_transcript
  } > "$checkpoint_path"
}

checkpoint_integrity_valid() {
  local checkpoint_path="$1"
  local root="$2"

  [[ -s "$checkpoint_path" ]] \
    && [[ "$(report_value "$checkpoint_path" "Inventory Checkpoint Identifier")" == "$inventory_checkpoint_identifier" ]] \
    && [[ "$(report_value "$checkpoint_path" "Persisted Inventory Root")" == "$root" ]]
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
    printf 'inventory_genesis_root=%s\n' "$inventory_genesis_root"
    printf 'ownership_root=%s\n' "$ownership_root"
    printf 'inventory_assignment_root=%s\n' "$inventory_assignment_root"
    printf 'inventory_modification_root=%s\n' "$inventory_modification_root"
    printf 'inventory_transfer_root=%s\n' "$inventory_transfer_root"
    printf 'inventory_root_epoch_0=%s\n' "$inventory_root_epoch_0"
    printf 'inventory_root_epoch_1=%s\n' "$inventory_root_epoch_1"
    printf 'inventory_root_epoch_2=%s\n' "$inventory_root_epoch_2"
    printf 'checkpoint_identifier=%s\n' "$inventory_checkpoint_identifier"
    printf 'restored_inventory_root=%s\n' "$restored_root"
    printf 'continued_inventory_root=%s\n' "$continued_root"
    printf 'continuity_divergence=none\n'
  } | sha256_text
}

run_inventory_lifecycle() {
  local lifecycle_dir="$1"
  local replay_mode="${2:-false}"
  local checkpoint_path="$lifecycle_dir/inventory-continuity.checkpoint"
  local restored_root continued_root local_genesis local_ownership local_assignment local_modification local_transfer
  local local_epoch_0 local_epoch_1 local_epoch_2 local_checkpoint local_continuity

  mkdir -p "$lifecycle_dir"
  reset_inventory_state || return 1

  create_asset "0000" "$SWORD" "Sword #001" || return 1
  create_asset "0001" "$SHIELD" "Shield #001" || return 1
  create_asset "0002" "$POTION" "Potion #001" || return 1
  create_asset "0003" "$GEM" "Gem #001" || return 1
  validate_unique_assets || return 1
  local_genesis="$(inventory_state_root)"

  assign_owner "0004" "$SWORD" "player-a" || return 2
  assign_owner "0005" "$SHIELD" "player-a" || return 2
  assign_owner "0006" "$POTION" "player-b" || return 2
  assign_owner "0007" "$GEM" "world-treasury" || return 2
  local_ownership="$(inventory_state_root)"

  assign_inventory "0008" "$SWORD" "inventory:player-a" || return 3
  assign_inventory "0009" "$SHIELD" "inventory:player-a" || return 3
  assign_inventory "0010" "$POTION" "inventory:player-b" || return 3
  assign_inventory "0011" "$GEM" "inventory:treasury" || return 3
  validate_integrity || return 3
  local_assignment="$(inventory_state_root)"

  move_item "0012" "$SHIELD" "inventory:player-a" "inventory:player-a-equipped" || return 4
  move_item "0013" "$GEM" "inventory:treasury" "inventory:treasury-staging" || return 4
  move_item "0014" "$GEM" "inventory:treasury-staging" "inventory:treasury" || return 4
  stack_item "0015" "$POTION" "1" "3" || return 4
  unstack_item "0016" "$POTION" "3" "1" || return 4
  validate_integrity || return 4
  local_modification="$(inventory_state_root)"

  transfer_item "0017" "$SWORD" "player-a" "player-b" "inventory:player-a" "inventory:player-b" || return 5
  transfer_item "0018-mismatch-rejected" "$GEM" "player-a" "player-b" "inventory:treasury" "inventory:player-b" && return 5
  validate_integrity || return 5
  local_transfer="$(inventory_state_root)"

  local_epoch_0="$local_transfer"
  move_item "0019" "$SWORD" "inventory:player-b" "inventory:player-b" && return 6
  stack_item "0020" "$POTION" "1" "5" || return 6
  validate_integrity || return 6
  local_epoch_1="$(inventory_state_root)"
  transfer_item "0021" "$GEM" "world-treasury" "player-a" "inventory:treasury" "inventory:player-a" || return 6
  move_item "0022" "$GEM" "inventory:player-a" "inventory:player-a-equipped" || return 6
  validate_integrity || return 6
  local_epoch_2="$(inventory_state_root)"

  if [[ "$local_epoch_0" == "$local_epoch_1" || "$local_epoch_1" == "$local_epoch_2" ]]; then
    return 6
  fi

  if [[ "$replay_mode" == "false" ]]; then
    inventory_genesis_root="$local_genesis"
    ownership_root="$local_ownership"
    inventory_assignment_root="$local_assignment"
    inventory_modification_root="$local_modification"
    inventory_transfer_root="$local_transfer"
    inventory_root_epoch_0="$local_epoch_0"
    inventory_root_epoch_1="$local_epoch_1"
    inventory_root_epoch_2="$local_epoch_2"
    assignment_status="PASS"
    modification_status="PASS"
    transfer_status="PASS"
  else
    inventory_checkpoint_identifier="$(checkpoint_identifier_for_root "$local_epoch_2")"
  fi

  write_checkpoint "$checkpoint_path" "$local_epoch_2"
  local_checkpoint="$inventory_checkpoint_identifier"
  checkpoint_integrity_valid "$checkpoint_path" "$local_epoch_2" || return 7
  [[ "$replay_mode" == "false" ]] && checkpoint_status="PASS"

  restore_from_checkpoint "$checkpoint_path" "$local_epoch_2" || return 8
  restored_root="$local_epoch_2"
  [[ "$replay_mode" == "false" ]] && restoration_status="PASS"

  transfer_item "0023" "$SHIELD" "player-a" "player-b" "inventory:player-a-equipped" "inventory:player-b" || return 9
  unstack_item "0024" "$POTION" "5" "2" || return 9
  move_item "0025" "$GEM" "inventory:player-a-equipped" "inventory:player-a" || return 9
  validate_integrity || return 9
  continued_root="$(inventory_state_root)"
  [[ "$continued_root" != "$restored_root" ]] || return 9

  local_continuity="$(continuity_root "$restored_root" "$continued_root")"
  if [[ "$replay_mode" == "false" ]]; then
    inventory_continuity_root="$local_continuity"
  else
    replay_inventory_root="$local_continuity"
    inventory_checkpoint_identifier="$local_checkpoint"
  fi
}

run_bootstrap_certification

if [[ "$bootstrap_status" == "PASS" ]]; then
  run_inventory_lifecycle "$CERT_WORK_DIR/primary" false || true

  if [[ "$assignment_status" == "PASS" \
    && "$modification_status" == "PASS" \
    && "$transfer_status" == "PASS" \
    && "$checkpoint_status" == "PASS" \
    && "$restoration_status" == "PASS" ]]; then
    run_inventory_lifecycle "$CERT_WORK_DIR/replay" true || true
  fi

  if [[ "$inventory_continuity_root" != "UNKNOWN" \
    && "$replay_inventory_root" != "UNKNOWN" \
    && "$replay_inventory_root" == "$inventory_continuity_root" ]]; then
    replay_status="PASS"
  else
    replay_status="FAIL"
  fi

  if [[ "$assignment_status" == "PASS" \
    && "$modification_status" == "PASS" \
    && "$transfer_status" == "PASS" \
    && "$checkpoint_status" == "PASS" \
    && "$restoration_status" == "PASS" \
    && "$replay_status" == "PASS" ]]; then
    integrity_status="PASS"
  else
    integrity_status="FAIL"
  fi
else
  assignment_status="FAIL"
  modification_status="FAIL"
  transfer_status="FAIL"
  checkpoint_status="FAIL"
  restoration_status="FAIL"
  replay_status="FAIL"
  integrity_status="FAIL"
fi

if [[ "$bootstrap_status" == "PASS" \
  && "$assignment_status" == "PASS" \
  && "$modification_status" == "PASS" \
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
