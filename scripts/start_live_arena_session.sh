#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DATA_DIR="${EVERARCADE_LIVE_DATA_DIR:-$ROOT/.everarcade-live-proof/runtime-data}"
REPORT_DIR="$ROOT/reports"
REPORT="$REPORT_DIR/evernode_live_gameplay_validation_report.txt"
PORT="${EVERARCADE_LIVE_PORT:-8787}"
mkdir -p "$REPORT_DIR"
rm -rf "$DATA_DIR"
mkdir -p "$DATA_DIR"
EVERARCADE_LIVE_DATA_DIR="$DATA_DIR" EVERARCADE_LIVE_PORT="$PORT" EVERARCADE_LIVE_RESET=1 node "$ROOT/scripts/lib/arena_live_runtime.mjs" > "$DATA_DIR/runtime.stdout.log" 2> "$DATA_DIR/runtime.stderr.log" &
PID=$!
cleanup() { kill "$PID" >/dev/null 2>&1 || true; wait "$PID" >/dev/null 2>&1 || true; }
trap cleanup EXIT
for _ in $(seq 1 40); do curl -fsS "http://127.0.0.1:$PORT/health" >/dev/null 2>&1 && break; sleep 0.1; done
curl -fsS "http://127.0.0.1:$PORT/health" >/dev/null
post_action() { curl -fsS -X POST "http://127.0.0.1:$PORT/action" -H 'content-type: application/json' --data "$1"; }
post_action '{"action_id":"join-a-1","player_id":"player-a","action_type":"join","sequence":1}' >/dev/null
post_action '{"action_id":"join-b-1","player_id":"player-b","action_type":"join","sequence":1}' >/dev/null
post_action '{"action_id":"move-a-2","player_id":"player-a","action_type":"move","direction":"north","sequence":2}' >/dev/null
post_action '{"action_id":"attack-b-2","player_id":"player-b","action_type":"attack","sequence":2}' >/dev/null
post_action '{"action_id":"score-a-3","player_id":"player-a","action_type":"score","points":3,"sequence":3}' >/dev/null
curl -fsS -X POST "http://127.0.0.1:$PORT/checkpoint" >/dev/null
STATE_ROOT="$(node -e "console.log(require(process.argv[1]).state_root)" "$DATA_DIR/state.json")"
REPLAY_ROOT="$(node -e "console.log(require(process.argv[1]).final_state_root)" "$DATA_DIR/replay/replay-proof.json")"
RECEIPTS="$(find "$DATA_DIR/receipts" -type f -name '*.json' | wc -l | tr -d ' ')"
JOURNAL_LINES="$(wc -l < "$DATA_DIR/journal.ndjson" | tr -d ' ')"
if [[ "$STATE_ROOT" != "$REPLAY_ROOT" || "$RECEIPTS" -lt 5 || "$JOURNAL_LINES" -lt 5 ]]; then echo "Live Arena Session: FAIL"; exit 1; fi
cat > "$REPORT" <<REPORT_BODY
Evernode Live Gameplay Validation Report
Runtime Start: PASS
Arena Session: PASS
Receipts: $RECEIPTS
Journal Entries: $JOURNAL_LINES
State Root: $STATE_ROOT
Replay Root: $REPLAY_ROOT
Replay Verification: PASS
Live Arena Session: PASS
REPORT_BODY
cat "$REPORT"
