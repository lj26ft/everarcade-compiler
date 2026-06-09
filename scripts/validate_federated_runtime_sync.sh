#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

REPORT="reports/federated_runtime_validation_report.txt"
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
node creator-sdk/cli/everarcade.mjs play-federated-local --project "$PROJECT" --template arena --runtime-root "$RUNTIME_ROOT" >/tmp/federated-runtime-sync.out
rg -q 'Federated Runtime Synchronization: PASS' /tmp/federated-runtime-sync.out || fail "Federated Runtime Command"

FED="$RUNTIME_ROOT/federation"
require_file "$FED/runtime-a/identity.json" "Runtime A Started"
require_file "$FED/runtime-b/identity.json" "Runtime B Started"
require_file "$FED/checkpoint-exchange.json" "Checkpoint Exchange"
require_file "$FED/receipt-exchange.json" "Receipt Exchange"
require_file "$FED/journal-exchange.json" "Journal Exchange"
require_file "$FED/sync-state.json" "Synchronization"
require_file "$FED/federated-gameplay-proof.json" "Federated Gameplay Sync"
require_file "$FED/divergence-proof.json" "Divergence Detection"
require_file "$FED/recovery-proof.json" "Recovery Verification"
require_file "$FED/replay/federation-replay-proof.json" "Replay Generation"

node_check "Runtime A Started" "import fs from 'node:fs'; const a=JSON.parse(fs.readFileSync('$FED/runtime-a/identity.json','utf8')); if (!a.runtime_id || !a.federation_id || !a.epoch || !a.authority_root) process.exit(1);"
node_check "Runtime B Started" "import fs from 'node:fs'; const a=JSON.parse(fs.readFileSync('$FED/runtime-a/identity.json','utf8')); const b=JSON.parse(fs.readFileSync('$FED/runtime-b/identity.json','utf8')); if (!b.runtime_id || a.runtime_id === b.runtime_id || a.federation_id !== b.federation_id || a.authority_root !== b.authority_root) process.exit(1);"
node_check "Checkpoint Exchange" "import fs from 'node:fs'; const x=JSON.parse(fs.readFileSync('$FED/checkpoint-exchange.json','utf8')); const ca=JSON.parse(fs.readFileSync('$FED/checkpoint-a.json','utf8')); const cb=JSON.parse(fs.readFileSync('$FED/checkpoint-b.json','utf8')); if (!x.checkpoint_verified || !x.checkpoint_accepted || !x.checkpoint_imported || !x.runtime_a_not_runtime_b || !x.same_federation_id || !ca.runtime_id || !cb.runtime_id || !ca.state_root || !cb.checkpoint_root) process.exit(1);"
node_check "Receipt Exchange" "import fs from 'node:fs'; const x=JSON.parse(fs.readFileSync('$FED/receipt-exchange.json','utf8')); const rows=fs.readFileSync('$FED/receipt-stream-a.jsonl','utf8').trim().split(/\n+/).map(JSON.parse); if (!x.receipts_verified || !x.receipts_imported || x.local_mutation !== false || rows.length < 5 || rows.some(r=>!r.runtime_id || !r.epoch || !r.state_root || !r.receipt_root)) process.exit(1);"
node_check "Journal Exchange" "import fs from 'node:fs'; const x=JSON.parse(fs.readFileSync('$FED/journal-exchange.json','utf8')); const rows=fs.readFileSync('$FED/journal-a.jsonl','utf8').trim().split(/\n+/).map(JSON.parse); if (!x.journal_verified || !x.journal_synchronized || rows.length < 5 || rows.some(r=>!r.runtime_id || !r.epoch || !r.state_root || !r.receipt_hash)) process.exit(1);"
node_check "Synchronization" "import fs from 'node:fs'; const s=JSON.parse(fs.readFileSync('$FED/sync-state.json','utf8')); if (!s.synchronized || s.runtime_a_root !== s.runtime_b_root || !s.state_root || !s.receipt_root || !s.checkpoint_root) process.exit(1);"
node_check "Federated Gameplay Sync" "import fs from 'node:fs'; const p=JSON.parse(fs.readFileSync('$FED/federated-gameplay-proof.json','utf8')); for (const a of ['join','move','attack','score_update']) if (!p.actions.includes(a)) process.exit(1); if (!p.gameplay_state_synchronized || p.reexecuted_locally !== false) process.exit(1);"
node_check "Divergence Detection" "import fs from 'node:fs'; const p=JSON.parse(fs.readFileSync('$FED/divergence-proof.json','utf8')); if (!p.divergence_detected || !p.checkpoint_rejected || !p.synchronization_halted || p.status !== 'PASS') process.exit(1);"
node_check "Recovery Verification" "import fs from 'node:fs'; const p=JSON.parse(fs.readFileSync('$FED/recovery-proof.json','utf8')); if (!p.recovery_successful || !p.state_restored || p.status !== 'PASS' || !p.state_root || !p.receipt_root || !p.checkpoint_root) process.exit(1);"
node_check "Replay Generation" "import fs from 'node:fs'; const p=JSON.parse(fs.readFileSync('$FED/replay/federation-replay-proof.json','utf8')); if (!p.runtime_a_replay_root || !p.runtime_b_replay_root || !p.federation_root) process.exit(1);"
node_check "Replay Verification" "import fs from 'node:fs'; const p=JSON.parse(fs.readFileSync('$FED/replay/federation-replay-proof.json','utf8')); if (p.runtime_a_replay_root !== p.runtime_b_replay_root || p.runtime_b_replay_root !== p.federation_root || p.replay_verification !== 'PASS') process.exit(1);"

echo "Federation Replay Verification: PASS" | tee -a "$REPORT"
echo "Overall Result: PASS" | tee -a "$REPORT"
