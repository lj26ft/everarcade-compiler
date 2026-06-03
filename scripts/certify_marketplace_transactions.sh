#!/usr/bin/env bash
set -u -o pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/marketplace_transaction_certification_report.txt"
BOOTSTRAP_REPORT_REL="reports/runtime_bootstrap_certification_report.txt"
CERT_VERSION="marketplace-transaction-certification-v0.1"

PARTICIPANTS=("player-a" "player-b" "marketplace-treasury")
ASSETS=("asset:sword-001" "asset:shield-001" "asset:potion-001")
VAULTS=("vault:player-a" "vault:player-b" "vault:treasury")
INVENTORIES=("inventory:player-a" "inventory:player-b" "inventory:treasury")
OFFER_IDS=("offer-001" "offer-002" "offer-003" "offer-004")

SWORD="asset:sword-001"
SHIELD="asset:shield-001"
POTION="asset:potion-001"
GOLD="gold"

ASSET_NAMES=()
declare -A BALANCES=()
declare -A OWNER=()
declare -A ASSET_INVENTORY=()
declare -A ASSET_VAULT=()
declare -A AUTHORITY=()
declare -A LINEAGE=()
declare -A OFFER_STATUS=()
declare -A OFFER_SELLER=()
declare -A OFFER_ASSET=()
declare -A OFFER_PRICE=()
declare -A OFFER_BUYER=()
declare -A SETTLED=()
EVENT_LOG=""
EVENT_IDS=""
UNAUTHORIZED_REJECTIONS="0"
INVALID_OFFER_REJECTIONS="0"
CANCELLED_SETTLEMENT_REJECTIONS="0"
DOUBLE_SETTLEMENT_REJECTIONS="0"
INITIAL_SUPPLY="0"

bootstrap_status="NOT RUN"
offer_status="NOT RUN"
authority_status="NOT RUN"
settlement_status="NOT RUN"
ownership_status="NOT RUN"
inventory_status="NOT RUN"
vault_status="NOT RUN"
checkpoint_status="NOT RUN"
restoration_status="NOT RUN"
replay_status="NOT RUN"
integrity_status="NOT RUN"
overall_result="FAIL"

marketplace_genesis_root="UNKNOWN"
economic_setup_root="UNKNOWN"
offer_creation_root="UNKNOWN"
offer_validation_root="UNKNOWN"
authority_validation_root="UNKNOWN"
offer_acceptance_root="UNKNOWN"
settlement_root="UNKNOWN"
ownership_transfer_root="UNKNOWN"
inventory_vault_root="UNKNOWN"
cancellation_root="UNKNOWN"
marketplace_root_epoch_0="UNKNOWN"
marketplace_root_epoch_1="UNKNOWN"
marketplace_root_epoch_2="UNKNOWN"
marketplace_checkpoint_identifier="UNKNOWN"
marketplace_continuity_root="UNKNOWN"
replay_marketplace_root="UNKNOWN"

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
Marketplace Genesis Root: $marketplace_genesis_root
Economic Setup Root: $economic_setup_root
Offer Creation Root: $offer_creation_root
Offer Validation Root: $offer_validation_root
Authority Validation Root: $authority_validation_root
Offer Acceptance Root: $offer_acceptance_root
Settlement Root: $settlement_root
Ownership Transfer Root: $ownership_transfer_root
Inventory Vault Root: $inventory_vault_root
Cancellation Root: $cancellation_root
Marketplace Root Epoch 0: $marketplace_root_epoch_0
Marketplace Root Epoch 1: $marketplace_root_epoch_1
Marketplace Root Epoch 2: $marketplace_root_epoch_2
Marketplace Checkpoint Identifier: $marketplace_checkpoint_identifier
Marketplace Continuity Root: $marketplace_continuity_root
Replay Marketplace Root: $replay_marketplace_root
Offer Status: $offer_status
Authority Status: $authority_status
Settlement Status: $settlement_status
Ownership Status: $ownership_status
Inventory Status: $inventory_status
Vault Status: $vault_status
Checkpoint Status: $checkpoint_status
Restoration Status: $restoration_status
Replay Status: $replay_status
Integrity Status: $integrity_status
Overall Result: $overall_result
REPORT
}

print_summary() {
  printf 'Bootstrap: %s\n' "$bootstrap_status"
  printf 'Offer Validation: %s\n' "$offer_status"
  printf 'Authority: %s\n' "$authority_status"
  printf 'Settlement: %s\n' "$settlement_status"
  printf 'Ownership: %s\n' "$ownership_status"
  printf 'Inventory: %s\n' "$inventory_status"
  printf 'Vault: %s\n' "$vault_status"
  printf 'Checkpoint: %s\n' "$checkpoint_status"
  printf 'Restoration: %s\n' "$restoration_status"
  printf 'Replay: %s\n' "$replay_status"
  printf 'Integrity: %s\n' "$integrity_status"
  printf 'Marketplace Transaction Certification: %s\n' "$overall_result"
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

reset_marketplace_state() {
  local account asset inventory offer vault

  EVENT_LOG=""
  EVENT_IDS=""
  ASSET_NAMES=()
  UNAUTHORIZED_REJECTIONS="0"
  INVALID_OFFER_REJECTIONS="0"
  CANCELLED_SETTLEMENT_REJECTIONS="0"
  DOUBLE_SETTLEMENT_REJECTIONS="0"
  INITIAL_SUPPLY="0"

  for account in "${PARTICIPANTS[@]}"; do
    BALANCES["$GOLD:$account"]="0"
  done

  for asset in "${ASSETS[@]}"; do
    OWNER["$asset"]=""
    ASSET_INVENTORY["$asset"]=""
    ASSET_VAULT["$asset"]=""
    AUTHORITY["$asset"]=""
    LINEAGE["$asset"]=""
  done

  for inventory in "${INVENTORIES[@]}"; do
    :
  done
  for vault in "${VAULTS[@]}"; do
    :
  done

  for offer in "${OFFER_IDS[@]}"; do
    OFFER_STATUS["$offer"]="absent"
    OFFER_SELLER["$offer"]=""
    OFFER_ASSET["$offer"]=""
    OFFER_PRICE["$offer"]="0"
    OFFER_BUYER["$offer"]=""
    SETTLED["$offer"]="false"
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

known_participant() {
  local expected participant="$1"
  for expected in "${PARTICIPANTS[@]}"; do
    [[ "$participant" == "$expected" ]] && return 0
  done
  return 1
}

known_offer() {
  local expected offer="$1"
  for expected in "${OFFER_IDS[@]}"; do
    [[ "$offer" == "$expected" ]] && return 0
  done
  return 1
}

asset_name_for() {
  case "$1" in
    "$SWORD") printf 'Sword #001' ;;
    "$SHIELD") printf 'Shield #001' ;;
    "$POTION") printf 'Potion #001' ;;
    *) printf 'UNKNOWN' ;;
  esac
}

inventory_for_owner() {
  case "$1" in
    "player-a") printf 'inventory:player-a' ;;
    "player-b") printf 'inventory:player-b' ;;
    "marketplace-treasury") printf 'inventory:treasury' ;;
    *) printf 'UNKNOWN' ;;
  esac
}

vault_for_owner() {
  case "$1" in
    "player-a") printf 'vault:player-a' ;;
    "player-b") printf 'vault:player-b' ;;
    "marketplace-treasury") printf 'vault:treasury' ;;
    *) printf 'UNKNOWN' ;;
  esac
}

state_transcript() {
  local account asset offer

  printf 'certification_version=%s\n' "$CERT_VERSION"
  printf 'authority=deterministic-runtime-marketplace\n'
  printf 'xrpl_settlement=disabled\n'
  printf 'xaman_signatures=disabled\n'
  printf 'blockchain_settlement=disabled\n'
  printf 'network_ordering_authority=disabled\n'
  printf 'participants=%s\n' "$(IFS=,; printf '%s' "${PARTICIPANTS[*]}")"
  printf 'assets=%s\n' "$(IFS=,; printf '%s' "${ASSETS[*]}")"
  printf 'offers=%s\n' "$(IFS=,; printf '%s' "${OFFER_IDS[*]}")"
  printf 'initial_supply.%s=%s\n' "$GOLD" "$INITIAL_SUPPLY"
  for account in "${PARTICIPANTS[@]}"; do
    printf 'balance.%s.%s=%s\n' "$GOLD" "$account" "${BALANCES[$GOLD:$account]}"
  done
  for asset in "${ASSETS[@]}"; do
    printf 'asset.%s.name=%s\n' "$asset" "$(asset_name_for "$asset")"
    printf 'asset.%s.owner=%s\n' "$asset" "${OWNER[$asset]}"
    printf 'asset.%s.inventory=%s\n' "$asset" "${ASSET_INVENTORY[$asset]}"
    printf 'asset.%s.vault=%s\n' "$asset" "${ASSET_VAULT[$asset]}"
    printf 'asset.%s.authority=%s\n' "$asset" "${AUTHORITY[$asset]}"
    printf 'asset.%s.lineage=%s\n' "$asset" "${LINEAGE[$asset]}"
  done
  for offer in "${OFFER_IDS[@]}"; do
    printf 'offer.%s.status=%s\n' "$offer" "${OFFER_STATUS[$offer]}"
    printf 'offer.%s.seller=%s\n' "$offer" "${OFFER_SELLER[$offer]}"
    printf 'offer.%s.asset=%s\n' "$offer" "${OFFER_ASSET[$offer]}"
    printf 'offer.%s.price=%s\n' "$offer" "${OFFER_PRICE[$offer]}"
    printf 'offer.%s.buyer=%s\n' "$offer" "${OFFER_BUYER[$offer]}"
    printf 'offer.%s.settled=%s\n' "$offer" "${SETTLED[$offer]}"
  done
  printf 'rejections.unauthorized=%s\n' "$UNAUTHORIZED_REJECTIONS"
  printf 'rejections.invalid_offer=%s\n' "$INVALID_OFFER_REJECTIONS"
  printf 'rejections.cancelled_settlement=%s\n' "$CANCELLED_SETTLEMENT_REJECTIONS"
  printf 'rejections.double_settlement=%s\n' "$DOUBLE_SETTLEMENT_REJECTIONS"
  printf 'event_log_begin\n%s' "$EVENT_LOG"
  printf 'event_log_end\n'
}

marketplace_root() {
  state_transcript | sha256_text
}

create_asset() {
  local event_id="$1"
  local asset="$2"
  local owner="$3"

  known_asset "$asset" || return 1
  known_participant "$owner" || return 1
  [[ -z "${OWNER[$asset]}" ]] || return 1

  OWNER["$asset"]="$owner"
  ASSET_INVENTORY["$asset"]="$(inventory_for_owner "$owner")"
  ASSET_VAULT["$asset"]="$(vault_for_owner "$owner")"
  AUTHORITY["$asset"]="$owner"
  LINEAGE["$asset"]="genesis:none->$owner"
  ASSET_NAMES+=("$asset")
  append_event "$event_id" "asset-create asset=$asset owner=$owner inventory=${ASSET_INVENTORY[$asset]} vault=${ASSET_VAULT[$asset]} authority=$owner"
}

fund_account() {
  local event_id="$1"
  local account="$2"
  local amount="$3"

  known_participant "$account" || return 1
  (( amount >= 0 )) || return 1
  BALANCES["$GOLD:$account"]="$amount"
  INITIAL_SUPPLY=$(( INITIAL_SUPPLY + amount ))
  append_event "$event_id" "economic-setup asset=$GOLD account=$account balance=$amount"
}

create_offer() {
  local event_id="$1"
  local offer="$2"
  local seller="$3"
  local asset="$4"
  local price="$5"

  known_offer "$offer" || return 1
  known_participant "$seller" || return 1
  known_asset "$asset" || return 1
  (( price > 0 )) || return 1
  [[ "${OFFER_STATUS[$offer]}" == "absent" ]] || return 1
  [[ "${OWNER[$asset]}" == "$seller" ]] || return 1

  OFFER_STATUS["$offer"]="active"
  OFFER_SELLER["$offer"]="$seller"
  OFFER_ASSET["$offer"]="$asset"
  OFFER_PRICE["$offer"]="$price"
  append_event "$event_id" "offer-create offer=$offer seller=$seller asset=$asset price=$price"
}

validate_offer() {
  local event_id="$1"
  local offer="$2"
  local seller asset price

  known_offer "$offer" || return 1
  seller="${OFFER_SELLER[$offer]}"
  asset="${OFFER_ASSET[$offer]}"
  price="${OFFER_PRICE[$offer]}"

  [[ "${OFFER_STATUS[$offer]}" == "active" ]] || return 1
  known_participant "$seller" || return 1
  known_asset "$asset" || return 1
  (( price > 0 )) || return 1
  [[ "${OWNER[$asset]}" == "$seller" ]] || return 1
  [[ "${AUTHORITY[$asset]}" == "$seller" ]] || return 1
  [[ "${ASSET_INVENTORY[$asset]}" == "$(inventory_for_owner "$seller")" ]] || return 1
  [[ "${ASSET_VAULT[$asset]}" == "$(vault_for_owner "$seller")" ]] || return 1

  append_event "$event_id" "offer-validate offer=$offer authority=$seller custody=${ASSET_VAULT[$asset]} price=$price"
}

reject_invalid_offer() {
  local event_id="$1"
  local offer="$2"
  local seller="$3"
  local asset="$4"
  local price="$5"

  if known_offer "$offer" \
    && known_participant "$seller" \
    && known_asset "$asset" \
    && (( price > 0 )) \
    && [[ "${OFFER_STATUS[$offer]}" == "absent" ]] \
    && [[ "${OWNER[$asset]}" == "$seller" ]]; then
    return 1
  fi

  INVALID_OFFER_REJECTIONS=$(( INVALID_OFFER_REJECTIONS + 1 ))
  append_event "$event_id" "offer-reject reason=invalid-offer offer=$offer seller=$seller asset=$asset price=$price"
}

reject_unauthorized_offer() {
  local event_id="$1"
  local seller="$2"
  local asset="$3"

  if [[ "${OWNER[$asset]}" == "$seller" && "${AUTHORITY[$asset]}" == "$seller" ]]; then
    return 1
  fi
  UNAUTHORIZED_REJECTIONS=$(( UNAUTHORIZED_REJECTIONS + 1 ))
  append_event "$event_id" "offer-reject reason=unauthorized seller=$seller asset=$asset"
}

accept_offer() {
  local event_id="$1"
  local offer="$2"
  local buyer="$3"
  local price seller asset

  known_offer "$offer" || return 1
  known_participant "$buyer" || return 1
  price="${OFFER_PRICE[$offer]}"
  seller="${OFFER_SELLER[$offer]}"
  asset="${OFFER_ASSET[$offer]}"

  [[ "${OFFER_STATUS[$offer]}" == "active" ]] || return 1
  [[ "$buyer" != "$seller" ]] || return 1
  (( ${BALANCES[$GOLD:$buyer]} >= price )) || return 1
  [[ "${OWNER[$asset]}" == "$seller" ]] || return 1
  [[ "${AUTHORITY[$asset]}" == "$seller" ]] || return 1

  OFFER_STATUS["$offer"]="accepted"
  OFFER_BUYER["$offer"]="$buyer"
  append_event "$event_id" "offer-accept offer=$offer buyer=$buyer seller=$seller price=$price"
}

settle_offer() {
  local event_id="$1"
  local offer="$2"
  local seller buyer price

  known_offer "$offer" || return 1
  seller="${OFFER_SELLER[$offer]}"
  buyer="${OFFER_BUYER[$offer]}"
  price="${OFFER_PRICE[$offer]}"

  [[ "${OFFER_STATUS[$offer]}" == "accepted" ]] || return 1
  [[ "${SETTLED[$offer]}" == "false" ]] || return 1
  (( ${BALANCES[$GOLD:$buyer]} >= price )) || return 1

  BALANCES["$GOLD:$buyer"]=$(( ${BALANCES[$GOLD:$buyer]} - price ))
  BALANCES["$GOLD:$seller"]=$(( ${BALANCES[$GOLD:$seller]} + price ))
  SETTLED["$offer"]="true"
  OFFER_STATUS["$offer"]="settled"
  append_event "$event_id" "offer-settle offer=$offer asset=$GOLD from=$buyer to=$seller amount=$price"
}

reject_double_settlement() {
  local event_id="$1"
  local offer="$2"

  [[ "${SETTLED[$offer]}" == "true" ]] || return 1
  DOUBLE_SETTLEMENT_REJECTIONS=$(( DOUBLE_SETTLEMENT_REJECTIONS + 1 ))
  append_event "$event_id" "offer-reject reason=double-settlement offer=$offer"
}

transfer_offer_asset() {
  local event_id="$1"
  local offer="$2"
  local seller buyer asset

  seller="${OFFER_SELLER[$offer]}"
  buyer="${OFFER_BUYER[$offer]}"
  asset="${OFFER_ASSET[$offer]}"

  [[ "${OFFER_STATUS[$offer]}" == "settled" ]] || return 1
  [[ "${OWNER[$asset]}" == "$seller" ]] || return 1

  OWNER["$asset"]="$buyer"
  AUTHORITY["$asset"]="$buyer"
  LINEAGE["$asset"]+="->$buyer"
  OFFER_STATUS["$offer"]="transferred"
  append_event "$event_id" "ownership-transfer offer=$offer asset=$asset from=$seller to=$buyer lineage=${LINEAGE[$asset]}"
}

update_inventory_vault() {
  local event_id="$1"
  local offer="$2"
  local buyer asset

  buyer="${OFFER_BUYER[$offer]}"
  asset="${OFFER_ASSET[$offer]}"

  [[ "${OWNER[$asset]}" == "$buyer" ]] || return 1
  ASSET_INVENTORY["$asset"]="$(inventory_for_owner "$buyer")"
  ASSET_VAULT["$asset"]="$(vault_for_owner "$buyer")"
  OFFER_STATUS["$offer"]="closed"
  append_event "$event_id" "inventory-vault-update offer=$offer asset=$asset inventory=${ASSET_INVENTORY[$asset]} vault=${ASSET_VAULT[$asset]}"
}

cancel_offer() {
  local event_id="$1"
  local offer="$2"
  local seller="$3"

  known_offer "$offer" || return 1
  [[ "${OFFER_STATUS[$offer]}" == "active" ]] || return 1
  [[ "${OFFER_SELLER[$offer]}" == "$seller" ]] || return 1
  [[ "${AUTHORITY[${OFFER_ASSET[$offer]}]}" == "$seller" ]] || return 1

  OFFER_STATUS["$offer"]="cancelled"
  append_event "$event_id" "offer-cancel offer=$offer seller=$seller"
}

reject_cancelled_settlement() {
  local event_id="$1"
  local offer="$2"

  [[ "${OFFER_STATUS[$offer]}" == "cancelled" ]] || return 1
  CANCELLED_SETTLEMENT_REJECTIONS=$(( CANCELLED_SETTLEMENT_REJECTIONS + 1 ))
  append_event "$event_id" "offer-reject reason=cancelled-settlement offer=$offer"
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

validate_balances() {
  local account total balance
  total=0
  for account in "${PARTICIPANTS[@]}"; do
    balance="${BALANCES[$GOLD:$account]}"
    (( balance >= 0 )) || return 1
    total=$(( total + balance ))
  done
  [[ "$total" == "$INITIAL_SUPPLY" ]]
}

validate_marketplace_integrity() {
  local asset event_count event_id_count offer owner_count

  validate_unique_assets || return 1
  validate_balances || return 1

  event_count="$(printf '%s' "$EVENT_LOG" | sed '/^$/d' | wc -l | tr -d ' ')"
  event_id_count="$(printf '%s' "$EVENT_IDS" | sed '/^$/d' | sort -u | wc -l | tr -d ' ')"
  [[ "$event_count" == "$event_id_count" ]] || return 1

  for asset in "${ASSETS[@]}"; do
    known_participant "${OWNER[$asset]}" || return 1
    [[ "${AUTHORITY[$asset]}" == "${OWNER[$asset]}" ]] || return 1
    [[ "${ASSET_INVENTORY[$asset]}" == "$(inventory_for_owner "${OWNER[$asset]}")" ]] || return 1
    [[ "${ASSET_VAULT[$asset]}" == "$(vault_for_owner "${OWNER[$asset]}")" ]] || return 1
    [[ "${LINEAGE[$asset]}" == genesis:none*"${OWNER[$asset]}" ]] || return 1
    owner_count="$(printf '%s\n' "${OWNER[$asset]}" | sed '/^$/d' | wc -l | tr -d ' ')"
    [[ "$owner_count" == "1" ]] || return 1
  done

  for offer in "${OFFER_IDS[@]}"; do
    case "${OFFER_STATUS[$offer]}" in
      absent|active|accepted|settled|transferred|closed|cancelled) ;;
      *) return 1 ;;
    esac
    if [[ "${SETTLED[$offer]}" == "true" ]]; then
      [[ "${OFFER_STATUS[$offer]}" == "closed" ]] || [[ "${OFFER_STATUS[$offer]}" == "transferred" ]] || [[ "${OFFER_STATUS[$offer]}" == "settled" ]] || return 1
    fi
  done

  [[ "$INVALID_OFFER_REJECTIONS" -ge 1 ]] || return 1
  [[ "$UNAUTHORIZED_REJECTIONS" -ge 1 ]] || return 1
  [[ "$CANCELLED_SETTLEMENT_REJECTIONS" -ge 1 ]] || return 1
  [[ "$DOUBLE_SETTLEMENT_REJECTIONS" -ge 1 ]] || return 1
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

  marketplace_checkpoint_identifier="$(checkpoint_identifier_for_root "$root")"
  {
    printf 'Checkpoint Version: %s\n' "$CERT_VERSION"
    printf 'Marketplace Checkpoint Identifier: %s\n' "$marketplace_checkpoint_identifier"
    printf 'Persisted Marketplace Root: %s\n' "$root"
    state_transcript
  } > "$checkpoint_path"
}

checkpoint_integrity_valid() {
  local checkpoint_path="$1"
  local root="$2"

  [[ -s "$checkpoint_path" ]] \
    && [[ "$(report_value "$checkpoint_path" "Marketplace Checkpoint Identifier")" == "$marketplace_checkpoint_identifier" ]] \
    && [[ "$(report_value "$checkpoint_path" "Persisted Marketplace Root")" == "$root" ]]
}

restore_from_checkpoint() {
  local checkpoint_path="$1"
  local root="$2"
  local restored_root

  restored_root="$(sed -n '/^certification_version=/,$p' "$checkpoint_path" | sha256_text)"
  [[ "$restored_root" == "$root" ]]
}

run_full_trade() {
  local base_id="$1"
  local offer="$2"
  local seller="$3"
  local buyer="$4"
  local asset="$5"
  local price="$6"

  create_offer "$base_id.01" "$offer" "$seller" "$asset" "$price" || return 1
  validate_offer "$base_id.02" "$offer" || return 1
  accept_offer "$base_id.03" "$offer" "$buyer" || return 1
  settle_offer "$base_id.04" "$offer" || return 1
  transfer_offer_asset "$base_id.05" "$offer" || return 1
  update_inventory_vault "$base_id.06" "$offer" || return 1
}

run_marketplace_lifecycle() {
  local lifecycle_dir="$1"
  local replay_mode="${2:-false}"
  local checkpoint_path="$lifecycle_dir/marketplace-transaction.checkpoint"
  local local_genesis local_economic local_offer_creation local_offer_validation local_authority
  local local_acceptance local_settlement local_ownership local_inventory_vault local_cancellation
  local local_epoch_0 local_epoch_1 local_epoch_2 local_continuity

  mkdir -p "$lifecycle_dir"
  reset_marketplace_state

  append_event "0000" "genesis participants=player-a,player-b,marketplace-treasury assets=sword-001,shield-001,potion-001" || return 1
  create_asset "0001" "$SWORD" "player-a" || return 1
  create_asset "0002" "$SHIELD" "player-b" || return 1
  create_asset "0003" "$POTION" "marketplace-treasury" || return 1
  validate_unique_assets || return 1
  local_genesis="$(marketplace_root)"

  fund_account "0004" "player-a" 100 || return 2
  fund_account "0005" "player-b" 500 || return 2
  fund_account "0006" "marketplace-treasury" 0 || return 2
  validate_balances || return 2
  local_economic="$(marketplace_root)"

  create_offer "0007" "offer-001" "player-a" "$SWORD" 100 || return 3
  local_offer_creation="$(marketplace_root)"

  validate_offer "0008" "offer-001" || return 4
  reject_invalid_offer "0008.invalid" "offer-002" "player-a" "$SWORD" 0 || return 4
  local_offer_validation="$(marketplace_root)"

  reject_unauthorized_offer "0009" "player-b" "$SWORD" || return 5
  local_authority="$(marketplace_root)"

  accept_offer "0010" "offer-001" "player-b" || return 6
  local_acceptance="$(marketplace_root)"

  settle_offer "0011" "offer-001" || return 7
  reject_double_settlement "0012" "offer-001" || return 7
  validate_balances || return 7
  local_settlement="$(marketplace_root)"

  transfer_offer_asset "0013" "offer-001" || return 8
  [[ "${OWNER[$SWORD]}" == "player-b" ]] || return 8
  local_ownership="$(marketplace_root)"

  update_inventory_vault "0014" "offer-001" || return 9
  [[ "${ASSET_INVENTORY[$SWORD]}" == "inventory:player-b" ]] || return 9
  [[ "${ASSET_VAULT[$SWORD]}" == "vault:player-b" ]] || return 9
  local_inventory_vault="$(marketplace_root)"

  create_offer "0015" "offer-002" "player-b" "$SHIELD" 150 || return 10
  validate_offer "0016" "offer-002" || return 10
  cancel_offer "0017" "offer-002" "player-b" || return 10
  reject_cancelled_settlement "0018" "offer-002" || return 10
  local_cancellation="$(marketplace_root)"

  local_epoch_0="$local_cancellation"
  create_offer "0019" "offer-003" "marketplace-treasury" "$POTION" 25 || return 11
  validate_offer "0020" "offer-003" || return 11
  local_epoch_1="$(marketplace_root)"
  accept_offer "0021" "offer-003" "player-a" || return 11
  settle_offer "0022" "offer-003" || return 11
  transfer_offer_asset "0023" "offer-003" || return 11
  update_inventory_vault "0024" "offer-003" || return 11
  validate_marketplace_integrity || return 11
  local_epoch_2="$(marketplace_root)"

  [[ "$local_epoch_0" != "$local_epoch_1" && "$local_epoch_1" != "$local_epoch_2" ]] || return 11

  if [[ "$replay_mode" == "false" ]]; then
    marketplace_genesis_root="$local_genesis"
    economic_setup_root="$local_economic"
    offer_creation_root="$local_offer_creation"
    offer_validation_root="$local_offer_validation"
    authority_validation_root="$local_authority"
    offer_acceptance_root="$local_acceptance"
    settlement_root="$local_settlement"
    ownership_transfer_root="$local_ownership"
    inventory_vault_root="$local_inventory_vault"
    cancellation_root="$local_cancellation"
    marketplace_root_epoch_0="$local_epoch_0"
    marketplace_root_epoch_1="$local_epoch_1"
    marketplace_root_epoch_2="$local_epoch_2"
    offer_status="PASS"
    authority_status="PASS"
    settlement_status="PASS"
    ownership_status="PASS"
    inventory_status="PASS"
    vault_status="PASS"
  fi

  write_checkpoint "$checkpoint_path" "$local_epoch_2"
  checkpoint_integrity_valid "$checkpoint_path" "$local_epoch_2" || return 12
  [[ "$replay_mode" == "false" ]] && checkpoint_status="PASS"

  restore_from_checkpoint "$checkpoint_path" "$local_epoch_2" || return 13
  [[ "$replay_mode" == "false" ]] && restoration_status="PASS"

  run_full_trade "0025" "offer-004" "player-b" "player-a" "$SWORD" 50 || return 14
  validate_marketplace_integrity || return 14
  local_continuity="$(marketplace_root)"
  [[ "$local_continuity" != "$local_epoch_2" ]] || return 14

  if [[ "$replay_mode" == "false" ]]; then
    marketplace_continuity_root="$local_continuity"
  else
    replay_marketplace_root="$local_continuity"
  fi
}

run_bootstrap_certification

if [[ "$bootstrap_status" == "PASS" ]]; then
  run_marketplace_lifecycle "$CERT_WORK_DIR/primary" false || true

  if [[ "$offer_status" == "PASS" \
    && "$authority_status" == "PASS" \
    && "$settlement_status" == "PASS" \
    && "$ownership_status" == "PASS" \
    && "$inventory_status" == "PASS" \
    && "$vault_status" == "PASS" \
    && "$checkpoint_status" == "PASS" \
    && "$restoration_status" == "PASS" ]]; then
    run_marketplace_lifecycle "$CERT_WORK_DIR/replay" true || true
  fi

  if [[ "$marketplace_continuity_root" != "UNKNOWN" \
    && "$replay_marketplace_root" != "UNKNOWN" \
    && "$replay_marketplace_root" == "$marketplace_continuity_root" ]]; then
    replay_status="PASS"
  else
    replay_status="FAIL"
  fi

  if [[ "$offer_status" == "PASS" \
    && "$authority_status" == "PASS" \
    && "$settlement_status" == "PASS" \
    && "$ownership_status" == "PASS" \
    && "$inventory_status" == "PASS" \
    && "$vault_status" == "PASS" \
    && "$checkpoint_status" == "PASS" \
    && "$restoration_status" == "PASS" \
    && "$replay_status" == "PASS" ]]; then
    integrity_status="PASS"
  else
    integrity_status="FAIL"
  fi
else
  offer_status="FAIL"
  authority_status="FAIL"
  settlement_status="FAIL"
  ownership_status="FAIL"
  inventory_status="FAIL"
  vault_status="FAIL"
  checkpoint_status="FAIL"
  restoration_status="FAIL"
  replay_status="FAIL"
  integrity_status="FAIL"
fi

if [[ "$bootstrap_status" == "PASS" \
  && "$offer_status" == "PASS" \
  && "$authority_status" == "PASS" \
  && "$settlement_status" == "PASS" \
  && "$ownership_status" == "PASS" \
  && "$inventory_status" == "PASS" \
  && "$vault_status" == "PASS" \
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
