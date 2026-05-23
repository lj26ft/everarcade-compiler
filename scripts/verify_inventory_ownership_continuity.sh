#!/usr/bin/env bash
set -euo pipefail
cargo test -p execution-core --test inventory_tests -- --nocapture
cargo test -p execution-core --test vault_continuity_tests -- --nocapture
