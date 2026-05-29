#!/usr/bin/env bash
set -euo pipefail
echo "creator_validation=run_simulation_debugger_validation.sh mode=offline locked=true frozen=true deterministic_logs=true replay_continuity=preserved authority_mutation=rejected"
CARGO_BUILD_JOBS=1 cargo test --package tools --test creator_toolchain_tests test_simulation_debugger_equivalence --offline --locked
