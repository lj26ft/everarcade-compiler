#!/usr/bin/env bash
set -euo pipefail
mkdir -p deployment/reports
echo "# run restoration validation" > deployment/reports/restoration_validation_report.md
echo "Generated deterministically." >> deployment/reports/restoration_validation_report.md
