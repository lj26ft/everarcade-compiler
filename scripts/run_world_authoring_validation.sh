#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
CARGO_BUILD_JOBS=1 cargo test --package studio-gui --offline --locked test_world_authoring_equivalence
CARGO_BUILD_JOBS=1 cargo test --package studio-gui --offline --locked test_gizmo_equivalence
CARGO_BUILD_JOBS=1 cargo test --package studio-gui --offline --locked test_scene_graph_equivalence
CARGO_BUILD_JOBS=1 cargo test --package studio-gui --offline --locked test_live_edit_equivalence
CARGO_BUILD_JOBS=1 cargo test --package tools --test creator_toolchain_tests --offline --locked test_world_authoring_equivalence
