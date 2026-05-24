#!/usr/bin/env bash
set -euo pipefail
mkdir -p deployment/reports
echo "# run release validation" > deployment/reports/release_validation_report.md
echo "Generated deterministically." >> deployment/reports/release_validation_report.md
