#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TMP="$(mktemp -d)"
tar -xzf "$ROOT/dist/everarcade-runtime-linux-x86_64.tar.gz" -C "$TMP"
cd "$TMP/everarcade-runtime"
./scripts/bootstrap.sh
./scripts/validate.sh
./scripts/start.sh --foreground >/tmp/everarcade-runtime-start.log 2>&1 &
PID=$!
sleep 2
kill "$PID" || true
./scripts/shutdown.sh || true
