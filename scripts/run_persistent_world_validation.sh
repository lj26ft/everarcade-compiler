#!/usr/bin/env bash
set -euo pipefail
cargo test --package execution-core --test persistent_world_runtime_tests --offline --locked
cargo test --package execution-core --test local_runtime_demo_tests --offline --locked
cargo test --package execution-core --test game_runtime_tests --offline --locked
