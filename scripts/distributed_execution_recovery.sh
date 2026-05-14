#!/usr/bin/env bash
set -euo pipefail
cargo test -q -p everarcade-host partition_failover_tests execution_resume_tests
