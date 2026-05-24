#!/usr/bin/env bash
set -euo pipefail
mkdir -p deployment/reports
echo "# run deployment validation" > deployment/reports/deployment_validation_report.md
echo "Generated deterministically." >> deployment/reports/deployment_validation_report.md
