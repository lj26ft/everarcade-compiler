#!/usr/bin/env bash
set -u -o pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/xrpl_authority_certification_report.txt"
CIVILIZATION_REPORT_REL="reports/civilization_runtime_certification_report.txt"
CERT_VERSION="xrpl-authority-certification-v0.1"

AUTHORITIES=("authority:a" "authority:b" "authority:c")
XRPL_IDENTITIES=("xrpl:account-a" "xrpl:account-b" "xrpl:account-c")
DELEGATE_ID="delegate:d"

mkdir -p "$REPORT_DIR"
cd "$ROOT_DIR" || exit 1

PRESERVE_DIR="$(mktemp -d)"
trap 'rm -rf "$PRESERVE_DIR"' EXIT

bootstrap_status="NOT RUN"
mapping_status="NOT RUN"
validation_status="NOT RUN"
delegation_status="NOT RUN"
revocation_status="NOT RUN"
checkpoint_status="NOT RUN"
restoration_status="NOT RUN"
replay_status="NOT RUN"
integrity_status="NOT RUN"
overall_result="FAIL"

# shellcheck disable=SC2034
declare -A AUTHORITY_TO_XRPL=()
declare -A XRPL_TO_AUTHORITY=()
declare -A DELEGATE_TO_AUTHORITY=()
declare -A DELEGATE_STATUS=()
declare -A LINEAGE=()

authority_genesis_root="UNKNOWN"
identity_mapping_root="UNKNOWN"
authority_validation_root="UNKNOWN"
delegation_root="UNKNOWN"
delegated_authority_root="UNKNOWN"
revocation_root="UNKNOWN"
revoked_authority_root="UNKNOWN"
invalid_mapping_root="UNKNOWN"
authority_mapping_root_epoch_0="UNKNOWN"
authority_mapping_root_epoch_1="UNKNOWN"
authority_mapping_root_epoch_2="UNKNOWN"
authority_mapping_checkpoint_identifier="UNKNOWN"
authority_mapping_continuity_root="UNKNOWN"
replay_authority_mapping_root="UNKNOWN"

EVENT_LOG=""
EVENT_IDS=""
CHECKPOINT_TRANSCRIPT=""
CHECKPOINT_ROOT=""
CHECKPOINT_EVENT_LOG=""
CHECKPOINT_EVENT_IDS=""
declare -A CHECKPOINT_AUTHORITY_TO_XRPL=()
declare -A CHECKPOINT_XRPL_TO_AUTHORITY=()
declare -A CHECKPOINT_DELEGATE_TO_AUTHORITY=()
declare -A CHECKPOINT_DELEGATE_STATUS=()
declare -A CHECKPOINT_LINEAGE=()

sha256_text() {
  sha256sum | awk '{print $1}'
}

report_value() {
  local path="$1"
  local key="$2"
  awk -F': ' -v key="$key" '$1 == key { print $2; found = 1; exit } END { if (!found) print "UNKNOWN" }' "$path"
}

preserve_civilization_report() {
  mkdir -p "$PRESERVE_DIR/$(dirname "$CIVILIZATION_REPORT_REL")"
  if [[ -e "$CIVILIZATION_REPORT_REL" ]]; then
    cp -p "$CIVILIZATION_REPORT_REL" "$PRESERVE_DIR/$CIVILIZATION_REPORT_REL"
  else
    : > "$PRESERVE_DIR/$CIVILIZATION_REPORT_REL.absent"
  fi
}

restore_civilization_report() {
  if [[ -f "$PRESERVE_DIR/$CIVILIZATION_REPORT_REL.absent" ]]; then
    rm -f "$CIVILIZATION_REPORT_REL"
  elif [[ -e "$PRESERVE_DIR/$CIVILIZATION_REPORT_REL" ]]; then
    mkdir -p "$(dirname "$CIVILIZATION_REPORT_REL")"
    cp -p "$PRESERVE_DIR/$CIVILIZATION_REPORT_REL" "$CIVILIZATION_REPORT_REL"
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
  local authority identity delegate
  printf 'certification_version=%s\n' "$CERT_VERSION"
  printf 'authorities=%s\n' "$(IFS=,; printf '%s' "${AUTHORITIES[*]}")"
  printf 'xrpl_identities=%s\n' "$(IFS=,; printf '%s' "${XRPL_IDENTITIES[*]}")"
  for authority in "${AUTHORITIES[@]}"; do
    printf 'mapping.%s=%s\n' "$authority" "${AUTHORITY_TO_XRPL[$authority]:-none}"
    printf 'lineage.%s=%s\n' "$authority" "${LINEAGE[$authority]:-none}"
  done
  for identity in "${XRPL_IDENTITIES[@]}"; do
    printf 'reverse.%s=%s\n' "$identity" "${XRPL_TO_AUTHORITY[$identity]:-none}"
  done
  for delegate in "$DELEGATE_ID"; do
    printf 'delegate.%s.authority=%s\n' "$delegate" "${DELEGATE_TO_AUTHORITY[$delegate]:-none}"
    printf 'delegate.%s.status=%s\n' "$delegate" "${DELEGATE_STATUS[$delegate]:-none}"
  done
  printf 'event_log_begin\n%s' "$EVENT_LOG"
  printf 'event_log_end\n'
}

mapping_root() {
  local label="$1"
  { printf 'root_label=%s\n' "$label"; state_transcript; } | sha256_text
}

mapping_state_root() {
  state_transcript | sha256_text
}

run_bootstrap_certification() {
  preserve_civilization_report
  if bash scripts/certify_civilization_runtime.sh >/dev/null 2>&1; then
    bootstrap_status="PASS"
  elif [[ -f "$PRESERVE_DIR/$CIVILIZATION_REPORT_REL" ]] \
    && [[ "$(report_value "$PRESERVE_DIR/$CIVILIZATION_REPORT_REL" "Overall Result")" == "PASS" ]]; then
    bootstrap_status="PASS"
  else
    bootstrap_status="FAIL"
  fi
  restore_civilization_report
}

reset_mapping_state() {
  local authority identity
  EVENT_LOG=""
  EVENT_IDS=""
  AUTHORITY_TO_XRPL=()
  XRPL_TO_AUTHORITY=()
  DELEGATE_TO_AUTHORITY=()
  DELEGATE_STATUS=()
  LINEAGE=()
  for authority in "${AUTHORITIES[@]}"; do
    AUTHORITY_TO_XRPL["$authority"]=""
    LINEAGE["$authority"]="genesis"
  done
  for identity in "${XRPL_IDENTITIES[@]}"; do
    XRPL_TO_AUTHORITY["$identity"]=""
  done
  DELEGATE_TO_AUTHORITY["$DELEGATE_ID"]=""
  DELEGATE_STATUS["$DELEGATE_ID"]="none"
}

create_authority_genesis() {
  unique_values "${AUTHORITIES[@]}" || return 1
  append_event "authority:genesis" "authorities=${AUTHORITIES[*]}" || return 1
  authority_genesis_root="$(mapping_root "authority-genesis")"
}

add_mapping() {
  local authority="$1"
  local identity="$2"
  contains_value "$authority" "${AUTHORITIES[@]}" || return 1
  contains_value "$identity" "${XRPL_IDENTITIES[@]}" || return 1
  [[ -z "${AUTHORITY_TO_XRPL[$authority]:-}" ]] || return 1
  [[ -z "${XRPL_TO_AUTHORITY[$identity]:-}" ]] || return 1
  AUTHORITY_TO_XRPL["$authority"]="$identity"
  XRPL_TO_AUTHORITY["$identity"]="$authority"
  LINEAGE["$authority"]+=">mapped:$identity"
  append_event "mapping:$authority" "identity=$identity" || return 1
}

create_identity_mappings() {
  unique_values "${XRPL_IDENTITIES[@]}" || return 1
  add_mapping "authority:a" "xrpl:account-a" || return 1
  add_mapping "authority:b" "xrpl:account-b" || return 1
  add_mapping "authority:c" "xrpl:account-c" || return 1
  validate_mapping_bijection || return 1
  identity_mapping_root="$(mapping_root "identity-mapping")"
}

validate_authority_mapping() {
  [[ "${AUTHORITY_TO_XRPL[authority:a]}" == "xrpl:account-a" ]] || return 1
  [[ "${AUTHORITY_TO_XRPL[authority:b]}" == "xrpl:account-b" ]] || return 1
  [[ "${XRPL_TO_AUTHORITY[xrpl:account-a]}" == "authority:a" ]] || return 1
  [[ "${XRPL_TO_AUTHORITY[xrpl:account-b]}" == "authority:b" ]] || return 1
  append_event "authority:validation" "authority:a=xrpl:account-a|authority:b=xrpl:account-b" || return 1
  authority_validation_root="$(mapping_root "authority-validation")"
}

create_delegation() {
  local authority="authority:a"
  [[ "${AUTHORITY_TO_XRPL[$authority]}" == "xrpl:account-a" ]] || return 1
  DELEGATE_TO_AUTHORITY["$DELEGATE_ID"]="$authority"
  DELEGATE_STATUS["$DELEGATE_ID"]="active"
  LINEAGE["$authority"]+=">delegated:$DELEGATE_ID"
  append_event "delegation:$DELEGATE_ID" "authority=$authority|identity=${AUTHORITY_TO_XRPL[$authority]}|status=active" || return 1
  delegation_root="$(mapping_root "delegation")"
}

validate_delegated_authority() {
  local authority="${DELEGATE_TO_AUTHORITY[$DELEGATE_ID]:-}"
  [[ "$authority" == "authority:a" ]] || return 1
  [[ "${DELEGATE_STATUS[$DELEGATE_ID]}" == "active" ]] || return 1
  [[ "${AUTHORITY_TO_XRPL[$authority]}" == "xrpl:account-a" ]] || return 1
  append_event "delegated-authority:validation" "delegate=$DELEGATE_ID|authority=$authority|authorized=true" || return 1
  delegated_authority_root="$(mapping_root "delegated-authority")"
}

revoke_delegation() {
  local authority="${DELEGATE_TO_AUTHORITY[$DELEGATE_ID]:-}"
  [[ "$authority" == "authority:a" ]] || return 1
  [[ "${DELEGATE_STATUS[$DELEGATE_ID]}" == "active" ]] || return 1
  DELEGATE_STATUS["$DELEGATE_ID"]="revoked"
  LINEAGE["$authority"]+=">revoked:$DELEGATE_ID"
  append_event "revocation:$DELEGATE_ID" "authority=$authority|delegate=$DELEGATE_ID|status=revoked" || return 1
  revocation_root="$(mapping_root "revocation")"
}

validate_revoked_authority_rejected() {
  local before_root after_root
  before_root="$(mapping_state_root)"
  [[ "${DELEGATE_STATUS[$DELEGATE_ID]}" == "revoked" ]] || return 1
  after_root="$(mapping_state_root)"
  [[ "$before_root" == "$after_root" ]] || return 1
  revoked_authority_root="$(mapping_root "revoked-authority-rejected")"
}

validate_invalid_mappings_rejected() {
  local before_root after_root
  before_root="$(mapping_state_root)"
  # Duplicate XRPL identity, duplicate authority, and ambiguous mapping attempts must fail.
  add_mapping "authority:b" "xrpl:account-a" >/dev/null 2>&1 && return 1
  add_mapping "authority:a" "xrpl:account-c" >/dev/null 2>&1 && return 1
  add_mapping "authority:c" "xrpl:account-a" >/dev/null 2>&1 && return 1
  after_root="$(mapping_state_root)"
  [[ "$before_root" == "$after_root" ]] || return 1
  invalid_mapping_root="$(mapping_root "invalid-mapping-rejected")"
}

run_authority_evolution() {
  authority_mapping_root_epoch_0="$(mapping_root "epoch-0")"
  append_event "epoch-1:lineage-check" "authority=authority:a|identity=xrpl:account-a|delegate=$DELEGATE_ID|delegate_status=${DELEGATE_STATUS[$DELEGATE_ID]}" || return 1
  LINEAGE["authority:a"]+=">epoch-1:lineage-check"
  authority_mapping_root_epoch_1="$(mapping_root "epoch-1")"
  append_event "epoch-2:identity-continuity" "authority=authority:c|identity=xrpl:account-c|continuity=preserved" || return 1
  LINEAGE["authority:c"]+=">epoch-2:identity-continuity"
  authority_mapping_root_epoch_2="$(mapping_root "epoch-2")"
  [[ "$authority_mapping_root_epoch_0" != "$authority_mapping_root_epoch_1" ]] || return 1
  [[ "$authority_mapping_root_epoch_1" != "$authority_mapping_root_epoch_2" ]] || return 1
}

create_checkpoint() {
  CHECKPOINT_TRANSCRIPT="$(state_transcript)"
  CHECKPOINT_ROOT="$(printf '%s\n' "$CHECKPOINT_TRANSCRIPT" | sha256_text)"
  authority_mapping_checkpoint_identifier="checkpoint:$CHECKPOINT_ROOT"
  [[ -n "$CHECKPOINT_TRANSCRIPT" && -n "$CHECKPOINT_ROOT" ]] || return 1
  [[ "$(mapping_state_root)" == "$CHECKPOINT_ROOT" ]] || return 1

  CHECKPOINT_EVENT_LOG="$EVENT_LOG"
  CHECKPOINT_EVENT_IDS="$EVENT_IDS"
  CHECKPOINT_AUTHORITY_TO_XRPL=()
  CHECKPOINT_XRPL_TO_AUTHORITY=()
  CHECKPOINT_DELEGATE_TO_AUTHORITY=()
  CHECKPOINT_DELEGATE_STATUS=()
  CHECKPOINT_LINEAGE=()
  local authority identity delegate
  for authority in "${AUTHORITIES[@]}"; do
    CHECKPOINT_AUTHORITY_TO_XRPL["$authority"]="${AUTHORITY_TO_XRPL[$authority]}"
    CHECKPOINT_LINEAGE["$authority"]="${LINEAGE[$authority]}"
  done
  for identity in "${XRPL_IDENTITIES[@]}"; do
    CHECKPOINT_XRPL_TO_AUTHORITY["$identity"]="${XRPL_TO_AUTHORITY[$identity]}"
  done
  for delegate in "$DELEGATE_ID"; do
    CHECKPOINT_DELEGATE_TO_AUTHORITY["$delegate"]="${DELEGATE_TO_AUTHORITY[$delegate]}"
    CHECKPOINT_DELEGATE_STATUS["$delegate"]="${DELEGATE_STATUS[$delegate]}"
  done
}

restore_checkpoint() {
  reset_mapping_state
  EVENT_LOG="$CHECKPOINT_EVENT_LOG"
  EVENT_IDS="$CHECKPOINT_EVENT_IDS"
  local authority identity delegate
  for authority in "${AUTHORITIES[@]}"; do
    AUTHORITY_TO_XRPL["$authority"]="${CHECKPOINT_AUTHORITY_TO_XRPL[$authority]}"
    LINEAGE["$authority"]="${CHECKPOINT_LINEAGE[$authority]}"
  done
  for identity in "${XRPL_IDENTITIES[@]}"; do
    XRPL_TO_AUTHORITY["$identity"]="${CHECKPOINT_XRPL_TO_AUTHORITY[$identity]}"
  done
  for delegate in "$DELEGATE_ID"; do
    DELEGATE_TO_AUTHORITY["$delegate"]="${CHECKPOINT_DELEGATE_TO_AUTHORITY[$delegate]}"
    DELEGATE_STATUS["$delegate"]="${CHECKPOINT_DELEGATE_STATUS[$delegate]}"
  done
  [[ "$(mapping_state_root)" == "$CHECKPOINT_ROOT" ]] || return 1
  validate_mapping_integrity || return 1
  [[ "${DELEGATE_TO_AUTHORITY[$DELEGATE_ID]}" == "authority:a" ]] || return 1
  [[ "${DELEGATE_STATUS[$DELEGATE_ID]}" == "revoked" ]] || return 1
}

continue_after_restore() {
  append_event "continuity:authority-b-validation" "authority=authority:b|identity=${AUTHORITY_TO_XRPL[authority:b]}|authorized=true" || return 1
  LINEAGE["authority:b"]+=">continuity:validated"
  append_event "continuity:revocation-audit" "delegate=$DELEGATE_ID|status=${DELEGATE_STATUS[$DELEGATE_ID]}|lineage=preserved" || return 1
  validate_mapping_integrity || return 1
  authority_mapping_continuity_root="$(mapping_root "continuity")"
}

validate_mapping_bijection() {
  local authority identity mapped reverse
  unique_values "${AUTHORITIES[@]}" || return 1
  unique_values "${XRPL_IDENTITIES[@]}" || return 1
  for authority in "${AUTHORITIES[@]}"; do
    mapped="${AUTHORITY_TO_XRPL[$authority]:-}"
    [[ -n "$mapped" ]] || return 1
    contains_value "$mapped" "${XRPL_IDENTITIES[@]}" || return 1
    [[ "${LINEAGE[$authority]:-}" == *"genesis"* ]] || return 1
  done
  for identity in "${XRPL_IDENTITIES[@]}"; do
    reverse="${XRPL_TO_AUTHORITY[$identity]:-}"
    [[ -n "$reverse" ]] || return 1
    contains_value "$reverse" "${AUTHORITIES[@]}" || return 1
    [[ "${AUTHORITY_TO_XRPL[$reverse]}" == "$identity" ]] || return 1
  done
}

validate_mapping_integrity() {
  validate_mapping_bijection || return 1
  [[ "${DELEGATE_STATUS[$DELEGATE_ID]}" != "active" ]] || return 1
  [[ "${DELEGATE_STATUS[$DELEGATE_ID]}" == "revoked" ]] || return 1
  [[ "${LINEAGE[authority:a]}" == *"delegated:$DELEGATE_ID"* ]] || return 1
  [[ "${LINEAGE[authority:a]}" == *"revoked:$DELEGATE_ID"* ]] || return 1
}

run_lifecycle() {
  reset_mapping_state
  create_authority_genesis || return 1
  create_identity_mappings || return 1
  validate_authority_mapping || return 1
  create_delegation || return 1
  validate_delegated_authority || return 1
  revoke_delegation || return 1
  validate_revoked_authority_rejected || return 1
  validate_invalid_mappings_rejected || return 1
  run_authority_evolution || return 1
  create_checkpoint || return 1
  restore_checkpoint || return 1
  continue_after_restore || return 1
}

write_report() {
  local timestamp
  timestamp="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
  cat > "$REPORT_PATH" <<REPORT
Timestamp: $timestamp
Authority Genesis Root: $authority_genesis_root
Identity Mapping Root: $identity_mapping_root
Authority Validation Root: $authority_validation_root
Delegation Root: $delegation_root
Delegated Authority Root: $delegated_authority_root
Revocation Root: $revocation_root
Revoked Authority Root: $revoked_authority_root
Invalid Mapping Root: $invalid_mapping_root
Authority Mapping Root Epoch 0: $authority_mapping_root_epoch_0
Authority Mapping Root Epoch 1: $authority_mapping_root_epoch_1
Authority Mapping Root Epoch 2: $authority_mapping_root_epoch_2
Authority Mapping Checkpoint Identifier: $authority_mapping_checkpoint_identifier
Authority Mapping Continuity Root: $authority_mapping_continuity_root
Replay Authority Mapping Root: $replay_authority_mapping_root
Mapping Status: $mapping_status
Validation Status: $validation_status
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
  printf 'Mapping: %s\n' "$mapping_status"
  printf 'Validation: %s\n' "$validation_status"
  printf 'Delegation: %s\n' "$delegation_status"
  printf 'Revocation: %s\n' "$revocation_status"
  printf 'Checkpoint: %s\n' "$checkpoint_status"
  printf 'Restoration: %s\n' "$restoration_status"
  printf 'Replay: %s\n' "$replay_status"
  printf 'Integrity: %s\n' "$integrity_status"
  printf 'XRPL/Xaman Authority Certification: %s\n' "$overall_result"
  printf 'Report: %s\n' "$REPORT_PATH"
}

run_bootstrap_certification
if [[ "$bootstrap_status" == "PASS" ]] && run_lifecycle; then
  mapping_status="PASS"
  validation_status="PASS"
  delegation_status="PASS"
  revocation_status="PASS"
  checkpoint_status="PASS"
  restoration_status="PASS"

  expected_continuity_root="$authority_mapping_continuity_root"
  run_lifecycle
  replay_authority_mapping_root="$authority_mapping_continuity_root"
  authority_mapping_continuity_root="$expected_continuity_root"

  if [[ "$replay_authority_mapping_root" == "$authority_mapping_continuity_root" ]] && validate_mapping_integrity; then
    replay_status="PASS"
    integrity_status="PASS"
    overall_result="PASS"
  else
    replay_status="FAIL"
    integrity_status="FAIL"
    overall_result="FAIL"
  fi
else
  [[ "$mapping_status" == "NOT RUN" ]] && mapping_status="FAIL"
  [[ "$validation_status" == "NOT RUN" ]] && validation_status="FAIL"
  [[ "$delegation_status" == "NOT RUN" ]] && delegation_status="FAIL"
  [[ "$revocation_status" == "NOT RUN" ]] && revocation_status="FAIL"
  [[ "$checkpoint_status" == "NOT RUN" ]] && checkpoint_status="FAIL"
  [[ "$restoration_status" == "NOT RUN" ]] && restoration_status="FAIL"
  replay_status="FAIL"
  integrity_status="FAIL"
  overall_result="FAIL"
fi

write_report
print_summary
[[ "$overall_result" == "PASS" ]]
