#!/usr/bin/env bash
set -euo pipefail
echo "validation=run_civilization_validation mode=deterministic replay_continuity=append-only authority_mutation=rejected divergence=rejected"
CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS:-1} cargo test --package execution-core --test world_runtime_tests --offline --locked
