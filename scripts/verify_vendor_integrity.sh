#!/usr/bin/env bash
set -euo pipefail
if [ ! -d vendor ]; then
  echo "vendor directory missing" >&2
  exit 1
fi
LC_ALL=C find vendor -type f -print0 | LC_ALL=C sort -z | xargs -0 sha256sum | LC_ALL=C sort > release/VENDOR_SHA256SUMS
sha256sum release/VENDOR_SHA256SUMS | awk '{print $1}' > release/VENDOR_HASH
echo "vendor integrity materialized: $(cat release/VENDOR_HASH)"
