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

if command -v sha256sum >/dev/null 2>&1; then
  (
    cd "$DIST_DIR"
    sha256sum -c vendor.tar.gz.sha256
  )
elif command -v shasum >/dev/null 2>&1; then
  expected="$(awk '{print $1}' "$CHECKSUM")"
  actual="$(shasum -a 256 "$ARTIFACT" | awk '{print $1}')"
  [[ "$expected" == "$actual" ]] || {
    echo "Checksum mismatch for dist/vendor.tar.gz" >&2
    exit 1
  }
else
  echo "Need sha256sum or shasum to verify dist/vendor.tar.gz" >&2
  exit 1
fi

rm -rf "$VENDOR_DIR"
tar -xzf "$ARTIFACT" -C "$ROOT_DIR"

if [[ ! -d "$VENDOR_DIR" ]]; then
  echo "Artifact extracted successfully but vendor/ was not created." >&2
  exit 1
fi

echo "vendor/ restored from dist/vendor.tar.gz"
echo "Artifact source may be a local build, a release attachment, or a copied dist/ directory."
