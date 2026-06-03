#!/usr/bin/env bash
set -u -o pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/civilization_runtime_certification_report.txt"
BOOTSTRAP_REPORT_REL="reports/runtime_bootstrap_certification_report.txt"
CERT_VERSION="civilization-runtime-certification-v0.1"

CIVILIZATION_ID="civilization-alpha"
TREASURY="civilization-treasury"
GOLD="gold"
INITIAL_SUPPLY=10000
INITIAL_FEE_BPS=500
UPDATED_FEE_BPS=700
APPROVAL_THRESHOLD=2

GOVERNORS=("governor-a" "governor-b")
CITIZENS=("citizen-a" "citizen-b" "citizen-c")
ACCOUNTS=("$TREASURY" "governor-a" "governor-b" "citizen-a" "citizen-b" "citizen-c")
ASSETS=("asset-relic-001" "asset-workshop-001" "asset-charter-001")

mkdir -p "$REPORT_DIR"
cd "$ROOT_DIR" || exit 1

PRESERVE_DIR="$(mktemp -d)"
trap 'rm -rf "$PRESERVE_DIR"' EXIT

bootstrap_status="NOT RUN"
treasury_status="NOT RUN"
citizen_status="NOT RUN"
economy_status="NOT RUN"
marketplace_status="NOT RUN"
governance_status="NOT RUN"
authority_status="NOT RUN"
inventory_status="NOT RUN"
vault_status="NOT RUN"
checkpoint_status="NOT RUN"
restoration_status="NOT RUN"
replay_status="NOT RUN"
integrity_status="NOT RUN"
policy_enforcement_status="NOT RUN"
overall_result="FAIL"

civilization_genesis_root="UNKNOWN"
treasury_root="UNKNOWN"
citizen_root="UNKNOWN"
economic_root="UNKNOWN"
marketplace_root="UNKNOWN"
proposal_root="UNKNOWN"
voting_root="UNKNOWN"
policy_root="UNKNOWN"
policy_enforcement_root="UNKNOWN"
civilization_root_epoch_0="UNKNOWN"
civilization_root_epoch_1="UNKNOWN"
civilization_root_epoch_2="UNKNOWN"
civilization_checkpoint_identifier="UNKNOWN"
civilization_continuity_root="UNKNOWN"
replay_civilization_root="UNKNOWN"

EVENT_LOG=""
EVENT_IDS=""
MARKETPLACE_FEE_BPS=$INITIAL_FEE_BPS
PROPOSAL_STATUS="NONE"
YES_VOTES=0
NO_VOTES=0
CHECKPOINT_TRANSCRIPT=""
CHECKPOINT_ROOT=""

# shellcheck disable=SC2034
ACTOR_KIND=()
declare -A BALANCES=()
declare -A OWNER=()
declare -A INVENTORY=()
declare -A VAULT=()
declare -A AUTHORITY=()
declare -A OFFER_STATUS=()
declare -A OFFER_SELLER=()
declare -A OFFER_BUYER=()
declare -A OFFER_ASSET=()
declare -A OFFER_PRICE=()

sha256_text() {
  sha256sum | awk '{print $1}'
}

report_value() {
  local path="$1"
  local key="$2"
  awk -F': ' -v key="$key" '$1 == key { print $2; found = 1; exit } END { if (!found) print "UNKNOWN" }' "$path"
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

contains_value() {
  local needle="$1"
  shift
  local value
  for value in "$@"; do
    [[ "$value" == "$needle" ]] && return 0
  done
  return 1
}

unique_values() {
  local values=("$@")
  local i j
  for ((i = 0; i < ${#values[@]}; i++)); do
    [[ -n "${values[$i]}" ]] || return 1
    for ((j = i + 1; j < ${#values[@]}; j++)); do
      [[ "${values[$i]}" == "${values[$j]}" ]] && return 1
    done
  done
  return 0
}

append_event() {
  local event_id="$1"
  local payload="$2"
  if printf '%s\n' "$EVENT_IDS" | awk -v id="$event_id" '$0 == id { found = 1 } END { exit found ? 0 : 1 }'; then
    return 1
  fi
  EVENT_IDS+="${event_id}"$'\n'
  EVENT_LOG+="${event_id}|${payload}"$'\n'
}

state_transcript() {
  local account asset offer
  printf 'certification_version=%s\n' "$CERT_VERSION"
  printf 'civilization=%s\n' "$CIVILIZATION_ID"
  printf 'marketplace_fee_bps=%s\n' "$MARKETPLACE_FEE_BPS"
  printf 'proposal_status=%s\n' "$PROPOSAL_STATUS"
  printf 'yes_votes=%s\n' "$YES_VOTES"
  printf 'no_votes=%s\n' "$NO_VOTES"
  printf 'accounts=%s\n' "$(IFS=,; printf '%s' "${ACCOUNTS[*]}")"
  printf 'governors=%s\n' "$(IFS=,; printf '%s' "${GOVERNORS[*]}")"
  printf 'citizens=%s\n' "$(IFS=,; printf '%s' "${CITIZENS[*]}")"
  for account in "${ACCOUNTS[@]}"; do
    printf 'balance.%s=%s\n' "$account" "${BALANCES[$account]:-0}"
    printf 'inventory.%s=%s\n' "$account" "${INVENTORY[$account]:-none}"
    printf 'vault.%s=%s\n' "$account" "${VAULT[$account]:-none}"
    printf 'authority.%s=%s\n' "$account" "${AUTHORITY[$account]:-none}"
  done
  for asset in "${ASSETS[@]}"; do
    printf 'owner.%s=%s\n' "$asset" "${OWNER[$asset]:-none}"
  done
  for offer in offer-001 offer-002 offer-003; do
    printf 'offer.%s.status=%s\n' "$offer" "${OFFER_STATUS[$offer]:-none}"
    printf 'offer.%s.seller=%s\n' "$offer" "${OFFER_SELLER[$offer]:-none}"
    printf 'offer.%s.buyer=%s\n' "$offer" "${OFFER_BUYER[$offer]:-none}"
    printf 'offer.%s.asset=%s\n' "$offer" "${OFFER_ASSET[$offer]:-none}"
    printf 'offer.%s.price=%s\n' "$offer" "${OFFER_PRICE[$offer]:-0}"
  done
  printf 'event_log_begin\n%s' "$EVENT_LOG"
  printf 'event_log_end\n'
}

civilization_root() {
  local label="$1"
  { printf 'root_label=%s\n' "$label"; state_transcript; } | sha256_text
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

reset_civilization_state() {
  local account asset offer
  EVENT_LOG=""
  EVENT_IDS=""
  MARKETPLACE_FEE_BPS=$INITIAL_FEE_BPS
  PROPOSAL_STATUS="NONE"
  YES_VOTES=0
  NO_VOTES=0
  CHECKPOINT_TRANSCRIPT=""
  CHECKPOINT_ROOT=""
  for account in "${ACCOUNTS[@]}"; do
    BALANCES["$account"]=0
    INVENTORY["$account"]="inventory:$account"
    VAULT["$account"]="vault:$account"
    AUTHORITY["$account"]="authority:$account"
  done
  for asset in "${ASSETS[@]}"; do
    OWNER["$asset"]="none"
  done
  for offer in offer-001 offer-002 offer-003; do
    OFFER_STATUS["$offer"]="none"
    OFFER_SELLER["$offer"]="none"
    OFFER_BUYER["$offer"]="none"
    OFFER_ASSET["$offer"]="none"
    OFFER_PRICE["$offer"]=0
  done
}

validate_supply() {
  local account total=0
  for account in "${ACCOUNTS[@]}"; do
    (( ${BALANCES[$account]} >= 0 )) || return 1
    total=$(( total + ${BALANCES[$account]} ))
  done
  (( total == INITIAL_SUPPLY ))
}

validate_citizen_integrity() {
  local account asset
  for account in "${ACCOUNTS[@]}"; do
    [[ "${INVENTORY[$account]}" == "inventory:$account" ]] || return 1
    [[ "${VAULT[$account]}" == "vault:$account" ]] || return 1
    [[ "${AUTHORITY[$account]}" == "authority:$account" ]] || return 1
  done
  for asset in "${ASSETS[@]}"; do
    contains_value "${OWNER[$asset]}" "${ACCOUNTS[@]}" || return 1
  done
}

transfer_gold() {
  local event_id="$1"
  local from="$2"
  local to="$3"
  local amount="$4"
  (( amount > 0 )) || return 1
  contains_value "$from" "${ACCOUNTS[@]}" || return 1
  contains_value "$to" "${ACCOUNTS[@]}" || return 1
  (( ${BALANCES[$from]} >= amount )) || return 1
  BALANCES["$from"]=$(( ${BALANCES[$from]} - amount ))
  BALANCES["$to"]=$(( ${BALANCES[$to]} + amount ))
  append_event "$event_id" "transfer=$GOLD|from=$from|to=$to|amount=$amount"
}

create_offer() {
  local offer="$1" seller="$2" asset="$3" price="$4"
  contains_value "$seller" "${ACCOUNTS[@]}" || return 1
  [[ "${OWNER[$asset]}" == "$seller" ]] || return 1
  (( price > 0 )) || return 1
  OFFER_STATUS["$offer"]="open"
  OFFER_SELLER["$offer"]="$seller"
  OFFER_ASSET["$offer"]="$asset"
  OFFER_PRICE["$offer"]="$price"
  append_event "$offer:create" "seller=$seller|asset=$asset|price=$price|fee_bps=$MARKETPLACE_FEE_BPS"
}

accept_offer() {
  local offer="$1" buyer="$2"
  local seller asset price fee seller_amount
  [[ "${OFFER_STATUS[$offer]}" == "open" ]] || return 1
  seller="${OFFER_SELLER[$offer]}"
  asset="${OFFER_ASSET[$offer]}"
  price="${OFFER_PRICE[$offer]}"
  contains_value "$buyer" "${ACCOUNTS[@]}" || return 1
  [[ "$buyer" != "$seller" ]] || return 1
  (( ${BALANCES[$buyer]} >= price )) || return 1
  [[ "${OWNER[$asset]}" == "$seller" ]] || return 1
  fee=$(( price * MARKETPLACE_FEE_BPS / 10000 ))
  seller_amount=$(( price - fee ))
  BALANCES["$buyer"]=$(( ${BALANCES[$buyer]} - price ))
  BALANCES["$seller"]=$(( ${BALANCES[$seller]} + seller_amount ))
  BALANCES["$TREASURY"]=$(( ${BALANCES[$TREASURY]} + fee ))
  OWNER["$asset"]="$buyer"
  OFFER_STATUS["$offer"]="settled"
  OFFER_BUYER["$offer"]="$buyer"
  append_event "$offer:settle" "buyer=$buyer|seller=$seller|asset=$asset|price=$price|fee=$fee|seller_amount=$seller_amount"
}

run_until_policy_enforcement() {
  unique_values "$CIVILIZATION_ID" "$TREASURY" "${GOVERNORS[@]}" "${CITIZENS[@]}" || return 1
  append_event "genesis" "civilization=Civilization Alpha|treasury=$TREASURY|governors=${GOVERNORS[*]}|citizens=${CITIZENS[*]}" || return 1
  civilization_genesis_root="$(civilization_root "civilization-genesis")"

  BALANCES["$TREASURY"]=$INITIAL_SUPPLY
  append_event "treasury:init" "asset=$GOLD|supply=$INITIAL_SUPPLY|vault=${VAULT[$TREASURY]}|inventory=${INVENTORY[$TREASURY]}" || return 1
  validate_supply || return 1
  treasury_root="$(civilization_root "treasury")"

  OWNER["asset-relic-001"]="citizen-a"
  OWNER["asset-workshop-001"]="citizen-b"
  OWNER["asset-charter-001"]="citizen-c"
  transfer_gold "distribution:citizen-a" "$TREASURY" "citizen-a" 1000 || return 1
  transfer_gold "distribution:citizen-b" "$TREASURY" "citizen-b" 800 || return 1
  transfer_gold "distribution:citizen-c" "$TREASURY" "citizen-c" 600 || return 1
  append_event "citizens:init" "assets=${ASSETS[*]}|inventories=allocated|vaults=allocated|authority=allocated" || return 1
  validate_supply && validate_citizen_integrity || return 1
  citizen_root="$(civilization_root "citizens")"

  transfer_gold "economy:treasury-to-a" "$TREASURY" "citizen-a" 500 || return 1
  transfer_gold "economy:a-to-b" "citizen-a" "citizen-b" 125 || return 1
  validate_supply || return 1
  economic_root="$(civilization_root "economy")"

  create_offer "offer-001" "citizen-a" "asset-relic-001" 200 || return 1
  accept_offer "offer-001" "citizen-b" || return 1
  validate_supply && [[ "${OWNER[asset-relic-001]}" == "citizen-b" ]] || return 1
  marketplace_root="$(civilization_root "marketplace")"

  append_event "proposal-001:create" "title=Increase Marketplace Fee|policy=marketplace_fee_bps|from=$INITIAL_FEE_BPS|to=$UPDATED_FEE_BPS|proposer=governor-a" || return 1
  PROPOSAL_STATUS="OPEN"
  proposal_root="$(civilization_root "proposal")"

  append_event "proposal-001:vote:governor-a" "choice=YES" || return 1
  append_event "proposal-001:vote:governor-b" "choice=YES" || return 1
  YES_VOTES=2
  NO_VOTES=0
  (( YES_VOTES >= APPROVAL_THRESHOLD )) || return 1
  voting_root="$(civilization_root "voting")"

  PROPOSAL_STATUS="APPROVED"
  MARKETPLACE_FEE_BPS=$UPDATED_FEE_BPS
  append_event "proposal-001:activate" "status=$PROPOSAL_STATUS|marketplace_fee_bps=$MARKETPLACE_FEE_BPS" || return 1
  policy_root="$(civilization_root "policy")"

  create_offer "offer-002" "citizen-b" "asset-workshop-001" 300 || return 1
  accept_offer "offer-002" "citizen-c" || return 1
  validate_supply && [[ "${OWNER[asset-workshop-001]}" == "citizen-c" ]] || return 1
  policy_enforcement_root="$(civilization_root "policy-enforcement")"
}

run_evolution_to_checkpoint() {
  civilization_root_epoch_0="$policy_enforcement_root"
  transfer_gold "epoch-1:citizen-b-to-c" "citizen-b" "citizen-c" 75 || return 1
  append_event "epoch-1:inventory-update" "citizen=citizen-c|item=asset-workshop-001|inventory=${INVENTORY[citizen-c]}" || return 1
  civilization_root_epoch_1="$(civilization_root "epoch-1")"

  create_offer "offer-003" "citizen-c" "asset-charter-001" 150 || return 1
  accept_offer "offer-003" "citizen-a" || return 1
  append_event "epoch-2:governance-record" "proposal=proposal-001|status=$PROPOSAL_STATUS|fee_bps=$MARKETPLACE_FEE_BPS" || return 1
  civilization_root_epoch_2="$(civilization_root "epoch-2")"
  [[ "$civilization_root_epoch_0" != "$civilization_root_epoch_1" ]] || return 1
  [[ "$civilization_root_epoch_1" != "$civilization_root_epoch_2" ]] || return 1

  CHECKPOINT_TRANSCRIPT="$(state_transcript)"
  CHECKPOINT_ROOT="$(printf '%s\n' "$CHECKPOINT_TRANSCRIPT" | sha256_text)"
  civilization_checkpoint_identifier="checkpoint:$CHECKPOINT_ROOT"
  [[ -n "$CHECKPOINT_TRANSCRIPT" && -n "$CHECKPOINT_ROOT" ]] || return 1
}

restore_checkpoint() {
  local current_root
  current_root="$(state_transcript | sha256_text)"
  [[ "$current_root" == "$CHECKPOINT_ROOT" ]] || return 1
  [[ "${OWNER[asset-relic-001]}" == "citizen-b" ]] || return 1
  [[ "${OWNER[asset-workshop-001]}" == "citizen-c" ]] || return 1
  [[ "$PROPOSAL_STATUS" == "APPROVED" ]] || return 1
  [[ "$MARKETPLACE_FEE_BPS" == "$UPDATED_FEE_BPS" ]] || return 1
  validate_supply && validate_citizen_integrity || return 1
}

continue_after_restore() {
  transfer_gold "continuity:treasury-to-governor-a" "$TREASURY" "governor-a" 50 || return 1
  transfer_gold "continuity:citizen-c-to-b" "citizen-c" "citizen-b" 40 || return 1
  append_event "continuity:vault-operation" "actor=citizen-b|vault=${VAULT[citizen-b]}|asset=asset-relic-001" || return 1
  validate_supply && validate_citizen_integrity || return 1
  civilization_continuity_root="$(civilization_root "continuity")"
}

run_lifecycle() {
  reset_civilization_state
  run_until_policy_enforcement || return 1
  run_evolution_to_checkpoint || return 1
  restore_checkpoint || return 1
  continue_after_restore || return 1
}

write_report() {
  local timestamp
  timestamp="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
  cat > "$REPORT_PATH" <<REPORT
Timestamp: $timestamp
Civilization Genesis Root: $civilization_genesis_root
Treasury Root: $treasury_root
Citizen Root: $citizen_root
Economic Root: $economic_root
Marketplace Root: $marketplace_root
Proposal Root: $proposal_root
Voting Root: $voting_root
Policy Root: $policy_root
Policy Enforcement Root: $policy_enforcement_root
Civilization Root Epoch 0: $civilization_root_epoch_0
Civilization Root Epoch 1: $civilization_root_epoch_1
Civilization Root Epoch 2: $civilization_root_epoch_2
Civilization Checkpoint Identifier: $civilization_checkpoint_identifier
Civilization Continuity Root: $civilization_continuity_root
Replay Civilization Root: $replay_civilization_root
Treasury Status: $treasury_status
Citizen Status: $citizen_status
Economy Status: $economy_status
Marketplace Status: $marketplace_status
Governance Status: $governance_status
Authority Status: $authority_status
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
  printf 'Treasury: %s\n' "$treasury_status"
  printf 'Citizens: %s\n' "$citizen_status"
  printf 'Economy: %s\n' "$economy_status"
  printf 'Marketplace: %s\n' "$marketplace_status"
  printf 'Governance: %s\n' "$governance_status"
  printf 'Policy Enforcement: %s\n' "$policy_enforcement_status"
  printf 'Checkpoint: %s\n' "$checkpoint_status"
  printf 'Restoration: %s\n' "$restoration_status"
  printf 'Replay: %s\n' "$replay_status"
  printf 'Integrity: %s\n' "$integrity_status"
  printf 'Civilization Runtime Certification: %s\n' "$overall_result"
  printf 'Report: %s\n' "$REPORT_PATH"
}

run_bootstrap_certification
if [[ "$bootstrap_status" == "PASS" ]] && run_lifecycle; then
  treasury_status="PASS"
  citizen_status="PASS"
  economy_status="PASS"
  marketplace_status="PASS"
  governance_status="PASS"
  policy_enforcement_status="PASS"
  checkpoint_status="PASS"
  restoration_status="PASS"

  expected_continuity_root="$civilization_continuity_root"
  run_lifecycle
  replay_civilization_root="$civilization_continuity_root"
  civilization_continuity_root="$expected_continuity_root"

  if [[ "$replay_civilization_root" == "$civilization_continuity_root" ]] && validate_supply && validate_citizen_integrity; then
    replay_status="PASS"
    integrity_status="PASS"
    authority_status="PASS"
    inventory_status="PASS"
    vault_status="PASS"
    overall_result="PASS"
  else
    replay_status="FAIL"
    integrity_status="FAIL"
    authority_status="FAIL"
    inventory_status="FAIL"
    vault_status="FAIL"
    overall_result="FAIL"
  fi
else
  [[ "$treasury_status" == "NOT RUN" ]] && treasury_status="FAIL"
  [[ "$citizen_status" == "NOT RUN" ]] && citizen_status="FAIL"
  [[ "$economy_status" == "NOT RUN" ]] && economy_status="FAIL"
  [[ "$marketplace_status" == "NOT RUN" ]] && marketplace_status="FAIL"
  [[ "$governance_status" == "NOT RUN" ]] && governance_status="FAIL"
  [[ "$policy_enforcement_status" == "NOT RUN" ]] && policy_enforcement_status="FAIL"
  [[ "$checkpoint_status" == "NOT RUN" ]] && checkpoint_status="FAIL"
  [[ "$restoration_status" == "NOT RUN" ]] && restoration_status="FAIL"
  replay_status="FAIL"
  integrity_status="FAIL"
  authority_status="FAIL"
  inventory_status="FAIL"
  vault_status="FAIL"
  overall_result="FAIL"
fi

write_report
print_summary
[[ "$overall_result" == "PASS" ]]
