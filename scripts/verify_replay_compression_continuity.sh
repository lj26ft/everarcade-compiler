#!/usr/bin/env bash
set -euo pipefail
cargo test -p execution-core replay_compression_tests::compression_continuity -- --nocapture
