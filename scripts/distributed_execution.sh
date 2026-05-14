#!/usr/bin/env bash
set -euo pipefail
cargo test -p everarcade-host distributed_execution_tests -- --nocapture
cargo test -p everarcade-host task_assignment_tests -- --nocapture
cargo test -p everarcade-host distributed_receipt_tests -- --nocapture
echo "distributed_execution=ok"
