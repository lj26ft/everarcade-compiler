#!/usr/bin/env bash
set -euo pipefail
cargo test --package execution-core --test ci_orchestration_tests --offline --locked -- test_release_signature_verification
