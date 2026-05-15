#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

VENDOR_DIR="${VENDOR_DIR:-$ROOT/vendor}"
VENDOR_ARCHIVE="${VENDOR_ARCHIVE:-}"

mkdir -p .cargo
BACKUP=""
if [[ -f .cargo/config.toml ]]; then
  BACKUP=".cargo/config.toml.bak"
  cp .cargo/config.toml "$BACKUP"
fi

cleanup() {
  if [[ -n "$BACKUP" && -f "$BACKUP" ]]; then
    mv "$BACKUP" .cargo/config.toml
  fi
}
trap cleanup EXIT

rm -f .cargo/config.toml
cargo generate-lockfile
cargo vendor --locked "$VENDOR_DIR" > .cargo/config.toml
cat >> .cargo/config.toml <<'CFG'

[net]
offline = true
CFG

rm -f "$BACKUP"
trap - EXIT

if [[ -n "$VENDOR_ARCHIVE" ]]; then
  mkdir -p "$(dirname "$VENDOR_ARCHIVE")"
  tar -C "$(dirname "$VENDOR_DIR")" -czf "$VENDOR_ARCHIVE" "$(basename "$VENDOR_DIR")"
  echo "vendor_archive=$VENDOR_ARCHIVE"
fi

echo "dependency_vendor=ok"
