#!/usr/bin/env bash
set -euo pipefail
cargo test -p execution-core federation_snapshot_tests -- --nocapture
