#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/player_gateway_validation_report.txt"
mkdir -p "$REPORT_DIR"

hash_file() {
  python3 - "$ROOT_DIR/$1" <<'PY'
from pathlib import Path
import hashlib
import sys
print(hashlib.sha256(Path(sys.argv[1]).read_bytes()).hexdigest())
PY
}

check_component() {
  local label="$1"
  local dir_rel="$2"
  local record_file="$3"
  local root_file="$4"
  local required_pattern="$5"
  local dir_path="$ROOT_DIR/$dir_rel"
  local record_path="$dir_path/$record_file"
  local root_path="$dir_path/$root_file"

  if [[ ! -d "$dir_path" ]]; then
    printf '%s: FAIL (missing directory %s)\n' "$label" "$dir_rel"
    return 1
  fi
  if [[ ! -s "$record_path" ]]; then
    printf '%s: FAIL (missing %s/%s)\n' "$label" "$dir_rel" "$record_file"
    return 1
  fi
  if [[ ! -s "$root_path" ]]; then
    printf '%s: FAIL (missing %s/%s)\n' "$label" "$dir_rel" "$root_file"
    return 1
  fi
  if ! grep -Eq "$required_pattern" "$record_path"; then
    printf '%s: FAIL (missing required records matching %s)\n' "$label" "$required_pattern"
    return 1
  fi

  local expected actual
  expected="$(tr -d '[:space:]' < "$root_path")"
  actual="$(hash_file "$dir_rel/$record_file")"
  if [[ "$label" == "Replay" ]]; then
    local gateway_root
    gateway_root="$(tr -d '[:space:]' < "$ROOT_DIR/player-gateway/records/GATEWAY_ROOT")"
    if [[ "$expected" != "$gateway_root" ]]; then
      printf '%s: FAIL (replay root does not equal gateway root)\n' "$label"
      printf '  replay:  %s\n' "$expected"
      printf '  gateway: %s\n' "$gateway_root"
      return 1
    fi
    if ! grep -q 'replay_equivalence=REPLAY_ROOT==GATEWAY_ROOT' "$record_path"; then
      printf '%s: FAIL (missing replay equivalence record)\n' "$label"
      return 1
    fi
  elif [[ "$expected" != "$actual" ]]; then
    printf '%s: FAIL (root mismatch)\n' "$label"
    printf '  expected: %s\n' "$expected"
    printf '  actual:   %s\n' "$actual"
    return 1
  fi

  printf '%s: PASS\n' "$label"
}

check_exact_root() {
  local label="$1"
  local record_rel="$2"
  local root_rel="$3"
  if [[ ! -s "$ROOT_DIR/$record_rel" || ! -s "$ROOT_DIR/$root_rel" ]]; then
    printf '%s: FAIL (missing root inputs)\n' "$label"
    return 1
  fi
  local expected actual
  expected="$(tr -d '[:space:]' < "$ROOT_DIR/$root_rel")"
  actual="$(hash_file "$record_rel")"
  if [[ "$expected" != "$actual" ]]; then
    printf '%s: FAIL (root mismatch)\n' "$label"
    printf '  expected: %s\n' "$expected"
    printf '  actual:   %s\n' "$actual"
    return 1
  fi
  printf '%s: PASS\n' "$label"
}

check_file() {
  local label="$1"
  local path_rel="$2"
  if [[ -s "$ROOT_DIR/$path_rel" ]]; then
    printf '%s: PASS\n' "$label"
  else
    printf '%s: FAIL (missing %s)\n' "$label" "$path_rel"
    return 1
  fi
}

run_checks() {
  check_component "Profiles" "player-gateway/profiles" "player_registry.records" "PLAYER_REGISTRY_ROOT" 'profile_action=(register|update|suspend|recover)'
  check_component "Characters" "player-gateway/characters" "character_system.records" "CHARACTER_ROOT" 'character_action=(create|archive|transfer|restore)'
  check_component "Library" "player-gateway/games" "game_library.records" "GAME_LIBRARY_ROOT" 'library_action=(browse|install|launch)'
  check_component "Sessions" "player-gateway/sessions" "session_layer.records" "SESSION_ROOT" 'session_action=(join|leave|resume)'
  check_component "Inventory" "player-gateway/inventory" "inventory_dashboard.records" "INVENTORY_ROOT" '(item=|equipment=|container=|vault_asset=|marketplace_asset=)'
  check_component "Marketplace" "player-gateway/marketplace" "marketplace_dashboard.records" "MARKETPLACE_ROOT" '(listing=|purchase=|sale=|creator_asset=|player_asset=)'
  check_component "Civilizations" "player-gateway/civilizations" "civilization_dashboard.records" "CIVILIZATION_ROOT" '(world=|region=|settlement=|governance=|event=)'
  check_component "Social" "player-gateway/social" "social_layer.records" "SOCIAL_ROOT" '(friendship=|guild=|group=|contact=)'
  check_component "Authority" "player-gateway/authority" "xrpl_xaman_surface.records" "AUTHORITY_ROOT" '(authority_status=|wallet_mapping=|settlement_history=|authorization_event=)'
  check_component "Analytics" "player-gateway/analytics" "analytics_layer.records" "ANALYTICS_ROOT" '(games_played=|session_time=|inventory_activity=|marketplace_activity=|civilization_activity=)'
  check_component "Replay" "player-gateway/replay" "gateway_replay.records" "REPLAY_ROOT" 'replay=(player-activity|characters|inventory|marketplace|sessions)'
  check_component "Launcher" "player-gateway/launcher" "launcher_integration.records" "LAUNCHER_ROOT" 'launcher_action=(browse|install|launch|update|remove)'
  check_component "Player Success" "player-gateway/metrics" "player_success.records" "PLAYER_SUCCESS_ROOT" '(registrations=|characters_created=|games_installed=|games_played=|marketplace_activity=|retention=)'
  check_exact_root "Gateway Layout Root" "player-gateway/records/gateway_layout.records" "player-gateway/records/GATEWAY_LAYOUT_ROOT"
  check_exact_root "Gateway Root" "player-gateway/records/gateway_root.records" "player-gateway/records/GATEWAY_ROOT"
  check_file "Player Guide" "docs/player-gateway/player-guide.md"
  check_file "Character Guide" "docs/player-gateway/character-guide.md"
  check_file "Inventory Guide" "docs/player-gateway/inventory-guide.md"
  check_file "Marketplace Guide" "docs/player-gateway/marketplace-guide.md"
  check_file "Game Library Guide" "docs/player-gateway/game-library-guide.md"
  check_file "Civilization Guide" "docs/player-gateway/civilization-guide.md"
}

timestamp="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
status="PASS"
output="$(run_checks 2>&1)" || status="FAIL"

{
  printf 'EverArcade Player Gateway Validation Report\n'
  printf 'Version: v0.1\n'
  printf 'Timestamp: %s\n\n' "$timestamp"
  printf '%s\n\n' "$output"
  printf 'Player Gateway Validation: %s\n' "$status"
} > "$REPORT_PATH"

cat "$REPORT_PATH"
[[ "$status" == "PASS" ]]
