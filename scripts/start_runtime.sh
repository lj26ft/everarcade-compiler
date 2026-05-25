#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
mkdir -p runtime/state runtime/logs
echo "starting deterministic runtime with runtime/config/evernode-runtime.toml"
