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

tar -xzf "$ARCHIVE" -C "$WORK"
bash "$WORK/runtime/scripts/start-everarcade.sh"
"$WORK/runtime/bin/everarcade-host" verify --state "$WORK/runtime/world/state"

echo "validate_runtime_distribution=ok archive=$ARCHIVE"
