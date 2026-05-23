#!/usr/bin/env bash
set -euo pipefail
cargo test -p execution-core wasm_dag_tests::dag_reordered_input_same_output -- --exact --nocapture
echo "wasm dag equivalence verified"
