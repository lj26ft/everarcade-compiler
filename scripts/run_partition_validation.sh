#!/usr/bin/env bash
set -euo pipefail
echo "[everarcade] partition validation: deterministic replay continuity check"
CARGO_BUILD_JOBS=1 cargo test --package execution-core --test simulation_runtime_tests --offline --locked
