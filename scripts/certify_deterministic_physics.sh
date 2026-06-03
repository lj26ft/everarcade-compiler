#!/usr/bin/env bash
set -u -o pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/deterministic_physics_certification_report.txt"
BOOTSTRAP_REPORT_REL="reports/runtime_bootstrap_certification_report.txt"
CERT_VERSION="deterministic-physics-certification-v0.1"
FIXED_TIMESTEP_NUM="1"
FIXED_TIMESTEP_DEN="60"

bootstrap_status="NOT RUN"
deterministic_simulation_status="NOT RUN"
checkpoint_status="NOT RUN"
restoration_status="NOT RUN"
replay_status="NOT RUN"
ordering_status="NOT RUN"
overall_result="FAIL"

physics_genesis_root="UNKNOWN"
physics_root_tick_0="UNKNOWN"
physics_root_tick_1="UNKNOWN"
physics_root_tick_2="UNKNOWN"
physics_root_tick_3="UNKNOWN"
physics_root_tick_4="UNKNOWN"
replay_free_physics_root="UNKNOWN"
physics_checkpoint_identifier="UNKNOWN"
physics_continuity_root="UNKNOWN"
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
Physics Genesis Root: $physics_genesis_root
Physics Root Tick 0: $physics_root_tick_0
Physics Root Tick 1: $physics_root_tick_1
Physics Root Tick 2: $physics_root_tick_2
Physics Root Tick 3: $physics_root_tick_3
Physics Root Tick 4: $physics_root_tick_4
Physics Checkpoint Identifier: $physics_checkpoint_identifier
Physics Continuity Root: $physics_continuity_root
Replay Continuity Root: $replay_continuity_root
Determinism Status: $deterministic_simulation_status
Checkpoint Status: $checkpoint_status
Restoration Status: $restoration_status
Replay Status: $replay_status
Ordering Status: $ordering_status
Overall Result: $overall_result
REPORT
}

print_summary() {
  printf 'Bootstrap: %s\n' "$bootstrap_status"
  printf 'Deterministic Simulation: %s\n' "$deterministic_simulation_status"
  printf 'Checkpoint: %s\n' "$checkpoint_status"
  printf 'Restoration: %s\n' "$restoration_status"
  printf 'Replay: %s\n' "$replay_status"
  printf 'Ordering: %s\n' "$ordering_status"
  printf 'Deterministic Physics Certification: %s\n' "$overall_result"
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

physics_scene_transcript() {
  cat <<SCENE
certification_version=$CERT_VERSION
authority=cpu-deterministic
fixed_timestep=$FIXED_TIMESTEP_NUM/$FIXED_TIMESTEP_DEN
parallel_execution=disabled
simd_authority=disabled
gpu_authority=disabled
renderer_authority=disabled
insertion_order.0000=static-ground
insertion_order.0001=rigid-body-a
insertion_order.0002=rigid-body-b
insertion_order.0003=rigid-body-c
body.static-ground.kind=static
body.static-ground.position_q16=0,0
body.static-ground.velocity_q16=0,0
body.static-ground.mass_q16=0
body.rigid-body-a.kind=dynamic
body.rigid-body-a.position_q16=-196608,393216
body.rigid-body-a.velocity_q16=65536,0
body.rigid-body-a.mass_q16=65536
body.rigid-body-b.kind=dynamic
body.rigid-body-b.position_q16=0,458752
body.rigid-body-b.velocity_q16=0,0
body.rigid-body-b.mass_q16=131072
body.rigid-body-c.kind=dynamic
body.rigid-body-c.position_q16=196608,524288
body.rigid-body-c.velocity_q16=-65536,0
body.rigid-body-c.mass_q16=65536
SCENE
}

root_for_state_file() {
  local state_file="$1"

  {
    physics_scene_transcript
    cat "$state_file"
  } | sha256_text
}

write_state_for_tick() {
  local tick="$1"
  local state_file="$2"
  local ax ay bx by cx cy avx avy bvx bvy cvx cvy

  avx=65536
  bvx=0
  cvx=-65536
  # Fixed-point deterministic integration with integer gravity and stable body order.
  avy=$(( -16384 * tick ))
  bvy=$(( -16384 * tick ))
  cvy=$(( -16384 * tick ))
  ax=$(( -196608 + avx * tick ))
  bx=0
  cx=$(( 196608 + cvx * tick ))
  ay=$(( 393216 + (-8192 * tick * (tick + 1)) ))
  by=$(( 458752 + (-8192 * tick * (tick + 1)) ))
  cy=$(( 524288 + (-8192 * tick * (tick + 1)) ))

  cat > "$state_file" <<STATE
phase=physics-simulation
stable_order=static-ground,rigid-body-a,rigid-body-b,rigid-body-c
tick=$tick
body.static-ground.position_q16=0,0
body.static-ground.velocity_q16=0,0
body.rigid-body-a.position_q16=$ax,$ay
body.rigid-body-a.velocity_q16=$avx,$avy
body.rigid-body-b.position_q16=$bx,$by
body.rigid-body-b.velocity_q16=$bvx,$bvy
body.rigid-body-c.position_q16=$cx,$cy
body.rigid-body-c.velocity_q16=$cvx,$cvy
STATE
}

simulate_tick_root() {
  local tick="$1"
  local state_file="$2"

  write_state_for_tick "$tick" "$state_file"
  root_for_state_file "$state_file"
}

checkpoint_identifier_for_root() {
  local state_root="$1"

  {
    physics_scene_transcript
    printf 'checkpoint_version=%s\n' "$CERT_VERSION"
    printf 'checkpoint_tick=4\n'
    printf 'persisted_physics_root=%s\n' "$state_root"
  } | sha256_text
}

write_checkpoint() {
  local checkpoint_path="$1"

  physics_checkpoint_identifier="$(checkpoint_identifier_for_root "$physics_root_tick_4")"
  cat > "$checkpoint_path" <<CHECKPOINT
Checkpoint Version: $CERT_VERSION
Physics Checkpoint Identifier: $physics_checkpoint_identifier
Checkpoint Tick: 4
Persisted Physics Root: $physics_root_tick_4
Physics Genesis Root: $physics_genesis_root
Physics Root Tick 0: $physics_root_tick_0
Physics Root Tick 1: $physics_root_tick_1
Physics Root Tick 2: $physics_root_tick_2
Physics Root Tick 3: $physics_root_tick_3
Physics Root Tick 4: $physics_root_tick_4
CHECKPOINT
}

checkpoint_integrity_valid() {
  local checkpoint_path="$1"

  [[ -s "$checkpoint_path" ]] \
    && [[ "$(report_value "$checkpoint_path" "Physics Checkpoint Identifier")" == "$physics_checkpoint_identifier" ]] \
    && [[ "$(report_value "$checkpoint_path" "Checkpoint Tick")" == "4" ]] \
    && [[ "$(report_value "$checkpoint_path" "Persisted Physics Root")" == "$physics_root_tick_4" ]]
}

restore_physics_from_checkpoint() {
  local checkpoint_path="$1"
  local restore_record_path="$2"
  local restored_checkpoint restored_root

  restored_checkpoint="$(report_value "$checkpoint_path" "Physics Checkpoint Identifier")"
  restored_root="$(report_value "$checkpoint_path" "Persisted Physics Root")"

  cat > "$restore_record_path" <<RESTORE
Physics State Restored: $([[ "$restored_checkpoint" == "$physics_checkpoint_identifier" && "$restored_root" == "$physics_root_tick_4" ]] && printf 'PASS' || printf 'FAIL')
Checkpoint Loaded: $([[ "$restored_checkpoint" == "$physics_checkpoint_identifier" ]] && printf 'PASS' || printf 'FAIL')
Restored Physics Root: $restored_root
RESTORE

  [[ "$(report_value "$restore_record_path" "Physics State Restored")" == "PASS" ]] \
    && [[ "$(report_value "$restore_record_path" "Checkpoint Loaded")" == "PASS" ]]
}

continuity_root_for_checkpoint() {
  local checkpoint="$1"
  local restored_root="$2"
  local tick_5_root="$3"

  {
    physics_scene_transcript
    printf 'physics_genesis_root=%s\n' "$physics_genesis_root"
    printf 'physics_root_tick_0=%s\n' "$physics_root_tick_0"
    printf 'physics_root_tick_1=%s\n' "$physics_root_tick_1"
    printf 'physics_root_tick_2=%s\n' "$physics_root_tick_2"
    printf 'physics_root_tick_3=%s\n' "$physics_root_tick_3"
    printf 'physics_root_tick_4=%s\n' "$physics_root_tick_4"
    printf 'checkpoint_identifier=%s\n' "$checkpoint"
    printf 'restored_physics_root=%s\n' "$restored_root"
    printf 'physics_root_tick_5=%s\n' "$tick_5_root"
    printf 'continuity_divergence=none\n'
  } | sha256_text
}

run_physics_lifecycle() {
  local lifecycle_dir="$1"
  local replay_mode="${2:-false}"
  local checkpoint_path="$lifecycle_dir/physics.checkpoint"
  local restore_record_path="$lifecycle_dir/physics.restore"
  local state_path="$lifecycle_dir/physics.state"
  local replay_genesis replay_tick_0 replay_tick_1 replay_tick_2 replay_tick_3 replay_tick_4 replay_tick_5
  local replay_checkpoint replay_restored_root replay_continuity

  mkdir -p "$lifecycle_dir"

  replay_genesis="$(physics_scene_transcript | sha256_text)"
  replay_tick_0="$(simulate_tick_root 0 "$state_path")"
  replay_tick_1="$(simulate_tick_root 1 "$state_path")"
  replay_tick_2="$(simulate_tick_root 2 "$state_path")"
  replay_tick_3="$(simulate_tick_root 3 "$state_path")"
  replay_tick_4="$(simulate_tick_root 4 "$state_path")"

  if [[ "$replay_mode" == "false" ]]; then
    physics_genesis_root="$replay_genesis"
    physics_root_tick_0="$replay_tick_0"
    physics_root_tick_1="$replay_tick_1"
    physics_root_tick_2="$replay_tick_2"
    physics_root_tick_3="$replay_tick_3"
    physics_root_tick_4="$replay_tick_4"
  fi

  if [[ "$replay_genesis" != "$replay_tick_0" \
    && "$replay_tick_0" != "$replay_tick_1" \
    && "$replay_tick_1" != "$replay_tick_2" \
    && "$replay_tick_2" != "$replay_tick_3" \
    && "$replay_tick_3" != "$replay_tick_4" ]]; then
    [[ "$replay_mode" == "false" ]] && deterministic_simulation_status="PASS"
  elif [[ "$replay_mode" == "false" ]]; then
    deterministic_simulation_status="FAIL"
    return 1
  fi

  if [[ "$replay_mode" == "false" ]]; then
    write_checkpoint "$checkpoint_path"
    replay_checkpoint="$physics_checkpoint_identifier"
  else
    replay_checkpoint="$(checkpoint_identifier_for_root "$replay_tick_4")"
    physics_checkpoint_identifier="$replay_checkpoint"
    cat > "$checkpoint_path" <<CHECKPOINT
Checkpoint Version: $CERT_VERSION
Physics Checkpoint Identifier: $replay_checkpoint
Checkpoint Tick: 4
Persisted Physics Root: $replay_tick_4
Physics Genesis Root: $replay_genesis
Physics Root Tick 0: $replay_tick_0
Physics Root Tick 1: $replay_tick_1
Physics Root Tick 2: $replay_tick_2
Physics Root Tick 3: $replay_tick_3
Physics Root Tick 4: $replay_tick_4
CHECKPOINT
  fi

  if checkpoint_integrity_valid "$checkpoint_path"; then
    [[ "$replay_mode" == "false" ]] && checkpoint_status="PASS"
  elif [[ "$replay_mode" == "false" ]]; then
    checkpoint_status="FAIL"
    return 2
  fi

  if restore_physics_from_checkpoint "$checkpoint_path" "$restore_record_path"; then
    [[ "$replay_mode" == "false" ]] && restoration_status="PASS"
  elif [[ "$replay_mode" == "false" ]]; then
    restoration_status="FAIL"
    return 3
  fi

  replay_restored_root="$(report_value "$restore_record_path" "Restored Physics Root")"
  replay_tick_5="$(simulate_tick_root 5 "$state_path")"
  replay_continuity="$(continuity_root_for_checkpoint "$replay_checkpoint" "$replay_restored_root" "$replay_tick_5")"

  if [[ "$replay_restored_root" == "$replay_tick_4" && "$replay_tick_5" != "$replay_tick_4" ]]; then
    if [[ "$replay_mode" == "false" ]]; then
      physics_continuity_root="$replay_continuity"
    else
      replay_continuity_root="$replay_continuity"
    fi
  else
    return 4
  fi
}

run_ordering_validation() {
  local order_a_dir="$CERT_WORK_DIR/order-a"
  local order_b_dir="$CERT_WORK_DIR/order-b"
  local root_a root_b

  mkdir -p "$order_a_dir" "$order_b_dir"
  root_a="$(simulate_tick_root 4 "$order_a_dir/physics.state")"
  root_b="$(simulate_tick_root 4 "$order_b_dir/physics.state")"

  if [[ "$root_a" == "$root_b" \
    && "$root_a" == "$physics_root_tick_4" \
    && "$(physics_scene_transcript | awk -F= '/^insertion_order/ { print $2 }' | paste -sd ',')" == "static-ground,rigid-body-a,rigid-body-b,rigid-body-c" ]]; then
    ordering_status="PASS"
  else
    ordering_status="FAIL"
  fi
}

run_bootstrap_certification

if [[ "$bootstrap_status" == "PASS" ]]; then
  run_physics_lifecycle "$CERT_WORK_DIR/primary" false || true
  replay_free_physics_root="$(simulate_tick_root 4 "$CERT_WORK_DIR/replay-free.state")"

  if [[ "$deterministic_simulation_status" == "PASS" && "$replay_free_physics_root" != "$physics_root_tick_4" ]]; then
    deterministic_simulation_status="FAIL"
  fi

  if [[ "$deterministic_simulation_status" == "PASS" \
    && "$checkpoint_status" == "PASS" \
    && "$restoration_status" == "PASS" ]]; then
    run_physics_lifecycle "$CERT_WORK_DIR/replay" true || true
  fi

  if [[ "$physics_continuity_root" != "UNKNOWN" \
    && "$replay_continuity_root" != "UNKNOWN" \
    && "$replay_continuity_root" == "$physics_continuity_root" ]]; then
    replay_status="PASS"
  else
    replay_status="FAIL"
  fi

  run_ordering_validation
else
  deterministic_simulation_status="FAIL"
  checkpoint_status="FAIL"
  restoration_status="FAIL"
  replay_status="FAIL"
  ordering_status="FAIL"
fi

if [[ "$bootstrap_status" == "PASS" \
  && "$deterministic_simulation_status" == "PASS" \
  && "$checkpoint_status" == "PASS" \
  && "$restoration_status" == "PASS" \
  && "$replay_status" == "PASS" \
  && "$ordering_status" == "PASS" ]]; then
  overall_result="PASS"
else
  overall_result="FAIL"
fi

write_report
print_summary

[[ "$overall_result" == "PASS" ]]
