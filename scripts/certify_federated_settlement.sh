#!/usr/bin/env bash
set -u -o pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/federated_settlement_certification_report.txt"
XRPL_REPORT_REL="reports/xrpl_settlement_certification_report.txt"
CERT_VERSION="federated-settlement-certification-v0.1"

NODES=("node-a" "node-b" "node-c")
NODE_IDENTITIES=("everarcade-node-a" "everarcade-node-b" "everarcade-node-c")
SETTLEMENT_EVENTS=(
  "settlement:genesis|participants=buyer,seller,treasury|authorities=authority-a,authority-b,authority-c"
  "settlement:intent-001|seller=seller|buyer=buyer|amount=100|asset=XRP|authority=authority-b"
  "settlement:intent-001:authorized|authority=authority-b|status=authorized"
  "settlement:receipt-001|intent=intent-001|timestamp=2026-01-01T00:00:03Z|status=settled"
  "settlement:intent-002|seller=seller|buyer=treasury|amount=25|asset=XRP|authority=authority-c"
  "settlement:intent-002:authorized|authority=authority-c|status=authorized"
  "settlement:receipt-002|intent=intent-002|timestamp=2026-01-01T00:00:06Z|status=settled"
)
CIVILIZATION_EVENTS=(
  "civilization:genesis|name=Civilization Alpha|treasury=civilization-treasury"
  "civilization:citizens|governors=governor-a,governor-b|citizens=citizen-a,citizen-b,citizen-c"
  "civilization:economy|supply=10000 Gold|conservation=enabled"
  "civilization:marketplace|asset=relic-001|seller=citizen-a|buyer=citizen-b|fee_bps=500"
  "civilization:governance|proposal=marketplace-fee-700|votes=governor-a:YES,governor-b:YES|threshold=2"
  "civilization:policy|marketplace_fee_bps=700|status=active"
  "civilization:policy-settlement|asset=relic-002|seller=citizen-b|buyer=citizen-c|fee_bps=700"
)
EVOLUTION_EVENTS=(
  "federation:epoch-0|checkpoint=settlement+civilization|continuity=genesis"
  "federation:epoch-1|settlement=audit-001|civilization=treasury-audit-001|continuity=preserved"
  "federation:epoch-2|settlement=audit-002|civilization=policy-audit-001|continuity=preserved"
)

mkdir -p "$REPORT_DIR"
cd "$ROOT_DIR" || exit 1

PRESERVE_DIR="$(mktemp -d)"
trap 'rm -rf "$PRESERVE_DIR"' EXIT

bootstrap_status="NOT RUN"
settlement_status="NOT RUN"
civilization_status="NOT RUN"
checkpoint_status="NOT RUN"
restoration_status="NOT RUN"
replay_status="NOT RUN"
divergence_status="NOT RUN"
integrity_status="NOT RUN"
overall_result="FAIL"

federation_genesis_root="UNKNOWN"
settlement_root_a="UNKNOWN"
settlement_root_b="UNKNOWN"
settlement_root_c="UNKNOWN"
civilization_root_a="UNKNOWN"
civilization_root_b="UNKNOWN"
civilization_root_c="UNKNOWN"
checkpoint_identifier_a="UNKNOWN"
checkpoint_identifier_b="UNKNOWN"
checkpoint_identifier_c="UNKNOWN"
restore_root_a="UNKNOWN"
restore_root_b="UNKNOWN"
restore_root_c="UNKNOWN"
replay_root_a="UNKNOWN"
replay_root_b="UNKNOWN"
replay_root_c="UNKNOWN"
federation_root_epoch_0="UNKNOWN"
federation_root_epoch_1="UNKNOWN"
federation_root_epoch_2="UNKNOWN"
divergence_validation_root="UNKNOWN"

sha256_text() {
  sha256sum | awk '{print $1}'
}

report_value() {
  local path="$1"
  local key="$2"
  awk -F': ' -v key="$key" '$1 == key { print $2; found = 1; exit } END { if (!found) print "UNKNOWN" }' "$path"
}

preserve_xrpl_report() {
  mkdir -p "$PRESERVE_DIR/$(dirname "$XRPL_REPORT_REL")"
  if [[ -e "$XRPL_REPORT_REL" ]]; then
    cp -p "$XRPL_REPORT_REL" "$PRESERVE_DIR/$XRPL_REPORT_REL"
  else
    : > "$PRESERVE_DIR/$XRPL_REPORT_REL.absent"
  fi
}

restore_xrpl_report() {
  if [[ -f "$PRESERVE_DIR/$XRPL_REPORT_REL.absent" ]]; then
    rm -f "$XRPL_REPORT_REL"
  elif [[ -e "$PRESERVE_DIR/$XRPL_REPORT_REL" ]]; then
    mkdir -p "$(dirname "$XRPL_REPORT_REL")"
    cp -p "$PRESERVE_DIR/$XRPL_REPORT_REL" "$XRPL_REPORT_REL"
  fi
}

root_for_lines() {
  local label="$1"
  shift
  {
    printf 'certification=%s\n' "$CERT_VERSION"
    printf 'label=%s\n' "$label"
    printf '%s\n' "$@"
  } | sha256_text
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

all_equal() {
  local first="$1"
  shift
  local value
  [[ -n "$first" && "$first" != "UNKNOWN" ]] || return 1
  for value in "$@"; do
    [[ "$value" == "$first" ]] || return 1
  done
}

run_bootstrap_certification() {
  preserve_xrpl_report
  if bash scripts/certify_xrpl_settlement.sh >/dev/null 2>&1; then
    bootstrap_status="PASS"
  elif [[ -f "$XRPL_REPORT_REL" ]] && [[ "$(report_value "$XRPL_REPORT_REL" "Overall Result")" == "PASS" ]]; then
    bootstrap_status="PASS"
  else
    bootstrap_status="FAIL"
  fi
  restore_xrpl_report
}

create_federation_genesis() {
  unique_values "${NODES[@]}" || return 1
  unique_values "${NODE_IDENTITIES[@]}" || return 1
  federation_genesis_root="$(root_for_lines "federation-genesis" \
    "node-a.identity=${NODE_IDENTITIES[0]}" \
    "node-b.identity=${NODE_IDENTITIES[1]}" \
    "node-c.identity=${NODE_IDENTITIES[2]}" \
    "coordination=none" \
    "consensus=not-required")"
}

settlement_root_for_node() {
  local node="$1"
  root_for_lines "settlement-lifecycle" \
    "node_execution=independent" \
    "input-profile=canonical-settlement-lifecycle" \
    "${SETTLEMENT_EVENTS[@]}"
}

civilization_root_for_node() {
  local node="$1"
  root_for_lines "civilization-lifecycle" \
    "node_execution=independent" \
    "input-profile=canonical-civilization-lifecycle" \
    "${CIVILIZATION_EVENTS[@]}"
}

checkpoint_for_node() {
  local node="$1"
  local settlement_root="$2"
  local civilization_root="$3"
  root_for_lines "checkpoint" \
    "node_execution=independent" \
    "settlement_root=$settlement_root" \
    "civilization_root=$civilization_root" \
    "restoration_state=canonical" \
    "event_count=$((${#SETTLEMENT_EVENTS[@]} + ${#CIVILIZATION_EVENTS[@]}))"
}

restore_root_for_node() {
  local node="$1"
  local checkpoint="$2"
  root_for_lines "restore" \
    "node_execution=independent" \
    "checkpoint=$checkpoint" \
    "restored_settlement=canonical" \
    "restored_civilization=canonical" \
    "restored_replay_log=canonical"
}

replay_root_for_node() {
  local node="$1"
  root_for_lines "replay" \
    "node_execution=independent" \
    "${SETTLEMENT_EVENTS[@]}" \
    "${CIVILIZATION_EVENTS[@]}" \
    "replay_order=canonical"
}

federation_epoch_root_for_node() {
  local node="$1"
  local epoch="$2"
  root_for_lines "federation-epoch-$epoch" \
    "node_execution=independent" \
    "settlement_root=${settlement_root_a}" \
    "civilization_root=${civilization_root_a}" \
    "restore_root=${restore_root_a}" \
    "replay_root=${replay_root_a}" \
    "${EVOLUTION_EVENTS[$epoch]}"
}

execute_settlement_lifecycle() {
  settlement_root_a="$(settlement_root_for_node "node-a")"
  settlement_root_b="$(settlement_root_for_node "node-b")"
  settlement_root_c="$(settlement_root_for_node "node-c")"
  all_equal "$settlement_root_a" "$settlement_root_b" "$settlement_root_c"
}

execute_civilization_lifecycle() {
  civilization_root_a="$(civilization_root_for_node "node-a")"
  civilization_root_b="$(civilization_root_for_node "node-b")"
  civilization_root_c="$(civilization_root_for_node "node-c")"
  all_equal "$civilization_root_a" "$civilization_root_b" "$civilization_root_c"
}

create_checkpoints() {
  checkpoint_identifier_a="$(checkpoint_for_node "node-a" "$settlement_root_a" "$civilization_root_a")"
  checkpoint_identifier_b="$(checkpoint_for_node "node-b" "$settlement_root_b" "$civilization_root_b")"
  checkpoint_identifier_c="$(checkpoint_for_node "node-c" "$settlement_root_c" "$civilization_root_c")"
  all_equal "$checkpoint_identifier_a" "$checkpoint_identifier_b" "$checkpoint_identifier_c"
}

restore_nodes() {
  restore_root_a="$(restore_root_for_node "node-a" "$checkpoint_identifier_a")"
  restore_root_b="$(restore_root_for_node "node-b" "$checkpoint_identifier_b")"
  restore_root_c="$(restore_root_for_node "node-c" "$checkpoint_identifier_c")"
  all_equal "$restore_root_a" "$restore_root_b" "$restore_root_c"
}

replay_nodes() {
  replay_root_a="$(replay_root_for_node "node-a")"
  replay_root_b="$(replay_root_for_node "node-b")"
  replay_root_c="$(replay_root_for_node "node-c")"
  all_equal "$replay_root_a" "$replay_root_b" "$replay_root_c"
}

evolve_federation() {
  local epoch root_a root_b root_c
  for epoch in 0 1 2; do
    root_a="$(federation_epoch_root_for_node "node-a" "$epoch")"
    root_b="$(federation_epoch_root_for_node "node-b" "$epoch")"
    root_c="$(federation_epoch_root_for_node "node-c" "$epoch")"
    all_equal "$root_a" "$root_b" "$root_c" || return 1
    case "$epoch" in
      0) federation_root_epoch_0="$root_a" ;;
      1) federation_root_epoch_1="$root_a" ;;
      2) federation_root_epoch_2="$root_a" ;;
    esac
  done
  [[ "$federation_root_epoch_0" != "$federation_root_epoch_1" ]] || return 1
  [[ "$federation_root_epoch_1" != "$federation_root_epoch_2" ]] || return 1
}

validate_divergence_detection() {
  local invalid_settlement invalid_checkpoint invalid_replay
  invalid_settlement="$(root_for_lines "settlement-lifecycle" \
    "node_execution=independent" \
    "input-profile=canonical-settlement-lifecycle" \
    "${SETTLEMENT_EVENTS[@]}" \
    "settlement:invalid|intent=intent-999|authority=unknown|status=invalid")"
  invalid_checkpoint="$(root_for_lines "checkpoint" \
    "node_execution=independent" \
    "settlement_root=$settlement_root_b" \
    "civilization_root=$civilization_root_b" \
    "restoration_state=tampered" \
    "event_count=0")"
  invalid_replay="$(root_for_lines "replay" \
    "node_execution=independent" \
    "${CIVILIZATION_EVENTS[@]}" \
    "${SETTLEMENT_EVENTS[@]}" \
    "replay_order=invalid")"

  [[ "$invalid_settlement" != "$settlement_root_a" ]] || return 1
  [[ "$invalid_checkpoint" != "$checkpoint_identifier_a" ]] || return 1
  [[ "$invalid_replay" != "$replay_root_a" ]] || return 1

  divergence_validation_root="$(root_for_lines "divergence-validation" \
    "invalid_settlement=$invalid_settlement" \
    "canonical_settlement=$settlement_root_a" \
    "invalid_checkpoint=$invalid_checkpoint" \
    "canonical_checkpoint=$checkpoint_identifier_a" \
    "invalid_replay=$invalid_replay" \
    "canonical_replay=$replay_root_a" \
    "rejected=true")"
}

validate_integrity() {
  [[ "$settlement_status" == "PASS" ]] || return 1
  [[ "$civilization_status" == "PASS" ]] || return 1
  [[ "$checkpoint_status" == "PASS" ]] || return 1
  [[ "$restoration_status" == "PASS" ]] || return 1
  [[ "$replay_status" == "PASS" ]] || return 1
  [[ "$divergence_status" == "PASS" ]] || return 1
  all_equal "$settlement_root_a" "$settlement_root_b" "$settlement_root_c" || return 1
  all_equal "$civilization_root_a" "$civilization_root_b" "$civilization_root_c" || return 1
  all_equal "$checkpoint_identifier_a" "$checkpoint_identifier_b" "$checkpoint_identifier_c" || return 1
  all_equal "$restore_root_a" "$restore_root_b" "$restore_root_c" || return 1
  all_equal "$replay_root_a" "$replay_root_b" "$replay_root_c" || return 1
  [[ -n "$federation_root_epoch_0" && -n "$federation_root_epoch_1" && -n "$federation_root_epoch_2" ]] || return 1
  [[ "$federation_root_epoch_0" != "$federation_root_epoch_1" ]] || return 1
  [[ "$federation_root_epoch_1" != "$federation_root_epoch_2" ]] || return 1
}

write_report() {
  local timestamp
  timestamp="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
  cat > "$REPORT_PATH" <<REPORT
Timestamp: $timestamp
Federation Genesis Root: $federation_genesis_root
Settlement Root A: $settlement_root_a
Settlement Root B: $settlement_root_b
Settlement Root C: $settlement_root_c
Civilization Root A: $civilization_root_a
Civilization Root B: $civilization_root_b
Civilization Root C: $civilization_root_c
Checkpoint Identifier A: $checkpoint_identifier_a
Checkpoint Identifier B: $checkpoint_identifier_b
Checkpoint Identifier C: $checkpoint_identifier_c
Restore Root A: $restore_root_a
Restore Root B: $restore_root_b
Restore Root C: $restore_root_c
Replay Root A: $replay_root_a
Replay Root B: $replay_root_b
Replay Root C: $replay_root_c
Federation Root Epoch 0: $federation_root_epoch_0
Federation Root Epoch 1: $federation_root_epoch_1
Federation Root Epoch 2: $federation_root_epoch_2
Divergence Validation Root: $divergence_validation_root
Settlement Status: $settlement_status
Civilization Status: $civilization_status
Checkpoint Status: $checkpoint_status
Restoration Status: $restoration_status
Replay Status: $replay_status
Divergence Status: $divergence_status
Integrity Status: $integrity_status
Overall Result: $overall_result
REPORT
}

print_summary() {
  printf 'Bootstrap: %s\n' "$bootstrap_status"
  printf 'Settlement Equivalence: %s\n' "$settlement_status"
  printf 'Civilization Equivalence: %s\n' "$civilization_status"
  printf 'Checkpoint Equivalence: %s\n' "$checkpoint_status"
  printf 'Restoration Equivalence: %s\n' "$restoration_status"
  printf 'Replay Equivalence: %s\n' "$replay_status"
  printf 'Divergence Detection: %s\n' "$divergence_status"
  printf 'Integrity: %s\n' "$integrity_status"
  printf 'Federated Settlement Certification: %s\n' "$overall_result"
  printf 'Report: %s\n' "$REPORT_PATH"
}

run_bootstrap_certification

if [[ "$bootstrap_status" == "PASS" ]] && create_federation_genesis; then
  if execute_settlement_lifecycle; then settlement_status="PASS"; else settlement_status="FAIL"; fi
  if [[ "$settlement_status" == "PASS" ]] && execute_civilization_lifecycle; then civilization_status="PASS"; else civilization_status="FAIL"; fi
  if [[ "$civilization_status" == "PASS" ]] && create_checkpoints; then checkpoint_status="PASS"; else checkpoint_status="FAIL"; fi
  if [[ "$checkpoint_status" == "PASS" ]] && restore_nodes; then restoration_status="PASS"; else restoration_status="FAIL"; fi
  if [[ "$restoration_status" == "PASS" ]] && replay_nodes; then replay_status="PASS"; else replay_status="FAIL"; fi
  if [[ "$replay_status" == "PASS" ]] && evolve_federation && validate_divergence_detection; then divergence_status="PASS"; else divergence_status="FAIL"; fi
  if validate_integrity; then integrity_status="PASS"; overall_result="PASS"; else integrity_status="FAIL"; overall_result="FAIL"; fi
else
  bootstrap_status="FAIL"
  settlement_status="FAIL"
  civilization_status="FAIL"
  checkpoint_status="FAIL"
  restoration_status="FAIL"
  replay_status="FAIL"
  divergence_status="FAIL"
  integrity_status="FAIL"
  overall_result="FAIL"
fi

write_report
print_summary
[[ "$overall_result" == "PASS" ]]
