#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/player_gateway_certification_report.txt"
mkdir -p "$REPORT_DIR"

VALIDATION_OUTPUT="$(bash "$ROOT_DIR/scripts/validate_player_gateway.sh")"
status="PASS"

require_pass() {
  local label="$1"
  if ! grep -q "^${label}: PASS$" <<<"$VALIDATION_OUTPUT"; then
    printf '%s: FAIL\n' "$label"
    return 1
  fi
  printf '%s: PASS\n' "$label"
}

run_checks() {
  require_pass "Profiles"
  require_pass "Characters"
  require_pass "Library"
  require_pass "Sessions"
  require_pass "Inventory"
  require_pass "Marketplace"
  require_pass "Civilizations"
  require_pass "Social"
  require_pass "Authority"
  require_pass "Analytics"
  require_pass "Replay"
  require_pass "Launcher"
}

timestamp="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
output="$(run_checks 2>&1)" || status="FAIL"

{
  printf 'EverArcade Player Gateway Certification Report\n'
  printf 'Version: v0.1\n'
  printf 'Timestamp: %s\n\n' "$timestamp"
  printf '%s\n\n' "$output"
  printf 'Player Gateway v0.1: %s\n' "$status"
} > "$REPORT_PATH"

cat "$REPORT_PATH"
[[ "$status" == "PASS" ]]
