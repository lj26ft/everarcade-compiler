#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
CARGO_BUILD_JOBS=1 cargo test --package studio-gui --offline --locked test_live_simulation_equivalence
CARGO_BUILD_JOBS=1 cargo test --package studio-gui --offline --locked test_replay_visualization_equivalence
CARGO_BUILD_JOBS=1 cargo test --package studio-gui --offline --locked test_replay_safe_creator_workflow
