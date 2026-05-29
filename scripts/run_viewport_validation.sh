#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
CARGO_BUILD_JOBS=1 cargo test --package studio-gui --offline --locked test_viewport_projection_integrity
CARGO_BUILD_JOBS=1 cargo test --package tools --test creator_toolchain_tests test_viewport_projection_integrity --offline --locked
