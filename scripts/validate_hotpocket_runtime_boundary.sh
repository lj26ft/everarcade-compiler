#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DATA_DIR="${EVERARCADE_HOTPOCKET_BOUNDARY_DIR:-$ROOT/.everarcade-live-proof/hotpocket-boundary}"
REPORT_DIR="$ROOT/reports"
REPORT="$REPORT_DIR/hotpocket_runtime_boundary_report.txt"
PORT="${EVERARCADE_HOTPOCKET_BOUNDARY_PORT:-8788}"
rm -rf "$DATA_DIR"; mkdir -p "$DATA_DIR" "$REPORT_DIR"
EVERARCADE_LIVE_DATA_DIR="$DATA_DIR" EVERARCADE_LIVE_PORT="$PORT" EVERARCADE_LIVE_RESET=1 node "$ROOT/scripts/lib/arena_live_runtime.mjs" >/dev/null 2>&1 &
PID=$!
cleanup(){ kill "$PID" >/dev/null 2>&1 || true; wait "$PID" >/dev/null 2>&1 || true; }
trap cleanup EXIT
for _ in $(seq 1 40); do curl -fsS "http://127.0.0.1:$PORT/health" >/dev/null 2>&1 && break; sleep 0.1; done
runtime=FAIL; inputs=FAIL; outputs=FAIL; checkpoints=FAIL; replay=FAIL; overall=FAIL
curl -fsS "http://127.0.0.1:$PORT/health" >/dev/null && runtime=PASS
curl -fsS -X POST "http://127.0.0.1:$PORT/action" -H 'content-type: application/json' --data '{"action_id":"hp-join-a-1","player_id":"player-a","action_type":"join","sequence":1}' >/dev/null && inputs=PASS
curl -fsS "http://127.0.0.1:$PORT/state" > "$DATA_DIR/exported-output.json" && [[ -s "$DATA_DIR/exported-output.json" ]] && outputs=PASS
curl -fsS -X POST "http://127.0.0.1:$PORT/checkpoint" >/dev/null && [[ -s "$DATA_DIR/checkpoints/latest-checkpoint.json" ]] && checkpoints=PASS
curl -fsS "http://127.0.0.1:$PORT/replay" > "$DATA_DIR/exported-replay.json" && [[ -s "$DATA_DIR/exported-replay.json" ]] && replay=PASS
[[ "$runtime$inputs$outputs$checkpoints$replay" == PASSPASSPASSPASSPASS ]] && overall=PASS
cat > "$REPORT" <<REPORT_BODY
HotPocket Runtime Boundary Check Report
Runtime Start: $runtime
Inputs Accepted: $inputs
Outputs Exported: $outputs
Checkpoints Written: $checkpoints
Replay Evidence Exported: $replay
Consensus Claims: none
HotPocket Boundary: $overall
REPORT_BODY
cat "$REPORT"
[[ "$overall" == PASS ]]
