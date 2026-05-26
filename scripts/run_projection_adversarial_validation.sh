#!/usr/bin/env bash
set -euo pipefail
cargo test --package execution-core --test projection_runtime_integration_tests test_projection_corruption_detection "$@"
cargo test --package execution-core --test projection_runtime_integration_tests test_projection_transport_replay_injection_rejection "$@"
echo "projection_adversarial=pass"
