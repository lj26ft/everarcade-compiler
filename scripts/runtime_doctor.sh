#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [[ -x "$SCRIPT_DIR/../bin/everarcade-runtime" ]]; then
  APPLIANCE_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
else
  APPLIANCE_DIR="${EVERARCADE_APPLIANCE_PREFIX:-$(cd "$SCRIPT_DIR/.." && pwd)/appliance/installed}"
fi
WORLD_ID="${EVERARCADE_WORLD_ID:-civilization-alpha}"
RUNTIME_ROOT="${EVERARCADE_RUNTIME_ROOT:-$APPLIANCE_DIR/runtime}"
PACKAGE_PATH="${EVERARCADE_PACKAGE_PATH:-$RUNTIME_ROOT/worlds/$WORLD_ID/package}"
ERRORS=()
require_dir() { [[ -d "$1" ]] || ERRORS+=("missing directory: $1"); }
require_file() { [[ -f "$1" ]] || ERRORS+=("missing file: $1"); }
require_exec() { [[ -x "$1" ]] || ERRORS+=("missing executable: $1"); }

require_exec "$APPLIANCE_DIR/bin/everarcade-runtime"
require_dir "$RUNTIME_ROOT/config"
require_dir "$RUNTIME_ROOT/worlds"
require_dir "$RUNTIME_ROOT/worlds/$WORLD_ID/journals"
require_dir "$RUNTIME_ROOT/worlds/$WORLD_ID/checkpoints"
require_dir "$RUNTIME_ROOT/worlds/$WORLD_ID/backups"
require_dir "$RUNTIME_ROOT/reports"
require_dir "$RUNTIME_ROOT/vendor"
require_file "$RUNTIME_ROOT/config/runtime.env"
require_file "$PACKAGE_PATH/manifest.json"
require_file "$RUNTIME_ROOT/vendor/VENDOR_ARTIFACTS.txt"

if [[ ${#ERRORS[@]} -eq 0 ]]; then
  if ! "$APPLIANCE_DIR/bin/everarcade-runtime" doctor "$RUNTIME_ROOT" "$WORLD_ID" "$PACKAGE_PATH" >/dev/null; then
    ERRORS+=("runtime doctor command failed")
  fi
fi

if [[ ${#ERRORS[@]} -ne 0 ]]; then
  printf 'FAIL\n' >&2
  printf 'Actionable errors:\n' >&2
  printf ' - %s\n' "${ERRORS[@]}" >&2
  exit 1
fi

echo "PASS"
echo "Doctor: PASS"
