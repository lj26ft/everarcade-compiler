#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

REPORT="reports/developer_onboarding_validation_report.txt"
TMP="$(mktemp -d)"
PROJECT="$TMP/onboarding-arena"
RUNTIME_ROOT="$TMP/runtime-root"
mkdir -p reports
: > "$REPORT"
trap 'rm -rf "$TMP"' EXIT

pass() { printf '%s: PASS\n' "$1" | tee -a "$REPORT"; }
fail() { printf '%s: FAIL\n' "$1" | tee -a "$REPORT"; exit 1; }
require_file() { [[ -f "$1" ]] || fail "$2"; }

require_file README.md "Repository Bootstrap"
require_file docs/onboarding/30-minute-developer-journey.md "Repository Bootstrap"
require_file docs/repository/repository-map.md "Repository Bootstrap"
require_file creator-sdk/cli/everarcade.mjs "Repository Bootstrap"
require_file runtime/everarcade-runtime/Cargo.toml "Repository Bootstrap"
pass "Repository Bootstrap"

node creator-sdk/cli/everarcade.mjs new --template arena --name onboarding-arena --dir "$PROJECT" >/tmp/everarcade-onboarding-new.out || fail "Creator SDK"
require_file "$PROJECT/everarcade.game.json" "Creator SDK"

node creator-sdk/cli/everarcade.mjs build --project "$PROJECT" >/tmp/everarcade-onboarding-build.out || fail "Developer Build"
node creator-sdk/cli/everarcade.mjs test --project "$PROJECT" >/tmp/everarcade-onboarding-test.out || fail "Developer Build"
require_file "$PROJECT/dist/build.json" "Developer Build"
pass "Developer Build"
pass "Creator SDK"

node creator-sdk/cli/everarcade.mjs package --project "$PROJECT" >/tmp/everarcade-onboarding-package.out || fail "Runtime Package"
require_file "$PROJECT/dist/runtime-package/manifest.json" "Runtime Package"
require_file "$PROJECT/dist/runtime-package/world.json" "Runtime Package"
require_file "$PROJECT/dist/runtime-package/world.wasm" "Runtime Package"
pass "Runtime Package"

CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" node creator-sdk/cli/everarcade.mjs play-local --project "$PROJECT" --template arena --runtime-root "$RUNTIME_ROOT" >/tmp/everarcade-onboarding-play.out || fail "Playable Local Game"
rg -q 'Playable Local Game: PASS' /tmp/everarcade-onboarding-play.out || fail "Playable Local Game"
require_file "$RUNTIME_ROOT/sessions/session-0001.json" "Playable Local Game"
require_file "$RUNTIME_ROOT/gameplay/arena-state.json" "Playable Local Game"
require_file "$RUNTIME_ROOT/journals/journal.jsonl" "Playable Local Game"
require_file "$RUNTIME_ROOT/replay/gameplay-replay-proof.json" "Playable Local Game"
pass "Playable Local Game"

node --input-type=module -e "import fs from 'node:fs'; const replay=JSON.parse(fs.readFileSync('$RUNTIME_ROOT/replay/gameplay-replay-proof.json','utf8')); if (replay.replay_verification !== 'PASS' || !replay.replay_root || replay.replay_root !== replay.state_root) process.exit(1);" || fail "Replay Verification"
pass "Replay Verification"

printf '\nDeveloper Onboarding Validation: PASS\n' | tee -a "$REPORT"
