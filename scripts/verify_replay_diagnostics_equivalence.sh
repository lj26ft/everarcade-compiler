#!/usr/bin/env bash
set -euo pipefail
cargo test -p execution-core --test operator_diagnostics_tests --test sovereign_operations_layer_tests -- --nocapture
