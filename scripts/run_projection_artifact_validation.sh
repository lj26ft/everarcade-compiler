#!/usr/bin/env bash
set -euo pipefail
cargo test --manifest-path execution-core/Cargo.toml --offline --locked --test projection_artifact_tests
printf 'projection_artifact_validation=pass\n'
