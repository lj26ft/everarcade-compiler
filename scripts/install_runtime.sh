#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
TARBALL="${1:-dist/everarcade-v0.1.0-linux-amd64.tar.gz}"
sha256sum -c "${TARBALL}.sha256"
tar -xzf "$TARBALL" -C dist
bash scripts/verify_vendor_integrity.sh
