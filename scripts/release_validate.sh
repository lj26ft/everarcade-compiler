#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DIST_DIR="${DIST_DIR:-$ROOT/dist}"
STATE_DIR="${STATE_DIR:-$(mktemp -d)}"
PKG_FILE="${PKG_FILE:-}"

cleanup() {
  rm -rf "$STATE_DIR"
}
trap cleanup EXIT

run() {
  echo "+ $*"
  "$@"
}

run cargo build --release -p everarcade-host
run "$ROOT/scripts/release_smoke.sh"

if [[ -z "$PKG_FILE" ]]; then
  PKG_FILE="$(ls -1t "$DIST_DIR"/everarcade-host-operator-v*.tar.gz | head -n1)"
fi

echo "release_archive=$PKG_FILE"

echo "release_validation=ok"
