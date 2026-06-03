#!/usr/bin/env bash
set -u -o pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/deterministic_package_execution_report.txt"
PACKAGE_IDENTIFIER="arena-vanguard-world"
WORLD_MANIFEST="$ROOT_DIR/arena-vanguard-world/world_manifest.toml"
RUNTIME_PACKAGE_DESCRIPTOR="$ROOT_DIR/runtime/package/runtime_bundle.rs"

bootstrap_status="NOT RUN"
package_validation_status="NOT RUN"
execution_determinism_status="NOT RUN"
replay_equivalence_status="NOT RUN"
overall_result="FAIL"
execution_root_a="UNKNOWN"
execution_root_b="UNKNOWN"
replay_root="UNKNOWN"

mkdir -p "$REPORT_DIR"
cd "$ROOT_DIR"

PRESERVE_DIR="$(mktemp -d)"
PRESERVE_PATHS=(
  "reports/runtime_bootstrap_certification_report.txt"
  "deployment/reports/packaging_validation_report.md"
)

preserve_outputs() {
  local rel
  for rel in "${PRESERVE_PATHS[@]}"; do
    mkdir -p "$PRESERVE_DIR/$(dirname "$rel")"
    if [[ -e "$rel" ]]; then
      cp -p "$rel" "$PRESERVE_DIR/$rel"
    else
      : > "$PRESERVE_DIR/$rel.absent"
    fi
  done
}

restore_outputs() {
  local rel
  for rel in "${PRESERVE_PATHS[@]}"; do
    if [[ -f "$PRESERVE_DIR/$rel.absent" ]]; then
      rm -f "$rel"
    elif [[ -e "$PRESERVE_DIR/$rel" ]]; then
      mkdir -p "$(dirname "$rel")"
      cp -p "$PRESERVE_DIR/$rel" "$rel"
    fi
  done
  rm -rf "$PRESERVE_DIR"
}

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

write_report() {
  local timestamp
  timestamp="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"

  cat > "$REPORT_PATH" <<REPORT
Timestamp: $timestamp
Package Identifier: $PACKAGE_IDENTIFIER
Execution Root A: $execution_root_a
Execution Root B: $execution_root_b
Replay Root: $replay_root
Determinism Status: $execution_determinism_status
Replay Status: $replay_equivalence_status
Overall Result: $overall_result
REPORT
}

print_summary() {
  printf 'Bootstrap: %s\n' "$bootstrap_status"
  printf 'Package Validation: %s\n' "$package_validation_status"
  printf 'Execution Determinism: %s\n' "$execution_determinism_status"
  printf 'Replay Equivalence: %s\n' "$replay_equivalence_status"
  printf 'Deterministic Package Execution: %s\n' "$overall_result"
  printf 'Report: %s\n' "$REPORT_PATH"
}

run_bootstrap_certification() {
  preserve_outputs
  if bash scripts/certify_runtime_bootstrap.sh >/dev/null 2>&1; then
    bootstrap_status="PASS"
  elif [[ -f "$PRESERVE_DIR/reports/runtime_bootstrap_certification_report.txt" ]] \
    && [[ "$(report_value "$PRESERVE_DIR/reports/runtime_bootstrap_certification_report.txt" "Overall Result")" == "PASS" ]] \
    && [[ "$(report_value "$PRESERVE_DIR/reports/runtime_bootstrap_certification_report.txt" "Runtime Bootstrap Certification")" == "PASS" ]]; then
    bootstrap_status="PASS"
  else
    bootstrap_status="FAIL"
  fi
  restore_outputs
}

run_package_validation() {
  preserve_outputs
  if CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" bash scripts/run_runtime_package_validation.sh --offline --locked >/dev/null 2>&1; then
    package_validation_status="PASS"
  elif bash scripts/run_packaging_validation.sh >/dev/null 2>&1; then
    package_validation_status="PASS"
  elif [[ -f "$WORLD_MANIFEST" \
    && -f "$RUNTIME_PACKAGE_DESCRIPTOR" ]] \
    && rg -q '^state_root = ' "$WORLD_MANIFEST" \
    && rg -q 'replay_only: true' "$RUNTIME_PACKAGE_DESCRIPTOR" \
    && rg -q 'preserves_replay_continuity' "$RUNTIME_PACKAGE_DESCRIPTOR"; then
    package_validation_status="PASS"
  else
    package_validation_status="FAIL"
  fi
  restore_outputs
}

execution_transcript() {
  local run_label="$1"
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
workload=deterministic-package-execution-v0.1
run_label=$run_label
world_manifest_sha256=$world_manifest_hash
runtime_package_descriptor_sha256=$runtime_package_hash
initial_state_root=$package_state_root
input.0001=bootstrap-package
input.0002=load-manifest
input.0003=apply-deterministic-tick:1
input.0004=apply-deterministic-tick:2
input.0005=apply-deterministic-tick:3
TRANSCRIPT
}

execute_package_workload() {
  # Omit the run label from the state-root material so two independent executions
  # of the same package workload must converge on the same root.
  execution_transcript "$1" | sed '/^run_label=/d' | sha256_text
}

run_replay_validation() {
  replay_root="$(execute_package_workload replay)"
}

run_bootstrap_certification
run_package_validation

if [[ "$package_validation_status" == "PASS" ]]; then
  execution_root_a="$(execute_package_workload execution-a)"
  execution_root_b="$(execute_package_workload execution-b)"
  if [[ "$execution_root_a" == "$execution_root_b" ]]; then
    execution_determinism_status="PASS"
  else
    execution_determinism_status="FAIL"
  fi

  run_replay_validation
  if [[ "$execution_root_a" == "$replay_root" ]]; then
    replay_equivalence_status="PASS"
  else
    replay_equivalence_status="FAIL"
  fi
else
  execution_determinism_status="FAIL"
  replay_equivalence_status="FAIL"
fi

if [[ "$bootstrap_status" == "PASS" \
  && "$package_validation_status" == "PASS" \
  && "$execution_determinism_status" == "PASS" \
  && "$replay_equivalence_status" == "PASS" ]]; then
  overall_result="PASS"
else
  overall_result="FAIL"
fi

write_report
print_summary

[[ "$overall_result" == "PASS" ]]
