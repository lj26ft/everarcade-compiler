#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

REPORT="reports/multiplayer_local_session_validation_report.txt"
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
node creator-sdk/cli/everarcade.mjs play-local-multiplayer --project "$PROJECT" --template arena --runtime-root "$RUNTIME_ROOT" >/tmp/multiplayer-local-session.out
rg -q 'Multiplayer Local Session: PASS' /tmp/multiplayer-local-session.out || fail "Multiplayer Local Session Command"

SESSION="$RUNTIME_ROOT/sessions/session-0001.json"
STATE="$RUNTIME_ROOT/gameplay/arena-state.json"
TRANSCRIPT="$RUNTIME_ROOT/gameplay/multiplayer-session-transcript.json"
JOURNAL="$RUNTIME_ROOT/journals/journal.jsonl"
REPLAY="$RUNTIME_ROOT/replay/multiplayer-replay-proof.json"
RECEIPT_DIR="$RUNTIME_ROOT/receipts"
require_file "$SESSION" "Session Started"
require_file "$STATE" "Arena State"
require_file "$TRANSCRIPT" "Multiplayer Transcript"
require_file "$JOURNAL" "Journal Stream"
require_file "$REPLAY" "Replay Generated"

node_check "Session Started" "import fs from 'node:fs'; const s=JSON.parse(fs.readFileSync('$SESSION','utf8')); if (s.session_id!=='session-0001' || s.tick < 5 || s.player_count !== 2 || !s.players.includes('player-a') || !s.players.includes('player-b') || s.status!=='Multiplayer Local Session Proven') process.exit(1);"
node_check "Player A Joined" "import fs from 'node:fs'; const s=JSON.parse(fs.readFileSync('$STATE','utf8')); if (!s.players['player-a'] || s.player_count !== 2) process.exit(1);"
node_check "Player B Joined" "import fs from 'node:fs'; const s=JSON.parse(fs.readFileSync('$STATE','utf8')); if (!s.players['player-b'] || s.player_count !== 2) process.exit(1);"
node_check "Player A Moved" "import fs from 'node:fs'; const s=JSON.parse(fs.readFileSync('$STATE','utf8')); const p=s.positions['player-a']; if (!p || p.y !== 1) process.exit(1);"
node_check "Player B Moved" "import fs from 'node:fs'; const s=JSON.parse(fs.readFileSync('$STATE','utf8')); const p=s.positions['player-b']; if (!p || p.y !== -1) process.exit(1);"
node_check "Player Interaction" "import fs from 'node:fs'; const s=JSON.parse(fs.readFileSync('$STATE','utf8')); if (s.health['player-b'] !== 90 || !s.events.some(e=>e.includes('player-a attacked player-b'))) process.exit(1);"
node_check "Shared State" "import fs from 'node:fs'; const s=JSON.parse(fs.readFileSync('$STATE','utf8')); if (s.session_id !== 'session-0001' || s.player_count !== 2 || !s.positions['player-a'] || !s.positions['player-b'] || s.scores['player-a'] < 15) process.exit(1);"
node_check "Multi-Tick Progression" "import fs from 'node:fs'; const t=JSON.parse(fs.readFileSync('$TRANSCRIPT','utf8')); const roots=new Set(t.map(e=>e.state_root)); if (t.length < 6 || t[0].tick !== 1 || t[t.length-1].tick < 5 || roots.size < 5) process.exit(1);"
node_check "Receipt Stream" "import fs from 'node:fs'; const files=fs.readdirSync('$RECEIPT_DIR').filter(f=>f.endsWith('.json')); const receipts=files.map(f=>JSON.parse(fs.readFileSync('$RECEIPT_DIR/'+f,'utf8')).payload ?? JSON.parse(fs.readFileSync('$RECEIPT_DIR/'+f,'utf8'))); if (!receipts.some(r=>r.player_id==='player-a') || !receipts.some(r=>r.player_id==='player-b')) process.exit(1); for (const r of receipts) if (!r.player_id || !r.session_id || !r.action || !r.state_root || !r.receipt_hash || !r.guest_hash) process.exit(1);"
node_check "Journal Stream" "import fs from 'node:fs'; const entries=fs.readFileSync('$JOURNAL','utf8').trim().split(/\n+/).map(JSON.parse); if (!entries.some(e=>e.player_id==='player-a') || !entries.some(e=>e.player_id==='player-b')) process.exit(1); for (const e of entries) if (!e.tick || !e.player_id || !e.action || !e.state_root || !e.receipt_hash) process.exit(1);"
node_check "Replay Generated" "import fs from 'node:fs'; const p=JSON.parse(fs.readFileSync('$REPLAY','utf8')); if (!p.replay_root || !p.final_session_root || p.guest_execution.indexOf('Arena Guest WASM') < 0) process.exit(1);"
node_check "Replay Verified" "import fs from 'node:fs'; const p=JSON.parse(fs.readFileSync('$REPLAY','utf8')); if (p.replay_verification !== 'PASS' || p.replay_root !== p.final_session_root || p.status !== 'Multiplayer Local Session: PASS') process.exit(1);"

node_check "Multiplayer Transcript" "import fs from 'node:fs'; const t=JSON.parse(fs.readFileSync('$TRANSCRIPT','utf8')); const checks=[['join','player-a'],['join','player-b'],['move','player-a'],['move','player-b'],['attack','player-a'],['score_update','player-a']]; for (const [action,player] of checks) if (!t.some(e=>e.action===action && e.player_id===player)) process.exit(1);"
echo "Multiplayer Replay Verification: PASS" | tee -a "$REPORT"
echo "Overall Result: PASS" | tee -a "$REPORT"
