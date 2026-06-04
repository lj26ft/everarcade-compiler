#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

VERSION="v0.1"
NAME="everarcade-lease-handoff-$VERSION"
DIST_DIR="$ROOT/dist"
PACKAGE="${1:-$DIST_DIR/$NAME.tar.gz}"
CHECKSUM="${2:-$PACKAGE.sha256}"
REPORT_DIR="$ROOT/reports"
REPORT="$REPORT_DIR/lease_handoff_verify_report.txt"
VERIFY_ROOT="$ROOT/tmp/lease-handoff-verify"
EXTRACTED="$VERIFY_ROOT/$NAME"

mkdir -p "$REPORT_DIR"
rm -rf "$VERIFY_ROOT"
mkdir -p "$VERIFY_ROOT"

package_status="FAIL"
checksum_status="FAIL"
extract_status="FAIL"
manifest_status="FAIL"
layout_status="FAIL"
required_status="FAIL"
contents_status="FAIL"
overall="FAIL"
checksum_output=""
contents_output=""

[[ -s "$PACKAGE" ]] && package_status="PASS"
if [[ -s "$CHECKSUM" ]]; then
  if checksum_output="$(cd "$(dirname "$CHECKSUM")" && sha256sum -c "$(basename "$CHECKSUM")" 2>&1)"; then
    checksum_status="PASS"
  fi
fi

if [[ "$package_status" == PASS ]]; then
  if tar -xzf "$PACKAGE" -C "$VERIFY_ROOT"; then
    extract_status="PASS"
  fi
fi

if [[ -s "$EXTRACTED/manifests/deployment-manifest.txt" ]] \
  && grep -q 'EverArcade Evernode Lease Handoff Package v0.1' "$EXTRACTED/manifests/deployment-manifest.txt"; then
  manifest_status="PASS"
fi

layout_paths=(
  package
  docs
  scripts
  manifests
  checksums
  reports
)
required_paths=(
  package/evernode/lease/deployment-manifest.txt
  package/runtime/config
  package/runtime/deployment
  package/node/config
  package/node/state
  package/hotpocket/adapter
  package/hotpocket/README.md
  docs/operator-runbook.txt
  docs/runtime-platform/evernode-lease-handoff-v0.1.md
  manifests/deployment-manifest.txt
  manifests/evernode-deployment-manifest.txt
  checksums/package-files.sha256
  reports/README.txt
  scripts/install_lease_handoff_package.sh
  scripts/validate_lease_handoff_package.sh
)

status_for_extracted_paths() {
  local status="PASS" path
  for path in "$@"; do
    if [[ ! -e "$EXTRACTED/$path" ]]; then
      status="FAIL"
      break
    fi
  done
  printf '%s' "$status"
}

if [[ "$extract_status" == PASS ]]; then
  layout_status="$(status_for_extracted_paths "${layout_paths[@]}")"
  required_status="$(status_for_extracted_paths "${required_paths[@]}")"
  if [[ -s "$EXTRACTED/checksums/package-files.sha256" ]]; then
    if contents_output="$(cd "$EXTRACTED" && sha256sum -c checksums/package-files.sha256 2>&1)"; then
      contents_status="PASS"
    fi
  fi
fi

if [[ "$package_status" == PASS \
  && "$checksum_status" == PASS \
  && "$extract_status" == PASS \
  && "$manifest_status" == PASS \
  && "$layout_status" == PASS \
  && "$required_status" == PASS \
  && "$contents_status" == PASS ]]; then
  overall="PASS"
fi

cat > "$REPORT" <<REPORT_BODY
Evernode Lease Handoff Verification Report
Package Artifact: $package_status
Checksum: $checksum_status
Extract: $extract_status
Manifest: $manifest_status
Deployment Layout: $layout_status
Required Files: $required_status
Content Checksums: $contents_status
Package: $PACKAGE
Checksum File: $CHECKSUM
Lease Handoff Verification: $overall

Checksum Output:
$checksum_output

Content Checksum Output:
$contents_output
REPORT_BODY

echo "Lease Handoff Verification: $overall"
[[ "$overall" == PASS ]]
