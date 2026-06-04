#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/protocol_node_appliance_certification_report.txt"
mkdir -p "$REPORT_DIR"

run_step() {
  local label="$1"
  local script="$2"
  local output
  output="$(mktemp)"
  if bash "$ROOT_DIR/scripts/$script" >"$output" 2>&1; then
    rm -f "$output"
    printf '%s: PASS\n' "$label"
    return 0
  fi
  cat "$output" >&2
  rm -f "$output"
  printf '%s: FAIL\n' "$label"
  return 1
}

initialization="FAIL"
start="FAIL"
status="FAIL"
checkpoint="FAIL"
replay="FAIL"
restore="FAIL"
stop="FAIL"
doctor="FAIL"
overall="FAIL"

run_step "Node Initialization" "node_init.sh" && initialization="PASS"
run_step "Node Start" "node_start.sh" && start="PASS"
run_step "Node Status" "node_status.sh" && status="PASS"
run_step "Checkpoint" "node_checkpoint.sh" && checkpoint="PASS"
run_step "Replay" "node_replay.sh" && replay="PASS"
run_step "Restore" "node_restore.sh" && restore="PASS"
run_step "Stop" "node_stop.sh" && stop="PASS"
run_step "Doctor" "node_doctor.sh" && doctor="PASS"

if [[ "$initialization" == "PASS" && "$start" == "PASS" && "$status" == "PASS" && "$checkpoint" == "PASS" && "$replay" == "PASS" && "$restore" == "PASS" && "$stop" == "PASS" && "$doctor" == "PASS" ]]; then
  overall="PASS"
fi

cat > "$REPORT_PATH" <<REPORT
Protocol Node Appliance Certification Report
Initialization: $initialization
Start: $start
Node Status: $status
Checkpoint: $checkpoint
Replay: $replay
Restore: $restore
Stop: $stop
Doctor: $doctor
Protocol Node Appliance: $overall
REPORT

cat "$REPORT_PATH"
if [[ "$overall" != "PASS" ]]; then
  exit 1
fi
echo "Protocol Node Appliance: PASS"
