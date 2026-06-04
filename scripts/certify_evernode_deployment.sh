#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

REPORT_DIR="$ROOT/reports"
REPORT="$REPORT_DIR/evernode_deployment_certification_report.txt"
mkdir -p "$REPORT_DIR"

bootstrap_status="FAIL"
validation_status="FAIL"
hotpocket_certification_status="FAIL"
overall="FAIL"
bootstrap_output=""
validation_output=""
hotpocket_certification_output=""

if bootstrap_output="$(bash scripts/evernode_bootstrap.sh 2>&1)"; then
  bootstrap_status="PASS"
fi
if validation_output="$(bash scripts/validate_evernode_deployment.sh 2>&1)"; then
  validation_status="PASS"
fi
if hotpocket_certification_output="$(bash scripts/certify_hotpocket_integration.sh 2>&1)"; then
  hotpocket_certification_status="PASS"
fi

if [[ "$bootstrap_status" == PASS && "$validation_status" == PASS && "$hotpocket_certification_status" == PASS ]]; then
  overall="PASS"
fi

cat > "$REPORT" <<REPORT_BODY
Evernode Deployment Certification Report
Bootstrap: $bootstrap_status
Validation: $validation_status
HotPocket Certification: $hotpocket_certification_status
Evernode Deployment Layer: $overall

Bootstrap Output:
$bootstrap_output

Validation Output:
$validation_output

HotPocket Certification Output:
$hotpocket_certification_output
REPORT_BODY

echo "Evernode Deployment Layer: $overall"
[[ "$overall" == PASS ]]
