#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=lib/common.sh
source "$ROOT/scripts/lib/common.sh"

cd "$ROOT"

verify_vendor_manifest() {
  [[ -f "$ROOT/vendor.sha256" && -f "$ROOT/vendor-manifest.json" ]] || return 0
  local expected actual
  expected="$(tr -d '\n' < "$ROOT/vendor.sha256")"
  actual="$(find "$ROOT/vendor" -type f -print0 | sort -z | xargs -0 sha256sum | sha256sum | awk '{print $1}')"
  [[ "$expected" == "$actual" ]]
}

if vendor_offline_ok "$ROOT" && offline_cargo_check_workspace "$ROOT" /tmp/everarcade-vendor-offline-check.log; then
  if verify_vendor_manifest; then
    printf 'vendor offline: PASS (existing tree)\n'
    exit 0
  fi
  printf 'vendor offline: tree hash mismatch; restoring from dist/vendor.tar.gz\n' >&2
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

if ! vendor_offline_ok "$ROOT"; then
  printf 'vendor offline: FAIL - restore completed but offline metadata check failed\n' >&2
  print_vendor_fix_hint
  exit 1
fi

if ! offline_cargo_check_workspace "$ROOT" /tmp/everarcade-vendor-offline-check.log; then
  printf 'vendor offline: FAIL - workspace cargo check --offline failed after restore\n' >&2
  tail -20 /tmp/everarcade-vendor-offline-check.log >&2 || true
  print_vendor_fix_hint
  exit 1
fi

printf 'vendor offline: PASS (restored)\n'
exit 0