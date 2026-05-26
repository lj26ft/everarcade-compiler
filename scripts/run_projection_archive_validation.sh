#!/usr/bin/env bash
set -euo pipefail
cargo test --manifest-path execution-core/Cargo.toml --offline --locked --test projection_artifact_tests test_projection_archive_restoration test_projection_manifest_integrity
printf 'projection_archive_validation=pass\n'
