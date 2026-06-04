#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
REPORT_DIR="$ROOT/reports"; REPORT="$REPORT_DIR/evernode_stop_report.txt"; mkdir -p "$REPORT_DIR" evernode/reports evernode/journals
state_status="FAIL"; journal_status="FAIL"; stop_status="FAIL"; overall="FAIL"
if bash scripts/node_stop.sh >/dev/null; then stop_status="PASS"; fi
[[ -s node/state/runtime_state.env && "$(awk -F= '/^NODE_STATUS=/{print $2}' node/state/runtime_state.env | tail -n1)" == stopped ]] && state_status="PASS"
[[ -s node/journals/civilization-alpha.journal ]] && journal_status="PASS"
cp node/journals/civilization-alpha.journal evernode/journals/civilization-alpha.journal
if [[ "$stop_status" == PASS && "$state_status" == PASS && "$journal_status" == PASS ]]; then overall="PASS"; fi
cat > "$REPORT" <<REPORT_BODY
Evernode Stop Report
Runtime Stop: $stop_status
State Persisted: $state_status
Journals Flushed: $journal_status
Stop: $overall
REPORT_BODY
cp "$REPORT" evernode/reports/
echo "Stop: $overall"
[[ "$overall" == PASS ]]
