#!/usr/bin/env bash
set -euo pipefail
cargo test -p everarcade-host --test operator_reassignment_tests -- --nocapture
echo "operator_failover=ok"
