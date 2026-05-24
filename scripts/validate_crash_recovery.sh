#!/usr/bin/env bash
set -euo pipefail
mkdir -p security/reports
cargo test --package execution-core --test runtime_security_tests -- --nocapture | tee security/reports/validate_crash_recovery.log
