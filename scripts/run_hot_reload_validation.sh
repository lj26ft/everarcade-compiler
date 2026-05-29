#!/usr/bin/env bash
set -euo pipefail
echo "creator_validation=run_hot_reload_validation.sh mode=offline locked=true frozen=true deterministic_logs=true replay_continuity=preserved authority_mutation=rejected"
CARGO_BUILD_JOBS=1 cargo test --package tools --test creator_toolchain_tests test_hot_reload_restoration --offline --locked
