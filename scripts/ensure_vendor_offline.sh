#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=lib/common.sh
source "$ROOT/scripts/lib/common.sh"

cd "$ROOT"

if vendor_offline_ok "$ROOT"; then
  printf 'vendor offline: PASS (existing tree)\n'
  exit 0
fi

ARTIFACT="$ROOT/dist/vendor.tar.gz"
CHECKSUM="$ROOT/dist/vendor.tar.gz.sha256"

if [[ ! -f "$ARTIFACT" || ! -f "$CHECKSUM" ]]; then
  printf 'vendor offline: FAIL - missing dist/vendor.tar.gz or checksum\n' >&2
  printf 'Obtain a complete checkout or run bash scripts/vendor_deps.sh as a maintainer.\n' >&2
  exit 1
fi

printf 'vendor offline: restoring from dist/vendor.tar.gz\n' >&2
bash "$ROOT/scripts/restore_vendor_artifact.sh"

if vendor_offline_ok "$ROOT"; then
  printf 'vendor offline: PASS (restored)\n'
  exit 0
fi

printf 'vendor offline: FAIL - restore completed but offline metadata check failed\n' >&2
exit 1