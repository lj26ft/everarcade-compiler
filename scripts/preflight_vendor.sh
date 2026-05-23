#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

if [[ ! -d "$ROOT_DIR/vendor" ]]; then
  echo "Missing vendor/. Run: bash scripts/vendor_deps.sh" >&2
  exit 1
fi
