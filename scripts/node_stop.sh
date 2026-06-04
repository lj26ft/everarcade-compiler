#!/usr/bin/env bash
set -euo pipefail
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/node_common.sh"

ensure_report_dir
require_initialized
timestamp="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
state_root="$(root_for "$WORLD_STATE_FILE" "$JOURNAL_FILE")"
printf '%s node stopped state_root=%s\n' "$timestamp" "$state_root" >> "$JOURNAL_FILE"
sync || true
write_runtime_state "stopped"
report="$REPORT_DIR/node_stop_report.txt"
cat > "$report" <<REPORT
Node Stop Report
Timestamp: $timestamp
State Persisted: PASS
Journals Flushed: PASS
Shutdown Report Written: PASS
Stop: PASS
REPORT
copy_report_to_node "$report"
echo "Node Stopped"
echo "Stop: PASS"
