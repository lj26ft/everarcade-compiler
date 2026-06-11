#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
node runtime/xrpl-anchor-proof/validation/xrpl-anchor-proof.js validate >/tmp/everarcade-xrpl-anchor-proof.log 2>&1
cat reports/anchor_publication_certification_report.txt
if ! grep -q 'XRPL / Xahau Anchor Publication Proof v0.1: PASS' reports/anchor_publication_certification_report.txt; then
  echo "XRPL/Xahau anchor proof log: /tmp/everarcade-xrpl-anchor-proof.log" >&2
  exit 1
fi
