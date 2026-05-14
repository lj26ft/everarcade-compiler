#!/usr/bin/env bash
set -euo pipefail
cargo test -p everarcade-host --test execution_reconciliation_tests -- --nocapture
cargo test -p everarcade-host --test networked_execution_tests -- --nocapture
echo "reassignment_recovery=ok"
