#!/usr/bin/env bash
set -euo pipefail
mkdir -p world/reports
cargo test --package execution-core test_scheduler_determinism
cargo test --package execution-core test_world_federation_recovery
echo "scheduler:ok" > world/reports/scheduler-equivalence.txt
