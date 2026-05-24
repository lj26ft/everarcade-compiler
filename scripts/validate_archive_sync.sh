#!/usr/bin/env bash
set -euo pipefail
mkdir -p federation/reports
cargo test --test federation_simulation_tests test_archive_sync_equivalence -- --nocapture | tee federation/reports/archive-sync.txt
