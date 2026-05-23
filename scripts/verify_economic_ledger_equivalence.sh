#!/usr/bin/env bash
set -euo pipefail
cargo test -p execution-core economic_ledger_tests::ledger_determinism -- --nocapture
