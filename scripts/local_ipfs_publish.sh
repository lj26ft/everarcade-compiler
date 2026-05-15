#!/usr/bin/env bash
set -euo pipefail

if [[ "${ENABLE_IPFS_LIVE:-0}" != "1" ]]; then
  echo "ipfs-live disabled; set ENABLE_IPFS_LIVE=1 to execute"
  exit 0
fi

cargo test -p everarcade-host --features ipfs-live --test ipfs_publication_tests
