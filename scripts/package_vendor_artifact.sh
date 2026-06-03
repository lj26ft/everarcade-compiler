#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VENDOR_DIR="$ROOT_DIR/vendor"
DIST_DIR="$ROOT_DIR/dist"
ARTIFACT="$DIST_DIR/vendor.tar.gz"
CHECKSUM="$DIST_DIR/vendor.tar.gz.sha256"

if [[ ! -d "$VENDOR_DIR" ]]; then
  echo "vendor/ is missing; cannot package artifact." >&2
  echo "This script packages an existing vendor/ directory and does not run cargo vendor." >&2
  exit 1
fi

mkdir -p "$DIST_DIR"

tar -czf "$ARTIFACT" -C "$ROOT_DIR" vendor
(
  cd "$DIST_DIR"
  sha256sum vendor.tar.gz > vendor.tar.gz.sha256
)

hash_value="$(awk '{print $1}' "$CHECKSUM")"
echo "vendor artifact: $ARTIFACT"
echo "sha256: $hash_value"
