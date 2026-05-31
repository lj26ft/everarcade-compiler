#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

test -f xrpl-anchor/anchor_records.json
if grep -q '"xrpl_submission_in_runtime": false' xrpl-anchor/anchor_records.json; then
  :
else
  echo "XRPL runtime submission boundary missing" >&2
  exit 1
fi

CARGO_BUILD_JOBS=1 cargo test -p execution-core --test arena_vanguard_certification_tests --offline --locked

echo "xrpl anchor validation complete"
