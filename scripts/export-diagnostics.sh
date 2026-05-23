#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT="${1:-$ROOT/logs/diagnostics-$(date -u +%Y%m%dT%H%M%SZ).tar.gz}"
tar -czf "$OUT" -C "$ROOT" logs runtime/manifests runtime/replay
echo "$OUT"
