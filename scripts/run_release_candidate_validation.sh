#!/usr/bin/env bash
set -uo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIST_DIR="$ROOT_DIR/dist"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/release_candidate_validation_report.txt"
OFFLINE_REPORT_PATH="$REPORT_DIR/runtime_offline_gate_report.txt"
RESTORE_SCRIPT="$ROOT_DIR/scripts/restore_vendor_artifact.sh"
OFFLINE_GATE_SCRIPT="$ROOT_DIR/scripts/check_runtime_offline_gate.sh"

ARTIFACT="$DIST_DIR/vendor.tar.gz"
CHECKSUM="$DIST_DIR/vendor.tar.gz.sha256"

vendor_restore_status="NOT RUN"
metadata_status="NOT RUN"
check_status="NOT RUN"
test_status="NOT RUN"
overall_result="FAIL"

mkdir -p "$REPORT_DIR"

write_report() {
  local timestamp
  timestamp="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"

  cat > "$REPORT_PATH" <<REPORT
Timestamp: $timestamp
Vendor Restore Status: $vendor_restore_status
Offline Metadata Status: $metadata_status
Offline Check Status: $check_status
Offline Test Status: $test_status
Overall Result: $overall_result
REPORT
}

print_summary() {
  printf 'Vendor Restore: %s\n' "$vendor_restore_status"
  printf 'Cargo Metadata: %s\n' "$metadata_status"
  printf 'Runtime Check: %s\n' "$check_status"
  printf 'Runtime Tests: %s\n' "$test_status"
  printf 'Release Candidate Validation: %s\n' "$overall_result"
  printf 'Report: %s\n' "$REPORT_PATH"
}

fail_with_report() {
  overall_result="FAIL"
  write_report
  print_summary
  exit 1
}

gate_step_status() {
  local description="$1"

  if [[ ! -f "$OFFLINE_REPORT_PATH" ]]; then
    printf 'UNKNOWN'
    return 0
  fi

  awk -v desc="$description" '
    $0 == "== " desc " ==" { in_step = 1; next }
    in_step && /^status: pass/ { print "PASS"; found = 1; exit }
    in_step && /^status: fail/ { print "FAIL"; found = 1; exit }
    END { if (!found) print "UNKNOWN" }
  ' "$OFFLINE_REPORT_PATH"
}

if [[ ! -f "$ARTIFACT" ]]; then
  echo "Missing required vendor artifact: dist/vendor.tar.gz" >&2
  echo "Create or copy the release vendor artifact before running release candidate validation." >&2
  vendor_restore_status="MISSING ARTIFACT"
  fail_with_report
fi

if [[ ! -f "$CHECKSUM" ]]; then
  echo "Missing required vendor checksum: dist/vendor.tar.gz.sha256" >&2
  echo "Create or copy the vendor checksum before running release candidate validation." >&2
  vendor_restore_status="MISSING CHECKSUM"
  fail_with_report
fi

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

gate_output="$(mktemp)"
if bash "$OFFLINE_GATE_SCRIPT" >"$gate_output" 2>&1; then
  metadata_status="PASS"
  check_status="PASS"
  test_status="PASS"
  overall_result="PASS"
  cat "$gate_output"
else
  cat "$gate_output" >&2
  metadata_status="$(gate_step_status "Cargo metadata offline locked")"
  check_status="$(gate_step_status "Runtime cargo check offline locked")"
  test_status="$(gate_step_status "Runtime cargo tests offline locked")"
  rm -f "$gate_output"
  fail_with_report
fi
rm -f "$gate_output"

write_report
print_summary
