#!/usr/bin/env bash
set -euo pipefail
mkdir -p world/reports
cargo test --package execution-core test_multi_era_archive_equivalence
echo "archive:ok" > world/reports/civilization-archive.txt
