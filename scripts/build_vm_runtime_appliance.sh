#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DIST_DIR="${DIST_DIR:-$ROOT/dist}"
OUT="$DIST_DIR/everarcade-runtime-appliance-v0.1.0.tar.gz"
WORK="$(mktemp -d /tmp/everarcade-appliance.XXXXXX)"
trap 'rm -rf "$WORK"' EXIT
bash "$ROOT/scripts/build_runtime_layout.sh" "$WORK/runtime"
mkdir -p "$WORK/runtime/sdk-docs" "$WORK/runtime/local-federation"
cp -R "$ROOT/docs/developer/." "$WORK/runtime/sdk-docs/" || true
cp -R "$ROOT/tools/local-federation/." "$WORK/runtime/local-federation/" || true
mkdir -p "$DIST_DIR"
tar --sort=name --mtime='UTC 2020-01-01' --owner=0 --group=0 --numeric-owner -C "$WORK" -cf - runtime | gzip -n > "$OUT"
echo "vm_runtime_appliance=ok archive=$OUT"
