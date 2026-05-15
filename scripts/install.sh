#!/usr/bin/env bash
set -euo pipefail

PREFIX="${PREFIX:-/opt/everarcade}"
BIN_DIR="${BIN_DIR:-/usr/local/bin}"
PKG_DIR="${1:-$(cd "$(dirname "$0")/.." && pwd)}"

install -d "$PREFIX"
install -d "$PREFIX/bin"
install -d "$PREFIX/deploy"

install -m 0755 "$PKG_DIR/bin/everarcade-host" "$PREFIX/bin/everarcade-host"
cp -R "$PKG_DIR/deploy/." "$PREFIX/deploy/"
if [[ -d "$PKG_DIR/meta" ]]; then
  install -d "$PREFIX/meta"
  cp -R "$PKG_DIR/meta/." "$PREFIX/meta/"
fi
ln -sfn "$PREFIX/bin/everarcade-host" "$BIN_DIR/everarcade-host"

echo "install=ok prefix=$PREFIX bin_link=$BIN_DIR/everarcade-host"
