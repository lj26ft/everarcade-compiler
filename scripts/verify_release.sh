#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
sha256sum -c release/SHA256SUMS
bash scripts/verify_vendor_integrity.sh
test -s release/RUNTIME_ROOT && test -s release/VALIDATION_ROOT
