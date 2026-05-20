#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT_ROOT="${1:-$ROOT/dist/runtime}"
PROFILE="${PROFILE:-release}"
TARGET_DIR="$ROOT/target/$PROFILE"
RUNTIME_SRC="$ROOT/runtime"

require_file() {
  [[ -f "$1" ]] || { echo "missing required file: $1" >&2; exit 1; }
}

mkdir -p "$OUT_ROOT"
rm -rf "$OUT_ROOT"
mkdir -p "$OUT_ROOT"/{bin,config,world,logs,contracts,vendor,scripts}
mkdir -p "$OUT_ROOT/world"/{federation,topology,leases}
mkdir -p "$OUT_ROOT/world"/{state,checkpoints,journal,receipts}

if [[ "$PROFILE" == "release" ]]; then
  cargo build --release --locked --frozen --offline -p everarcade-host -p everarcade-cli
else
  cargo build --locked --frozen --offline -p everarcade-host -p everarcade-cli
fi

require_file "$TARGET_DIR/everarcade-host"
cp "$TARGET_DIR/everarcade-host" "$OUT_ROOT/bin/everarcade-host"
strip "$OUT_ROOT/bin/everarcade-host" 2>/dev/null || true
if [[ -f "$TARGET_DIR/everarcade" ]]; then
  cp "$TARGET_DIR/everarcade" "$OUT_ROOT/bin/everarcade"
  strip "$OUT_ROOT/bin/everarcade" 2>/dev/null || true
fi

for optional_bin in execution-core-tool verification-tool scheduler-tool; do
  if [[ -f "$TARGET_DIR/$optional_bin" ]]; then
    cp "$TARGET_DIR/$optional_bin" "$OUT_ROOT/bin/$optional_bin"
    strip "$OUT_ROOT/bin/$optional_bin" 2>/dev/null || true
  fi
done

require_file "$RUNTIME_SRC/config/runtime.toml"
require_file "$RUNTIME_SRC/config/federation.toml"
require_file "$RUNTIME_SRC/config/storage.toml"
require_file "$RUNTIME_SRC/scripts/start-everarcade.sh"

cp "$RUNTIME_SRC/config"/*.toml "$OUT_ROOT/config/"
cp "$RUNTIME_SRC/scripts/start-everarcade.sh" "$OUT_ROOT/scripts/start-everarcade.sh"
chmod +x "$OUT_ROOT/scripts/start-everarcade.sh" "$OUT_ROOT/bin/everarcade-host"

if [[ -d "$ROOT/vendor" ]]; then
  cp -R "$ROOT/vendor/." "$OUT_ROOT/vendor/"
fi
if [[ -d "$ROOT/contracts" ]]; then
  cp -R "$ROOT/contracts/." "$OUT_ROOT/contracts/"
fi

printf '{\n  "layout_version": 1,\n  "deterministic": true,\n  "profile": "%s",\n  "entrypoint": "scripts/start-everarcade.sh"\n}\n' "$PROFILE" > "$OUT_ROOT/runtime-manifest.json"

echo "build_runtime_layout=ok out=$OUT_ROOT"
