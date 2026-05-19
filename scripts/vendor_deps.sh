#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

VENDOR_DIR="${VENDOR_DIR:-vendor}"
VENDOR_ARCHIVE="${VENDOR_ARCHIVE:-}"

if [[ "$VENDOR_DIR" = /* ]]; then
  echo "error: VENDOR_DIR must be repo-relative for portable builds" >&2
  exit 1
fi

if [[ "$VENDOR_DIR" != "vendor" ]]; then
  echo "error: VENDOR_DIR must be 'vendor' for checked-in portable config" >&2
  exit 1
fi

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
cargo vendor --locked "$VENDOR_DIR" > /tmp/everarcade-vendor-config.toml

cat > .cargo/config.toml <<'CFG'
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"

[net]
offline = true
CFG

rm -f "$BACKUP"
trap - EXIT

if [[ -n "$VENDOR_ARCHIVE" ]]; then
  mkdir -p "$(dirname "$VENDOR_ARCHIVE")"
  tar -C "$ROOT" -czf "$VENDOR_ARCHIVE" "$VENDOR_DIR"
  echo "vendor_archive=$VENDOR_ARCHIVE"
fi

echo "dependency_vendor=ok"
