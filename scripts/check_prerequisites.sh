#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=lib/common.sh
source "$ROOT/scripts/lib/common.sh"

cd "$ROOT"

failures=0
warnings=0

pass() { printf '%s: PASS\n' "$1"; }
warn() { printf '%s: WARNING - %s\n' "$1" "$2"; warnings=$((warnings + 1)); }
fail() { printf '%s: FAIL - %s\n' "$1" "$2" >&2; failures=$((failures + 1)); }

require_cmd() {
  local cmd="$1"
  local label="$2"
  local hint="$3"
  if command -v "$cmd" >/dev/null 2>&1; then
    pass "$label"
  else
    fail "$label" "$hint"
  fi
}

cat <<'BANNER'
EverArcade prerequisite check
BANNER

require_cmd bash "bash" "Install bash (should be present on Ubuntu/macOS)."
require_cmd cargo "cargo" "Install Rust via https://rustup.rs"
require_cmd node "node" "Install Node.js 18+ (https://nodejs.org or nvm)."
require_cmd tar "tar" "Install tar (usually preinstalled)."
if command -v sha256sum >/dev/null 2>&1 || command -v shasum >/dev/null 2>&1; then
  pass "checksum tool"
else
  fail "checksum tool" "Install sha256sum (coreutils) or shasum (macOS)."
fi
require_cmd grep "grep" "Install grep (usually preinstalled)."

if command -v rg >/dev/null 2>&1; then
  pass "ripgrep (rg)"
else
  warn "ripgrep (rg)" "optional; scripts fall back to grep"
fi

if command -v openssl >/dev/null 2>&1; then
  pass "openssl"
else
  warn "openssl" "optional for manifest signing scripts"
fi

for path in \
  README.md \
  scripts/validate_developer_onboarding.sh \
  scripts/validate_open_source_readiness.sh \
  scripts/ensure_vendor_offline.sh \
  examples/reference-certified-world-v1/operator/verify.sh \
  dist/vendor.tar.gz \
  dist/vendor.tar.gz.sha256 \
  .cargo/config.toml; do
  if [[ -e "$ROOT/$path" ]]; then
    pass "file $path"
  else
    fail "file $path" "missing required path"
  fi
done

if [[ -f "$ROOT/.cargo/config.toml" ]] && text_matches 'offline = true' "$ROOT/.cargo/config.toml"; then
  pass "cargo offline policy"
else
  fail "cargo offline policy" ".cargo/config.toml must set offline = true"
fi

printf '\nChecking offline vendor restore...\n'
if ! bash "$ROOT/scripts/ensure_vendor_offline.sh"; then
  fail "vendor offline restore" "run bash scripts/ensure_vendor_offline.sh"
fi

if vendor_offline_ok "$ROOT"; then
  pass "cargo metadata offline"
else
  fail "cargo metadata offline" "CARGO_NET_OFFLINE=true cargo metadata --offline --locked failed"
  print_vendor_fix_hint
fi

printf '\nChecking offline Cargo build capability...\n'
OFFLINE_LOG=/tmp/everarcade-prereq-offline-check.log
if offline_cargo_check_workspace "$ROOT" "$OFFLINE_LOG"; then
  pass "cargo check workspace offline"
else
  fail "cargo check workspace offline" "see $OFFLINE_LOG"
  tail -25 "$OFFLINE_LOG" >&2 || true
  print_vendor_fix_hint
fi

if CARGO_NET_OFFLINE=true CARGO_BUILD_JOBS=1 cargo check -p everarcade-runtime --offline --locked >/tmp/everarcade-prereq-runtime-check.log 2>&1; then
  pass "cargo check everarcade-runtime offline"
else
  fail "cargo check everarcade-runtime offline" "see /tmp/everarcade-prereq-runtime-check.log"
  tail -20 /tmp/everarcade-prereq-runtime-check.log >&2 || true
  print_vendor_fix_hint
fi

printf '\n'
if [[ "$failures" -eq 0 ]]; then
  if [[ "$warnings" -eq 0 ]]; then
    printf 'Prerequisites: PASS\n'
  else
    printf 'Prerequisites: PASS (%d warning(s); grep fallback active)\n' "$warnings"
  fi
  exit 0
fi

printf 'Prerequisites: FAIL (%d failure(s))\n' "$failures" >&2
exit 1