#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
CARGO_BUILD_JOBS=1 cargo test --package studio-gui --offline --locked test_terrain_sculpting_equivalence
CARGO_BUILD_JOBS=1 cargo test --package studio-gui --offline --locked test_region_painting_equivalence
