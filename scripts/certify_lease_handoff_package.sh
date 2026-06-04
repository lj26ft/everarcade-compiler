#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

REPORT_DIR="$ROOT/reports"
REPORT="$REPORT_DIR/lease_handoff_certification_report.txt"
mkdir -p "$REPORT_DIR"

build_status="FAIL"
verify_status="FAIL"
install_status="FAIL"
validation_status="FAIL"
overall="FAIL"
build_output=""
verify_output=""
install_output=""
validation_output=""

if build_output="$(bash scripts/build_lease_handoff_package.sh 2>&1)"; then
  build_status="PASS"
fi
if [[ "$build_status" == PASS ]]; then
  if verify_output="$(bash scripts/verify_lease_handoff_package.sh 2>&1)"; then
    verify_status="PASS"
  fi
fi
if [[ "$verify_status" == PASS ]]; then
  if install_output="$(bash scripts/install_lease_handoff_package.sh 2>&1)"; then
    install_status="PASS"
  fi
fi
if [[ "$install_status" == PASS ]]; then
  if validation_output="$(bash scripts/validate_lease_handoff_package.sh 2>&1)"; then
    validation_status="PASS"
  fi
fi

if [[ "$build_status" == PASS && "$verify_status" == PASS && "$install_status" == PASS && "$validation_status" == PASS ]]; then
  overall="PASS"
fi

cat > "$REPORT" <<REPORT_BODY
Evernode Lease Handoff Package Certification Report
Build: $build_status
Verify: $verify_status
Install: $install_status
Validate: $validation_status
Evernode Lease Handoff Package: $overall

Build Output:
$build_output

Verify Output:
$verify_output

Install Output:
$install_output

Validation Output:
$validation_output
REPORT_BODY

echo "Evernode Lease Handoff Package: $overall"
[[ "$overall" == PASS ]]
