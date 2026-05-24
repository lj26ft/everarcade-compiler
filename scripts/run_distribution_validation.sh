#!/usr/bin/env bash
set -euo pipefail
mkdir -p deployment/reports
echo "# run distribution validation" > deployment/reports/distribution_validation_report.md
echo "Generated deterministically." >> deployment/reports/distribution_validation_report.md
