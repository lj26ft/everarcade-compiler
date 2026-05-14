#!/usr/bin/env bash
set -euo pipefail
cargo test -p everarcade-host --test distributed_execution_tests -- --nocapture
cargo test -p everarcade-host --test task_assignment_tests -- --nocapture
cargo test -p everarcade-host --test distributed_receipt_tests -- --nocapture
echo "distributed_execution=ok"
