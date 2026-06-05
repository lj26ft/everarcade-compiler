#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
REPORT="reports/creator_sdk_certification_report.txt"
mkdir -p reports
: > "$REPORT"

bash scripts/validate_creator_sdk.sh >/tmp/creator_sdk_validation_for_certification.log
cat /tmp/creator_sdk_validation_for_certification.log >> "$REPORT"

for component in CLI Templates Assets Inventory Economy Civilization Deployment Monetization Examples Onboarding; do
  if ! grep -q "^${component}: PASS$" /tmp/creator_sdk_validation_for_certification.log; then
    echo "${component}: FAIL" | tee -a "$REPORT"
    exit 1
  fi
done

echo "Creator SDK v0.1: PASS" | tee -a "$REPORT"
