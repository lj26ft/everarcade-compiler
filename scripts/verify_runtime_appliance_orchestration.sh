#!/usr/bin/env bash
set -euo pipefail
cargo test -p execution-core --test appliance_orchestration_tests --test lifecycle_tests -- --nocapture
