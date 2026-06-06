#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

required_files=(
  "reports/repo_reality_audit_report.txt"
  "docs/architecture/repo-reality-audit.md"
  "reports/local_game_launch_report.txt"
  "reports/repo_readability_report.txt"
  "docs/architecture/human-readable-repo-map.md"
  "docs/security/abuse-and-adversarial-analysis.md"
  "reports/abuse_analysis_report.txt"
  "reports/pseudocode_annotation_report.txt"
  "docs/open-source/open-source-readiness.md"
  "reports/open_source_readiness_report.txt"
  "reports/everarcade_v0_1_go_no_go_report.txt"
)

missing=0
for file in "${required_files[@]}"; do
  if [[ ! -s "$file" ]]; then
    printf 'MISSING: %s\n' "$file" >&2
    missing=1
  fi
done

if [[ "$missing" -ne 0 ]]; then
  printf 'Repo reality audit validation: FAIL\n' >&2
  exit 1
fi

if ! grep -q 'Final classification: Simulated' reports/local_game_launch_report.txt; then
  printf 'Expected local launch classification not found.\n' >&2
  exit 1
fi

if ! grep -q 'Final result: NOT READY' reports/open_source_readiness_report.txt; then
  printf 'Expected open-source readiness result not found.\n' >&2
  exit 1
fi

if ! grep -q 'Decision: NO-GO' reports/everarcade_v0_1_go_no_go_report.txt; then
  printf 'Expected go/no-go decision not found.\n' >&2
  exit 1
fi

printf 'Repo reality audit validation: PASS (%s required files)\n' "${#required_files[@]}"
