#!/usr/bin/env bash
set -euo pipefail
node validation/hotpocket-live/arena-vanguard-live-cluster.mjs
bash scripts/detect-live-divergence.sh reports/hotpocket-live
