#!/usr/bin/env bash
set -euo pipefail
cargo test -q -p everarcade-host --test workload_partition_tests
