#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

VERSION="v0.1"
NAME="everarcade-lease-handoff-$VERSION"
INSTALL_ROOT="${1:-$ROOT/handoff/lease-install}"
INSTALLED="$INSTALL_ROOT/$NAME"
REPORT_DIR="$ROOT/reports"
REPORT="$REPORT_DIR/lease_handoff_validation_report.txt"
mkdir -p "$REPORT_DIR"

status_for_paths() {
  local status="PASS" path
  for path in "$@"; do
    if [[ ! -e "$INSTALLED/$path" ]]; then
      status="FAIL"
      break
    fi
  done
  printf '%s' "$status"
}

runtime_paths=(
  package/runtime/config
  package/runtime/deployment
  scripts/runtime_doctor.sh
  scripts/runtime_start.sh
  scripts/runtime_stop.sh
  scripts/runtime_status.sh
)
node_paths=(
  package/node/config
  package/node/state
  scripts/node_common.sh
  scripts/node_doctor.sh
  scripts/node_start.sh
  scripts/node_stop.sh
)
hotpocket_paths=(
  package/hotpocket/adapter
  package/hotpocket/input
  package/hotpocket/output
  package/hotpocket/checkpoint
  package/hotpocket/replay
  package/hotpocket/settlement
  scripts/validate_hotpocket_integration.sh
  scripts/certify_hotpocket_integration.sh
)
evernode_paths=(
  package/evernode/lease/deployment-manifest.txt
  package/evernode/runtime
  package/evernode/node
  package/evernode/hotpocket
  scripts/evernode_bootstrap.sh
  scripts/evernode_start.sh
  scripts/evernode_stop.sh
  scripts/validate_evernode_deployment.sh
  scripts/certify_evernode_deployment.sh
)
report_paths=(
  reports
  reports/README.txt
)
documentation_paths=(
  docs/operator-runbook.txt
  docs/runtime-platform/runtime-appliance-v0.1.md
  docs/runtime-platform/protocol-node-appliance-v0.1.md
  docs/runtime-platform/hotpocket-integration-v0.1.md
  docs/runtime-platform/evernode-deployment-v0.1.md
  docs/runtime-platform/evernode-lease-handoff-v0.1.md
)
manifest_paths=(
  manifests/deployment-manifest.txt
  manifests/evernode-deployment-manifest.txt
  checksums/package-files.sha256
)

installed_status="FAIL"
manifest_status="FAIL"
runtime_status="FAIL"
node_status="FAIL"
hotpocket_status="FAIL"
evernode_status="FAIL"
reports_status="FAIL"
documentation_status="FAIL"
checksums_status="FAIL"
overall="FAIL"
checksum_output=""

if [[ -d "$INSTALLED" ]]; then
  installed_status="PASS"
  manifest_status="$(status_for_paths "${manifest_paths[@]}")"
  runtime_status="$(status_for_paths "${runtime_paths[@]}")"
  node_status="$(status_for_paths "${node_paths[@]}")"
  hotpocket_status="$(status_for_paths "${hotpocket_paths[@]}")"
  evernode_status="$(status_for_paths "${evernode_paths[@]}")"
  reports_status="$(status_for_paths "${report_paths[@]}")"
  documentation_status="$(status_for_paths "${documentation_paths[@]}")"
  if [[ -s "$INSTALLED/checksums/package-files.sha256" ]]; then
    if checksum_output="$(cd "$INSTALLED" && sha256sum -c checksums/package-files.sha256 2>&1)"; then
      checksums_status="PASS"
    fi
  fi
fi

if [[ "$installed_status" == PASS \
  && "$manifest_status" == PASS \
  && "$runtime_status" == PASS \
  && "$node_status" == PASS \
  && "$hotpocket_status" == PASS \
  && "$evernode_status" == PASS \
  && "$reports_status" == PASS \
  && "$documentation_status" == PASS \
  && "$checksums_status" == PASS ]]; then
  overall="PASS"
fi

cat > "$REPORT" <<REPORT_BODY
Evernode Lease Handoff Validation Report
Installed Package: $installed_status
Manifest and Checksums: $manifest_status
Runtime Assets: $runtime_status
Node Assets: $node_status
HotPocket Assets: $hotpocket_status
Evernode Assets: $evernode_status
Reports: $reports_status
Documentation: $documentation_status
Content Checksum Run: $checksums_status
Install Target: $INSTALLED
Lease Handoff Validation: $overall

Content Checksum Output:
$checksum_output
REPORT_BODY

echo "Lease Handoff Validation: $overall"
[[ "$overall" == PASS ]]
