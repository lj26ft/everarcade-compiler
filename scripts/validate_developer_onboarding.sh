#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=lib/common.sh
source "$REPO_ROOT/scripts/lib/common.sh"
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
fail_with_hint() {
  local label="$1"
  local log="$2"
  printf '%s: FAIL\n' "$label" | tee -a "$REPORT"
  if [[ -f "$log" ]]; then
    tail -30 "$log" >&2 || true
  fi
  print_vendor_fix_hint
  exit 1
}
require_file() { [[ -f "$1" ]] || fail "$2"; }
section() { printf '\n== %s ==\n' "$1" | tee -a "$REPORT"; }

section "Vendor offline"
bash "$REPO_ROOT/scripts/ensure_vendor_offline.sh" >/tmp/everarcade-onboarding-vendor.out 2>&1 || fail_with_hint "Vendor Offline" /tmp/everarcade-onboarding-vendor.out
pass "Vendor Offline"

section "Repository bootstrap"
require_file README.md "Repository Bootstrap"
require_file docs/onboarding/30-minute-developer-journey.md "Repository Bootstrap"
require_file docs/repository/repository-map.md "Repository Bootstrap"
require_file creator-sdk/cli/everarcade.mjs "Repository Bootstrap"
require_file runtime/everarcade-runtime/Cargo.toml "Repository Bootstrap"
pass "Repository Bootstrap"

section "Creator SDK"
node creator-sdk/cli/everarcade.mjs new --template arena --name onboarding-arena --dir "$PROJECT" >/tmp/everarcade-onboarding-new.out 2>&1 || fail_with_hint "Creator SDK" /tmp/everarcade-onboarding-new.out
require_file "$PROJECT/everarcade.game.json" "Creator SDK"

node creator-sdk/cli/everarcade.mjs build --project "$PROJECT" >/tmp/everarcade-onboarding-build.out 2>&1 || fail_with_hint "Developer Build" /tmp/everarcade-onboarding-build.out
node creator-sdk/cli/everarcade.mjs test --project "$PROJECT" >/tmp/everarcade-onboarding-test.out 2>&1 || fail_with_hint "Developer Build" /tmp/everarcade-onboarding-test.out
require_file "$PROJECT/dist/build.json" "Developer Build"
pass "Developer Build"
pass "Creator SDK"

section "Runtime package"
node creator-sdk/cli/everarcade.mjs package --project "$PROJECT" >/tmp/everarcade-onboarding-package.out 2>&1 || fail_with_hint "Runtime Package" /tmp/everarcade-onboarding-package.out
require_file "$PROJECT/dist/runtime-package/manifest.json" "Runtime Package"
require_file "$PROJECT/dist/runtime-package/world.json" "Runtime Package"
require_file "$PROJECT/dist/runtime-package/world.wasm" "Runtime Package"
pass "Runtime Package"

section "Runtime local-session (play-local)"
printf 'Running play-local against repo-root offline vendor...\n' | tee -a "$REPORT"
if ! CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" CARGO_NET_OFFLINE=true \
  node creator-sdk/cli/everarcade.mjs play-local --project "$PROJECT" --template arena --runtime-root "$RUNTIME_ROOT" \
  >/tmp/everarcade-onboarding-play.out 2>&1; then
  printf 'ERROR: Runtime local-session failed.\n' >&2
  printf 'Most common cause: incomplete vendor/ or play-local not using repo-root cargo workspace.\n' >&2
  fail_with_hint "Playable Local Game" /tmp/everarcade-onboarding-play.out
fi
text_matches 'Playable Local Game: PASS' /tmp/everarcade-onboarding-play.out || fail_with_hint "Playable Local Game" /tmp/everarcade-onboarding-play.out
require_file "$RUNTIME_ROOT/sessions/session-0001.json" "Playable Local Game"
require_file "$RUNTIME_ROOT/gameplay/arena-state.json" "Playable Local Game"
require_file "$RUNTIME_ROOT/journals/journal.jsonl" "Playable Local Game"
require_file "$RUNTIME_ROOT/replay/gameplay-replay-proof.json" "Playable Local Game"
pass "Playable Local Game"

section "Replay verification"
node --input-type=module -e "import fs from 'node:fs'; const replay=JSON.parse(fs.readFileSync('$RUNTIME_ROOT/replay/gameplay-replay-proof.json','utf8')); if (replay.replay_verification !== 'PASS' || !replay.replay_root || replay.replay_root !== replay.state_root) process.exit(1);" || fail "Replay Verification"
pass "Replay Verification"

printf '\nDeveloper Onboarding Validation: PASS\n' | tee -a "$REPORT"