#!/usr/bin/env bash
set -euo pipefail
cargo test --package execution-core --test world_epoch_tests "$@"
