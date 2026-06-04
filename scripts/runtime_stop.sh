#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [[ -x "$SCRIPT_DIR/../bin/everarcade-runtime" ]]; then APPLIANCE_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"; else APPLIANCE_DIR="${EVERARCADE_APPLIANCE_PREFIX:-$(cd "$SCRIPT_DIR/.." && pwd)/appliance/installed}"; fi
WORLD_ID="${EVERARCADE_WORLD_ID:-civilization-alpha}"
RUNTIME_ROOT="${EVERARCADE_RUNTIME_ROOT:-$APPLIANCE_DIR/runtime}"
PACKAGE_PATH="${EVERARCADE_PACKAGE_PATH:-$RUNTIME_ROOT/worlds/$WORLD_ID/package}"
"$APPLIANCE_DIR/bin/everarcade-runtime" stop "$RUNTIME_ROOT" "$WORLD_ID" "$PACKAGE_PATH"
