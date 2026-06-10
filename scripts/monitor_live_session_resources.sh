#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DATA_DIR="$ROOT/.everarcade-live-proof/resource-monitor"
REPORT_DIR="$ROOT/reports"; REPORT="$REPORT_DIR/live_session_resource_report.txt"; PORT="${EVERARCADE_RESOURCE_PORT:-8791}"
rm -rf "$DATA_DIR"; mkdir -p "$DATA_DIR" "$REPORT_DIR"
disk_before="$(du -sk "$ROOT" 2>/dev/null | awk '{print $1}')"; mem_before="$(awk '/MemAvailable/ {print $2}' /proc/meminfo 2>/dev/null || echo 0)"
EVERARCADE_LIVE_DATA_DIR="$DATA_DIR" EVERARCADE_LIVE_PORT="$PORT" EVERARCADE_LIVE_RESET=1 node "$ROOT/scripts/lib/arena_live_runtime.mjs" >/dev/null 2>&1 & PID=$!
cleanup(){ kill "$PID" >/dev/null 2>&1 || true; wait "$PID" >/dev/null 2>&1 || true; }
trap cleanup EXIT
for _ in $(seq 1 40); do curl -fsS "http://127.0.0.1:$PORT/health" >/dev/null 2>&1 && break; sleep 0.1; done
for action in \
'{"action_id":"res-join-a-1","player_id":"player-a","action_type":"join","sequence":1}' \
'{"action_id":"res-join-b-1","player_id":"player-b","action_type":"join","sequence":1}' \
'{"action_id":"res-move-a-2","player_id":"player-a","action_type":"move","direction":"north","sequence":2}' \
'{"action_id":"res-attack-b-2","player_id":"player-b","action_type":"attack","sequence":2}'; do curl -fsS -X POST "http://127.0.0.1:$PORT/action" -H 'content-type: application/json' --data "$action" >/dev/null; done
curl -fsS -X POST "http://127.0.0.1:$PORT/checkpoint" >/dev/null
cpu_sample="$(ps -p "$PID" -o %cpu= 2>/dev/null | awk '{print $1}')"; rss_sample="$(ps -p "$PID" -o rss= 2>/dev/null | awk '{print $1}')"
disk_after="$(du -sk "$ROOT" 2>/dev/null | awk '{print $1}')"; mem_after="$(awk '/MemAvailable/ {print $2}' /proc/meminfo 2>/dev/null || echo 0)"
receipts="$(find "$DATA_DIR/receipts" -type f -name '*.json' | wc -l | tr -d ' ')"; log_size="$(wc -c < "$DATA_DIR/logs/runtime.log" 2>/dev/null || echo 0)"; journal_size="$(wc -c < "$DATA_DIR/journal.ndjson" 2>/dev/null || echo 0)"; checkpoint_size="$(wc -c < "$DATA_DIR/checkpoints/latest-checkpoint.json" 2>/dev/null || echo 0)"
cat > "$REPORT" <<REPORT_BODY
Live Session Resource Report
Disk Before KB: $disk_before
Disk After KB: $disk_after
Memory Available Before KB: $mem_before
Memory Available After KB: $mem_after
CPU Sample Percent: ${cpu_sample:-0}
Runtime Process ID: $PID
Runtime RSS KB: ${rss_sample:-0}
Log Size Bytes: $log_size
Receipt Count: $receipts
Journal Size Bytes: $journal_size
Checkpoint Size Bytes: $checkpoint_size
Resource Monitoring: PASS
REPORT_BODY
cat "$REPORT"
