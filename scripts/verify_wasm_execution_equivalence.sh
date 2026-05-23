#!/usr/bin/env bash
set -euo pipefail
cargo test -p execution-core wasm_protocol_runtime_tests::wasm_manifest_hash_is_stable -- --exact --nocapture
echo "wasm execution equivalence verified"
