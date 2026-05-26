#!/usr/bin/env bash
set -euo pipefail
cargo test --package execution-core --test projection_runtime_integration_tests test_projection_cryptographic_hash_stability "$@"
echo "projection_cryptographic=pass"
