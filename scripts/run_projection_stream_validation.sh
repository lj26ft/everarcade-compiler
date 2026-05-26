#!/usr/bin/env bash
set -euo pipefail
cargo test --package execution-core --test projection_stream_tests --offline --locked
