#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

REPORT_DIR="$ROOT/reports"
REPORT="$REPORT_DIR/evernode_bootstrap_report.txt"
MANIFEST="$ROOT/evernode/lease/deployment-manifest.txt"

layout_dirs=(
  evernode/lease
  evernode/runtime
  evernode/node
  evernode/hotpocket
  evernode/checkpoints
  evernode/journals
  evernode/backups
  evernode/logs
  evernode/reports
)

protocol_scripts=(
  scripts/node_init.sh
  scripts/node_start.sh
  scripts/node_checkpoint.sh
  scripts/node_replay.sh
  scripts/node_restore.sh
  scripts/node_doctor.sh
)

hotpocket_scripts=(
  hotpocket/adapter/hotpocket_adapter.sh
  scripts/validate_hotpocket_integration.sh
  scripts/certify_hotpocket_integration.sh
)

status_for_paths() {
  local status="PASS"
  local path
  for path in "$@"; do
    if [[ ! -e "$path" ]]; then
      status="FAIL"
      break
    fi
  done
  printf '%s' "$status"
}

mkdir -p "$REPORT_DIR"
mkdir -p "${layout_dirs[@]}"
for dir in "${layout_dirs[@]}"; do
  touch "$dir/.gitkeep"
done

layout_status="$(status_for_paths "${layout_dirs[@]}")"
protocol_status="$(status_for_paths "${protocol_scripts[@]}")"
hotpocket_status="$(status_for_paths "${hotpocket_scripts[@]}")"
manifest_status="FAIL"
overall="FAIL"

cat > "$MANIFEST" <<MANIFEST_BODY
Evernode Deployment Manifest v0.1
Layer: Evernode-Compatible Deployment Layout
Protocol Node Appliance: required
HotPocket Integration Layer: required
Live Evernode Registration: disabled
Lease Upload: disabled
Networking: disabled
Consensus: disabled
XRPL RPC: disabled
Xaman: disabled
Hooks: disabled
MANIFEST_BODY

if [[ -s "$MANIFEST" ]]; then
  manifest_status="PASS"
fi

if [[ "$layout_status" == PASS && "$protocol_status" == PASS && "$hotpocket_status" == PASS && "$manifest_status" == PASS ]]; then
  overall="PASS"
fi

cat > "$REPORT" <<REPORT_BODY
Evernode Bootstrap Report
Layout: $layout_status
Protocol Node Scripts: $protocol_status
HotPocket Adapter Scripts: $hotpocket_status
Deployment Manifest: $manifest_status
Evernode Bootstrap: $overall
REPORT_BODY

echo "Evernode Bootstrap: $overall"
[[ "$overall" == PASS ]]
