#!/usr/bin/env bash
set -euo pipefail
mkdir -p deployment/reports
echo "# run scheduler validation" > deployment/reports/scheduler_validation_report.md
echo "Generated deterministically." >> deployment/reports/scheduler_validation_report.md
