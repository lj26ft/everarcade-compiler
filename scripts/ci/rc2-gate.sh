#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

EVERARCADE_DETERMINISTIC_ATTEST=1 CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" bash scripts/ci/run-deterministic-world-factory.sh
