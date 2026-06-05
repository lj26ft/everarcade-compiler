#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
REPORT="reports/game_templates_certification_report.txt"
LOG="$(mktemp)"
trap 'rm -f "$LOG"' EXIT
mkdir -p reports
: > "$REPORT"

bash scripts/validate_game_templates.sh > "$LOG"
cat "$LOG" >> "$REPORT"

for component in Registry Arena RPG Trading Civilization Sandbox "Asset Packs" Generator Deployments Metrics; do
  if ! grep -q "^${component}: PASS$" "$LOG"; then
    echo "${component}: FAIL" | tee -a "$REPORT"
    exit 1
  fi
done

echo "Game Templates v0.1: PASS" | tee -a "$REPORT"
