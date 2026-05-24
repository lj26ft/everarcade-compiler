#!/usr/bin/env bash
set -euo pipefail
bash scripts/vendor_deps.sh
cargo test --package execution-core --test runtime_operations_tests "$@" test_release_pipeline_equivalence
