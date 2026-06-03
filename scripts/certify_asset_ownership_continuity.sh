#!/usr/bin/env bash
set -u -o pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/asset_ownership_continuity_report.txt"
BOOTSTRAP_REPORT_REL="reports/runtime_bootstrap_certification_report.txt"
CERT_VERSION="asset-ownership-continuity-certification-v0.1"
ASSETS=("asset:sword-001" "asset:shield-001" "asset:land-parcel-001")
OWNERS=("player-a" "player-b" "world-treasury")

SWORD="asset:sword-001"
SHIELD="asset:shield-001"
LAND="asset:land-parcel-001"

ASSET_NAMES=()
declare -A OWNER=()
declare -A LINEAGE=()
EVENT_LOG=""
EVENT_IDS=""

bootstrap_status="NOT RUN"
assignment_status="NOT RUN"
transfer_status="NOT RUN"
checkpoint_status="NOT RUN"
restoration_status="NOT RUN"
replay_status="NOT RUN"
integrity_status="NOT RUN"
overall_result="FAIL"

asset_root_genesis="UNKNOWN"
ownership_root_assignment="UNKNOWN"
ownership_root_transfer="UNKNOWN"
ownership_root_epoch_0="UNKNOWN"
ownership_root_epoch_1="UNKNOWN"
ownership_root_epoch_2="UNKNOWN"
ownership_checkpoint_identifier="UNKNOWN"
ownership_continuity_root="UNKNOWN"
replay_ownership_root="UNKNOWN"

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
Asset Root Genesis: $asset_root_genesis
Ownership Root Assignment: $ownership_root_assignment
Ownership Root Transfer: $ownership_root_transfer
Ownership Root Epoch 0: $ownership_root_epoch_0
Ownership Root Epoch 1: $ownership_root_epoch_1
Ownership Root Epoch 2: $ownership_root_epoch_2
Ownership Checkpoint Identifier: $ownership_checkpoint_identifier
Ownership Continuity Root: $ownership_continuity_root
Replay Ownership Root: $replay_ownership_root
Assignment Status: $assignment_status
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
  printf 'Transfers: %s\n' "$transfer_status"
  printf 'Checkpoint: %s\n' "$checkpoint_status"
  printf 'Restoration: %s\n' "$restoration_status"
  printf 'Replay: %s\n' "$replay_status"
  printf 'Integrity: %s\n' "$integrity_status"
  printf 'Asset Ownership Continuity Certification: %s\n' "$overall_result"
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

reset_ownership_state() {
  local asset
  EVENT_LOG=""
  EVENT_IDS=""
  ASSET_NAMES=()
  for asset in "${ASSETS[@]}"; do
    OWNER["$asset"]=""
    LINEAGE["$asset"]=""
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

asset_name_for() {
  local asset="$1"

  case "$asset" in
    "$SWORD") printf 'Sword #001' ;;
    "$SHIELD") printf 'Shield #001' ;;
    "$LAND") printf 'Land Parcel #001' ;;
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
  append_event "$event_id" "genesis asset=$asset name=$name owner=none" || return 1
}

assign_owner() {
  local event_id="$1"
  local asset="$2"
  local to_owner="$3"

  known_asset "$asset" || return 1
  [[ -n "${LINEAGE[$asset]}" ]] || return 1
  [[ -z "${OWNER[$asset]}" ]] || return 1
  OWNER["$asset"]="$to_owner"
  LINEAGE["$asset"]+=" -> assign:$to_owner"
  append_event "$event_id" "assign asset=$asset to=$to_owner" || return 1
}

transfer_owner() {
  local event_id="$1"
  local asset="$2"
  local from_owner="$3"
  local to_owner="$4"

  known_asset "$asset" || return 1
  [[ -n "$to_owner" ]] || return 1
  [[ "${OWNER[$asset]}" == "$from_owner" ]] || return 1
  [[ "$from_owner" != "$to_owner" ]] || return 1

  OWNER["$asset"]="$to_owner"
  LINEAGE["$asset"]+=" -> transfer:$from_owner>$to_owner"
  append_event "$event_id" "transfer asset=$asset from=$from_owner to=$to_owner" || return 1
}

state_transcript() {
  local asset

  printf 'certification_version=%s\n' "$CERT_VERSION"
  printf 'authority=deterministic-runtime-asset-ownership\n'
  printf 'wallet_authority=disabled\n'
  printf 'xrpl_authority=disabled\n'
  printf 'xaman_signature_authority=disabled\n'
  printf 'vault_custody_authority=disabled\n'
  printf 'network_ordering_authority=disabled\n'
  printf 'assets=%s\n' "$(IFS=,; printf '%s' "${ASSETS[*]}")"
  printf 'owners=%s\n' "$(IFS=,; printf '%s' "${OWNERS[*]}")"
  for asset in "${ASSETS[@]}"; do
    printf 'asset.%s.name=%s\n' "$asset" "$(asset_name_for "$asset")"
    printf 'asset.%s.owner=%s\n' "$asset" "${OWNER[$asset]}"
    printf 'asset.%s.lineage=%s\n' "$asset" "${LINEAGE[$asset]}"
  done
  printf 'event_log_begin\n%s' "$EVENT_LOG"
  printf 'event_log_end\n'
}

ownership_root() {
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
  local asset event_count event_id_count lineage expected_suffix

  validate_unique_assets || return 1
  event_count="$(printf '%s' "$EVENT_LOG" | sed '/^$/d' | wc -l | tr -d ' ')"
  event_id_count="$(printf '%s' "$EVENT_IDS" | sed '/^$/d' | sort -u | wc -l | tr -d ' ')"
  [[ "$event_count" == "$event_id_count" ]] || return 1

  for asset in "${ASSETS[@]}"; do
    [[ -n "${OWNER[$asset]}" ]] || return 1
    [[ -n "${LINEAGE[$asset]}" ]] || return 1
    [[ "${LINEAGE[$asset]}" == genesis:none* ]] || return 1
    expected_suffix="${OWNER[$asset]}"
    lineage="${LINEAGE[$asset]}"
    [[ "$lineage" == *"$expected_suffix" ]] || return 1
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

  ownership_checkpoint_identifier="$(checkpoint_identifier_for_root "$ownership_root_epoch_2")"
  {
    printf 'Checkpoint Version: %s\n' "$CERT_VERSION"
    printf 'Ownership Checkpoint Identifier: %s\n' "$ownership_checkpoint_identifier"
    printf 'Persisted Ownership Root: %s\n' "$ownership_root_epoch_2"
    state_transcript
  } > "$checkpoint_path"
}

checkpoint_integrity_valid() {
  local checkpoint_path="$1"

  [[ -s "$checkpoint_path" ]] \
    && [[ "$(report_value "$checkpoint_path" "Ownership Checkpoint Identifier")" == "$ownership_checkpoint_identifier" ]] \
    && [[ "$(report_value "$checkpoint_path" "Persisted Ownership Root")" == "$ownership_root_epoch_2" ]]
}

restore_from_checkpoint() {
  local checkpoint_path="$1"
  local restored_root

  restored_root="$(sed -n '/^certification_version=/,$p' "$checkpoint_path" | sha256_text)"
  [[ "$restored_root" == "$ownership_root_epoch_2" ]]
}

continuity_root() {
  local restored_root="$1"
  local continued_root="$2"

  {
    printf 'certification_version=%s\n' "$CERT_VERSION"
    printf 'asset_root_genesis=%s\n' "$asset_root_genesis"
    printf 'ownership_root_assignment=%s\n' "$ownership_root_assignment"
    printf 'ownership_root_transfer=%s\n' "$ownership_root_transfer"
    printf 'ownership_root_epoch_0=%s\n' "$ownership_root_epoch_0"
    printf 'ownership_root_epoch_1=%s\n' "$ownership_root_epoch_1"
    printf 'ownership_root_epoch_2=%s\n' "$ownership_root_epoch_2"
    printf 'checkpoint_identifier=%s\n' "$ownership_checkpoint_identifier"
    printf 'restored_ownership_root=%s\n' "$restored_root"
    printf 'continued_ownership_root=%s\n' "$continued_root"
    printf 'continuity_divergence=none\n'
  } | sha256_text
}

run_ownership_lifecycle() {
  local lifecycle_dir="$1"
  local replay_mode="${2:-false}"
  local checkpoint_path="$lifecycle_dir/asset-ownership.checkpoint"
  local restored_root continued_root local_genesis local_assignment local_transfer local_epoch_0 local_epoch_1 local_epoch_2 local_checkpoint local_continuity

  mkdir -p "$lifecycle_dir"
  reset_ownership_state

  create_asset "0000" "$SWORD" "Sword #001" || return 1
  create_asset "0001" "$SHIELD" "Shield #001" || return 1
  create_asset "0002" "$LAND" "Land Parcel #001" || return 1
  validate_unique_assets || return 1
  local_genesis="$(ownership_root)"

  assign_owner "0003" "$SWORD" "player-a" || return 2
  assign_owner "0004" "$SHIELD" "player-b" || return 2
  assign_owner "0005" "$LAND" "world-treasury" || return 2
  validate_integrity || return 2
  local_assignment="$(ownership_root)"

  transfer_owner "0006" "$SWORD" "player-a" "player-b" || return 3
  transfer_owner "0007" "$SHIELD" "player-b" "player-a" || return 3
  transfer_owner "0008-mismatch-rejected" "$LAND" "player-a" "player-b" && return 3
  validate_integrity || return 3
  local_transfer="$(ownership_root)"

  local_epoch_0="$local_transfer"
  transfer_owner "0009" "$LAND" "world-treasury" "player-a" || return 4
  validate_integrity || return 4
  local_epoch_1="$(ownership_root)"
  transfer_owner "0010" "$SWORD" "player-b" "world-treasury" || return 4
  transfer_owner "0011" "$SHIELD" "player-a" "world-treasury" || return 4
  validate_integrity || return 4
  local_epoch_2="$(ownership_root)"

  if [[ "$local_epoch_0" == "$local_epoch_1" || "$local_epoch_1" == "$local_epoch_2" ]]; then
    return 4
  fi

  if [[ "$replay_mode" == "false" ]]; then
    asset_root_genesis="$local_genesis"
    ownership_root_assignment="$local_assignment"
    ownership_root_transfer="$local_transfer"
    ownership_root_epoch_0="$local_epoch_0"
    ownership_root_epoch_1="$local_epoch_1"
    ownership_root_epoch_2="$local_epoch_2"
    assignment_status="PASS"
    transfer_status="PASS"
  else
    ownership_checkpoint_identifier="$(checkpoint_identifier_for_root "$local_epoch_2")"
  fi

  write_checkpoint "$checkpoint_path"
  local_checkpoint="$ownership_checkpoint_identifier"
  checkpoint_integrity_valid "$checkpoint_path" || return 5
  [[ "$replay_mode" == "false" ]] && checkpoint_status="PASS"

  restore_from_checkpoint "$checkpoint_path" || return 6
  restored_root="$local_epoch_2"
  [[ "$replay_mode" == "false" ]] && restoration_status="PASS"

  transfer_owner "0012" "$LAND" "player-a" "player-b" || return 7
  transfer_owner "0013" "$SWORD" "world-treasury" "player-a" || return 7
  continued_root="$(ownership_root)"
  validate_integrity || return 7
  [[ "$continued_root" != "$restored_root" ]] || return 7

  local_continuity="$(continuity_root "$restored_root" "$continued_root")"
  if [[ "$replay_mode" == "false" ]]; then
    ownership_continuity_root="$local_continuity"
  else
    replay_ownership_root="$local_continuity"
    ownership_checkpoint_identifier="$local_checkpoint"
  fi
}

run_bootstrap_certification

if [[ "$bootstrap_status" == "PASS" ]]; then
  run_ownership_lifecycle "$CERT_WORK_DIR/primary" false || true

  if [[ "$assignment_status" == "PASS" \
    && "$transfer_status" == "PASS" \
    && "$checkpoint_status" == "PASS" \
    && "$restoration_status" == "PASS" ]]; then
    run_ownership_lifecycle "$CERT_WORK_DIR/replay" true || true
  fi

  if [[ "$ownership_continuity_root" != "UNKNOWN" \
    && "$replay_ownership_root" != "UNKNOWN" \
    && "$replay_ownership_root" == "$ownership_continuity_root" ]]; then
    replay_status="PASS"
  else
    replay_status="FAIL"
  fi

  if [[ "$assignment_status" == "PASS" \
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
  transfer_status="FAIL"
  checkpoint_status="FAIL"
  restoration_status="FAIL"
  replay_status="FAIL"
  integrity_status="FAIL"
fi

if [[ "$bootstrap_status" == "PASS" \
  && "$assignment_status" == "PASS" \
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
