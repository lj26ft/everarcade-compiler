#!/usr/bin/env bash
set -euo pipefail
mkdir -p deployment/reports
echo "# run runtime bootstrap" > deployment/reports/runtime_bootstrap_report.md
echo "Generated deterministically." >> deployment/reports/runtime_bootstrap_report.md
