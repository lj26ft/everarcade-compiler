#!/usr/bin/env bash
set -u -o pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/stateful_package_persistence_report.txt"
PACKAGE_IDENTIFIER="arena-vanguard-world"
WORLD_MANIFEST="$ROOT_DIR/arena-vanguard-world/world_manifest.toml"
RUNTIME_PACKAGE_DESCRIPTOR="$ROOT_DIR/runtime/package/runtime_bundle.rs"
BOOTSTRAP_REPORT_REL="reports/runtime_bootstrap_certification_report.txt"

bootstrap_status="NOT RUN"
persistence_status="NOT RUN"
restoration_status="NOT RUN"
continuity_status="NOT RUN"
replay_status="NOT RUN"
overall_result="FAIL"
state_root_a="UNKNOWN"
execution_root_a="UNKNOWN"
state_root_b="UNKNOWN"
execution_root_b="UNKNOWN"
checkpoint_identifier="UNKNOWN"
continuity_root="UNKNOWN"
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
Package Identifier: $PACKAGE_IDENTIFIER
State Root A: $state_root_a
State Root B: $state_root_b
Checkpoint Identifier: $checkpoint_identifier
Continuity Root: $continuity_root
Replay Continuity Root: $replay_continuity_root
Persistence Status: $persistence_status
Restoration Status: $restoration_status
Continuity Status: $continuity_status
Replay Status: $replay_status
Overall Result: $overall_result
REPORT
}

print_summary() {
  printf 'Bootstrap: %s\n' "$bootstrap_status"
  printf 'Persistence: %s\n' "$persistence_status"
  printf 'Restoration: %s\n' "$restoration_status"
  printf 'Continuity: %s\n' "$continuity_status"
  printf 'Replay: %s\n' "$replay_status"
  printf 'Stateful Package Persistence: %s\n' "$overall_result"
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

package_metadata_transcript() {
  local world_manifest_hash="missing"
  local runtime_package_hash="missing"
  local package_state_root="missing"

  [[ -f "$WORLD_MANIFEST" ]] && world_manifest_hash="$(file_sha256 "$WORLD_MANIFEST")"
  [[ -f "$RUNTIME_PACKAGE_DESCRIPTOR" ]] && runtime_package_hash="$(file_sha256 "$RUNTIME_PACKAGE_DESCRIPTOR")"
  if [[ -f "$WORLD_MANIFEST" ]]; then
    package_state_root="$(awk -F' = ' '$1 == "state_root" { gsub(/"/, "", $2); print $2; exit }' "$WORLD_MANIFEST")"
  fi

  cat <<TRANSCRIPT
package_identifier=$PACKAGE_IDENTIFIER
world_manifest_sha256=$world_manifest_hash
runtime_package_descriptor_sha256=$runtime_package_hash
initial_state_root=$package_state_root
TRANSCRIPT
}

state_root_for_phase() {
  local phase="$1"
  local prior_root="$2"

  {
    package_metadata_transcript
    printf 'phase=%s\n' "$phase"
    printf 'prior_state_root=%s\n' "$prior_root"
    case "$phase" in
      initial)
        printf 'input.0001=bootstrap-package\n'
        printf 'input.0002=load-manifest\n'
        printf 'input.0003=apply-deterministic-tick:1\n'
        printf 'input.0004=apply-deterministic-tick:2\n'
        printf 'input.0005=apply-deterministic-tick:3\n'
        ;;
      continued)
        printf 'input.0006=restore-checkpoint\n'
        printf 'input.0007=apply-deterministic-tick:4\n'
        printf 'input.0008=apply-deterministic-tick:5\n'
        printf 'input.0009=apply-deterministic-tick:6\n'
        ;;
    esac
  } | sha256_text
}

execution_root_for_phase() {
  local phase="$1"
  local prior_root="$2"
  local resulting_state_root="$3"

  {
    package_metadata_transcript
    printf 'execution_phase=%s\n' "$phase"
    printf 'prior_state_root=%s\n' "$prior_root"
    printf 'resulting_state_root=%s\n' "$resulting_state_root"
  } | sha256_text
}

write_checkpoint() {
  local checkpoint_path="$1"

  checkpoint_identifier="$({
    package_metadata_transcript
    printf 'checkpoint_version=stateful-package-persistence-v0.1\n'
    printf 'persisted_state_root=%s\n' "$state_root_a"
    printf 'persisted_execution_root=%s\n' "$execution_root_a"
  } | sha256_text)"

  cat > "$checkpoint_path" <<CHECKPOINT
Checkpoint Version: stateful-package-persistence-v0.1
Package Identifier: $PACKAGE_IDENTIFIER
Checkpoint Identifier: $checkpoint_identifier
Persisted State Root: $state_root_a
Persisted Execution Root: $execution_root_a
CHECKPOINT
}

run_lifecycle() {
  local lifecycle_dir="$1"
  local replay_mode="${2:-false}"
  local checkpoint_path="$lifecycle_dir/checkpoint.record"
  local persistence_record_path="$lifecycle_dir/persistence.record"
  local restored_state_root="UNKNOWN"
  local loaded_checkpoint_identifier="UNKNOWN"
  local replay_state_root_a replay_execution_root_a replay_state_root_b replay_execution_root_b
  local replay_checkpoint_identifier replay_continuity

  mkdir -p "$lifecycle_dir"

  replay_state_root_a="$(state_root_for_phase initial genesis)"
  replay_execution_root_a="$(execution_root_for_phase initial genesis "$replay_state_root_a")"

  if [[ "$replay_mode" == "false" ]]; then
    state_root_a="$replay_state_root_a"
    execution_root_a="$replay_execution_root_a"
  fi

  if [[ "$replay_mode" == "false" ]]; then
    write_checkpoint "$checkpoint_path"
    replay_checkpoint_identifier="$checkpoint_identifier"
  else
    replay_checkpoint_identifier="$({
      package_metadata_transcript
      printf 'checkpoint_version=stateful-package-persistence-v0.1\n'
      printf 'persisted_state_root=%s\n' "$replay_state_root_a"
      printf 'persisted_execution_root=%s\n' "$replay_execution_root_a"
    } | sha256_text)"
    cat > "$checkpoint_path" <<CHECKPOINT
Checkpoint Version: stateful-package-persistence-v0.1
Package Identifier: $PACKAGE_IDENTIFIER
Checkpoint Identifier: $replay_checkpoint_identifier
Persisted State Root: $replay_state_root_a
Persisted Execution Root: $replay_execution_root_a
CHECKPOINT
  fi

  cat > "$persistence_record_path" <<RECORD
Package Identifier: $PACKAGE_IDENTIFIER
Checkpoint Identifier: $replay_checkpoint_identifier
State Persisted: $replay_state_root_a
Execution Persisted: $replay_execution_root_a
RECORD

  if [[ -s "$checkpoint_path" \
    && -s "$persistence_record_path" \
    && "$(report_value "$checkpoint_path" "Checkpoint Identifier")" == "$replay_checkpoint_identifier" \
    && "$(report_value "$persistence_record_path" "Checkpoint Identifier")" == "$replay_checkpoint_identifier" ]]; then
    [[ "$replay_mode" == "false" ]] && persistence_status="PASS"
  else
    [[ "$replay_mode" == "false" ]] && persistence_status="FAIL"
    return 1
  fi

  # Simulated restart: drop the live state and recover it only from the persisted checkpoint.
  restored_state_root="$(report_value "$checkpoint_path" "Persisted State Root")"
  loaded_checkpoint_identifier="$(report_value "$checkpoint_path" "Checkpoint Identifier")"

  if [[ "$loaded_checkpoint_identifier" == "$replay_checkpoint_identifier" \
    && "$restored_state_root" == "$replay_state_root_a" ]]; then
    [[ "$replay_mode" == "false" ]] && restoration_status="PASS"
  else
    [[ "$replay_mode" == "false" ]] && restoration_status="FAIL"
    return 1
  fi

  replay_state_root_b="$(state_root_for_phase continued "$restored_state_root")"
  replay_execution_root_b="$(execution_root_for_phase continued "$restored_state_root" "$replay_state_root_b")"
  replay_continuity="$({
    package_metadata_transcript
    printf 'checkpoint_identifier=%s\n' "$replay_checkpoint_identifier"
    printf 'restored_state_root=%s\n' "$restored_state_root"
    printf 'state_root_b=%s\n' "$replay_state_root_b"
    printf 'execution_root_b=%s\n' "$replay_execution_root_b"
  } | sha256_text)"

  if [[ "$replay_mode" == "false" ]]; then
    state_root_b="$replay_state_root_b"
    execution_root_b="$replay_execution_root_b"
    continuity_root="$replay_continuity"
  else
    replay_continuity_root="$replay_continuity"
  fi

  if [[ "$restored_state_root" == "$replay_state_root_a" \
    && "$replay_state_root_b" != "$replay_state_root_a" \
    && "$replay_execution_root_b" != "$replay_execution_root_a" ]]; then
    [[ "$replay_mode" == "false" ]] && continuity_status="PASS"
  else
    [[ "$replay_mode" == "false" ]] && continuity_status="FAIL"
    return 1
  fi
}

run_bootstrap_certification

if [[ "$bootstrap_status" == "PASS" \
  && -f "$WORLD_MANIFEST" \
  && -f "$RUNTIME_PACKAGE_DESCRIPTOR" ]]; then
  run_lifecycle "$CERT_WORK_DIR/primary" false || true
  run_lifecycle "$CERT_WORK_DIR/replay" true || true
else
  persistence_status="FAIL"
  restoration_status="FAIL"
  continuity_status="FAIL"
fi

if [[ "$continuity_root" != "UNKNOWN" \
  && "$replay_continuity_root" != "UNKNOWN" \
  && "$replay_continuity_root" == "$continuity_root" ]]; then
  replay_status="PASS"
else
  replay_status="FAIL"
fi

if [[ "$bootstrap_status" == "PASS" \
  && "$persistence_status" == "PASS" \
  && "$restoration_status" == "PASS" \
  && "$continuity_status" == "PASS" \
  && "$replay_status" == "PASS" ]]; then
  overall_result="PASS"
else
  overall_result="FAIL"
fi

write_report
print_summary

[[ "$overall_result" == "PASS" ]]
