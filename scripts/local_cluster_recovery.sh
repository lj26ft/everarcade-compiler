#!/usr/bin/env bash
set -euo pipefail
cargo test -p everarcade-host --test operator_resync_tests
