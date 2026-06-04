#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

REPORT_DIR="$ROOT/reports"
REPORT="$REPORT_DIR/evernode_validation_report.txt"
mkdir -p "$REPORT_DIR"

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

layout_status="$(status_for_paths "${layout_dirs[@]}")"
manifest_status="FAIL"
protocol_status="$(status_for_paths "${protocol_scripts[@]}")"
hotpocket_validation_status="FAIL"
hotpocket_certification_status="FAIL"
node_doctor_status="FAIL"
hotpocket_run_status="FAIL"
overall="FAIL"
node_doctor_output=""
node_doctor_retry_output=""
hotpocket_validation_output=""

if [[ -s evernode/lease/deployment-manifest.txt ]]; then
  manifest_status="PASS"
fi
if [[ -x scripts/validate_hotpocket_integration.sh ]]; then
  hotpocket_validation_status="PASS"
fi
if [[ -x scripts/certify_hotpocket_integration.sh ]]; then
  hotpocket_certification_status="PASS"
fi

if node_doctor_output="$(bash scripts/node_doctor.sh 2>&1)"; then
  node_doctor_status="PASS"
fi
if hotpocket_validation_output="$(bash scripts/validate_hotpocket_integration.sh 2>&1)"; then
  hotpocket_run_status="PASS"
fi
if [[ "$node_doctor_status" != PASS ]]; then
  if node_doctor_retry_output="$(bash scripts/node_doctor.sh 2>&1)"; then
    node_doctor_status="PASS"
  fi
fi

if [[ "$layout_status" == PASS \
  && "$manifest_status" == PASS \
  && "$protocol_status" == PASS \
  && "$hotpocket_validation_status" == PASS \
  && "$hotpocket_certification_status" == PASS \
  && "$node_doctor_status" == PASS \
  && "$hotpocket_run_status" == PASS ]]; then
  overall="PASS"
fi

cat > "$REPORT" <<REPORT_BODY
Evernode Deployment Validation Report
Layout: $layout_status
Deployment Manifest: $manifest_status
Protocol Node Scripts: $protocol_status
HotPocket Validation Script: $hotpocket_validation_status
HotPocket Certification Script: $hotpocket_certification_status
Node Doctor: $node_doctor_status
HotPocket Validation Run: $hotpocket_run_status
Evernode Deployment Validation: $overall

Node Doctor Output:
$node_doctor_output

Node Doctor Retry Output:
$node_doctor_retry_output

HotPocket Validation Output:
$hotpocket_validation_output
REPORT_BODY

echo "Evernode Deployment Validation: $overall"
[[ "$overall" == PASS ]]
