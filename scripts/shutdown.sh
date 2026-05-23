#!/usr/bin/env bash
set -euo pipefail

if pgrep -f everarcade-host >/dev/null 2>&1; then
  pkill -f everarcade-host
fi

echo "[runtime][$(date -u +%Y-%m-%dT%H:%M:%SZ)][shutdown][deterministic=true]"
