#!/usr/bin/env bash
set -euo pipefail

if git ls-files | grep -E '(^|/)\.everarcade|everarcade-host/tests/fixtures/.*\.bin|(^|/)target/|\.tmp$|\.log$' >/dev/null; then
  echo 'Tracked generated artifacts detected. Remove from Git tracking before merge.'
  git ls-files | grep -E '(^|/)\.everarcade|everarcade-host/tests/fixtures/.*\.bin|(^|/)target/|\.tmp$|\.log$'
  exit 1
fi

echo 'No generated artifacts are tracked.'
