#!/usr/bin/env bash
set -euo pipefail
mkdir -p federation/reports
cargo test -p execution-core --test federation_simulation_tests test_federation_recovery_equivalence -- --nocapture
cargo test -p execution-core --test federation_simulation_tests test_partition_recovery_equivalence -- --nocapture | tee federation/reports/federation-recovery.txt
