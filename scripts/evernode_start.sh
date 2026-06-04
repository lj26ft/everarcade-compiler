#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
REPORT_DIR="$ROOT/reports"
REPORT="$REPORT_DIR/evernode_start_report.txt"
mkdir -p "$REPORT_DIR" evernode/reports evernode/checkpoints evernode/journals evernode/logs

runtime_status="FAIL"; node_status="FAIL"; hotpocket_status="FAIL"; start_status="FAIL"; checkpoint_status="FAIL"; overall="FAIL"
[[ -d runtime/config && -d runtime/deployment && -f scripts/runtime_start.sh ]] && runtime_status="PASS"
[[ -d node && -x scripts/node_init.sh && -x scripts/node_start.sh && -x scripts/node_checkpoint.sh ]] && node_status="PASS"
[[ -d hotpocket/adapter && -x hotpocket/adapter/hotpocket_adapter.sh ]] && hotpocket_status="PASS"

if [[ "$runtime_status" == PASS && "$node_status" == PASS && "$hotpocket_status" == PASS ]]; then
  bash scripts/node_init.sh >/dev/null
  bash scripts/node_start.sh >/dev/null
  bash scripts/node_checkpoint.sh >/dev/null
  bash hotpocket/adapter/hotpocket_adapter.sh init-layout >/dev/null
  bash hotpocket/adapter/hotpocket_adapter.sh export-checkpoint evernode-start-checkpoint >/dev/null
  start_status="PASS"
  [[ -s node/checkpoints/latest_checkpoint ]] && checkpoint_status="PASS"
  cp node/checkpoints/latest_checkpoint evernode/checkpoints/latest_checkpoint
  cp node/journals/civilization-alpha.journal evernode/journals/civilization-alpha.journal
fi

if [[ "$runtime_status" == PASS && "$node_status" == PASS && "$hotpocket_status" == PASS && "$start_status" == PASS && "$checkpoint_status" == PASS ]]; then overall="PASS"; fi
cat > "$REPORT" <<REPORT_BODY
Evernode Start Report
Runtime Exists: $runtime_status
Node Exists: $node_status
HotPocket Layer Exists: $hotpocket_status
Runtime Started: $start_status
Checkpoint Created: $checkpoint_status
Start: $overall
REPORT_BODY
cp "$REPORT" evernode/reports/
echo "Start: $overall"
[[ "$overall" == PASS ]]
