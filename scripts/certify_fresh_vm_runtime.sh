#!/usr/bin/env bash
set -uo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIST_DIR="$ROOT_DIR/dist"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/fresh_vm_certification_report.txt"
RESTORE_SCRIPT="$ROOT_DIR/scripts/restore_vendor_artifact.sh"
RELEASE_VALIDATION_SCRIPT="$ROOT_DIR/scripts/run_release_candidate_validation.sh"
VENDOR_DIR="$ROOT_DIR/vendor"
ARTIFACT="$DIST_DIR/vendor.tar.gz"
CHECKSUM="$DIST_DIR/vendor.tar.gz.sha256"

ubuntu_version="UNKNOWN"
vendor_restore_status="NOT RUN"
release_candidate_validation_status="NOT RUN"
overall_result="FAIL"

mkdir -p "$REPORT_DIR"

load_ubuntu_version() {
  if [[ -r /etc/os-release ]]; then
    # shellcheck disable=SC1091
    . /etc/os-release
    ubuntu_version="${PRETTY_NAME:-${NAME:-UNKNOWN}}"
  fi
}

write_report() {
  local timestamp
  timestamp="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"

  cat > "$REPORT_PATH" <<REPORT
Timestamp: $timestamp
Ubuntu Version: $ubuntu_version
Vendor Restore Status: $vendor_restore_status
Release Candidate Validation Status: $release_candidate_validation_status
Overall Result: $overall_result
REPORT
}

print_summary() {
  printf 'Fresh VM Certification: %s\n' "$overall_result"
  printf 'Report: %s\n' "$REPORT_PATH"
}

fail_with_report() {
  overall_result="FAIL"
  write_report
  print_summary
  exit 1
}

load_ubuntu_version

if [[ ! -f "$ARTIFACT" ]]; then
  echo "Missing required vendor artifact: dist/vendor.tar.gz" >&2
  echo "Copy the release vendor artifact into dist/ before running fresh VM certification." >&2
  vendor_restore_status="MISSING ARTIFACT"
  fail_with_report
fi

if [[ ! -f "$CHECKSUM" ]]; then
  echo "Missing required vendor checksum: dist/vendor.tar.gz.sha256" >&2
  echo "Copy the release vendor checksum into dist/ before running fresh VM certification." >&2
  vendor_restore_status="MISSING CHECKSUM"
  fail_with_report
fi

rm -rf "$VENDOR_DIR"

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
  release_candidate_validation_status="PASS"
  overall_result="PASS"
  cat "$validation_output"
else
  release_candidate_validation_status="FAIL"
  cat "$validation_output" >&2
  rm -f "$validation_output"
  fail_with_report
fi
rm -f "$validation_output"

write_report
print_summary
