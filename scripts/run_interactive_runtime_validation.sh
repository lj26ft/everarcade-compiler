#!/usr/bin/env bash
set -euo pipefail
cargo test --package execution-core --test interactive_runtime_tests --offline --locked
