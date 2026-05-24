#!/usr/bin/env bash
set -euo pipefail
args=("$@")
cargo test --package execution-core --test wasm_runtime_tests "${args[@]}"
