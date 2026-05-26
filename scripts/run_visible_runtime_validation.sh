#!/usr/bin/env bash
set -euo pipefail

echo "[visible-runtime] boot runtime"
cargo run --package runtime-client --offline --locked >/tmp/visible-runtime.log

echo "[visible-runtime] run renderer tests"
cargo test --package execution-core --test terminal_renderer_tests --offline --locked

echo "[visible-runtime] verify projection stream"
cargo test --package execution-core --test projection_stream_tests --offline --locked

echo "[visible-runtime] verify persistent runtime"
cargo test --package execution-core --test persistent_world_runtime_tests --offline --locked

echo "[visible-runtime] completed"
