#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
VALIDATION_REPORT="$REPORT_DIR/release_candidate_validation_report.txt"
CERTIFICATION_REPORT="$REPORT_DIR/release_candidate_certification_report.txt"
mkdir -p "$REPORT_DIR"

validation_output="$(bash "$ROOT_DIR/scripts/validate_release_candidate.sh" 2>&1)"

required_markers=(
  "Manifest: PASS"
  "Freeze: PASS"
  "Upgrade: PASS"
  "Recovery: PASS"
  "Distribution: PASS"
  "Testnet Readiness: PASS"
  "Release Candidate Validation: PASS"
)

status="PASS"
for marker in "${required_markers[@]}"; do
  if ! grep -Fq "$marker" <<< "$validation_output"; then
    status="FAIL"
  fi
done

timestamp="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
{
  printf 'EverArcade Release Candidate Certification Report\n'
  printf 'Version: v0.1-rc1\n'
  printf 'Timestamp: %s\n' "$timestamp"
  printf 'Validation Report: %s\n\n' "$VALIDATION_REPORT"
  printf '%s\n\n' "$validation_output"
  printf 'EverArcade Release Candidate v0.1: %s\n' "$status"
} > "$CERTIFICATION_REPORT"

cat "$CERTIFICATION_REPORT"
[[ "$status" == "PASS" ]]
