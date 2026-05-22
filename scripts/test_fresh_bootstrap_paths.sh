#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

rm -rf runtime .everarcade-dev
cargo run -p everarcade-cli -- start >/dev/null

test -f runtime/world/status.txt
test -f runtime/replay/latest/frame-0001.json
test -f runtime/games/2d-arena/game.toml

echo "fresh_bootstrap_paths=ok"
