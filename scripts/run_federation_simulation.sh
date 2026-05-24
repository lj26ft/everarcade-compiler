#!/usr/bin/env bash
set -euo pipefail
mkdir -p federation/reports
cargo test --test federation_simulation_tests test_multi_node_replay_equivalence -- --nocapture | tee federation/reports/federation-simulation.txt
