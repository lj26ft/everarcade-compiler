#!/usr/bin/env bash
set -euo pipefail
cargo test -q -p everarcade-host distributed_receipt_sync_tests
