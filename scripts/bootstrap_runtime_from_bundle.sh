#!/usr/bin/env bash
set -uo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIST_DIR="$ROOT_DIR/dist"
BUNDLE_NAME="everarcade-runtime-v0.1.0-bundle"
BUNDLE_TARBALL="$DIST_DIR/$BUNDLE_NAME.tar.gz"
BUNDLE_CHECKSUM="$DIST_DIR/$BUNDLE_NAME.sha256"
BOOTSTRAP_DIR="$ROOT_DIR/runtime-bootstrap"
EXTRACTED_BUNDLE_DIR="$BOOTSTRAP_DIR/$BUNDLE_NAME"
BOOTSTRAP_REPORT="$BOOTSTRAP_DIR/runtime_bootstrap_report.txt"
HEALTH_MARKER="$BOOTSTRAP_DIR/runtime-health.txt"
RESTORE_SCRIPT="$ROOT_DIR/scripts/restore_vendor_artifact.sh"
RELEASE_VALIDATION_SCRIPT="$ROOT_DIR/scripts/run_release_candidate_validation.sh"
RELEASE_CANDIDATE_REPORT="$ROOT_DIR/reports/release_candidate_validation_report.txt"

bundle_verification_status="NOT RUN"
vendor_restore_status="NOT RUN"
release_candidate_validation_status="NOT RUN"
runtime_bootstrap_status="NOT RUN"
overall_result="FAIL"

write_report() {
  local timestamp
  timestamp="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"

  mkdir -p "$BOOTSTRAP_DIR"
  cat > "$BOOTSTRAP_REPORT" <<REPORT
Timestamp: $timestamp
Bundle Path: dist/$BUNDLE_NAME.tar.gz
Checksum Path: dist/$BUNDLE_NAME.sha256
Bootstrap Directory: runtime-bootstrap/
Bundle Verification: $bundle_verification_status
Vendor Restore: $vendor_restore_status
Release Candidate Validation: $release_candidate_validation_status
Runtime Bootstrap: $runtime_bootstrap_status
Overall Result: $overall_result
REPORT
}

print_summary() {
  printf 'Bundle Verification: %s\n' "$bundle_verification_status"
  printf 'Vendor Restore: %s\n' "$vendor_restore_status"
  printf 'Release Candidate Validation: %s\n' "$release_candidate_validation_status"
  printf 'Runtime Bootstrap: %s\n' "$runtime_bootstrap_status"
  printf 'Report: %s\n' "$BOOTSTRAP_REPORT"
}

fail_with_report() {
  overall_result="FAIL"
  write_report
  print_summary
  exit 1
}

require_file() {
  local path="$1"
  local label="$2"

  if [[ ! -f "$path" ]]; then
    printf 'Missing required %s: %s\n' "$label" "${path#$ROOT_DIR/}" >&2
    fail_with_report
  fi
}

report_value() {
  local path="$1"
  local key="$2"

  awk -F': ' -v key="$key" '$1 == key { print $2; found = 1; exit } END { if (!found) print "UNKNOWN" }' "$path"
}

rm -rf "$BOOTSTRAP_DIR"
mkdir -p "$BOOTSTRAP_DIR"

require_file "$BUNDLE_TARBALL" "release artifact bundle"
require_file "$BUNDLE_CHECKSUM" "release artifact bundle checksum"

checksum_output="$(mktemp)"
if (cd "$DIST_DIR" && sha256sum -c "$BUNDLE_NAME.sha256") >"$checksum_output" 2>&1; then
  bundle_verification_status="PASS"
else
  bundle_verification_status="FAIL"
  cat "$checksum_output" >&2
  rm -f "$checksum_output"
  fail_with_report
fi
rm -f "$checksum_output"

extract_output="$(mktemp)"
if tar -xzf "$BUNDLE_TARBALL" -C "$BOOTSTRAP_DIR" >"$extract_output" 2>&1; then
  :
else
  runtime_bootstrap_status="FAIL"
  cat "$extract_output" >&2
  rm -f "$extract_output"
  fail_with_report
fi
rm -f "$extract_output"

require_file "$EXTRACTED_BUNDLE_DIR/vendor.tar.gz" "bundled vendor artifact"
require_file "$EXTRACTED_BUNDLE_DIR/vendor.tar.gz.sha256" "bundled vendor checksum"

cp "$EXTRACTED_BUNDLE_DIR/vendor.tar.gz" "$DIST_DIR/vendor.tar.gz"
cp "$EXTRACTED_BUNDLE_DIR/vendor.tar.gz.sha256" "$DIST_DIR/vendor.tar.gz.sha256"

restore_output="$(mktemp)"
if bash "$RESTORE_SCRIPT" >"$restore_output" 2>&1; then
  vendor_restore_status="PASS"
else
  vendor_restore_status="FAIL"
  cat "$restore_output" >&2
  rm -f "$restore_output"
  fail_with_report
fi
rm -f "$restore_output"

export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"

validation_output="$(mktemp)"
if bash "$RELEASE_VALIDATION_SCRIPT" >"$validation_output" 2>&1; then
  release_candidate_validation_status="$(report_value "$RELEASE_CANDIDATE_REPORT" "Overall Result")"
  if [[ "$release_candidate_validation_status" != "PASS" ]]; then
    release_candidate_validation_status="FAIL"
    cat "$validation_output" >&2
    rm -f "$validation_output"
    fail_with_report
  fi
else
  release_candidate_validation_status="FAIL"
  cat "$validation_output" >&2
  rm -f "$validation_output"
  fail_with_report
fi
rm -f "$validation_output"

cat > "$HEALTH_MARKER" <<HEALTH
Runtime Health: PASS
Source Bundle: dist/$BUNDLE_NAME.tar.gz
Vendor Restore: PASS
Release Candidate Validation: PASS
HEALTH

runtime_bootstrap_status="PASS"
overall_result="PASS"
write_report
print_summary
