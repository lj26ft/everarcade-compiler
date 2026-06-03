#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIST_DIR="$ROOT_DIR/dist"
ARTIFACT="$DIST_DIR/vendor.tar.gz"
CHECKSUM="$DIST_DIR/vendor.tar.gz.sha256"
VENDOR_DIR="$ROOT_DIR/vendor"

if [[ ! -f "$ARTIFACT" ]]; then
  echo "Missing artifact: dist/vendor.tar.gz" >&2
  echo "Obtain it from a local build, a release attachment, or a copied dist/ directory before restoring vendor/." >&2
  exit 1
fi

if [[ ! -f "$CHECKSUM" ]]; then
  echo "Missing checksum: dist/vendor.tar.gz.sha256" >&2
  echo "Cannot restore vendor/ without checksum verification." >&2
  exit 1
fi

(
  cd "$DIST_DIR"
  sha256sum -c vendor.tar.gz.sha256
)

rm -rf "$VENDOR_DIR"
tar -xzf "$ARTIFACT" -C "$ROOT_DIR"

if [[ ! -d "$VENDOR_DIR" ]]; then
  echo "Artifact extracted successfully but vendor/ was not created." >&2
  exit 1
fi

echo "vendor/ restored from dist/vendor.tar.gz"
echo "Artifact source may be a local build, a release attachment, or a copied dist/ directory."
