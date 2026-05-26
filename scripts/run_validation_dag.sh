#!/usr/bin/env bash
set -euo pipefail
cargo test --package execution-core --test workspace_scalability_tests "$@"
