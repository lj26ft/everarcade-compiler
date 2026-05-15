#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DIST_DIR="${DIST_DIR:-$ROOT/dist}"
WORK="$(mktemp -d /tmp/everarcade-install-smoke.XXXXXX)"
trap 'rm -rf "$WORK"' EXIT
PREFIX="$WORK/prefix"
BIN_DIR="$WORK/bin"
STATE="$WORK/state"
PKG_OUT="$WORK/pkg.bin"

"$ROOT/scripts/release_package.sh" "$DIST_DIR"
PKG_FILE="$(ls -1t "$DIST_DIR"/everarcade-host-operator-v*.tar.gz | head -n1)"
tar -xzf "$PKG_FILE" -C "$WORK"
PKG_DIR="$(find "$WORK" -mindepth 1 -maxdepth 1 -type d | head -n1)"

"$PKG_DIR/install.sh" --prefix "$PREFIX" --bin-dir "$BIN_DIR"
PATH="$BIN_DIR:$PATH"
which everarcade-host
everarcade-host init --state "$STATE"
everarcade-host generate-fixture --output "$PKG_OUT"
everarcade-host run --package "$PKG_OUT" --state "$STATE"
everarcade-host verify --state "$STATE"
everarcade-host status --state "$STATE"
"$PKG_DIR/uninstall.sh" --prefix "$PREFIX" --bin-dir "$BIN_DIR"
[[ ! -e "$BIN_DIR/everarcade-host" ]]
echo "install_smoke=ok package=$PKG_FILE"
