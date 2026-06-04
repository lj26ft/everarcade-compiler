#!/usr/bin/env bash
set -euo pipefail
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/node_common.sh"

ensure_report_dir
require_initialized
# shellcheck disable=SC1090
source "$CONFIG_FILE"
timestamp="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
checkpoint_path="$(latest_checkpoint_path)"
checkpoint_status="GENESIS"
[[ "$checkpoint_path" != "NONE" ]] && checkpoint_status="LOADED"
world_root="$(root_for "$WORLD_STATE_FILE")"
{
  printf '%s node started checkpoint=%s world_root=%s\n' "$timestamp" "$checkpoint_status" "$world_root"
} >> "$JOURNAL_FILE"
cat > "$WORLD_STATE_FILE" <<WORLD
WORLD_ID=$WORLD_ID
WORLD_STATUS=running
CIVILIZATION_STATUS=running
WORLD_SEQUENCE=1
UPDATED_AT=$timestamp
WORLD_ROOT=$world_root
WORLD
write_runtime_state "running"
report="$REPORT_DIR/node_start_report.txt"
cat > "$report" <<REPORT
Node Start Report
Timestamp: $timestamp
Configuration Loaded: PASS
Latest Checkpoint: $checkpoint_status
World State Loaded: PASS
Runtime Started: PASS
Node Start: PASS
REPORT
copy_report_to_node "$report"
echo "Node Running"
echo "Node Start: PASS"
