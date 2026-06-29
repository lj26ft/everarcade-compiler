#!/usr/bin/env bash
set -euo pipefail

# Regenerate local distribution archives for the Commitment V1 review bundle.
# Run from the repository root:
#   review-bundles/commitment-v1-review/PACKAGE_BUNDLE.sh

BUNDLE_DIR="review-bundles/commitment-v1-review"
TARBALL="review-bundles/commitment-v1-review.tar.gz"
ZIPFILE="review-bundles/commitment-v1-review.zip"

if [[ ! -d "$BUNDLE_DIR" ]]; then
  echo "error: $BUNDLE_DIR not found; run this script from the repository root" >&2
  exit 1
fi

rm -f "$TARBALL" "$ZIPFILE"
(
  cd review-bundles
  tar --exclude='__pycache__' -czf commitment-v1-review.tar.gz commitment-v1-review
  zip -qr commitment-v1-review.zip commitment-v1-review -x '*/__pycache__/*'
)

printf 'wrote %s\n' "$TARBALL"
printf 'wrote %s\n' "$ZIPFILE"
