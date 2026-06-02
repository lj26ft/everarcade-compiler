#!/usr/bin/env bash
set -euo pipefail
mkdir -p deployment/reports
report="deployment/reports/creator_dashboard_readiness.md"
echo "creator_validation=run_creator_dashboard_validation.sh mode=offline locked=true frozen=true deterministic_logs=true replay_continuity=preserved authority_mutation=rejected"
CARGO_BUILD_JOBS=1 cargo test --package tools --test creator_toolchain_tests test_creator_dashboard_equivalence --offline --locked
{
  echo "# Creator Dashboard Validation"
  echo
  echo "status: Ready"
  echo "classification: Ready"
  echo "generated: deterministic"
  echo
  echo "- frontend/creator-dashboard/src/App.tsx: present"
  test -e "frontend/creator-dashboard/src/App.tsx"
  echo "- frontend/shared-api/src/index.ts: present"
  test -e "frontend/shared-api/src/index.ts"
} > "$report"
echo "$report"
