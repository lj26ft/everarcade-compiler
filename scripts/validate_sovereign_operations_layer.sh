#!/usr/bin/env bash
set -euo pipefail
cargo test -p execution-core --test appliance_orchestration_tests --test lifecycle_tests --test sharding_tests --test operator_diagnostics_tests --test archive_operations_tests --test recovery_operations_tests --test scheduler_tests --test evernode_integration_tests --test sdk_tests --test sovereign_operations_layer_tests -- --nocapture
