#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

REPORT="reports/developer_experience_certification_report.txt"
OPEN_SOURCE_REPORT="reports/open_source_readiness_report.txt"
mkdir -p reports
: > "$REPORT"

status_for_file() {
  local path="$1"
  [[ -f "$path" ]] && echo PASS || echo FAIL
}

repo_map="$(status_for_file docs/repository/repository-map.md)"
developer_journey="$(status_for_file docs/onboarding/30-minute-developer-journey.md)"
governance="FAIL"
if [[ -f LICENSE && -f CONTRIBUTING.md && -f SECURITY.md && -f CODE_OF_CONDUCT.md ]]; then governance="PASS"; fi
proof_chain="$(status_for_file docs/runtime-platform/proof-chain.md)"
artifact_policy="$(status_for_file docs/repository/artifact-policy.md)"

open_source_output="$(bash scripts/validate_open_source_readiness.sh)"
open_source="FAIL"
if [[ -f "$OPEN_SOURCE_REPORT" ]] && rg -q '^Open Source Audit: PASS$' "$OPEN_SOURCE_REPORT"; then
  open_source="PASS"
fi

final="FAIL"
if [[ "$repo_map" == PASS && "$developer_journey" == PASS && "$governance" == PASS && "$proof_chain" == PASS && "$artifact_policy" == PASS && "$open_source" == PASS ]]; then
  final="PASS"
fi

cat > "$REPORT" <<EOF_REPORT
Developer Experience Certification Report
Date: 2026-06-09

Repository Map: $repo_map
Developer Journey: $developer_journey
Governance Files: $governance
Proof Chain Documentation: $proof_chain
Artifact Policy: $artifact_policy
Open Source Audit: $open_source

Open Source Validation Output:
$open_source_output

Developer Experience & Open Source Readiness v0.1: $final
EOF_REPORT

cat "$REPORT"
[[ "$final" == "PASS" ]]
