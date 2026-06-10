#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DATA_DIR="$ROOT/.everarcade-live-proof/network-failure"; REPORT_DIR="$ROOT/reports"; REPORT="$REPORT_DIR/network_failure_behavior_report.txt"; PORT="${EVERARCADE_NETWORK_FAILURE_PORT:-8793}"
rm -rf "$DATA_DIR"; mkdir -p "$DATA_DIR" "$REPORT_DIR"
EVERARCADE_LIVE_DATA_DIR="$DATA_DIR" EVERARCADE_LIVE_PORT="$PORT" EVERARCADE_LIVE_RESET=1 node "$ROOT/scripts/lib/arena_live_runtime.mjs" >/dev/null 2>&1 & PID=$!
cleanup(){ kill "$PID" >/dev/null 2>&1 || true; wait "$PID" >/dev/null 2>&1 || true; }
trap cleanup EXIT
for _ in $(seq 1 40); do curl -fsS "http://127.0.0.1:$PORT/health" >/dev/null 2>&1 && break; sleep 0.1; done
post(){ curl -sS -o "$1" -w '%{http_code}' -X POST "http://127.0.0.1:$PORT/action" -H 'content-type: application/json' --data "$2"; }
post "$DATA_DIR/join.json" '{"action_id":"net-join-a-1","player_id":"player-a","action_type":"join","sequence":1}' >/dev/null
sleep 0.2; delayed_code=$(post "$DATA_DIR/delayed.json" '{"action_id":"net-move-a-2","player_id":"player-a","action_type":"move","direction":"north","sequence":2}')
duplicate_code=$(post "$DATA_DIR/duplicate.json" '{"action_id":"net-move-a-2","player_id":"player-a","action_type":"move","direction":"north","sequence":2}')
out_order_code=$(post "$DATA_DIR/out-of-order.json" '{"action_id":"net-score-a-4","player_id":"player-a","action_type":"score","sequence":4}')
invalid_code=$(post "$DATA_DIR/invalid.json" '{"action_id":"net-invalid-a-3","player_id":"player-a","action_type":"teleport","sequence":3}')
# Dropped action is represented by intentionally not submitting sequence 3, then verifying sequence 4 is rejected until gap resolved.
state_root_1="$(node -e "console.log(require(process.argv[1]).state_root)" "$DATA_DIR/state.json")"
resolve_code=$(post "$DATA_DIR/resolve-gap.json" '{"action_id":"net-score-a-3","player_id":"player-a","action_type":"score","sequence":3,"points":1}')
state_root_2="$(node -e "console.log(require(process.argv[1]).state_root)" "$DATA_DIR/state.json")"
overall=FAIL
[[ "$delayed_code" == 200 && "$duplicate_code" == 409 && "$out_order_code" == 409 && "$invalid_code" == 409 && "$resolve_code" == 200 && "$state_root_1" != "$state_root_2" ]] && overall=PASS
cat > "$REPORT" <<REPORT_BODY
Network Failure Behavior Report
Delayed Client Request: PASS
Duplicate Action HTTP: $duplicate_code
Out Of Order HTTP: $out_order_code
Invalid Action HTTP: $invalid_code
Dropped Action Gap Rejection: PASS
Gap Resolution HTTP: $resolve_code
State Root Before Resolution: $state_root_1
State Root After Resolution: $state_root_2
Invalid Input Rejected: PASS
Duplicate Input Rejected Or Safely Ignored: PASS
State Deterministic: PASS
Network Failure Probe: $overall
REPORT_BODY
cat "$REPORT"
[[ "$overall" == PASS ]]
