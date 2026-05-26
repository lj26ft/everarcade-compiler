#!/usr/bin/env bash
set -euo pipefail
cargo run -p runtime-client >/tmp/runtime_demo.log
cargo test --package execution-core --test local_runtime_demo_tests --offline --locked
echo "demo runtime validation passed"
