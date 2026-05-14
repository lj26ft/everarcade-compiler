#!/usr/bin/env bash
set -euo pipefail
cargo test -p everarcade-host --features xrpl-live --test xrpl_testnet_tests
