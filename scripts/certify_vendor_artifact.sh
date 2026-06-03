#!/usr/bin/env bash
set -uo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIST_DIR="$ROOT_DIR/dist"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/vendor_artifact_certification_report.txt"
RESTORE_SCRIPT="$ROOT_DIR/scripts/restore_vendor_artifact.sh"

ARTIFACT="$DIST_DIR/vendor.tar.gz"
CHECKSUM="$DIST_DIR/vendor.tar.gz.sha256"

artifact_sha256="UNKNOWN"
restore_status="NOT RUN"
metadata_status="NOT RUN"
overall_result="FAIL"

mkdir -p "$REPORT_DIR"

write_report() {
  local timestamp
  timestamp="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"

  cat > "$REPORT_PATH" <<REPORT
Timestamp: $timestamp
Artifact Path: dist/vendor.tar.gz
Artifact SHA256: $artifact_sha256
Restore Status: $restore_status
Metadata Status: $metadata_status
Overall Result: $overall_result
REPORT
}

print_summary() {
  printf 'Vendor Artifact Certification: %s\n' "$overall_result"
  printf 'Restore Status: %s\n' "$restore_status"
  printf 'Metadata Status: %s\n' "$metadata_status"
  printf 'Report: %s\n' "$REPORT_PATH"
}

fail_with_report() {
  overall_result="FAIL"
  write_report
  print_summary
  exit 1
}

if [[ ! -f "$ARTIFACT" ]]; then
  echo "Missing required vendor artifact: dist/vendor.tar.gz" >&2
  restore_status="MISSING ARTIFACT"
  fail_with_report
fi

if [[ ! -f "$CHECKSUM" ]]; then
  echo "Missing required vendor checksum: dist/vendor.tar.gz.sha256" >&2
  restore_status="MISSING CHECKSUM"
  fail_with_report
fi

artifact_sha256="$(sha256sum "$ARTIFACT" | awk '{print $1}')"

restore_output="$(mktemp)"
if bash "$RESTORE_SCRIPT" >"$restore_output" 2>&1; then
  restore_status="PASS"
  cat "$restore_output"
else
  restore_status="FAIL"
  cat "$restore_output" >&2
  rm -f "$restore_output"
  fail_with_report
fi
rm -f "$restore_output"

metadata_output="$(mktemp)"
if cargo metadata --offline --locked --format-version 1 >"$metadata_output" 2>&1; then
  metadata_status="PASS"
  overall_result="PASS"
else
  metadata_status="FAIL"
  cat "$metadata_output" >&2
  rm -f "$metadata_output"
  fail_with_report
fi
rm -f "$metadata_output"

write_report
print_summary
