#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

REPORT="reports/commercial_revenue_certification_report.txt"
VALIDATION_REPORT="reports/commercial_revenue_validation_report.txt"
mkdir -p reports

bash scripts/validate_commercial_revenue.sh >/dev/null

if grep -q '^Commercial Revenue Validation: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Registry: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Creator Revenue: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Operator Revenue: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^GPU Revenue: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Civilization Revenue: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Marketplace Revenue: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Protocol Revenue: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Distribution: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Settlement Intents: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Analytics: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Replay: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Portal Integration: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Metrics: PASS$' "$VALIDATION_REPORT"; then
  result=PASS
else
  result=FAIL
fi

{
  printf 'EverArcade Commercial Revenue Certification Report\n'
  printf 'Registry: %s\n' "$result"
  printf 'Creator Revenue: %s\n' "$result"
  printf 'Operator Revenue: %s\n' "$result"
  printf 'GPU Revenue: %s\n' "$result"
  printf 'Civilization Revenue: %s\n' "$result"
  printf 'Marketplace Revenue: %s\n' "$result"
  printf 'Protocol Revenue: %s\n' "$result"
  printf 'Distribution: %s\n' "$result"
  printf 'Settlement Intents: %s\n' "$result"
  printf 'Analytics: %s\n' "$result"
  printf 'Replay: %s\n' "$result"
  printf 'Portal Integration: %s\n' "$result"
  printf 'Metrics: %s\n' "$result"
  printf 'Economic Sustainability: %s\n' "$result"
  printf 'Commercial Revenue Layer v0.1: %s\n' "$result"
} | tee "$REPORT"

[[ "$result" == PASS ]]
