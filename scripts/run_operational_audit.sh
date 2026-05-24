#!/usr/bin/env bash
set -euo pipefail
mkdir -p deployment/reports
echo "# run operational audit" > deployment/reports/operational_audit_report.md
echo "Generated deterministically." >> deployment/reports/operational_audit_report.md
