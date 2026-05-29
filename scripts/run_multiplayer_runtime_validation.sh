#!/usr/bin/env bash
set -euo pipefail
cargo test --package execution-core --test world_runtime_tests --offline --locked test_multiplayer_session_equivalence
cargo test --package execution-core --test world_runtime_tests --offline --locked test_session_continuity
cargo test --package execution-core --test world_runtime_tests --offline --locked test_replay_safe_multiplayer
