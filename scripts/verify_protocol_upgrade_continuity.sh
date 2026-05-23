#!/usr/bin/env bash
set -euo pipefail
cargo test -p execution-core sovereign_protocol_coordination_tests -- --nocapture
