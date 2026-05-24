#!/usr/bin/env bash
set -euo pipefail
mkdir -p federation/reports
cargo test -p execution-core --test federation_simulation_tests test_multi_node_replay_equivalence -- --nocapture
cargo test -p execution-core --test federation_simulation_tests test_restoration_replay_equivalence -- --nocapture | tee federation/reports/replay-sync.txt
