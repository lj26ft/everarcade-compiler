#!/usr/bin/env bash
set -euo pipefail
cargo test --package execution-core --test runtime_service_tests "$@" test_runtime_observer_stream_restore
echo "validation=ok script=$(basename "$0")"
