#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
CARGO_BUILD_JOBS=1 cargo test --package studio-gui --offline --locked test_viewport_selection_equivalence
CARGO_BUILD_JOBS=1 cargo test --package studio-gui --offline --locked test_entity_placement_equivalence
CARGO_BUILD_JOBS=1 cargo test --package studio-gui --offline --locked test_gizmo_equivalence
CARGO_BUILD_JOBS=1 cargo test --package studio-gui --offline --locked test_terrain_authoring_equivalence
CARGO_BUILD_JOBS=1 cargo test --package studio-gui --offline --locked test_undo_redo_equivalence
