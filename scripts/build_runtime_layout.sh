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

rm -rf "$OUT_ROOT"
mkdir -p "$OUT_ROOT"/{bin,config,world,games,assets,manifests,contracts,hooks,replay,tools,clients,examples,templates,logs,scripts,vendor}

if [[ "$PROFILE" == "release" ]]; then
  cargo build --release --locked --frozen --offline -p everarcade-host -p everarcade-cli
else
  cargo build --locked --frozen --offline -p everarcade-host -p everarcade-cli
fi

require_file "$TARGET_DIR/everarcade-host"
cp "$TARGET_DIR/everarcade-host" "$OUT_ROOT/bin/everarcade-host"
if [[ -f "$TARGET_DIR/everarcade" ]]; then
  cp "$TARGET_DIR/everarcade" "$OUT_ROOT/bin/everarcade"
fi
chmod +x "$OUT_ROOT/bin/everarcade-host" "$OUT_ROOT/bin/everarcade" 2>/dev/null || true

cp "$RUNTIME_SRC/config"/*.toml "$OUT_ROOT/config/"
cp "$RUNTIME_SRC/scripts/start-everarcade.sh" "$OUT_ROOT/scripts/start-everarcade.sh"
chmod +x "$OUT_ROOT/scripts/start-everarcade.sh"

for dir in games assets manifests hooks replay tools clients examples templates contracts; do
  if [[ -d "$ROOT/$dir" ]]; then cp -R "$ROOT/$dir/." "$OUT_ROOT/$dir/"; fi
done
if [[ -d "$ROOT/tools/asset-pipeline" ]]; then cp -R "$ROOT/tools/asset-pipeline/." "$OUT_ROOT/tools/"; fi
if [[ -d "$ROOT/runtime/hooks" ]]; then cp -R "$ROOT/runtime/hooks/." "$OUT_ROOT/hooks/"; fi
if [[ -d "$ROOT/vendor" ]]; then cp -R "$ROOT/vendor/." "$OUT_ROOT/vendor/"; fi

cat > "$OUT_ROOT/runtime-manifest.json" <<MAN
{
  "layout_version": 2,
  "profile": "$PROFILE",
  "entrypoint": "scripts/start-everarcade.sh",
  "client": "clients/web-reference/index.html"
}
MAN

echo "build_runtime_layout=ok out=$OUT_ROOT"
