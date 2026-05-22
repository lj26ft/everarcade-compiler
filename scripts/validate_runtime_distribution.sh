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

for d in bin config world games assets manifests contracts hooks replay tools clients examples templates logs scripts; do
  [[ -d "$WORK/runtime/$d" ]] || { echo "missing runtime/$d"; exit 1; }
done

"$WORK/runtime/bin/everarcade" list-games >/dev/null
"$WORK/runtime/bin/everarcade" asset-build >/dev/null
"$WORK/runtime/bin/everarcade" start-game 2d-arena >/dev/null

echo "validate_runtime_distribution=ok archive=$ARCHIVE"
