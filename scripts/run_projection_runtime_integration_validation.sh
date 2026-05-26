#!/usr/bin/env bash
set -euo pipefail
cargo test --package execution-core --test projection_runtime_integration_tests "$@"
echo "projection_runtime_integration=pass"
