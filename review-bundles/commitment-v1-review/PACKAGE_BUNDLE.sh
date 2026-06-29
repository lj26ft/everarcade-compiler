#!/usr/bin/env bash
set -euo pipefail

# Regenerate local distribution archives for the Commitment V1 review bundle.
# Supported invocations:
#   review-bundles/commitment-v1-review/PACKAGE_BUNDLE.sh
#   (cd review-bundles/commitment-v1-review && ./PACKAGE_BUNDLE.sh)

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUNDLE_NAME="commitment-v1-review"

if [[ "$(basename "$SCRIPT_DIR")" == "$BUNDLE_NAME" ]]; then
  BUNDLE_DIR="$SCRIPT_DIR"
  REVIEW_BUNDLES_DIR="$(dirname "$SCRIPT_DIR")"
else
  BUNDLE_DIR="review-bundles/$BUNDLE_NAME"
  REVIEW_BUNDLES_DIR="review-bundles"
fi

if [[ ! -d "$BUNDLE_DIR" ]]; then
  echo "error: $BUNDLE_DIR not found" >&2
  exit 1
fi

TARBALL="$REVIEW_BUNDLES_DIR/$BUNDLE_NAME.tar.gz"
ZIPFILE="$REVIEW_BUNDLES_DIR/$BUNDLE_NAME.zip"

rm -f "$TARBALL" "$ZIPFILE"
(
  cd "$REVIEW_BUNDLES_DIR"
  tar --exclude='__pycache__' -czf "$BUNDLE_NAME.tar.gz" "$BUNDLE_NAME"
  zip -qr "$BUNDLE_NAME.zip" "$BUNDLE_NAME" -x '*/__pycache__/*'
)

printf 'wrote %s\n' "$TARBALL"
printf 'wrote %s\n' "$ZIPFILE"
