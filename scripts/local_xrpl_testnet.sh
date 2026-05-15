#!/usr/bin/env bash
set -euo pipefail

if [[ "${ENABLE_XRPL_LIVE:-0}" != "1" ]]; then
  echo "xrpl-live disabled; set ENABLE_XRPL_LIVE=1 to execute"
  exit 0
fi

cargo test -p everarcade-host --features xrpl-live --test xrpl_testnet_tests
