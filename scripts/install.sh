#!/usr/bin/env bash
set -euo pipefail

PREFIX_DEFAULT="/opt/everarcade"
BIN_DIR_DEFAULT="/usr/local/bin"
PREFIX="${PREFIX:-$PREFIX_DEFAULT}"
BIN_DIR="${BIN_DIR:-$BIN_DIR_DEFAULT}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --prefix) PREFIX="$2"; shift 2 ;;
    --bin-dir) BIN_DIR="$2"; shift 2 ;;
    *) echo "unknown arg: $1"; exit 1 ;;
  esac
done

install -d "$PREFIX" "$PREFIX/bin" "$PREFIX/deploy" "$BIN_DIR"
install -m 0755 "$SCRIPT_DIR/bin/everarcade-host" "$PREFIX/bin/everarcade-host"
cp -R "$SCRIPT_DIR/deploy/." "$PREFIX/deploy/"
if [[ -d "$SCRIPT_DIR/meta" ]]; then
  install -d "$PREFIX/meta"
  cp -R "$SCRIPT_DIR/meta/." "$PREFIX/meta/"
fi
ln -sfn "$PREFIX/bin/everarcade-host" "$BIN_DIR/everarcade-host"
echo "install=ok prefix=$PREFIX bin_link=$BIN_DIR/everarcade-host"
