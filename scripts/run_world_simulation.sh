#!/usr/bin/env bash
set -euo pipefail
mkdir -p world/reports
cargo test --package execution-core test_world_tick_equivalence -- --nocapture
cat > world/reports/world-simulation.txt <<REPORT
simulation_scale: 1h,1d,1w,multi-era
status: deterministic
REPORT
