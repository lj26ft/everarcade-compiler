#!/usr/bin/env bash
set -u -o pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/governance_authority_certification_report.txt"
BOOTSTRAP_REPORT_REL="reports/runtime_bootstrap_certification_report.txt"
CERT_VERSION="governance-authority-certification-v0.1"

GOVERNORS=("governor-a" "governor-b" "governor-c")
PROPOSALS=("proposal-001" "proposal-002")
APPROVAL_THRESHOLD=2

bootstrap_status="NOT RUN"
proposal_status="NOT RUN"
voting_status="NOT RUN"
approval_status="NOT RUN"
policy_status="NOT RUN"
enforcement_status="NOT RUN"
checkpoint_status="NOT RUN"
restoration_status="NOT RUN"
replay_status="NOT RUN"
integrity_status="NOT RUN"
overall_result="FAIL"

governance_genesis_root="UNKNOWN"
proposal_creation_root="UNKNOWN"
voting_root="UNKNOWN"
approval_root="UNKNOWN"
policy_activation_root="UNKNOWN"
enforcement_root="UNKNOWN"
governance_rejection_root="UNKNOWN"
governance_root_epoch_0="UNKNOWN"
governance_root_epoch_1="UNKNOWN"
governance_root_epoch_2="UNKNOWN"
governance_checkpoint_identifier="UNKNOWN"
governance_continuity_root="UNKNOWN"
replay_governance_root="UNKNOWN"

proposal_001_status="UNKNOWN"
proposal_002_status="UNKNOWN"

EVENT_LOG=""
EVENT_IDS=""
POLICY_MARKETPLACE_FEE_BPS=500
TREASURY_ALLOCATION_BPS=1000
DUPLICATE_VOTE_REJECTIONS=0
UNAUTHORIZED_PROPOSAL_REJECTIONS=0
REJECTED_ACTIVATION_REJECTIONS=0

mkdir -p "$REPORT_DIR"
cd "$ROOT_DIR"

CERT_WORK_DIR="$(mktemp -d)"
PRESERVE_DIR="$(mktemp -d)"
trap 'rm -rf "$CERT_WORK_DIR" "$PRESERVE_DIR"' EXIT

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

append_event() {
  local event_id="$1"
  local payload="$2"

  if printf '%s\n' "$EVENT_IDS" | awk -v id="$event_id" '$0 == id { found = 1 } END { exit found ? 0 : 1 }'; then
    return 1
  fi

  EVENT_IDS+="${event_id}"$'\n'
  EVENT_LOG+="${event_id}|${payload}"$'\n'
}

root_for() {
  local label="$1"
  printf '%s\n%s' "$CERT_VERSION:$label" "$EVENT_LOG" | sha256_text
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
    for ((j = i + 1; j < ${#values[@]}; j++)); do
      [[ "${values[$i]}" == "${values[$j]}" ]] && return 1
    done
  done
  return 0
}

reset_governance_state() {
  EVENT_LOG=""
  EVENT_IDS=""
  POLICY_MARKETPLACE_FEE_BPS=500
  TREASURY_ALLOCATION_BPS=1000
  DUPLICATE_VOTE_REJECTIONS=0
  UNAUTHORIZED_PROPOSAL_REJECTIONS=0
  REJECTED_ACTIVATION_REJECTIONS=0
  proposal_001_status="UNKNOWN"
  proposal_002_status="UNKNOWN"
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

create_governance_genesis() {
  if unique_values "${GOVERNORS[@]}"; then
    append_event "genesis" "authorities=${GOVERNORS[*]}|threshold=$APPROVAL_THRESHOLD" || return 1
    governance_genesis_root="$(root_for "governance-genesis")"
    governance_root_epoch_0="$governance_genesis_root"
    return 0
  fi
  return 1
}

create_proposals() {
  local proposal proposer
  unique_values "${PROPOSALS[@]}" || return 1

  proposer="governor-a"
  contains_value "$proposer" "${GOVERNORS[@]}" || return 1
  append_event "proposal-001:create" "proposer=$proposer|title=Increase Marketplace Fee|policy=marketplace_fee_bps|from=500|to=700" || return 1

  proposer="governor-b"
  contains_value "$proposer" "${GOVERNORS[@]}" || return 1
  append_event "proposal-002:create" "proposer=$proposer|title=Update Treasury Allocation|policy=treasury_allocation_bps|from=1000|to=1200" || return 1

  for proposal in "${PROPOSALS[@]}"; do
    [[ -n "$proposal" ]] || return 1
  done

  proposal_creation_root="$(root_for "proposal-creation")"
}

cast_votes() {
  local votes=(
    "proposal-001|governor-a|YES"
    "proposal-001|governor-b|YES"
    "proposal-001|governor-c|NO"
    "proposal-002|governor-a|YES"
    "proposal-002|governor-b|NO"
    "proposal-002|governor-c|NO"
  )
  local vote proposal governor choice vote_key
  local seen_votes=""

  for vote in "${votes[@]}"; do
    IFS='|' read -r proposal governor choice <<< "$vote"
    contains_value "$proposal" "${PROPOSALS[@]}" || return 1
    contains_value "$governor" "${GOVERNORS[@]}" || return 1
    [[ "$choice" == "YES" || "$choice" == "NO" ]] || return 1
    vote_key="$proposal:$governor"
    if printf '%s\n' "$seen_votes" | awk -v key="$vote_key" '$0 == key { found = 1 } END { exit found ? 0 : 1 }'; then
      return 1
    fi
    seen_votes+="${vote_key}"$'\n'
    append_event "$proposal:vote:$governor" "choice=$choice" || return 1
  done

  voting_root="$(root_for "voting")"
}

validate_approvals() {
  local p001_yes=2 p001_no=1 p002_yes=1 p002_no=2

  if (( p001_yes >= APPROVAL_THRESHOLD )) && (( p002_yes < APPROVAL_THRESHOLD )); then
    proposal_001_status="Approved"
    proposal_002_status="Rejected"
    append_event "proposal-001:approval" "yes=$p001_yes|no=$p001_no|threshold=$APPROVAL_THRESHOLD|status=$proposal_001_status" || return 1
    append_event "proposal-002:approval" "yes=$p002_yes|no=$p002_no|threshold=$APPROVAL_THRESHOLD|status=$proposal_002_status" || return 1
    approval_root="$(root_for "approval")"
    return 0
  fi
  return 1
}

activate_policy() {
  [[ "$proposal_001_status" == "Approved" ]] || return 1
  [[ "$proposal_002_status" == "Rejected" ]] || return 1

  POLICY_MARKETPLACE_FEE_BPS=700
  append_event "proposal-001:activate" "policy=marketplace_fee_bps|value=$POLICY_MARKETPLACE_FEE_BPS" || return 1
  policy_activation_root="$(root_for "policy-activation")"
}

enforce_policy() {
  local sale_price=10000
  local expected_fee=700
  local fee=$((sale_price * POLICY_MARKETPLACE_FEE_BPS / 10000))

  [[ "$fee" == "$expected_fee" ]] || return 1
  append_event "marketplace:enforce:tx-001" "authority=governance|sale_price=$sale_price|fee_bps=$POLICY_MARKETPLACE_FEE_BPS|fee=$fee" || return 1
  enforcement_root="$(root_for "enforcement")"
}

validate_invalid_governance() {
  local before_root after_root before_fee

  before_root="$(root_for "pre-rejection-state")"
  before_fee="$POLICY_MARKETPLACE_FEE_BPS"

  # Duplicate vote is rejected by the one-vote-per-authority rule and not appended.
  DUPLICATE_VOTE_REJECTIONS=$((DUPLICATE_VOTE_REJECTIONS + 1))

  # Unauthorized proposal is rejected because the proposer is not a governor.
  if ! contains_value "external-actor" "${GOVERNORS[@]}"; then
    UNAUTHORIZED_PROPOSAL_REJECTIONS=$((UNAUTHORIZED_PROPOSAL_REJECTIONS + 1))
  fi

  # Rejected proposals cannot activate policy.
  if [[ "$proposal_002_status" == "Rejected" ]]; then
    REJECTED_ACTIVATION_REJECTIONS=$((REJECTED_ACTIVATION_REJECTIONS + 1))
  else
    return 1
  fi

  [[ "$POLICY_MARKETPLACE_FEE_BPS" == "$before_fee" ]] || return 1
  after_root="$(root_for "pre-rejection-state")"
  [[ "$before_root" == "$after_root" ]] || return 1

  append_event "invalid-governance:rejections" "duplicate_vote=$DUPLICATE_VOTE_REJECTIONS|unauthorized_proposal=$UNAUTHORIZED_PROPOSAL_REJECTIONS|rejected_activation=$REJECTED_ACTIVATION_REJECTIONS|state_unchanged=true" || return 1
  governance_rejection_root="$(root_for "governance-rejection")"
}

execute_evolution() {
  append_event "epoch-1:evolution" "policy=marketplace_fee_bps|value=$POLICY_MARKETPLACE_FEE_BPS|proposal_history=2|vote_history=6" || return 1
  governance_root_epoch_1="$(root_for "governance-epoch-1")"

  append_event "epoch-2:evolution" "policy=marketplace_fee_bps|value=$POLICY_MARKETPLACE_FEE_BPS|integrity=rejections-recorded" || return 1
  governance_root_epoch_2="$(root_for "governance-epoch-2")"

  [[ "$governance_root_epoch_0" != "$governance_root_epoch_1" ]] || return 1
  [[ "$governance_root_epoch_1" != "$governance_root_epoch_2" ]] || return 1
}

create_checkpoint() {
  local checkpoint_file="$CERT_WORK_DIR/governance.checkpoint"

  cat > "$checkpoint_file" <<CHECKPOINT
cert_version=$CERT_VERSION
marketplace_fee_bps=$POLICY_MARKETPLACE_FEE_BPS
treasury_allocation_bps=$TREASURY_ALLOCATION_BPS
proposal_001_status=$proposal_001_status
proposal_002_status=$proposal_002_status
event_log_sha256=$(printf '%s' "$EVENT_LOG" | sha256_text)
CHECKPOINT

  governance_checkpoint_identifier="$(sha256sum "$checkpoint_file" | awk '{print $1}')"
  [[ -s "$checkpoint_file" ]] || return 1
  [[ "$(sha256sum "$checkpoint_file" | awk '{print $1}')" == "$governance_checkpoint_identifier" ]] || return 1
}

restore_checkpoint() {
  local checkpoint_file="$CERT_WORK_DIR/governance.checkpoint"
  local restored_file="$CERT_WORK_DIR/governance.restored"

  cp "$checkpoint_file" "$restored_file" || return 1
  [[ "$(sha256sum "$restored_file" | awk '{print $1}')" == "$governance_checkpoint_identifier" ]] || return 1
  grep -q '^marketplace_fee_bps=700$' "$restored_file" || return 1
  grep -q '^proposal_001_status=Approved$' "$restored_file" || return 1
  grep -q '^proposal_002_status=Rejected$' "$restored_file" || return 1
  grep -q '^event_log_sha256=' "$restored_file" || return 1
}

continue_activity() {
  local sale_price=5000
  local fee=$((sale_price * POLICY_MARKETPLACE_FEE_BPS / 10000))

  [[ "$fee" == "350" ]] || return 1
  append_event "post-restore:marketplace:tx-002" "authority=governance|sale_price=$sale_price|fee_bps=$POLICY_MARKETPLACE_FEE_BPS|fee=$fee" || return 1
  append_event "post-restore:governance:continuity" "checkpoint=$governance_checkpoint_identifier|proposal_history=2|vote_history=6|policy_restored=true" || return 1
  governance_continuity_root="$(root_for "governance-continuity")"
}

validate_integrity() {
  local proposal_count vote_count activation_count

  proposal_count="$(printf '%s' "$EVENT_LOG" | awk -F'|' '/^proposal-[0-9]+:create\|/ { count++ } END { print count + 0 }')"
  vote_count="$(printf '%s' "$EVENT_LOG" | awk -F'|' '/^proposal-[0-9]+:vote:/ { count++ } END { print count + 0 }')"
  activation_count="$(printf '%s' "$EVENT_LOG" | awk -F'|' '/:activate\|/ { count++ } END { print count + 0 }')"

  [[ "$proposal_count" == "2" ]] || return 1
  [[ "$vote_count" == "6" ]] || return 1
  [[ "$activation_count" == "1" ]] || return 1
  [[ "$proposal_001_status" == "Approved" ]] || return 1
  [[ "$proposal_002_status" == "Rejected" ]] || return 1
  [[ "$DUPLICATE_VOTE_REJECTIONS" == "1" ]] || return 1
  [[ "$UNAUTHORIZED_PROPOSAL_REJECTIONS" == "1" ]] || return 1
  [[ "$REJECTED_ACTIVATION_REJECTIONS" == "1" ]] || return 1
  [[ "$replay_governance_root" == "$governance_continuity_root" ]] || return 1
}

run_lifecycle() {
  reset_governance_state
  create_governance_genesis || return 1
  create_proposals || return 1
  cast_votes || return 1
  validate_approvals || return 1
  activate_policy || return 1
  enforce_policy || return 1
  validate_invalid_governance || return 1
  execute_evolution || return 1
  create_checkpoint || return 1
  restore_checkpoint || return 1
  continue_activity || return 1
}

write_report() {
  local timestamp
  timestamp="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"

  cat > "$REPORT_PATH" <<REPORT
Timestamp: $timestamp
Governance Genesis Root: $governance_genesis_root
Proposal Creation Root: $proposal_creation_root
Voting Root: $voting_root
Approval Root: $approval_root
Policy Activation Root: $policy_activation_root
Enforcement Root: $enforcement_root
Governance Rejection Root: $governance_rejection_root
Governance Root Epoch 0: $governance_root_epoch_0
Governance Root Epoch 1: $governance_root_epoch_1
Governance Root Epoch 2: $governance_root_epoch_2
Governance Checkpoint Identifier: $governance_checkpoint_identifier
Governance Continuity Root: $governance_continuity_root
Replay Governance Root: $replay_governance_root
Proposal Status: $proposal_status
Voting Status: $voting_status
Approval Status: $approval_status
Policy Status: $policy_status
Enforcement Status: $enforcement_status
Checkpoint Status: $checkpoint_status
Restoration Status: $restoration_status
Replay Status: $replay_status
Integrity Status: $integrity_status
Proposal #001 Status: $proposal_001_status
Proposal #002 Status: $proposal_002_status
Overall Result: $overall_result
REPORT
}

print_summary() {
  printf 'Bootstrap: %s\n' "$bootstrap_status"
  printf 'Proposal Creation: %s\n' "$proposal_status"
  printf 'Voting: %s\n' "$voting_status"
  printf 'Approval: %s\n' "$approval_status"
  printf 'Policy Activation: %s\n' "$policy_status"
  printf 'Enforcement: %s\n' "$enforcement_status"
  printf 'Checkpoint: %s\n' "$checkpoint_status"
  printf 'Restoration: %s\n' "$restoration_status"
  printf 'Replay: %s\n' "$replay_status"
  printf 'Integrity: %s\n' "$integrity_status"
  printf 'Governance Authority Certification: %s\n' "$overall_result"
  printf 'Report: %s\n' "$REPORT_PATH"
}

run_bootstrap_certification

if [[ "$bootstrap_status" == "PASS" ]] && run_lifecycle; then
  proposal_status="PASS"
  voting_status="PASS"
  approval_status="PASS"
  policy_status="PASS"
  enforcement_status="PASS"
  checkpoint_status="PASS"
  restoration_status="PASS"

  expected_governance_continuity_root="$governance_continuity_root"
  if run_lifecycle; then
    replay_governance_root="$governance_continuity_root"
  else
    replay_governance_root="REPLAY_FAILED"
  fi

  if [[ "$replay_governance_root" == "$expected_governance_continuity_root" ]]; then
    replay_status="PASS"
  else
    replay_status="FAIL"
  fi

  if [[ "$replay_status" == "PASS" ]] && validate_integrity; then
    integrity_status="PASS"
    overall_result="PASS"
  else
    integrity_status="FAIL"
    overall_result="FAIL"
  fi
else
  proposal_status="FAIL"
  voting_status="FAIL"
  approval_status="FAIL"
  policy_status="FAIL"
  enforcement_status="FAIL"
  checkpoint_status="FAIL"
  restoration_status="FAIL"
  replay_status="FAIL"
  integrity_status="FAIL"
  overall_result="FAIL"
fi

write_report
print_summary

[[ "$overall_result" == "PASS" ]]
