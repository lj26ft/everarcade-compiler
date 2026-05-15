#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DIST_DIR="${DIST_DIR:-$ROOT/dist}"
STATE_DIR="${STATE_DIR:-$(mktemp -d /tmp/everarcade-release-smoke.XXXXXX)}"
PREFIX="${PREFIX:-$STATE_DIR/install-root}"
BIN_DIR="${BIN_DIR:-$STATE_DIR/bin}"
PKG_FILE="${PKG_FILE:-}"

cleanup() {
  rm -rf "$STATE_DIR"
}
trap cleanup EXIT

run() {
  echo "+ $*"
  "$@"
}

run "$ROOT/scripts/release_package.sh" "$DIST_DIR"

if [[ -z "$PKG_FILE" ]]; then
  PKG_FILE="$(ls -1t "$DIST_DIR"/everarcade-host-operator-v*.tar.gz | head -n1)"
fi
SHA_FILE="${PKG_FILE%.tar.gz}.sha256"

WORK_DIR="$(mktemp -d /tmp/everarcade-release-pkg.XXXXXX)"
trap 'rm -rf "$WORK_DIR"; cleanup' EXIT
run tar -xzf "$PKG_FILE" -C "$WORK_DIR"
PKG_EXTRACTED_DIR="$(find "$WORK_DIR" -mindepth 1 -maxdepth 1 -type d | head -n1)"

run test -x "$PKG_EXTRACTED_DIR/bin/everarcade-host"
run bash -c "cd "$(dirname "$SHA_FILE")" && sha256sum -c "$(basename "$SHA_FILE")""

mkdir -p "$BIN_DIR"
run env PREFIX="$PREFIX" BIN_DIR="$BIN_DIR" "$PKG_EXTRACTED_DIR/install.sh" "$PKG_EXTRACTED_DIR"

TMP_FIXTURE="$(mktemp)"
trap 'rm -f "$TMP_FIXTURE"; rm -rf "$WORK_DIR"; cleanup' EXIT
run "$BIN_DIR/everarcade-host" generate-fixture --output "$TMP_FIXTURE"
run "$BIN_DIR/everarcade-host" init --state "$STATE_DIR/runtime"
run "$BIN_DIR/everarcade-host" deploy-proof --package "$TMP_FIXTURE" --state "$STATE_DIR/runtime" --profile live --node evernode-operator-1
run "$BIN_DIR/everarcade-host" run --package "$TMP_FIXTURE" --state "$STATE_DIR/runtime"
run "$BIN_DIR/everarcade-host" verify --state "$STATE_DIR/runtime"

run env PREFIX="$PREFIX" BIN_DIR="$BIN_DIR" "$PKG_EXTRACTED_DIR/uninstall.sh"

echo "release_smoke=ok package=$PKG_FILE"
