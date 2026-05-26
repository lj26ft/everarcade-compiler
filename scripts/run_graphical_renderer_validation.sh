#!/usr/bin/env bash
set -euo pipefail
cargo run -p renderer-client --offline --locked
cargo test --package execution-core --test graphical_renderer_tests --offline --locked
