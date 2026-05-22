#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DIST_DIR="${DIST_DIR:-$ROOT/dist}"
WORK="$(mktemp -d /tmp/everarcade-runtime-validate.XXXXXX)"
trap 'rm -rf "$WORK"' EXIT

require_file() {
  local path="$1"
  [[ -s "$path" ]] || { echo "ERROR: required file missing or empty: $path" >&2; exit 1; }
}

require_json() {
  local path="$1"
  python3 - <<'PY' "$path"
import json, pathlib, sys
p = pathlib.Path(sys.argv[1])
json.loads(p.read_text())
PY
}

# 1) Validate source-tree runtime output after canonical start.
rm -rf "$ROOT/runtime/world" "$ROOT/runtime/replay/latest" "$ROOT/runtime/logs"
mkdir -p "$ROOT/runtime/replay"
cargo run -p everarcade-cli -- start >/dev/null
require_file "$ROOT/runtime/world/status.txt"
require_file "$ROOT/runtime/replay/latest/frame-0001.json"
require_file "$ROOT/runtime/games/2d-arena/game.toml"
require_file "$ROOT/clients/web-reference/index.html"
require_json "$ROOT/runtime/replay/latest/frame-0001.json"

# 2) Build runtime layout and validate packaged runtime appliance output.
bash "$ROOT/scripts/build_runtime_layout.sh" "$WORK/runtime"
ARCHIVE="$DIST_DIR/everarcade-runtime-v0.1.0.tar.gz"
mkdir -p "$DIST_DIR"
tar --sort=name --mtime='UTC 2020-01-01' --owner=0 --group=0 --numeric-owner -C "$WORK" -czf "$ARCHIVE" runtime

for d in bin config world games assets manifests contracts hooks replay tools clients examples templates logs scripts; do
  [[ -d "$WORK/runtime/$d" ]] || { echo "missing runtime/$d" >&2; exit 1; }
done

(
  cd "$WORK/runtime"
  ./bin/everarcade start-game 2d-arena >/dev/null
  if [[ -s "world/status.txt" ]]; then
    require_file "world/status.txt"
    require_file "replay/latest/frame-0001.json"
    require_file "games/2d-arena/game.toml"
    require_json "replay/latest/frame-0001.json"
  else
    require_file "runtime/world/status.txt"
    require_file "runtime/replay/latest/frame-0001.json"
    require_file "runtime/games/2d-arena/game.toml"
    require_json "runtime/replay/latest/frame-0001.json"
  fi
  require_file "clients/web-reference/index.html"
)

echo "validate_runtime_distribution=ok archive=$ARCHIVE"
