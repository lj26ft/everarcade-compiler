#!/usr/bin/env bash
set -euo pipefail
cargo test --package execution-core --test world_runtime_tests --offline --locked test_deployment_automation_equivalence
cargo test --package execution-core --test world_runtime_tests --offline --locked test_world_upgrade_equivalence
cargo test --package execution-core --test world_runtime_tests --offline --locked test_marketplace_runtime_integration
