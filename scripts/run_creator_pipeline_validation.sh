#!/usr/bin/env bash
set -euo pipefail

echo "creator_pipeline_validation=running mode=offline locked=true deterministic_logs=true replay_continuity=preserved authority_mutation=rejected"
CARGO_BUILD_JOBS=1 cargo test --package tools --test creator_toolchain_tests test_creator_pipeline_equivalence --offline --locked
CARGO_BUILD_JOBS=1 cargo test --package tools --test creator_toolchain_tests test_template_generation_equivalence --offline --locked
CARGO_BUILD_JOBS=1 cargo test --package tools --test creator_toolchain_tests test_play_mode_equivalence --offline --locked
CARGO_BUILD_JOBS=1 cargo test --package tools --test creator_toolchain_tests test_multiplayer_creation_equivalence --offline --locked
CARGO_BUILD_JOBS=1 cargo test --package tools --test creator_toolchain_tests test_creator_readiness --offline --locked
CARGO_BUILD_JOBS=1 cargo test --package tools --test creator_toolchain_tests test_replay_safe_pipeline --offline --locked
echo "creator_pipeline_validation=ok"
