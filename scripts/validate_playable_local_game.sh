#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

REPORT="reports/playable_local_game_validation_report.txt"
TMP="$(mktemp -d)"
PROJECT="$TMP/audit-arena"
RUNTIME_ROOT="$TMP/runtime-root"
WORLD_ID="audit-arena-world"
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
node creator-sdk/cli/everarcade.mjs build --project "$PROJECT" >/dev/null
node creator-sdk/cli/everarcade.mjs test --project "$PROJECT" >/dev/null
node creator-sdk/cli/everarcade.mjs package --project "$PROJECT" >/dev/null
node creator-sdk/cli/everarcade.mjs play-local --project "$PROJECT" --template arena --runtime-root "$RUNTIME_ROOT" >/tmp/playable-local-game.out
rg -q 'Playable Local Game: PASS' /tmp/playable-local-game.out || fail "Playable Local Game Command"

SESSION="$RUNTIME_ROOT/sessions/session-0001.json"
STATE="$RUNTIME_ROOT/gameplay/arena-state.json"
TRANSCRIPT="$RUNTIME_ROOT/gameplay/session-transcript.json"
JOURNAL="$RUNTIME_ROOT/journals/journal.jsonl"
REPLAY="$RUNTIME_ROOT/replay/gameplay-replay-proof.json"
RECEIPT_DIR="$RUNTIME_ROOT/receipts"
require_file "$SESSION" "Session Started"
require_file "$STATE" "Arena State"
require_file "$TRANSCRIPT" "Session Transcript"
require_file "$JOURNAL" "Journal Stream Generated"
require_file "$REPLAY" "Replay Root Generated"

node_check "Session Started" "import fs from 'node:fs'; const s=JSON.parse(fs.readFileSync('$SESSION','utf8')); if (s.session_id!=='session-0001' || s.tick < 5 || s.player_count < 1 || !s.state_root || s.status!=='Playable Local Game Proven') process.exit(1);"
node_check "Player Joined" "import fs from 'node:fs'; const s=JSON.parse(fs.readFileSync('$STATE','utf8')); if (s.session_id!=='session-0001' || !s.players['player-1']) process.exit(1);"
node_check "Move Applied" "import fs from 'node:fs'; const s=JSON.parse(fs.readFileSync('$STATE','utf8')); const p=s.positions['player-1']; const j=fs.readFileSync('$JOURNAL','utf8').trim().split(/\\n+/).map(JSON.parse); if (!p || p.y !== 1 || !j.some(e=>e.action==='move' && e.state_root)) process.exit(1);"
node_check "Attack Applied" "import fs from 'node:fs'; const s=JSON.parse(fs.readFileSync('$STATE','utf8')); const files=fs.readdirSync('$RECEIPT_DIR').filter(f=>f.endsWith('.json')); const receipts=files.map(f=>JSON.parse(fs.readFileSync('$RECEIPT_DIR/'+f,'utf8')).payload ?? JSON.parse(fs.readFileSync('$RECEIPT_DIR/'+f,'utf8'))); if (s.health.dummy !== 90 || !receipts.some(r=>r.action==='attack' && r.receipt_hash)) process.exit(1);"
node_check "Score Updated" "import fs from 'node:fs'; const t=JSON.parse(fs.readFileSync('$TRANSCRIPT','utf8')); const before=0; const after=t[t.length-1].score; if (!(after !== before && t.some(e=>e.action==='attack') && t.some(e=>e.action==='score_update'))) process.exit(1);"
node_check "Multi-Tick Progression" "import fs from 'node:fs'; const s=JSON.parse(fs.readFileSync('$STATE','utf8')); const t=JSON.parse(fs.readFileSync('$TRANSCRIPT','utf8')); const roots=new Set(t.map(e=>e.state_root)); if (s.tick < 5 || t.length < 5 || roots.size < 4) process.exit(1);"
node_check "Receipt Stream Generated" "import fs from 'node:fs'; const files=fs.readdirSync('$RECEIPT_DIR').filter(f=>f.endsWith('.json')); if (files.length < 5) process.exit(1); for (const f of files) { const raw=JSON.parse(fs.readFileSync('$RECEIPT_DIR/'+f,'utf8')); const r=raw.payload ?? raw; if (!r.session_id || !r.tick || r.player_count === undefined || !r.state_root) process.exit(1); }"
node_check "Journal Stream Generated" "import fs from 'node:fs'; const entries=fs.readFileSync('$JOURNAL','utf8').trim().split(/\\n+/).map(JSON.parse); for (const action of ['join','move','attack','score_update']) if (!entries.some(e=>e.action===action && e.player_id==='player-1' && e.tick && e.state_root && e.receipt_hash)) process.exit(1);"
node_check "Replay Root Generated" "import fs from 'node:fs'; const p=JSON.parse(fs.readFileSync('$REPLAY','utf8')); if (!p.replay_root || !p.state_root || p.session_id!=='session-0001') process.exit(1);"
node_check "Replay Verified" "import fs from 'node:fs'; const p=JSON.parse(fs.readFileSync('$REPLAY','utf8')); if (p.replay_verification !== 'PASS' || p.replay_root !== p.state_root) process.exit(1);"
node_check "Session Transcript" "import fs from 'node:fs'; const t=JSON.parse(fs.readFileSync('$TRANSCRIPT','utf8')); if (t.length < 4) process.exit(1); for (const action of ['join','move','attack','score_update']) if (!t.some(e=>e.action===action)) process.exit(1);"

echo "Gameplay Replay Verification: PASS" | tee -a "$REPORT"
echo "Overall Result: PASS" | tee -a "$REPORT"
