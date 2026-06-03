#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIST_DIR="$ROOT_DIR/dist"
REPORT_DIR="$ROOT_DIR/reports"
BUNDLE_NAME="everarcade-runtime-v0.1.0-bundle"
STAGING_DIR="$DIST_DIR/$BUNDLE_NAME"
BUNDLE_TARBALL="$DIST_DIR/$BUNDLE_NAME.tar.gz"
BUNDLE_CHECKSUM="$DIST_DIR/$BUNDLE_NAME.sha256"
MANIFEST_PATH="$STAGING_DIR/MANIFEST.txt"
REPORT_PATH="$REPORT_DIR/release_artifact_bundle_report.txt"

VENDOR_ARTIFACT="$DIST_DIR/vendor.tar.gz"
VENDOR_CHECKSUM="$DIST_DIR/vendor.tar.gz.sha256"
CERTIFICATION_REPORT="$REPORT_DIR/vendor_artifact_certification_report.txt"
OFFLINE_GATE_REPORT="$REPORT_DIR/runtime_offline_gate_report.txt"
RELEASE_CANDIDATE_REPORT="$REPORT_DIR/release_candidate_validation_report.txt"

require_file() {
  local path="$1"
  local label="$2"

  if [[ ! -f "$path" ]]; then
    printf 'Missing required %s: %s\n' "$label" "${path#$ROOT_DIR/}" >&2
    exit 1
  fi
}

report_value() {
  local path="$1"
  local key="$2"
  awk -F': ' -v key="$key" '$1 == key { print $2; found = 1; exit } END { if (!found) print "UNKNOWN" }' "$path"
}

write_report() {
  local status="$1"
  local created_at="$2"
  local vendor_sha256="$3"
  local certification_status="$4"
  local release_candidate_status="$5"

  mkdir -p "$REPORT_DIR"
  cat > "$REPORT_PATH" <<REPORT
Bundle Name: $BUNDLE_NAME
Created At: $created_at
Vendor SHA256: $vendor_sha256
Certification Status: $certification_status
Release Candidate Status: $release_candidate_status
Release Artifact Bundle: $status
Bundle Path: dist/$BUNDLE_NAME.tar.gz
Checksum Path: dist/$BUNDLE_NAME.sha256
REPORT
}

require_file "$VENDOR_ARTIFACT" "vendor artifact"
require_file "$VENDOR_CHECKSUM" "vendor checksum"

bash "$ROOT_DIR/scripts/certify_vendor_artifact.sh"
bash "$ROOT_DIR/scripts/run_release_candidate_validation.sh"

require_file "$CERTIFICATION_REPORT" "vendor certification report"
require_file "$OFFLINE_GATE_REPORT" "runtime offline gate report"
require_file "$RELEASE_CANDIDATE_REPORT" "release candidate validation report"

created_at="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
vendor_sha256="$(sha256sum "$VENDOR_ARTIFACT" | awk '{print $1}')"
certification_status="$(report_value "$CERTIFICATION_REPORT" "Overall Result")"
release_candidate_status="$(report_value "$RELEASE_CANDIDATE_REPORT" "Overall Result")"

if [[ "$certification_status" != "PASS" ]]; then
  write_report "FAIL" "$created_at" "$vendor_sha256" "$certification_status" "$release_candidate_status"
  printf 'Release Artifact Bundle: FAIL\n' >&2
  printf 'Vendor certification did not pass.\n' >&2
  exit 1
fi

if [[ "$release_candidate_status" != "PASS" ]]; then
  write_report "FAIL" "$created_at" "$vendor_sha256" "$certification_status" "$release_candidate_status"
  printf 'Release Artifact Bundle: FAIL\n' >&2
  printf 'Release candidate validation did not pass.\n' >&2
  exit 1
fi

rm -rf "$STAGING_DIR"
mkdir -p "$STAGING_DIR"

cp "$VENDOR_ARTIFACT" "$STAGING_DIR/vendor.tar.gz"
cp "$VENDOR_CHECKSUM" "$STAGING_DIR/vendor.tar.gz.sha256"
cp "$CERTIFICATION_REPORT" "$STAGING_DIR/vendor_artifact_certification_report.txt"
cp "$OFFLINE_GATE_REPORT" "$STAGING_DIR/runtime_offline_gate_report.txt"
cp "$RELEASE_CANDIDATE_REPORT" "$STAGING_DIR/release_candidate_validation_report.txt"

cat > "$MANIFEST_PATH" <<MANIFEST
Bundle Name: $BUNDLE_NAME
Created At: $created_at
Vendor SHA256: $vendor_sha256
Certification Status: $certification_status
Release Candidate Status: $release_candidate_status
MANIFEST

rm -f "$BUNDLE_TARBALL" "$BUNDLE_CHECKSUM"
tar -czf "$BUNDLE_TARBALL" -C "$DIST_DIR" "$BUNDLE_NAME"
(
  cd "$DIST_DIR"
  sha256sum "$BUNDLE_NAME.tar.gz" > "$BUNDLE_NAME.sha256"
)

write_report "PASS" "$created_at" "$vendor_sha256" "$certification_status" "$release_candidate_status"

printf 'Release Artifact Bundle: PASS\n'
printf 'Bundle: %s\n' "$BUNDLE_TARBALL"
printf 'Checksum: %s\n' "$BUNDLE_CHECKSUM"
printf '\nGenerated bundle is a release artifact. Do not commit dist/*.tar.gz.\n'
printf 'Attach the bundle and checksum to the release.\n'
