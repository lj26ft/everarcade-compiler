#!/usr/bin/env bash
set -euo pipefail
cargo test -q -p everarcade-host --test partition_failover_tests --test execution_resume_tests
