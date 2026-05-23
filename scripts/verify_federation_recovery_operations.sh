#!/usr/bin/env bash
set -euo pipefail
cargo test -p execution-core --test archive_operations_tests --test recovery_operations_tests -- --nocapture
