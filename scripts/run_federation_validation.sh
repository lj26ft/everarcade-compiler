#!/usr/bin/env bash
set -euo pipefail
mkdir -p deployment/reports
echo "# run federation validation" > deployment/reports/federation_validation_report.md
echo "Generated deterministically." >> deployment/reports/federation_validation_report.md
