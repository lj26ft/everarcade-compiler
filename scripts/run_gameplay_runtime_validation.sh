#!/usr/bin/env bash
set -euo pipefail
echo "validation=$0 mode=deterministic replay_continuity=append-only authority_mutation=rejected execution_divergence=rejected"
CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS:-1} cargo test --package execution-core --test gameplay_runtime_tests --offline --locked
