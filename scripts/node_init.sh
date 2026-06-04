#!/usr/bin/env bash
set -euo pipefail
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/node_common.sh"

ensure_report_dir
while IFS= read -r dir; do
  mkdir -p "$dir"
done < <(node_dirs)

timestamp="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
cat > "$CONFIG_FILE" <<CONFIG
NODE_ID=everarcade-protocol-node-v0.1
WORLD_ID=$WORLD_ID
NODE_MODE=single-node-lifecycle
NETWORKING=disabled
CONSENSUS=disabled
HOTPOCKET=disabled
EVERNODE=disabled
XRPL_RPC=disabled
CONFIG

cat > "$WORLD_STATE_FILE" <<WORLD
WORLD_ID=$WORLD_ID
WORLD_STATUS=initialized
CIVILIZATION_STATUS=ready
WORLD_SEQUENCE=0
UPDATED_AT=$timestamp
WORLD

cat > "$CIVILIZATION_LOG" <<CIV
$timestamp civilization initialized
CIV

cat > "$JOURNAL_FILE" <<JOURNAL
$timestamp node initialized
JOURNAL

write_runtime_state "initialized"
cat > "$MANIFEST_FILE" <<MANIFEST
{
  "appliance": "EverArcade Protocol Node Appliance",
  "version": "v0.1",
  "node_id": "everarcade-protocol-node-v0.1",
  "world_id": "$WORLD_ID",
  "created_at": "$timestamp",
  "layout": ["config", "state", "worlds", "journals", "checkpoints", "backups", "reports", "logs"],
  "non_goals": ["networking", "consensus", "hotpocket", "evernode", "xrpl_rpc", "renderer", "gpu_runtime", "protocol_changes"]
}
MANIFEST

report="$REPORT_DIR/node_initialization_report.txt"
cat > "$report" <<REPORT
Node Initialization Report
Timestamp: $timestamp
Node Directory: $NODE_DIR
Config: PASS
Runtime State: PASS
Manifest: PASS
World State: PASS
Node Initialization: PASS
REPORT
copy_report_to_node "$report"
echo "Node Initialized"
echo "Node Initialization: PASS"
