#!/usr/bin/env bash
set -euo pipefail

PREFIX_DEFAULT="/opt/everarcade"
BIN_DIR_DEFAULT="/usr/local/bin"
PREFIX="${PREFIX:-$PREFIX_DEFAULT}"
BIN_DIR="${BIN_DIR:-$BIN_DIR_DEFAULT}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PACKAGE_ROOT="$SCRIPT_DIR"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --prefix) PREFIX="$2"; shift 2 ;;
    --bin-dir) BIN_DIR="$2"; shift 2 ;;
    --package-root) PACKAGE_ROOT="$2"; shift 2 ;;
    /*|./*|../*)
      PACKAGE_ROOT="$1"; shift ;;
    *) echo "unknown arg: $1"; exit 1 ;;
  esac
done

install -d "$PREFIX" "$PREFIX/bin" "$PREFIX/deploy" "$BIN_DIR"
install -m 0755 "$PACKAGE_ROOT/bin/everarcade-host" "$PREFIX/bin/everarcade-host"
cp -R "$PACKAGE_ROOT/deploy/." "$PREFIX/deploy/"
if [[ -d "$PACKAGE_ROOT/meta" ]]; then
  install -d "$PREFIX/meta"
  cp -R "$PACKAGE_ROOT/meta/." "$PREFIX/meta/"
fi
ln -sfn "$PREFIX/bin/everarcade-host" "$BIN_DIR/everarcade-host"
echo "install=ok prefix=$PREFIX bin_link=$BIN_DIR/everarcade-host"
