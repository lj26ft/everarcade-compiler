#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

fail() {
  echo "ERROR: $*" >&2
  exit 1
}

require_file() {
  local path="$1"
  [[ -s "$path" ]] || fail "required file missing or empty: $path"
}

require_json() {
  local path="$1"
  python3 - <<'PY' "$path"
import json, pathlib, sys
p = pathlib.Path(sys.argv[1])
json.loads(p.read_text())
PY
}

# Only wipe generated runtime state; keep canonical runtime layout under version control.
rm -rf runtime/.everarcade-dev runtime/world runtime/replay/latest runtime/logs
mkdir -p runtime/replay

cargo run -p everarcade-cli -- start >/dev/null

for file in \
  runtime/config/federation.toml \
  runtime/config/runtime.toml \
  runtime/config/storage.toml \
  runtime/games/2d-arena/README.md \
  runtime/games/2d-arena/assets.toml \
  runtime/games/2d-arena/client.toml \
  runtime/games/2d-arena/game.toml \
  runtime/games/2d-arena/simulation.toml \
  runtime/games/2d-arena/world.toml \
  runtime/manifests/xahau-hooks.toml \
  runtime/replay/latest/frame-0001.json \
  runtime/scripts/start-everarcade.sh \
  runtime/world/status.txt
  do
  require_file "$file"
done

require_json runtime/replay/latest/frame-0001.json

# Stale primary path should not be required for runtime-first bootstrap.
if [[ -e .everarcade-dev/world/status.txt ]]; then
  fail "stale bootstrap artifact unexpectedly appeared in .everarcade-dev/world/status.txt"
fi

echo "fresh_bootstrap_paths=ok"
