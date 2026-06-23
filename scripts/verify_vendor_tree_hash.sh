#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=lib/common.sh
source "$ROOT/scripts/lib/common.sh"
cd "$ROOT"

if [[ ! -f vendor.sha256 ]]; then
  echo "vendor tree hash: FAIL - missing vendor.sha256" >&2
  exit 1
fi

if [[ ! -d vendor ]]; then
  echo "vendor tree hash: FAIL - vendor/ not restored; run bash scripts/ensure_vendor_offline.sh" >&2
  exit 1
fi

expected="$(tr -d '\r\n' < vendor.sha256)"
if [[ ${#expected} -ne 64 ]]; then
  echo "vendor tree hash: FAIL - vendor.sha256 must be a 64-char sha256 hex digest" >&2
  exit 1
fi

actual="$(vendor_tree_sha256 "$ROOT")"

if [[ "$expected" != "$actual" ]]; then
  echo "vendor tree hash: FAIL - tree hash mismatch" >&2
  echo "expected: $expected" >&2
  echo "actual:   $actual" >&2
  echo "Regenerate with: bash scripts/vendor_deps.sh (maintainers, network required)" >&2
  exit 1
fi

printf 'vendor tree hash: PASS (%s)\n' "$actual"