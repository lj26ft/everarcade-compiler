#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
export EVERARCADE_PROOF_REPORT_DIR="${EVERARCADE_PROOF_REPORT_DIR:-$ROOT/reports}"
export EVERARCADE_PROOF_ARTIFACT_DIR="${EVERARCADE_PROOF_ARTIFACT_DIR:-$ROOT/artifacts}"
mkdir -p "$EVERARCADE_PROOF_REPORT_DIR" "$EVERARCADE_PROOF_ARTIFACT_DIR"
node "$ROOT/scripts/runtime-proof.mjs"
