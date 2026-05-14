#!/usr/bin/env bash
set -euo pipefail
cargo test -q -p everarcade-host --test distributed_receipt_sync_tests
