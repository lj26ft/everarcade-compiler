#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

REPORT="reports/network_transport_validation_report.txt"
TMP="$(mktemp -d)"
PROJECT="$TMP/audit-arena"
RUNTIME_ROOT="$TMP/runtime-root"
mkdir -p reports
: > "$REPORT"
trap 'rm -rf "$TMP"' EXIT

pass() { echo "$1: PASS" | tee -a "$REPORT"; }
fail() { echo "$1: FAIL" | tee -a "$REPORT"; exit 1; }
require_file() { [[ -f "$1" ]] || fail "$2"; }
node_check() {
  local label="$1"
  local script="$2"
  node --input-type=module -e "$script" || fail "$label"
  pass "$label"
}

node creator-sdk/cli/everarcade.mjs new --template arena --name audit-arena --dir "$PROJECT" >/dev/null
node creator-sdk/cli/everarcade.mjs play-network-local --project "$PROJECT" --template arena --runtime-root "$RUNTIME_ROOT" >/tmp/network-transport-session.out
rg -q 'Network Transport Session: PASS' /tmp/network-transport-session.out || fail "Network Transport Session Command"

CLIENT_A="$RUNTIME_ROOT/network/client-a-state.json"
CLIENT_B="$RUNTIME_ROOT/network/client-b-state.json"
RUNTIME_STATE="$RUNTIME_ROOT/network/runtime-state.json"
TRANSPORT_LOG="$RUNTIME_ROOT/network/transport-log.json"
NETWORK_TRANSCRIPT="$RUNTIME_ROOT/network/session-transcript.json"
SESSION_TRANSCRIPT="$RUNTIME_ROOT/sessions/network-session-transcript.json"
RECEIPT_DELIVERY="$RUNTIME_ROOT/network/receipt-delivery.json"
JOURNAL="$RUNTIME_ROOT/journals/journal.jsonl"
REPLAY="$RUNTIME_ROOT/replay/network-session-replay-proof.json"
PROOF="$RUNTIME_ROOT/reports/network-transport-session-proof.json"

require_file "$CLIENT_A" "Client A Started"
require_file "$CLIENT_B" "Client B Started"
require_file "$RUNTIME_STATE" "Runtime State"
require_file "$TRANSPORT_LOG" "Transport Log"
require_file "$NETWORK_TRANSCRIPT" "Network Transcript"
require_file "$SESSION_TRANSCRIPT" "Session Transcript"
require_file "$RECEIPT_DELIVERY" "Receipt Delivery"
require_file "$JOURNAL" "Journal Observation"
require_file "$REPLAY" "Replay Generation"
require_file "$PROOF" "Network Proof"

node_check "Client A Started" "import fs from 'node:fs'; const s=JSON.parse(fs.readFileSync('$CLIENT_A','utf8')); if (s.client_id!=='client-a' || s.player_id!=='player-a' || !s.started || s.tick < 5) process.exit(1);"
node_check "Client B Started" "import fs from 'node:fs'; const s=JSON.parse(fs.readFileSync('$CLIENT_B','utf8')); if (s.client_id!=='client-b' || s.player_id!=='player-b' || !s.started || s.tick < 5) process.exit(1);"
node_check "Session Join" "import fs from 'node:fs'; const t=JSON.parse(fs.readFileSync('$SESSION_TRANSCRIPT','utf8')); if (!t.some(e=>e.action==='join' && e.player_id==='player-a') || !t.some(e=>e.action==='join' && e.player_id==='player-b')) process.exit(1);"
node_check "Movement Synchronization" "import fs from 'node:fs'; const t=JSON.parse(fs.readFileSync('$SESSION_TRANSCRIPT','utf8')); if (!t.some(e=>e.action==='move' && e.player_id==='player-a') || !t.some(e=>e.action==='move' && e.player_id==='player-b')) process.exit(1);"
node_check "Interaction Synchronization" "import fs from 'node:fs'; const t=JSON.parse(fs.readFileSync('$SESSION_TRANSCRIPT','utf8')); if (!t.some(e=>e.action==='attack' && e.player_id==='player-a' && e.target==='player-b')) process.exit(1);"
node_check "State Synchronization" "import fs from 'node:fs'; const a=JSON.parse(fs.readFileSync('$CLIENT_A','utf8')); const b=JSON.parse(fs.readFileSync('$CLIENT_B','utf8')); const r=JSON.parse(fs.readFileSync('$RUNTIME_STATE','utf8')); if (a.state_root!==r.state_root || b.state_root!==r.state_root || a.tick!==r.tick || b.tick!==r.tick || a.player_count!==r.player_count || b.player_count!==r.player_count) process.exit(1);"
node_check "Receipt Delivery" "import fs from 'node:fs'; const d=JSON.parse(fs.readFileSync('$RECEIPT_DELIVERY','utf8')); if (d.length < 5) process.exit(1); for (const x of d) if (x.generated_by!=='runtime-authority' || !x.delivered_to.includes('client-a') || !x.delivered_to.includes('client-b') || !x.acknowledged_by.includes('client-a') || !x.acknowledged_by.includes('client-b')) process.exit(1);"
node_check "Journal Observation" "import fs from 'node:fs'; const a=JSON.parse(fs.readFileSync('$CLIENT_A','utf8')); const b=JSON.parse(fs.readFileSync('$CLIENT_B','utf8')); const entries=fs.readFileSync('$JOURNAL','utf8').trim().split(/\n+/); if (entries.length !== a.observed_journal_length || entries.length !== b.observed_journal_length || entries.length < 5) process.exit(1);"
node_check "Multi-Tick Progression" "import fs from 'node:fs'; const t=JSON.parse(fs.readFileSync('$SESSION_TRANSCRIPT','utf8')); const ticks=new Set(t.map(e=>e.tick)); for (const n of [1,2,3,4,5]) if (!ticks.has(n)) process.exit(1);"
node_check "Replay Generation" "import fs from 'node:fs'; const p=JSON.parse(fs.readFileSync('$REPLAY','utf8')); if (!p.replay_root || !p.final_session_root || p.status!=='Network Replay Verification: PASS') process.exit(1);"
node_check "Replay Verification" "import fs from 'node:fs'; const p=JSON.parse(fs.readFileSync('$REPLAY','utf8')); if (p.replay_verification !== 'PASS' || p.replay_root !== p.final_session_root) process.exit(1);"

node_check "Transport Canonical Messages" "import fs from 'node:fs'; const m=JSON.parse(fs.readFileSync('$TRANSPORT_LOG','utf8')); const types=new Set(m.map(x=>x.message_type)); for (const type of ['JoinSession','MovePlayer','AttackPlayer','SessionState','Receipt','Heartbeat']) if (!types.has(type)) process.exit(1); for (const x of m) for (const key of ['message_id','session_id','sequence','sender','payload','hash']) if (!(key in x)) process.exit(1);"
echo "Network Replay Verification: PASS" | tee -a "$REPORT"
echo "Overall Result: PASS" | tee -a "$REPORT"
