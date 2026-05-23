#!/usr/bin/env bash
set -euo pipefail
pkill -f everarcade-host || true
echo "[runtime][$(date -u +%Y-%m-%dT%H:%M:%SZ)][shutdown][deterministic=true]"
