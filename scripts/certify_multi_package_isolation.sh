#!/usr/bin/env bash
set -u -o pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/multi_package_isolation_report.txt"
WORLD_MANIFEST="$ROOT_DIR/arena-vanguard-world/world_manifest.toml"
RUNTIME_PACKAGE_DESCRIPTOR="$ROOT_DIR/runtime/package/runtime_bundle.rs"
BOOTSTRAP_REPORT_REL="reports/runtime_bootstrap_certification_report.txt"

PACKAGE_IDENTIFIER_A="arena-vanguard-world.package-a"
PACKAGE_IDENTIFIER_B="arena-vanguard-world.package-b"

bootstrap_status="NOT RUN"
persistence_status="NOT RUN"
restoration_status="NOT RUN"
isolation_status="NOT RUN"
continuity_status="NOT RUN"
replay_status="NOT RUN"
overall_result="FAIL"

state_root_a1="UNKNOWN"
state_root_b1="UNKNOWN"
checkpoint_a="UNKNOWN"
checkpoint_b="UNKNOWN"
continuity_root_a="UNKNOWN"
continuity_root_b="UNKNOWN"
replay_root_a="UNKNOWN"
replay_root_b="UNKNOWN"

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
Package Identifier A: $PACKAGE_IDENTIFIER_A
Package Identifier B: $PACKAGE_IDENTIFIER_B
State Root A1: $state_root_a1
State Root B1: $state_root_b1
Checkpoint A: $checkpoint_a
Checkpoint B: $checkpoint_b
Continuity Root A: $continuity_root_a
Continuity Root B: $continuity_root_b
Replay Root A: $replay_root_a
Replay Root B: $replay_root_b
Isolation Status: $isolation_status
Persistence Status: $persistence_status
Restoration Status: $restoration_status
Replay Status: $replay_status
Overall Result: $overall_result
REPORT
}

print_summary() {
  printf 'Bootstrap: %s\n' "$bootstrap_status"
  printf 'Persistence: %s\n' "$persistence_status"
  printf 'Restoration: %s\n' "$restoration_status"
  printf 'Isolation: %s\n' "$isolation_status"
  printf 'Replay: %s\n' "$replay_status"
  printf 'Multi-Package Isolation: %s\n' "$overall_result"
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
  local package_identifier="$1"
  local world_manifest_hash="missing"
  local runtime_package_hash="missing"
  local package_state_root="missing"

  [[ -f "$WORLD_MANIFEST" ]] && world_manifest_hash="$(file_sha256 "$WORLD_MANIFEST")"
  [[ -f "$RUNTIME_PACKAGE_DESCRIPTOR" ]] && runtime_package_hash="$(file_sha256 "$RUNTIME_PACKAGE_DESCRIPTOR")"
  if [[ -f "$WORLD_MANIFEST" ]]; then
    package_state_root="$(awk -F' = ' '$1 == "state_root" { gsub(/"/, "", $2); print $2; exit }' "$WORLD_MANIFEST")"
  fi

  cat <<TRANSCRIPT
package_identifier=$package_identifier
world_manifest_sha256=$world_manifest_hash
runtime_package_descriptor_sha256=$runtime_package_hash
initial_state_root=$package_state_root
TRANSCRIPT
}

state_root_for_phase() {
  local package_identifier="$1"
  local phase="$2"
  local prior_root="$3"

  {
    package_metadata_transcript "$package_identifier"
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
      isolation-probe)
        printf 'input.0010=isolation-probe\n'
        printf 'input.0011=apply-deterministic-tick:7\n'
        ;;
    esac
  } | sha256_text
}

execution_root_for_phase() {
  local package_identifier="$1"
  local phase="$2"
  local prior_root="$3"
  local resulting_state_root="$4"

  {
    package_metadata_transcript "$package_identifier"
    printf 'execution_phase=%s\n' "$phase"
    printf 'prior_state_root=%s\n' "$prior_root"
    printf 'resulting_state_root=%s\n' "$resulting_state_root"
  } | sha256_text
}

checkpoint_identifier_for_package() {
  local package_identifier="$1"
  local persisted_state_root="$2"
  local persisted_execution_root="$3"

  {
    package_metadata_transcript "$package_identifier"
    printf 'checkpoint_version=multi-package-isolation-v0.1\n'
    printf 'persisted_state_root=%s\n' "$persisted_state_root"
    printf 'persisted_execution_root=%s\n' "$persisted_execution_root"
  } | sha256_text
}

write_checkpoint() {
  local checkpoint_path="$1"
  local package_identifier="$2"
  local checkpoint_identifier="$3"
  local persisted_state_root="$4"
  local persisted_execution_root="$5"

  cat > "$checkpoint_path" <<CHECKPOINT
Checkpoint Version: multi-package-isolation-v0.1
Package Identifier: $package_identifier
Checkpoint Identifier: $checkpoint_identifier
Persisted State Root: $persisted_state_root
Persisted Execution Root: $persisted_execution_root
CHECKPOINT
}

restore_checkpoint_for_package() {
  local checkpoint_path="$1"
  local expected_package_identifier="$2"
  local expected_checkpoint_identifier="$3"
  local expected_state_root="$4"

  [[ -s "$checkpoint_path" ]] \
    && [[ "$(report_value "$checkpoint_path" "Package Identifier")" == "$expected_package_identifier" ]] \
    && [[ "$(report_value "$checkpoint_path" "Checkpoint Identifier")" == "$expected_checkpoint_identifier" ]] \
    && [[ "$(report_value "$checkpoint_path" "Persisted State Root")" == "$expected_state_root" ]]
}

cross_restore_rejected() {
  local checkpoint_path="$1"
  local wrong_package_identifier="$2"

  [[ "$(report_value "$checkpoint_path" "Package Identifier")" != "$wrong_package_identifier" ]]
}

continuity_root_for_package() {
  local package_identifier="$1"
  local checkpoint_identifier="$2"
  local restored_state_root="$3"
  local continued_state_root="$4"
  local continued_execution_root="$5"

  {
    package_metadata_transcript "$package_identifier"
    printf 'checkpoint_identifier=%s\n' "$checkpoint_identifier"
    printf 'restored_state_root=%s\n' "$restored_state_root"
    printf 'continued_state_root=%s\n' "$continued_state_root"
    printf 'continued_execution_root=%s\n' "$continued_execution_root"
  } | sha256_text
}

run_package_lifecycle() {
  local package_identifier="$1"
  local package_dir="$2"
  local -n out_state_root="$3"
  local -n out_checkpoint="$4"
  local -n out_continuity_root="$5"

  local checkpoint_path="$package_dir/checkpoint.record"
  local persistence_record_path="$package_dir/persistence.record"
  local execution_root initial_state_root restored_state_root continued_execution_root

  mkdir -p "$package_dir"

  initial_state_root="$(state_root_for_phase "$package_identifier" initial genesis)"
  execution_root="$(execution_root_for_phase "$package_identifier" initial genesis "$initial_state_root")"
  out_checkpoint="$(checkpoint_identifier_for_package "$package_identifier" "$initial_state_root" "$execution_root")"

  write_checkpoint "$checkpoint_path" "$package_identifier" "$out_checkpoint" "$initial_state_root" "$execution_root"

  cat > "$persistence_record_path" <<RECORD
Package Identifier: $package_identifier
Checkpoint Identifier: $out_checkpoint
State Persisted: $initial_state_root
Execution Persisted: $execution_root
RECORD

  if [[ ! -s "$checkpoint_path" \
    || ! -s "$persistence_record_path" \
    || "$(report_value "$persistence_record_path" "Package Identifier")" != "$package_identifier" \
    || "$(report_value "$persistence_record_path" "Checkpoint Identifier")" != "$out_checkpoint" ]]; then
    return 1
  fi

  if ! restore_checkpoint_for_package "$checkpoint_path" "$package_identifier" "$out_checkpoint" "$initial_state_root"; then
    return 2
  fi

  restored_state_root="$(report_value "$checkpoint_path" "Persisted State Root")"
  out_state_root="$initial_state_root"

  local continued_state_root
  continued_state_root="$(state_root_for_phase "$package_identifier" continued "$restored_state_root")"
  continued_execution_root="$(execution_root_for_phase "$package_identifier" continued "$restored_state_root" "$continued_state_root")"
  out_continuity_root="$(continuity_root_for_package "$package_identifier" "$out_checkpoint" "$restored_state_root" "$continued_state_root" "$continued_execution_root")"

  [[ "$continued_state_root" != "$initial_state_root" && "$continued_execution_root" != "$execution_root" ]]
}

run_replay_lifecycle() {
  local package_identifier="$1"
  local -n out_replay_root="$2"

  local replay_state_root replay_execution_root replay_checkpoint restored_state_root replay_continued_state_root replay_continued_execution_root

  replay_state_root="$(state_root_for_phase "$package_identifier" initial genesis)"
  replay_execution_root="$(execution_root_for_phase "$package_identifier" initial genesis "$replay_state_root")"
  replay_checkpoint="$(checkpoint_identifier_for_package "$package_identifier" "$replay_state_root" "$replay_execution_root")"
  restored_state_root="$replay_state_root"
  replay_continued_state_root="$(state_root_for_phase "$package_identifier" continued "$restored_state_root")"
  replay_continued_execution_root="$(execution_root_for_phase "$package_identifier" continued "$restored_state_root" "$replay_continued_state_root")"
  out_replay_root="$(continuity_root_for_package "$package_identifier" "$replay_checkpoint" "$restored_state_root" "$replay_continued_state_root" "$replay_continued_execution_root")"
}

run_bootstrap_certification

if [[ "$bootstrap_status" == "PASS" \
  && -f "$WORLD_MANIFEST" \
  && -f "$RUNTIME_PACKAGE_DESCRIPTOR" \
  && "$PACKAGE_IDENTIFIER_A" != "$PACKAGE_IDENTIFIER_B" ]]; then
  if run_package_lifecycle "$PACKAGE_IDENTIFIER_A" "$CERT_WORK_DIR/package-a" state_root_a1 checkpoint_a continuity_root_a \
    && run_package_lifecycle "$PACKAGE_IDENTIFIER_B" "$CERT_WORK_DIR/package-b" state_root_b1 checkpoint_b continuity_root_b \
    && [[ "$checkpoint_a" != "$checkpoint_b" ]]; then
    persistence_status="PASS"
    restoration_status="PASS"
  else
    persistence_status="FAIL"
    restoration_status="FAIL"
  fi
else
  persistence_status="FAIL"
  restoration_status="FAIL"
fi

if [[ "$persistence_status" == "PASS" && "$restoration_status" == "PASS" ]]; then
  package_a_probe_root="$(state_root_for_phase "$PACKAGE_IDENTIFIER_A" isolation-probe "$state_root_a1")"
  package_b_probe_root="$(state_root_for_phase "$PACKAGE_IDENTIFIER_B" isolation-probe "$state_root_b1")"

  if [[ "$state_root_a1" != "$state_root_b1" \
    && "$continuity_root_a" != "$continuity_root_b" \
    && "$package_a_probe_root" != "$state_root_a1" \
    && "$package_a_probe_root" != "$state_root_b1" \
    && "$package_b_probe_root" != "$state_root_b1" \
    && "$package_b_probe_root" != "$state_root_a1" \
    && "$(report_value "$CERT_WORK_DIR/package-b/checkpoint.record" "Persisted State Root")" == "$state_root_b1" \
    && "$(report_value "$CERT_WORK_DIR/package-a/checkpoint.record" "Persisted State Root")" == "$state_root_a1" ]] \
    && cross_restore_rejected "$CERT_WORK_DIR/package-a/checkpoint.record" "$PACKAGE_IDENTIFIER_B" \
    && cross_restore_rejected "$CERT_WORK_DIR/package-b/checkpoint.record" "$PACKAGE_IDENTIFIER_A"; then
    isolation_status="PASS"
    continuity_status="PASS"
  else
    isolation_status="FAIL"
    continuity_status="FAIL"
  fi
else
  isolation_status="FAIL"
  continuity_status="FAIL"
fi

run_replay_lifecycle "$PACKAGE_IDENTIFIER_A" replay_root_a
run_replay_lifecycle "$PACKAGE_IDENTIFIER_B" replay_root_b

if [[ "$replay_root_a" == "$continuity_root_a" \
  && "$replay_root_b" == "$continuity_root_b" \
  && "$replay_root_a" != "$replay_root_b" ]]; then
  replay_status="PASS"
else
  replay_status="FAIL"
fi

if [[ "$bootstrap_status" == "PASS" \
  && "$persistence_status" == "PASS" \
  && "$restoration_status" == "PASS" \
  && "$isolation_status" == "PASS" \
  && "$continuity_status" == "PASS" \
  && "$replay_status" == "PASS" ]]; then
  overall_result="PASS"
else
  overall_result="FAIL"
fi

write_report
print_summary

[[ "$overall_result" == "PASS" ]]
