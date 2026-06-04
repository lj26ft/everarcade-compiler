#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VERSION="everarcade-runtime-v0.1"
DIST_DIR="${DIST_DIR:-$ROOT_DIR/dist}"
BUNDLE="$DIST_DIR/$VERSION.tar.gz"
CHECKSUM="$BUNDLE.sha256"
PREFIX="${EVERARCADE_APPLIANCE_PREFIX:-$ROOT_DIR/appliance/certification-install}"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/runtime_appliance_certification_report.txt"
mkdir -p "$REPORT_DIR"

bundle_status="FAIL"
checksum_status="FAIL"
install_status="FAIL"
validation_status="FAIL"
doctor_status="FAIL"
overall="FAIL"

[[ -s "$BUNDLE" ]] && bundle_status="PASS"
if [[ -s "$CHECKSUM" ]] && (cd "$(dirname "$BUNDLE")" && sha256sum -c "$(basename "$CHECKSUM")" >/dev/null); then
  checksum_status="PASS"
fi
if [[ "$bundle_status" == "PASS" && "$checksum_status" == "PASS" ]] && EVERARCADE_APPLIANCE_PREFIX="$PREFIX" bash "$ROOT_DIR/scripts/install_runtime_appliance.sh" "$BUNDLE" "$PREFIX" >/dev/null; then
  install_status="PASS"
fi
if [[ "$install_status" == "PASS" ]] && EVERARCADE_APPLIANCE_PREFIX="$PREFIX" bash "$ROOT_DIR/scripts/validate_runtime_appliance.sh" >/dev/null; then
  validation_status="PASS"
fi
if [[ "$install_status" == "PASS" ]] && EVERARCADE_APPLIANCE_PREFIX="$PREFIX" bash "$ROOT_DIR/scripts/runtime_doctor.sh" >/dev/null; then
  doctor_status="PASS"
fi
if [[ "$bundle_status" == "PASS" && "$checksum_status" == "PASS" && "$install_status" == "PASS" && "$validation_status" == "PASS" && "$doctor_status" == "PASS" ]]; then
  overall="PASS"
fi
cat > "$REPORT_PATH" <<REPORT
Runtime Appliance Certification Report
Version: $VERSION
Bundle Exists: $bundle_status
Checksum Valid: $checksum_status
Installation Succeeds: $install_status
Validation Succeeds: $validation_status
Doctor Succeeds: $doctor_status
Runtime Appliance v0.1: $overall
REPORT

if [[ "$overall" != "PASS" ]]; then
  cat "$REPORT_PATH" >&2
  exit 1
fi
cat "$REPORT_PATH"
echo "Certification: PASS"
echo "Runtime Appliance v0.1: PASS"
