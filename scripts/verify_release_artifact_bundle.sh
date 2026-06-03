#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIST_DIR="$ROOT_DIR/dist"
BUNDLE_NAME="everarcade-runtime-v0.1.0-bundle"
BUNDLE_TARBALL="$DIST_DIR/$BUNDLE_NAME.tar.gz"
BUNDLE_CHECKSUM="$DIST_DIR/$BUNDLE_NAME.sha256"
VERIFY_ROOT="/tmp/$BUNDLE_NAME-verify"
EXTRACTED_DIR="$VERIFY_ROOT/$BUNDLE_NAME"

fail() {
  printf 'Release Artifact Bundle: FAIL\n' >&2
  printf '%s\n' "$1" >&2
  exit 1
}

require_file() {
  local path="$1"
  local label="$2"

  if [[ ! -f "$path" ]]; then
    fail "Missing required $label: ${path#$ROOT_DIR/}"
  fi
}

require_file "$BUNDLE_TARBALL" "bundle artifact"
require_file "$BUNDLE_CHECKSUM" "bundle checksum"

if ! (cd "$DIST_DIR" && sha256sum -c "$BUNDLE_NAME.sha256"); then
  fail "Bundle checksum verification failed."
fi

rm -rf "$VERIFY_ROOT"
mkdir -p "$VERIFY_ROOT"
tar -xzf "$BUNDLE_TARBALL" -C "$VERIFY_ROOT"

[[ -d "$EXTRACTED_DIR" ]] || fail "Extracted bundle directory is missing: $EXTRACTED_DIR"
[[ -f "$EXTRACTED_DIR/vendor.tar.gz" ]] || fail "Bundle is missing vendor.tar.gz."
[[ -f "$EXTRACTED_DIR/vendor.tar.gz.sha256" ]] || fail "Bundle is missing vendor.tar.gz.sha256."
[[ -f "$EXTRACTED_DIR/MANIFEST.txt" ]] || fail "Bundle is missing MANIFEST.txt."

if ! (cd "$EXTRACTED_DIR" && sha256sum -c vendor.tar.gz.sha256); then
  fail "Bundled vendor checksum verification failed."
fi

printf 'Release Artifact Bundle: PASS\n'
printf 'Verified bundle: %s\n' "$BUNDLE_TARBALL"
printf 'Verified extraction: %s\n' "$EXTRACTED_DIR"
