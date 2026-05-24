#!/usr/bin/env bash
set -euo pipefail
mkdir -p deployment/reports
cargo test --package execution-core --test runtime_operations_tests
