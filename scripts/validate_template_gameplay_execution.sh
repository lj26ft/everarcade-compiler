#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

REPORT="reports/template_gameplay_validation_report.txt"
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
pass "Arena Package"

node creator-sdk/cli/everarcade.mjs execute-template --project "$PROJECT" --template arena --runtime-root "$RUNTIME_ROOT" >/tmp/template-gameplay-execute.out
rg -q 'Template Gameplay Execution: PASS' /tmp/template-gameplay-execute.out || fail "Template Gameplay Execution"

STATE="$RUNTIME_ROOT/gameplay/arena-state.json"
JOURNAL="$RUNTIME_ROOT/journals/journal.jsonl"
REPLAY="$RUNTIME_ROOT/replay/gameplay-replay-proof.json"
RECEIPT_DIR="$RUNTIME_ROOT/receipts"
require_file "$STATE" "Arena State Mutation"
require_file "$JOURNAL" "Arena Journal Generation"
require_file "$REPLAY" "Arena Replay Generation"

node_check "Arena Join" "import fs from 'node:fs'; const s=JSON.parse(fs.readFileSync('$STATE','utf8')); if (!s.players['player-1']) process.exit(1);"
node_check "Arena Move" "import fs from 'node:fs'; const s=JSON.parse(fs.readFileSync('$STATE','utf8')); const p=s.positions['player-1']; if (!p || p.y !== 1) process.exit(1);"
node_check "Arena Attack" "import fs from 'node:fs'; const s=JSON.parse(fs.readFileSync('$STATE','utf8')); if (s.health.dummy !== 90) process.exit(1);"
node_check "Arena State Mutation" "import fs from 'node:fs'; const p=JSON.parse(fs.readFileSync('$REPLAY','utf8')); if (!p.state_root_changed || !p.player_exists || !p.position_changed || !p.score_changed || !p.health_changed) process.exit(1);"
node_check "Arena Receipt Generation" "import fs from 'node:fs'; const files=fs.readdirSync('$RECEIPT_DIR').filter(f=>f.endsWith('.json')); if (files.length < 4) process.exit(1); const receipts=files.map(f=>{const r=JSON.parse(fs.readFileSync('$RECEIPT_DIR/'+f,'utf8')); return r.payload ?? r;}); for (const action of ['join','move','attack','score_update']) if (!receipts.some(r=>r.action===action && r.player_id==='player-1' && r.state_root && r.receipt_hash && r.tick && r.world_id==='$WORLD_ID')) process.exit(1);"
node_check "Arena Journal Generation" "import fs from 'node:fs'; const entries=fs.readFileSync('$JOURNAL','utf8').trim().split(/\\n+/).map(JSON.parse); for (const action of ['join','move','attack']) if (!entries.some(e=>e.action===action && e.player_id==='player-1' && e.tick && e.state_root && e.receipt_hash)) process.exit(1);"
node_check "Arena Replay Generation" "import fs from 'node:fs'; const p=JSON.parse(fs.readFileSync('$REPLAY','utf8')); if (p.proof_version !== 'template-gameplay-execution-proof-v0.1' || !p.replay_root) process.exit(1);"
node_check "Arena Replay Verification" "import fs from 'node:fs'; const p=JSON.parse(fs.readFileSync('$REPLAY','utf8')); if (p.replay_verification !== 'PASS' || p.replay_root !== p.execution_root) process.exit(1);"

echo "Gameplay Replay Verification: PASS" | tee -a "$REPORT"
echo "Overall Result: PASS" | tee -a "$REPORT"
