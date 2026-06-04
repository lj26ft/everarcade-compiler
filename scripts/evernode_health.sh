#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"; cd "$ROOT"
REPORT_DIR="$ROOT/reports"; REPORT="$REPORT_DIR/evernode_health_report.txt"; mkdir -p "$REPORT_DIR" evernode/reports
runtime_status="FAIL"; node_status="FAIL"; checkpoint_status="FAIL"; report_status="FAIL"; hotpocket_status="FAIL"; overall="FAIL"
[[ -d runtime/config && -d runtime/deployment && -d evernode/runtime ]] && runtime_status="PASS"
[[ -d node/config && -d node/state && -d node/journals && -d evernode/node ]] && node_status="PASS"
[[ -s node/checkpoints/latest_checkpoint && -f "$(cat node/checkpoints/latest_checkpoint)" ]] && checkpoint_status="PASS"
[[ -s reports/evernode_start_report.txt || -s reports/node_start_report.txt ]] && report_status="PASS"
[[ -d hotpocket/adapter && -x hotpocket/adapter/hotpocket_adapter.sh && -d evernode/hotpocket ]] && hotpocket_status="PASS"
if [[ "$runtime_status" == PASS && "$node_status" == PASS && "$checkpoint_status" == PASS && "$report_status" == PASS && "$hotpocket_status" == PASS ]]; then overall="PASS"; fi
cat > "$REPORT" <<REPORT_BODY
Evernode Health Report
Runtime Layout: $runtime_status
Node Layout: $node_status
Checkpoint Presence: $checkpoint_status
Report Presence: $report_status
HotPocket Presence: $hotpocket_status
Health: $overall
REPORT_BODY
cp "$REPORT" evernode/reports/
echo "Health: $overall"
[[ "$overall" == PASS ]]
