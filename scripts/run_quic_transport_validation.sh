#!/usr/bin/env bash
set -euo pipefail
echo "quic transport validation: deterministic replay continuity check"
CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS:-1} cargo test --package execution-core --test internet_runtime_fabric_tests "$@"
echo "quic transport validation: ok replay_only=true non_authoritative=true"
