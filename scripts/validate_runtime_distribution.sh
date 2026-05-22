#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DIST_DIR="${DIST_DIR:-$ROOT/dist}"
WORK="$(mktemp -d /tmp/everarcade-runtime-validate.XXXXXX)"
trap 'rm -rf "$WORK"' EXIT

bash "$ROOT/scripts/build_runtime_layout.sh" "$WORK/runtime"
ARCHIVE="$DIST_DIR/everarcade-runtime-v0.1.0.tar.gz"
mkdir -p "$DIST_DIR"
tar --sort=name --mtime='UTC 2020-01-01' --owner=0 --group=0 --numeric-owner -C "$WORK" -czf "$ARCHIVE" runtime

cargo run -p everarcade-cli -- start >/dev/null
test -f "$ROOT/runtime/world/status.txt"
test -f "$ROOT/runtime/replay/latest/frame-0001.json"
test -f "$ROOT/runtime/games/2d-arena/game.toml"

for d in bin config world games assets manifests contracts hooks replay tools clients examples templates logs scripts; do
  [[ -d "$WORK/runtime/$d" ]] || { echo "missing runtime/$d"; exit 1; }
done

(
  cd "$WORK/runtime"
  ./bin/everarcade start-game 2d-arena >/dev/null
  if [[ -f runtime/world/status.txt ]]; then
    test -f runtime/world/status.txt
  else
    test -f world/status.txt
  fi

  if [[ -f runtime/replay/latest/frame-0001.json ]]; then
    test -f runtime/replay/latest/frame-0001.json
  else
    test -f replay/latest/frame-0001.json
  fi
)

echo "validate_runtime_distribution=ok archive=$ARCHIVE"
