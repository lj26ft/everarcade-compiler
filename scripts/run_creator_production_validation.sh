#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
CARGO_BUILD_JOBS=1 cargo test --package studio-gui --offline --locked test_creator_production_workflow
CARGO_BUILD_JOBS=1 cargo test --package studio-gui --offline --locked test_asset_preview_equivalence
CARGO_BUILD_JOBS=1 cargo test --package tools --test creator_toolchain_tests --offline --locked test_creator_workflow_equivalence
