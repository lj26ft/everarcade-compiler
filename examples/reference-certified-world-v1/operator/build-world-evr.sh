#!/usr/bin/env bash
set -euo pipefail
ROOT="${1:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
OUT="${2:-$ROOT/world.evr}"
(
  cd "$ROOT"
  find README.md assets certification continuity genesis manifest operator proofs registry rustrigs world-contract \
    -type f -print | LC_ALL=C sort | tar --mtime='1970-01-01 UTC' --owner=0 --group=0 --numeric-owner -czf "$OUT" -T -
)
printf 'built %s\n' "$OUT"
