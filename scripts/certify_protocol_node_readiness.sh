#!/usr/bin/env bash
set -u -o pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/protocol_node_readiness_report.txt"

bootstrap_status="NOT RUN"
persistence_status="NOT RUN"
recovery_status="NOT RUN"
replay_status="NOT RUN"
checkpoint_status="NOT RUN"
overall_result="FAIL"

mkdir -p "$REPORT_DIR"
cd "$ROOT_DIR"

PRESERVE_DIR="$(mktemp -d)"
PRESERVE_PATHS=(
  "reports/runtime_bootstrap_certification_report.txt"
  "wasm/reports/execution-replay-validation.md"
  "wasm/reports/checkpoint-restore-validation.md"
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

preserve_outputs
trap restore_outputs EXIT

write_report() {
  local timestamp
  timestamp="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"

  cat > "$REPORT_PATH" <<REPORT
Timestamp: $timestamp
Bootstrap Status: $bootstrap_status
Persistence Status: $persistence_status
Recovery Status: $recovery_status
Replay Status: $replay_status
Checkpoint Status: $checkpoint_status
Overall Result: $overall_result
REPORT
}

print_summary() {
  printf 'Bootstrap: %s\n' "$bootstrap_status"
  printf 'Persistence: %s\n' "$persistence_status"
  printf 'Recovery: %s\n' "$recovery_status"
  printf 'Replay: %s\n' "$replay_status"
  printf 'Checkpoint: %s\n' "$checkpoint_status"
  printf 'Protocol Node Readiness: %s\n' "$overall_result"
  printf 'Report: %s\n' "$REPORT_PATH"
}

run_step() {
  local status_var="$1"
  shift

  if "$@"; then
    printf -v "$status_var" 'PASS'
  else
    printf -v "$status_var" 'FAIL'
  fi
}

export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"

run_step bootstrap_status \
  bash scripts/certify_runtime_bootstrap.sh

run_step persistence_status \
  cargo test -p everarcade-runtime --tests --offline --locked

run_step recovery_status \
  bash -c 'bash scripts/run_runtime_recovery_validation.sh && cargo test -p everarcade-runtime --test runtime_platform_tests --offline --locked recovery'

run_step replay_status \
  bash scripts/run_execution_replay_validation.sh --offline --locked

run_step checkpoint_status \
  bash scripts/run_checkpoint_restore_validation.sh --offline --locked

if [[ "$bootstrap_status" == "PASS" \
  && "$persistence_status" == "PASS" \
  && "$recovery_status" == "PASS" \
  && "$replay_status" == "PASS" \
  && "$checkpoint_status" == "PASS" ]]; then
  overall_result="PASS"
else
  overall_result="FAIL"
fi

write_report
print_summary

[[ "$overall_result" == "PASS" ]]
