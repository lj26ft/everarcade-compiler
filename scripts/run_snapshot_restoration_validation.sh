#!/usr/bin/env bash
set -euo pipefail
cargo test --package execution-core --test epoch_materialization_tests "$@"
