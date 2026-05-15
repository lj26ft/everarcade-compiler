#!/usr/bin/env bash
set -euo pipefail

YES=false
if [[ "${1:-}" == "--yes" ]]; then
  YES=true
fi

echo 'This will remove generated artifacts:'
echo '  - target'
echo '  - .everarcade*'
echo '  - tmp'

if [[ "$YES" != true ]]; then
  read -r -p 'Proceed? [y/N] ' reply
  if [[ ! "$reply" =~ ^[Yy]$ ]]; then
    echo 'Aborted.'
    exit 1
  fi
fi

rm -rf ./target
find . -maxdepth 1 -type d -name '.everarcade*' -exec rm -rf {} +
rm -rf ./tmp

echo 'Cleanup complete.'
