#!/usr/bin/env bash
set -euo pipefail
cargo test --package execution-core --test world_runtime_tests --offline --locked test_runtime_monitoring_equivalence
cargo test --package execution-core --test world_runtime_tests --offline --locked test_live_world_operations
cargo test --package execution-core --test world_runtime_tests --offline --locked test_authority_mutation_rejection
