#!/usr/bin/env bash
set -uo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIST_DIR="$ROOT_DIR/dist"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/runtime_bootstrap_certification_report.txt"
BUNDLE_NAME="everarcade-runtime-v0.1.0-bundle"
BUNDLE_TARBALL="$DIST_DIR/$BUNDLE_NAME.tar.gz"
BUNDLE_CHECKSUM="$DIST_DIR/$BUNDLE_NAME.sha256"
BOOTSTRAP_DIR="$ROOT_DIR/runtime-bootstrap"
EXTRACTED_BUNDLE_DIR="$BOOTSTRAP_DIR/$BUNDLE_NAME"
BOOTSTRAP_REPORT="$BOOTSTRAP_DIR/runtime_bootstrap_report.txt"
HEALTH_MARKER="$BOOTSTRAP_DIR/runtime-health.txt"
RELEASE_CANDIDATE_REPORT="$ROOT_DIR/reports/release_candidate_validation_report.txt"

bundle_verification_status="NOT RUN"
vendor_restore_status="NOT RUN"
release_candidate_validation_status="NOT RUN"
runtime_bootstrap_status="NOT RUN"
overall_result="FAIL"

mkdir -p "$REPORT_DIR"

write_report() {
  local timestamp
  timestamp="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"

  cat > "$REPORT_PATH" <<REPORT
Timestamp: $timestamp
Bundle Path: dist/$BUNDLE_NAME.tar.gz
Bootstrap Directory: runtime-bootstrap/
Bundle Verification: $bundle_verification_status
Vendor Restore: $vendor_restore_status
Release Candidate Validation: $release_candidate_validation_status
Runtime Bootstrap: $runtime_bootstrap_status
Overall Result: $overall_result
Runtime Bootstrap Certification: $overall_result
REPORT
}

print_summary() {
  printf 'Bundle Verification: %s\n' "$bundle_verification_status"
  printf 'Vendor Restore: %s\n' "$vendor_restore_status"
  printf 'Release Candidate Validation: %s\n' "$release_candidate_validation_status"
  printf 'Runtime Bootstrap: %s\n' "$runtime_bootstrap_status"
  printf 'Runtime Bootstrap Certification: %s\n' "$overall_result"
  printf 'Report: %s\n' "$REPORT_PATH"
}

fail_with_report() {
  overall_result="FAIL"
  write_report
  print_summary
  exit 1
}

report_value() {
  local path="$1"
  local key="$2"

  awk -F': ' -v key="$key" '$1 == key { print $2; found = 1; exit } END { if (!found) print "UNKNOWN" }' "$path"
}

if [[ -f "$BUNDLE_TARBALL" && -f "$BUNDLE_CHECKSUM" ]] && (cd "$DIST_DIR" && sha256sum -c "$BUNDLE_NAME.sha256" >/dev/null 2>&1); then
  bundle_verification_status="PASS"
else
  bundle_verification_status="FAIL"
  fail_with_report
fi

if [[ -d "$ROOT_DIR/vendor" && -f "$EXTRACTED_BUNDLE_DIR/vendor.tar.gz" && -f "$EXTRACTED_BUNDLE_DIR/vendor.tar.gz.sha256" ]]; then
  vendor_restore_status="PASS"
else
  vendor_restore_status="FAIL"
  fail_with_report
fi

if [[ -f "$RELEASE_CANDIDATE_REPORT" ]]; then
  release_candidate_validation_status="$(report_value "$RELEASE_CANDIDATE_REPORT" "Overall Result")"
else
  release_candidate_validation_status="FAIL"
fi

if [[ "$release_candidate_validation_status" != "PASS" ]]; then
  release_candidate_validation_status="FAIL"
  fail_with_report
fi

if [[ -f "$BOOTSTRAP_REPORT" && -f "$HEALTH_MARKER" ]] \
  && [[ "$(report_value "$BOOTSTRAP_REPORT" "Overall Result")" == "PASS" ]] \
  && [[ "$(report_value "$BOOTSTRAP_REPORT" "Runtime Bootstrap")" == "PASS" ]] \
  && grep -q '^Runtime Health: PASS$' "$HEALTH_MARKER"; then
  runtime_bootstrap_status="PASS"
else
  runtime_bootstrap_status="FAIL"
  fail_with_report
fi

overall_result="PASS"
write_report
print_summary
