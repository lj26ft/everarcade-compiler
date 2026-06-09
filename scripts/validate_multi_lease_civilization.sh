#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

REPORT="reports/multi_lease_civilization_validation_report.txt"
TMP="$(mktemp -d)"
PROJECT="$TMP/audit-civilization"
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

node creator-sdk/cli/everarcade.mjs new --template civilization --name audit-civilization --dir "$PROJECT" >/dev/null
node creator-sdk/cli/everarcade.mjs play-multi-lease-local --project "$PROJECT" --template civilization --runtime-root "$RUNTIME_ROOT" >/tmp/multi-lease-civilization.out
rg -q 'Multi-Lease Civilization Runtime: PASS' /tmp/multi-lease-civilization.out || fail "Multi-Lease Civilization Command"

CIV="$RUNTIME_ROOT/civilization"
require_file "$CIV/lease-a/identity.json" "Lease A Started"
require_file "$CIV/lease-b/identity.json" "Lease B Started"
require_file "$CIV/genesis.json" "Civilization Genesis"
require_file "$CIV/economy-ledger.jsonl" "Economy Continuity"
require_file "$CIV/inventory-ledger.jsonl" "Inventory Continuity"
require_file "$CIV/checkpoint-exchange.json" "Checkpoint Exchange"
require_file "$CIV/civilization-sync-proof.json" "Civilization Synchronization"
require_file "$CIV/lease-transition-proof.json" "Lease Transition"
require_file "$CIV/lease-failure-proof.json" "Lease Failure Simulation"
require_file "$CIV/civilization-recovery-proof.json" "Recovery Verification"
require_file "$CIV/replay/civilization-replay-proof.json" "Replay Generation"

node_check "Lease A Started" "import fs from 'node:fs'; const a=JSON.parse(fs.readFileSync('$CIV/lease-a/identity.json','utf8')); if (a.lease_id !== 'lease-a' || !a.runtime_id || !a.federation_id || !a.civilization_id || !a.epoch || !a.authority_root) process.exit(1);"
node_check "Lease B Started" "import fs from 'node:fs'; const a=JSON.parse(fs.readFileSync('$CIV/lease-a/identity.json','utf8')); const b=JSON.parse(fs.readFileSync('$CIV/lease-b/identity.json','utf8')); if (b.lease_id !== 'lease-b' || a.lease_id === b.lease_id || a.civilization_id !== b.civilization_id || a.federation_id !== b.federation_id || a.authority_root !== b.authority_root) process.exit(1);"
node_check "Civilization Genesis" "import fs from 'node:fs'; const g=JSON.parse(fs.readFileSync('$CIV/genesis.json','utf8')); if (!g.civilization_id || !g.founding_epoch || !g.population || !g.inventory_root || !g.economy_root || !g.world_root || g.status !== 'Genesis established') process.exit(1);"
node_check "Economy Continuity" "import fs from 'node:fs'; const p=JSON.parse(fs.readFileSync('$CIV/economy-continuity-proof.json','utf8')); const rows=fs.readFileSync('$CIV/economy-ledger.jsonl','utf8').trim().split(/\n+/).map(JSON.parse); for (const a of ['resource_creation','resource_consumption','economic_transfer','score_update']) if (!p.actions.includes(a)) process.exit(1); if (!p.economy_root_changed || !p.economy_root_preserved_after_synchronization || rows.length < 4 || rows.some(r=>!r.civilization_id || !r.lease_id || !r.epoch || !r.civilization_root || !r.economy_root || !r.inventory_root)) process.exit(1);"
node_check "Inventory Continuity" "import fs from 'node:fs'; const p=JSON.parse(fs.readFileSync('$CIV/inventory-continuity-proof.json','utf8')); const rows=fs.readFileSync('$CIV/inventory-ledger.jsonl','utf8').trim().split(/\n+/).map(JSON.parse); for (const a of ['item_creation','item_transfer','item_ownership_verification']) if (!p.actions.includes(a)) process.exit(1); if (!p.inventory_continuity_preserved || !p.owner_verified || rows.length < 3 || rows.some(r=>!r.civilization_id || !r.lease_id || !r.epoch || !r.civilization_root || !r.economy_root || !r.inventory_root)) process.exit(1);"
node_check "Checkpoint Exchange" "import fs from 'node:fs'; const x=JSON.parse(fs.readFileSync('$CIV/checkpoint-exchange.json','utf8')); const s=JSON.parse(fs.readFileSync('$CIV/lease-a/lease-a-state.json','utf8')); if (!x.checkpoint_imported || !x.checkpoint_verified || !s.checkpoint_generated || !s.receipt_generated || !s.journal_generated || !x.checkpoint_root || !x.receipt_root || !x.journal_root) process.exit(1);"
node_check "Civilization Synchronization" "import fs from 'node:fs'; const s=JSON.parse(fs.readFileSync('$CIV/civilization-sync-proof.json','utf8')); if (!s.synchronized || s.lease_a_civilization_root !== s.lease_b_civilization_root || s.status !== 'PASS') process.exit(1);"
node_check "Lease Transition" "import fs from 'node:fs'; const p=JSON.parse(fs.readFileSync('$CIV/lease-transition-proof.json','utf8')); if (!p.lease_a_not_lease_b || !p.same_civilization_id || !p.civilization_survives_transition || p.lease_a_continuity_root !== p.lease_b_continuity_root || p.status !== 'PASS') process.exit(1);"
node_check "Lease Failure Simulation" "import fs from 'node:fs'; const p=JSON.parse(fs.readFileSync('$CIV/lease-failure-proof.json','utf8')); if (!p.lease_a_unavailable || !p.lease_b_continues_civilization || p.state_loss_detected !== false || p.status !== 'PASS') process.exit(1);"
node_check "Recovery Verification" "import fs from 'node:fs'; const p=JSON.parse(fs.readFileSync('$CIV/civilization-recovery-proof.json','utf8')); if (p.checkpoint_import !== 'PASS' || p.receipt_replay !== 'PASS' || p.journal_replay !== 'PASS' || p.state_reconstruction !== 'PASS' || !p.civilization_restored || p.status !== 'PASS') process.exit(1);"
node_check "Replay Generation" "import fs from 'node:fs'; const p=JSON.parse(fs.readFileSync('$CIV/replay/civilization-replay-proof.json','utf8')); if (!p.replay_root || !p.civilization_root || !p.economy_root || !p.inventory_root) process.exit(1);"
node_check "Replay Verification" "import fs from 'node:fs'; const p=JSON.parse(fs.readFileSync('$CIV/replay/civilization-replay-proof.json','utf8')); if (!p.replay_root_matches_civilization_root || p.replay_root !== p.civilization_root || p.replay_verification !== 'PASS' || p.status !== 'Civilization Replay Verification: PASS') process.exit(1);"

echo "Civilization Replay Verification: PASS" | tee -a "$REPORT"
echo "Overall Result: PASS" | tee -a "$REPORT"
