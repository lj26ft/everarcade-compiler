#!/usr/bin/env bash
set -euo pipefail
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/node_common.sh"

errors=()
check_dir() { [[ -d "$1" ]] || errors+=("Create missing directory: $1 (run bash scripts/node_init.sh)"); }
check_file() { [[ -f "$1" ]] || errors+=("Create missing file: $1 (run bash scripts/node_init.sh)"); }
while IFS= read -r dir; do
  check_dir "$dir"
done < <(node_dirs)
check_file "$CONFIG_FILE"
check_file "$RUNTIME_STATE_FILE"
check_file "$MANIFEST_FILE"
check_file "$WORLD_STATE_FILE"
check_file "$JOURNAL_FILE"
check_dir "$REPORT_DIR"
[[ -f "$REPORT_DIR/node_initialization_report.txt" ]] || errors+=("Run initialization report generation: bash scripts/node_init.sh")
if [[ -f "$LATEST_CHECKPOINT_FILE" ]]; then
  checkpoint_path="$(cat "$LATEST_CHECKPOINT_FILE")"
  [[ -f "$checkpoint_path" ]] || errors+=("Latest checkpoint pointer is stale: $checkpoint_path")
else
  errors+=("Create a checkpoint: bash scripts/node_checkpoint.sh")
fi
runtime_status="$(runtime_appliance_status)"
[[ "$runtime_status" != "MISSING" ]] || errors+=("Runtime Appliance wrappers missing; restore scripts/runtime_*.sh")

if [[ ${#errors[@]} -ne 0 ]]; then
  echo "Node Unhealthy" >&2
  echo "Actionable errors:" >&2
  printf ' - %s\n' "${errors[@]}" >&2
  exit 1
fi

cat <<DOCTOR
Node Diagnostics
Directories: PASS
Config: PASS
State: PASS
Checkpoints: PASS
Journals: PASS
Reports: PASS
Runtime Appliance: $runtime_status
Node Healthy
Doctor: PASS
DOCTOR
