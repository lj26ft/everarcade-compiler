#!/usr/bin/env bash
set -euo pipefail
cargo test --package execution-core --test replay_transport_runtime_tests "$@"
echo "validation=ok script=$(basename "$0")"
