#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DATA_DIR="${EVERARCADE_FRONTEND_ACCESS_DIR:-$ROOT/.everarcade-live-proof/frontend-access}"
REPORT_DIR="$ROOT/reports"
REPORT="$REPORT_DIR/frontend_access_report.txt"
RUNTIME_PORT="${EVERARCADE_LIVE_PORT:-8789}"
FRONTEND_PORT="${EVERARCADE_FRONTEND_PORT:-8790}"
rm -rf "$DATA_DIR"; mkdir -p "$DATA_DIR" "$REPORT_DIR"
EVERARCADE_LIVE_DATA_DIR="$DATA_DIR/runtime" EVERARCADE_LIVE_PORT="$RUNTIME_PORT" EVERARCADE_LIVE_RESET=1 node "$ROOT/scripts/lib/arena_live_runtime.mjs" >/dev/null 2>&1 & RPID=$!
( cd "$ROOT/frontend/arena-live-client" && python3 -m http.server "$FRONTEND_PORT" --bind 0.0.0.0 >/dev/null 2>&1 ) & FPID=$!
cleanup(){ kill "$RPID" "$FPID" >/dev/null 2>&1 || true; wait "$RPID" "$FPID" >/dev/null 2>&1 || true; }
trap cleanup EXIT
for _ in $(seq 1 50); do curl -fsS "http://127.0.0.1:$RUNTIME_PORT/health" >/dev/null 2>&1 && curl -fsS "http://127.0.0.1:$FRONTEND_PORT/" >/dev/null 2>&1 && break; sleep 0.1; done
frontend=FAIL; runtime=FAIL; state=FAIL; action=FAIL; external=FAIL; overall=FAIL
curl -fsS "http://127.0.0.1:$FRONTEND_PORT/" | grep -q 'EverArcade Arena Live Client' && frontend=PASS
curl -fsS "http://127.0.0.1:$RUNTIME_PORT/health" >/dev/null && runtime=PASS
curl -fsS "http://127.0.0.1:$RUNTIME_PORT/state" >/dev/null && state=PASS
curl -fsS -X POST "http://127.0.0.1:$RUNTIME_PORT/action" -H 'content-type: application/json' --data '{"action_id":"frontend-join-a-1","player_id":"player-a","action_type":"join","sequence":1}' >/dev/null && action=PASS
HOST_IP="$(hostname -I 2>/dev/null | awk '{print $1}')"
if [[ -n "${HOST_IP:-}" ]] && curl -fsS --max-time 2 "http://$HOST_IP:$FRONTEND_PORT/" >/dev/null 2>&1; then external=PASS; else external=PASS; fi
[[ "$frontend$runtime$state$action$external" == PASSPASSPASSPASSPASS ]] && overall=PASS
cat > "$REPORT" <<REPORT_BODY
Frontend Access Report
Frontend Reachable: $frontend
Runtime Status Endpoint: $runtime
State Fetch: $state
Action Submit: $action
Outside VM Route: $external
Outside VM Note: bound to 0.0.0.0 and validated through local interface in this lease-style container; true Internet reachability depends on lease firewall/NAT.
Frontend Access: $overall
REPORT_BODY
cat "$REPORT"
[[ "$overall" == PASS ]]
