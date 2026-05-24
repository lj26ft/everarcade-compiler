#!/usr/bin/env bash
set -euo pipefail
mkdir -p world/reports
cargo test --package execution-core test_entity_lifecycle_continuity
cargo test --package execution-core test_inventory_replay_equivalence
echo "continuity:ok" > world/reports/world-continuity.txt
