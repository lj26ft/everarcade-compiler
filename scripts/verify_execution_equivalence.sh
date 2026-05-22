#!/usr/bin/env bash
set -euo pipefail

cargo test -p execution-core deterministic_runtime_hardening_tests -- --nocapture
cargo test -p execution-core receipt_tests -- --nocapture
echo "execution equivalence verification passed"
