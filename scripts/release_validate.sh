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
run "$ROOT/scripts/release_package.sh" "$DIST_DIR"

if [[ -z "$PKG_FILE" ]]; then
  PKG_FILE="$(ls -1t "$DIST_DIR"/everarcade-host-operator-*.tar.gz | head -n1)"
fi

echo "release_archive=$PKG_FILE"

TMP_FIXTURE="$(mktemp)"
trap 'rm -f "$TMP_FIXTURE"; cleanup' EXIT
run cargo run -p everarcade-host -- generate-fixture --output "$TMP_FIXTURE"
run cargo run -p everarcade-host -- deploy-proof --package "$TMP_FIXTURE" --state "$STATE_DIR" --profile live --node evernode-operator-1

run test -f "$PKG_FILE"
run test -f "${PKG_FILE%.tar.gz}.sha256"

echo "release_validation=ok"
