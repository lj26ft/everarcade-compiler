#!/usr/bin/env bash
set -euo pipefail
cargo test -p execution-core --test persistence_archive_tests -- --nocapture
cargo test -p execution-core --test persistence_restoration_tests -- --nocapture
