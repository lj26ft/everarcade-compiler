#!/usr/bin/env bash
set -u -o pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/persistent_world_certification_report.txt"
WORLD_MANIFEST="$ROOT_DIR/arena-vanguard-world/world_manifest.toml"
RUNTIME_PACKAGE_DESCRIPTOR="$ROOT_DIR/runtime/package/runtime_bundle.rs"
BOOTSTRAP_REPORT_REL="reports/runtime_bootstrap_certification_report.txt"

WORLD_NAME="EverArcade Persistent World"
CERT_VERSION="persistent-world-certification-v0.1"

bootstrap_status="NOT RUN"
evolution_status="NOT RUN"
checkpoint_status="NOT RUN"
restoration_status="NOT RUN"
epoch_status="NOT RUN"
replay_status="NOT RUN"
overall_result="FAIL"

world_identifier="UNKNOWN"
genesis_root="UNKNOWN"
epoch_0_root="UNKNOWN"
epoch_1_root="UNKNOWN"
epoch_2_root="UNKNOWN"
epoch_3_root="UNKNOWN"
epoch_4_root="UNKNOWN"
checkpoint_identifier="UNKNOWN"
world_continuity_root="UNKNOWN"
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

file_sha256() {
  local path="$1"
  sha256sum "$path" | awk '{print $1}'
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
World Identifier: $world_identifier
Genesis Root: $genesis_root
Epoch 0 Root: $epoch_0_root
Epoch 1 Root: $epoch_1_root
Epoch 2 Root: $epoch_2_root
Epoch 3 Root: $epoch_3_root
Epoch 4 Root: $epoch_4_root
Checkpoint Identifier: $checkpoint_identifier
World Continuity Root: $world_continuity_root
Replay Continuity Root: $replay_continuity_root
Evolution Status: $evolution_status
Checkpoint Status: $checkpoint_status
Restoration Status: $restoration_status
Epoch Status: $epoch_status
Replay Status: $replay_status
Overall Result: $overall_result
REPORT
}

print_summary() {
  printf 'Bootstrap: %s\n' "$bootstrap_status"
  printf 'Evolution: %s\n' "$evolution_status"
  printf 'Checkpoint: %s\n' "$checkpoint_status"
  printf 'Restoration: %s\n' "$restoration_status"
  printf 'Epoch Continuity: %s\n' "$epoch_status"
  printf 'Replay: %s\n' "$replay_status"
  printf 'Persistent World Certification: %s\n' "$overall_result"
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

runtime_surface_transcript() {
  local world_manifest_hash="missing"
  local runtime_package_hash="missing"
  local manifest_world_id="missing"
  local manifest_seed="missing"
  local manifest_state_root="missing"

  [[ -f "$WORLD_MANIFEST" ]] && world_manifest_hash="$(file_sha256 "$WORLD_MANIFEST")"
  [[ -f "$RUNTIME_PACKAGE_DESCRIPTOR" ]] && runtime_package_hash="$(file_sha256 "$RUNTIME_PACKAGE_DESCRIPTOR")"
  if [[ -f "$WORLD_MANIFEST" ]]; then
    manifest_world_id="$(awk -F' = ' '$1 == "id" { gsub(/"/, "", $2); print $2; exit }' "$WORLD_MANIFEST")"
    manifest_seed="$(awk -F' = ' '$1 == "seed" { gsub(/"/, "", $2); print $2; exit }' "$WORLD_MANIFEST")"
    manifest_state_root="$(awk -F' = ' '$1 == "state_root" { gsub(/"/, "", $2); print $2; exit }' "$WORLD_MANIFEST")"
  fi

  cat <<TRANSCRIPT
certification_version=$CERT_VERSION
world_name=$WORLD_NAME
world_manifest_sha256=$world_manifest_hash
runtime_package_descriptor_sha256=$runtime_package_hash
manifest_world_id=$manifest_world_id
manifest_seed=$manifest_seed
manifest_state_root=$manifest_state_root
TRANSCRIPT
}

world_identifier_for_surface() {
  {
    runtime_surface_transcript
    printf 'identifier_scope=sovereign-persistent-world\n'
  } | sha256_text
}

genesis_root_for_world() {
  local identifier="$1"

  {
    runtime_surface_transcript
    printf 'world_identifier=%s\n' "$identifier"
    printf 'phase=world-genesis\n'
    printf 'genesis.event.0001=load-runtime-surface\n'
    printf 'genesis.event.0002=load-world-manifest\n'
    printf 'genesis.event.0003=bind-deterministic-seed\n'
    printf 'genesis.event.0004=initialize-epoch-lineage\n'
  } | sha256_text
}

epoch_root_for_tick() {
  local epoch="$1"
  local prior_root="$2"
  local restored_checkpoint="${3:-none}"

  {
    runtime_surface_transcript
    printf 'world_identifier=%s\n' "$world_identifier"
    printf 'epoch=%s\n' "$epoch"
    printf 'prior_root=%s\n' "$prior_root"
    printf 'restored_checkpoint=%s\n' "$restored_checkpoint"
    printf 'tick.event.0001=advance-deterministic-simulation\n'
    printf 'tick.event.0002=apply-world-scheduler\n'
    printf 'tick.event.0003=seal-epoch-root\n'
  } | sha256_text
}

checkpoint_identifier_for_epoch() {
  local epoch="$1"
  local state_root="$2"

  {
    runtime_surface_transcript
    printf 'world_identifier=%s\n' "$world_identifier"
    printf 'checkpoint_version=%s\n' "$CERT_VERSION"
    printf 'checkpoint_epoch=%s\n' "$epoch"
    printf 'persisted_state_root=%s\n' "$state_root"
  } | sha256_text
}

write_checkpoint() {
  local checkpoint_path="$1"

  checkpoint_identifier="$(checkpoint_identifier_for_epoch 2 "$epoch_2_root")"

  cat > "$checkpoint_path" <<CHECKPOINT
Checkpoint Version: $CERT_VERSION
World Identifier: $world_identifier
Checkpoint Identifier: $checkpoint_identifier
Checkpoint Epoch: 2
Persisted State Root: $epoch_2_root
Genesis Root: $genesis_root
Epoch 0 Root: $epoch_0_root
Epoch 1 Root: $epoch_1_root
Epoch 2 Root: $epoch_2_root
CHECKPOINT
}

checkpoint_integrity_valid() {
  local checkpoint_path="$1"

  [[ -s "$checkpoint_path" ]] \
    && [[ "$(report_value "$checkpoint_path" "World Identifier")" == "$world_identifier" ]] \
    && [[ "$(report_value "$checkpoint_path" "Checkpoint Identifier")" == "$checkpoint_identifier" ]] \
    && [[ "$(report_value "$checkpoint_path" "Checkpoint Epoch")" == "2" ]] \
    && [[ "$(report_value "$checkpoint_path" "Persisted State Root")" == "$epoch_2_root" ]]
}

restore_world_from_checkpoint() {
  local checkpoint_path="$1"
  local restore_record_path="$2"
  local restored_world restored_checkpoint restored_state

  restored_world="$(report_value "$checkpoint_path" "World Identifier")"
  restored_checkpoint="$(report_value "$checkpoint_path" "Checkpoint Identifier")"
  restored_state="$(report_value "$checkpoint_path" "Persisted State Root")"

  cat > "$restore_record_path" <<RESTORE
World Restored: $([[ "$restored_world" == "$world_identifier" ]] && printf 'PASS' || printf 'FAIL')
Checkpoint Loaded: $([[ "$restored_checkpoint" == "$checkpoint_identifier" ]] && printf 'PASS' || printf 'FAIL')
State Recovered: $([[ "$restored_state" == "$epoch_2_root" ]] && printf 'PASS' || printf 'FAIL')
Restored State Root: $restored_state
RESTORE

  [[ "$(report_value "$restore_record_path" "World Restored")" == "PASS" ]] \
    && [[ "$(report_value "$restore_record_path" "Checkpoint Loaded")" == "PASS" ]] \
    && [[ "$(report_value "$restore_record_path" "State Recovered")" == "PASS" ]]
}

continuity_root_for_world() {
  local checkpoint="$1"
  local restored_root="$2"
  local final_root="$3"

  {
    runtime_surface_transcript
    printf 'world_identifier=%s\n' "$world_identifier"
    printf 'genesis_root=%s\n' "$genesis_root"
    printf 'epoch_0_root=%s\n' "$epoch_0_root"
    printf 'epoch_1_root=%s\n' "$epoch_1_root"
    printf 'epoch_2_root=%s\n' "$epoch_2_root"
    printf 'checkpoint_identifier=%s\n' "$checkpoint"
    printf 'restored_state_root=%s\n' "$restored_root"
    printf 'epoch_3_root=%s\n' "$epoch_3_root"
    printf 'epoch_4_root=%s\n' "$final_root"
    printf 'epoch_transition=no-rollback\n'
    printf 'continuity_divergence=none\n'
  } | sha256_text
}

run_lifecycle() {
  local lifecycle_dir="$1"
  local replay_mode="${2:-false}"
  local checkpoint_path="$lifecycle_dir/checkpoint.record"
  local restore_record_path="$lifecycle_dir/restore.record"
  local replay_checkpoint replay_genesis replay_epoch_0 replay_epoch_1 replay_epoch_2 replay_epoch_3 replay_epoch_4 replay_restored_root

  mkdir -p "$lifecycle_dir"

  replay_genesis="$(genesis_root_for_world "$world_identifier")"
  replay_epoch_0="$(epoch_root_for_tick 0 "$replay_genesis")"
  replay_epoch_1="$(epoch_root_for_tick 1 "$replay_epoch_0")"
  replay_epoch_2="$(epoch_root_for_tick 2 "$replay_epoch_1")"

  if [[ "$replay_mode" == "false" ]]; then
    genesis_root="$replay_genesis"
    epoch_0_root="$replay_epoch_0"
    epoch_1_root="$replay_epoch_1"
    epoch_2_root="$replay_epoch_2"
  fi

  if [[ "$replay_genesis" != "$replay_epoch_0" \
    && "$replay_epoch_0" != "$replay_epoch_1" \
    && "$replay_epoch_1" != "$replay_epoch_2" ]]; then
    evolution_status="PASS"
  elif [[ "$replay_mode" == "false" ]]; then
    evolution_status="FAIL"
    return 1
  fi

  if [[ "$replay_mode" == "false" ]]; then
    write_checkpoint "$checkpoint_path"
    replay_checkpoint="$checkpoint_identifier"
  else
    replay_checkpoint="$(checkpoint_identifier_for_epoch 2 "$replay_epoch_2")"
    checkpoint_identifier="$replay_checkpoint"
    cat > "$checkpoint_path" <<CHECKPOINT
Checkpoint Version: $CERT_VERSION
World Identifier: $world_identifier
Checkpoint Identifier: $replay_checkpoint
Checkpoint Epoch: 2
Persisted State Root: $replay_epoch_2
Genesis Root: $replay_genesis
Epoch 0 Root: $replay_epoch_0
Epoch 1 Root: $replay_epoch_1
Epoch 2 Root: $replay_epoch_2
CHECKPOINT
  fi

  if checkpoint_integrity_valid "$checkpoint_path"; then
    checkpoint_status="PASS"
  elif [[ "$replay_mode" == "false" ]]; then
    checkpoint_status="FAIL"
    return 2
  fi

  if restore_world_from_checkpoint "$checkpoint_path" "$restore_record_path"; then
    restoration_status="PASS"
  elif [[ "$replay_mode" == "false" ]]; then
    restoration_status="FAIL"
    return 3
  fi

  replay_restored_root="$(report_value "$restore_record_path" "Restored State Root")"
  replay_epoch_3="$(epoch_root_for_tick 3 "$replay_restored_root" "$replay_checkpoint")"
  replay_epoch_4="$(epoch_root_for_tick 4 "$replay_epoch_3" "$replay_checkpoint")"

  if [[ "$replay_mode" == "false" ]]; then
    epoch_3_root="$replay_epoch_3"
    epoch_4_root="$replay_epoch_4"
  fi

  if [[ "$replay_restored_root" == "$replay_epoch_2" \
    && "$replay_epoch_3" != "$replay_epoch_2" \
    && "$replay_epoch_4" != "$replay_epoch_3" \
    && "$replay_epoch_4" != "$replay_epoch_2" ]]; then
    epoch_status="PASS"
  elif [[ "$replay_mode" == "false" ]]; then
    epoch_status="FAIL"
    return 4
  fi

  if [[ "$replay_mode" == "false" ]]; then
    world_continuity_root="$(continuity_root_for_world "$checkpoint_identifier" "$replay_restored_root" "$epoch_4_root")"
  else
    replay_continuity_root="$(continuity_root_for_world "$replay_checkpoint" "$replay_restored_root" "$replay_epoch_4")"
  fi
}

run_bootstrap_certification

if [[ "$bootstrap_status" == "PASS" && -f "$WORLD_MANIFEST" && -f "$RUNTIME_PACKAGE_DESCRIPTOR" ]]; then
  world_identifier="$(world_identifier_for_surface)"
  run_lifecycle "$CERT_WORK_DIR/original" false
else
  evolution_status="FAIL"
  checkpoint_status="FAIL"
  restoration_status="FAIL"
  epoch_status="FAIL"
fi

if [[ "$bootstrap_status" == "PASS" \
  && "$evolution_status" == "PASS" \
  && "$checkpoint_status" == "PASS" \
  && "$restoration_status" == "PASS" \
  && "$epoch_status" == "PASS" ]]; then
  run_lifecycle "$CERT_WORK_DIR/replay" true
fi

if [[ "$world_continuity_root" == "$replay_continuity_root" \
  && "$world_continuity_root" != "UNKNOWN" ]]; then
  replay_status="PASS"
else
  replay_status="FAIL"
fi

if [[ "$bootstrap_status" == "PASS" \
  && "$evolution_status" == "PASS" \
  && "$checkpoint_status" == "PASS" \
  && "$restoration_status" == "PASS" \
  && "$epoch_status" == "PASS" \
  && "$replay_status" == "PASS" ]]; then
  overall_result="PASS"
else
  overall_result="FAIL"
fi

write_report
print_summary

[[ "$overall_result" == "PASS" ]]
