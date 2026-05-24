#!/usr/bin/env bash
set -euo pipefail
mkdir -p world/reports
cargo test --package execution-core test_checkpoint_restoration_equivalence
cargo test --package execution-core test_economy_ledger_restoration
echo "restoration:ok" > world/reports/world-restoration.txt
