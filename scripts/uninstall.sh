#!/usr/bin/env bash
set -euo pipefail
PREFIX_DEFAULT="/opt/everarcade"
BIN_DIR_DEFAULT="/usr/local/bin"
PREFIX="${PREFIX:-$PREFIX_DEFAULT}"
BIN_DIR="${BIN_DIR:-$BIN_DIR_DEFAULT}"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --prefix) PREFIX="$2"; shift 2 ;;
    --bin-dir) BIN_DIR="$2"; shift 2 ;;
    *) echo "unknown arg: $1"; exit 1 ;;
  esac
done

rm -f "$BIN_DIR/everarcade-host"
rm -rf "$PREFIX"
echo "uninstall=ok prefix=$PREFIX bin_dir=$BIN_DIR"
