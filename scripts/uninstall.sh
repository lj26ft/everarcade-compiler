#!/usr/bin/env bash
set -euo pipefail

PREFIX="${PREFIX:-/opt/everarcade}"
BIN_DIR="${BIN_DIR:-/usr/local/bin}"

rm -f "$BIN_DIR/everarcade-host"
rm -rf "$PREFIX"

echo "uninstall=ok prefix=$PREFIX"
