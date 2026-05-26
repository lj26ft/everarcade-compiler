#!/usr/bin/env bash
set -euo pipefail
cargo test --package execution-core --test historical_replay_dataplane_tests "$@"
