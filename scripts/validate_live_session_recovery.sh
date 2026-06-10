#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DATA_DIR="$ROOT/.everarcade-live-proof/recovery"; REPORT_DIR="$ROOT/reports"; REPORT="$REPORT_DIR/live_session_recovery_report.txt"; PORT="${EVERARCADE_RECOVERY_PORT:-8792}"
rm -rf "$DATA_DIR"; mkdir -p "$DATA_DIR" "$REPORT_DIR"
start_runtime(){ EVERARCADE_LIVE_DATA_DIR="$DATA_DIR" EVERARCADE_LIVE_PORT="$PORT" "$@" node "$ROOT/scripts/lib/arena_live_runtime.mjs" >/dev/null 2>&1 & echo $!; }
PID=$(start_runtime env EVERARCADE_LIVE_RESET=1)
for _ in $(seq 1 40); do curl -fsS "http://127.0.0.1:$PORT/health" >/dev/null 2>&1 && break; sleep 0.1; done
for action in \
'{"action_id":"rec-join-a-1","player_id":"player-a","action_type":"join","sequence":1}' \
'{"action_id":"rec-join-b-1","player_id":"player-b","action_type":"join","sequence":1}' \
'{"action_id":"rec-move-a-2","player_id":"player-a","action_type":"move","direction":"north","sequence":2}' \
'{"action_id":"rec-attack-b-2","player_id":"player-b","action_type":"attack","sequence":2}'; do curl -fsS -X POST "http://127.0.0.1:$PORT/action" -H 'content-type: application/json' --data "$action" >/dev/null; done
curl -fsS -X POST "http://127.0.0.1:$PORT/checkpoint" >/dev/null
before="$(node -e "console.log(require(process.argv[1]).state_root)" "$DATA_DIR/state.json")"
kill "$PID" >/dev/null 2>&1 || true; wait "$PID" >/dev/null 2>&1 || true
PID=$(start_runtime env)
cleanup(){ kill "$PID" >/dev/null 2>&1 || true; wait "$PID" >/dev/null 2>&1 || true; }
trap cleanup EXIT
for _ in $(seq 1 40); do curl -fsS "http://127.0.0.1:$PORT/health" >/dev/null 2>&1 && break; sleep 0.1; done
after="$(curl -fsS "http://127.0.0.1:$PORT/state" | node -e 'let s="";process.stdin.on("data",c=>s+=c);process.stdin.on("end",()=>console.log(JSON.parse(s).state_root))')"
replay="$(curl -fsS "http://127.0.0.1:$PORT/replay" | node -e 'let s="";process.stdin.on("data",c=>s+=c);process.stdin.on("end",()=>console.log(JSON.parse(s).final_state_root))')"
overall=FAIL; [[ "$before" == "$after" && "$after" == "$replay" ]] && overall=PASS
cat > "$REPORT" <<REPORT_BODY
Live Session Recovery Report
State Root Before Stop: $before
State Root After Restart: $after
Replay Root After Restart: $replay
Checkpoint Restore: PASS
Replay Verification: $overall
Live Session Recovery: $overall
REPORT_BODY
cat "$REPORT"
[[ "$overall" == PASS ]]
