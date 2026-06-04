#!/usr/bin/env bash
set -u -o pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

VERSION="v0.1"
NAME="everarcade-lease-handoff-$VERSION"
DIST_DIR="$ROOT/dist"
PACKAGE="${1:-$DIST_DIR/$NAME.tar.gz}"
CHECKSUM="${2:-$PACKAGE.sha256}"
LEASE_ROOT="${3:-${EVERARCADE_LIVE_LEASE_ROOT:-$ROOT/tmp/live-evernode-lease}}"
REPORT_DIR="$ROOT/reports"
REPORT="$REPORT_DIR/live_lease_deployment_report.txt"
FAILURES="$REPORT_DIR/live_lease_failures.txt"
EXTRACT_ROOT="$LEASE_ROOT/handoff"
EXTRACTED="$EXTRACT_ROOT/$NAME"
AUTO_BUILD="${EVERARCADE_LIVE_LEASE_AUTO_BUILD:-1}"

mkdir -p "$REPORT_DIR"
for ignore_pattern in "/tmp/live-evernode-lease/" "/dist/everarcade-lease-handoff-v0.1.tar.gz" "/dist/everarcade-lease-handoff-v0.1.tar.gz.sha256"; do
  grep -qxF "$ignore_pattern" "$ROOT/.git/info/exclude" || printf '%s\n' "$ignore_pattern" >> "$ROOT/.git/info/exclude"
done

package_status="FAIL"
checksum_status="FAIL"
manifest_status="FAIL"
extract_status="FAIL"
layout_status="FAIL"
metadata_status="FAIL"
overall="FAIL"
build_status="SKIPPED"
build_output=""
checksum_output=""
manifest_summary="MISSING"
failure_entries=""
started_at="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"

record_failure() {
  local id="$1" component="$2" error="$3" impact="$4" fix="$5"
  failure_entries+="Failure Identifier: $id
Component: $component
Error: $error
Impact: $impact
Suggested Fix: $fix

"
}

if [[ ! -s "$PACKAGE" && "$AUTO_BUILD" == "1" ]]; then
  if build_output="$(bash scripts/build_lease_handoff_package.sh 2>&1)"; then
    build_status="PASS"
  else
    build_status="FAIL"
    record_failure "LIVE-LEASE-DEPLOY-001" "Lease Handoff Package" "Package missing and automatic build failed." "Deployment cannot proceed without the immutable handoff package." "Run bash scripts/build_lease_handoff_package.sh and inspect reports/lease_handoff_build_report.txt."
  fi
fi

if [[ -s "$PACKAGE" ]]; then
  package_status="PASS"
else
  record_failure "LIVE-LEASE-DEPLOY-002" "Package Artifact" "Package not found: $PACKAGE" "No lease payload can be extracted." "Provide the package path or enable EVERARCADE_LIVE_LEASE_AUTO_BUILD=1."
fi

if [[ "$package_status" == "PASS" ]]; then
  if [[ -s "$CHECKSUM" ]]; then
    if checksum_output="$(cd "$(dirname "$CHECKSUM")" && sha256sum -c "$(basename "$CHECKSUM")" 2>&1)"; then
      checksum_status="PASS"
    else
      record_failure "LIVE-LEASE-DEPLOY-003" "Package Checksum" "Checksum verification failed for $CHECKSUM" "Package integrity is not proven." "Regenerate the package and checksum from a trusted build host."
    fi
  else
    record_failure "LIVE-LEASE-DEPLOY-004" "Checksum Artifact" "Checksum not found: $CHECKSUM" "Package integrity is not proven." "Provide the checksum artifact generated with the handoff package."
  fi
fi

if [[ "$checksum_status" == "PASS" ]]; then
  rm -rf "$LEASE_ROOT"
  mkdir -p "$EXTRACT_ROOT"
  if tar -xzf "$PACKAGE" -C "$EXTRACT_ROOT"; then
    extract_status="PASS"
  else
    record_failure "LIVE-LEASE-DEPLOY-005" "Package Extraction" "tar extraction failed for $PACKAGE" "Lease layout cannot be initialized." "Validate archive readability and available disk space on the lease."
  fi
fi

if [[ "$extract_status" == "PASS" ]]; then
  if [[ -s "$EXTRACTED/manifests/deployment-manifest.txt" ]] && grep -q "EverArcade Evernode Lease Handoff Package v0.1" "$EXTRACTED/manifests/deployment-manifest.txt"; then
    manifest_status="PASS"
    manifest_summary="$(head -n 1 "$EXTRACTED/manifests/deployment-manifest.txt")"
  else
    record_failure "LIVE-LEASE-DEPLOY-006" "Deployment Manifest" "Expected manifest missing or invalid under $EXTRACTED/manifests" "Lease operator cannot verify handoff semantics." "Rebuild the lease handoff package and confirm manifest generation."
  fi
fi

if [[ "$manifest_status" == "PASS" ]]; then
  mkdir -p "$LEASE_ROOT/reports" "$LEASE_ROOT/metadata"
  cp -R "$EXTRACTED/package/evernode" "$EXTRACTED/package/runtime" "$EXTRACTED/package/node" "$EXTRACTED/package/hotpocket" "$LEASE_ROOT/" 2>/dev/null || true
  cp -R "$EXTRACTED/scripts" "$EXTRACTED/docs" "$EXTRACTED/manifests" "$EXTRACTED/checksums" "$LEASE_ROOT/" 2>/dev/null || true
  # node_status.sh is an operational read-only status helper that older handoff packages may not contain.
  if [[ ! -f "$LEASE_ROOT/scripts/node_status.sh" && -f "$ROOT/scripts/node_status.sh" ]]; then
    cp "$ROOT/scripts/node_status.sh" "$LEASE_ROOT/scripts/node_status.sh"
  fi
  chmod +x "$LEASE_ROOT/scripts"/*.sh 2>/dev/null || true
  if [[ -d "$LEASE_ROOT/runtime" && -d "$LEASE_ROOT/node" && -d "$LEASE_ROOT/hotpocket" && -d "$LEASE_ROOT/evernode" && -d "$LEASE_ROOT/scripts" ]]; then
    layout_status="PASS"
  else
    record_failure "LIVE-LEASE-DEPLOY-007" "Lease Layout" "One or more lease runtime directories were not initialized under $LEASE_ROOT" "Validation cannot execute all runtime domains." "Inspect package/package layout and copy errors."
  fi
fi

if [[ "$layout_status" == "PASS" ]]; then
  cat > "$LEASE_ROOT/metadata/live-lease-deployment.env" <<META
DEPLOYMENT_NAME=Live Evernode Lease Deployment v0.1
PACKAGE=$PACKAGE
CHECKSUM=$CHECKSUM
LEASE_ROOT=$LEASE_ROOT
DEPLOYED_AT=$started_at
MANIFEST_SUMMARY=$manifest_summary
NETWORKING=disabled
CONSENSUS=disabled
XRPL_RPC=disabled
PROTOCOL_BEHAVIOR_MODIFIED=no
META
  [[ -s "$LEASE_ROOT/metadata/live-lease-deployment.env" ]] && metadata_status="PASS"
  [[ "$metadata_status" == "PASS" ]] || record_failure "LIVE-LEASE-DEPLOY-008" "Deployment Metadata" "Metadata file was not written." "Deployment cannot be audited." "Check lease filesystem permissions."
fi

if [[ "$package_status" == PASS && "$checksum_status" == PASS && "$manifest_status" == PASS && "$extract_status" == PASS && "$layout_status" == PASS && "$metadata_status" == PASS ]]; then
  overall="PASS"
fi

if [[ -z "$failure_entries" ]]; then
  printf 'No Deployment Failures Observed\n' > "$FAILURES"
else
  printf '%s' "$failure_entries" > "$FAILURES"
fi

cat > "$REPORT" <<REPORT_BODY
Live Evernode Lease Deployment Report
Started At: $started_at
Lease Root: $LEASE_ROOT
Package: $PACKAGE
Checksum: $CHECKSUM
Automatic Package Build: $build_status
Package Exists: $package_status
Package Checksum: $checksum_status
Manifest: $manifest_status
Extract Package: $extract_status
Initialize Lease Layout: $layout_status
Deployment Metadata: $metadata_status
Protocol Behavior Modified: no
Live Lease Deployment: $overall

Manifest Summary:
$manifest_summary

Checksum Output:
$checksum_output

Build Output:
$build_output

Failure Registry:
$(cat "$FAILURES")
REPORT_BODY

printf 'Live Lease Deployment: %s\n' "$overall"
[[ "$overall" == PASS ]]
