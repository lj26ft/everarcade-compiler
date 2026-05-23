#!/usr/bin/env bash
set -euo pipefail
cargo test -p execution-core --test sharding_tests --test scheduler_tests -- --nocapture
