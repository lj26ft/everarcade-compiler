#!/usr/bin/env bash
set -euo pipefail
cargo test --package execution-core --test release_certification_tests "$@"
