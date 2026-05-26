#!/usr/bin/env bash
set -euo pipefail
cargo test --manifest-path execution-core/Cargo.toml --offline --locked --test projection_artifact_tests test_projection_replay_equivalence test_projection_replay_determinism
printf 'projection_replay_validation=pass\n'
