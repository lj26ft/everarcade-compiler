#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
CARGO_BUILD_JOBS=1 cargo test --package studio-gui --offline --locked test_interactive_editing_equivalence
CARGO_BUILD_JOBS=1 cargo test --package studio-gui --offline --locked test_entity_dragdrop_equivalence
CARGO_BUILD_JOBS=1 cargo test --package studio-gui --offline --locked test_undo_redo_equivalence
CARGO_BUILD_JOBS=1 cargo test --package studio-gui --offline --locked test_replay_safe_editor_behavior
