#!/usr/bin/env bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [[ -x "$SCRIPT_DIR/../bin/everarcade-runtime" ]]; then
  APPLIANCE_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
  REPORT_DIR="$APPLIANCE_DIR/runtime/reports"
else
  ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
  APPLIANCE_DIR="${EVERARCADE_APPLIANCE_PREFIX:-$ROOT_DIR/appliance/installed}"
  REPORT_DIR="$ROOT_DIR/reports"
fi
WORLD_ID="${EVERARCADE_WORLD_ID:-civilization-alpha}"
RUNTIME_ROOT="${EVERARCADE_RUNTIME_ROOT:-$APPLIANCE_DIR/runtime}"
PACKAGE_PATH="${EVERARCADE_PACKAGE_PATH:-$RUNTIME_ROOT/worlds/$WORLD_ID/package}"
REPORT_PATH="$REPORT_DIR/runtime_appliance_validation_report.txt"
mkdir -p "$REPORT_DIR"

status_for_report() {
  local report="$1" key="${2:-Overall Result}"
  if [[ -f "$report" ]] && awk -F': ' -v key="$key" '$1 == key && $2 == "PASS" { found=1 } END { exit found ? 0 : 1 }' "$report"; then
    printf 'PASS'
  else
    printf 'FAIL'
  fi
}

vendor_status="FAIL"
release_status="FAIL"
protocol_status="FAIL"
operations_status="FAIL"

if [[ -f "$RUNTIME_ROOT/vendor/VENDOR_ARTIFACTS.txt" && -f "$PACKAGE_PATH/manifest.json" ]]; then
  vendor_status="PASS"
fi
if [[ -x "$APPLIANCE_DIR/bin/everarcade-runtime" && -f "$RUNTIME_ROOT/config/runtime.env" ]]; then
  release_status="PASS"
fi

if [[ -d "${ROOT_DIR:-}/reports" ]]; then
  protocol_report="$ROOT_DIR/reports/protocol_sovereignty_certification_report.txt"
  if [[ ! -f "$protocol_report" || "$(status_for_report "$protocol_report")" != "PASS" ]]; then
    bash "$ROOT_DIR/scripts/certify_protocol_sovereignty.sh" >/dev/null
  fi
  [[ "$(status_for_report "$protocol_report")" == "PASS" ]] && protocol_status="PASS"
else
  protocol_status="PASS"
fi

"$APPLIANCE_DIR/bin/everarcade-runtime" start "$RUNTIME_ROOT" "$WORLD_ID" "$PACKAGE_PATH" >/dev/null
"$APPLIANCE_DIR/bin/everarcade-runtime" replay-verify "$RUNTIME_ROOT" "$WORLD_ID" "$PACKAGE_PATH" >/dev/null
"$APPLIANCE_DIR/bin/everarcade-runtime" checkpoint "$RUNTIME_ROOT" "$WORLD_ID" "$PACKAGE_PATH" >/dev/null
"$APPLIANCE_DIR/bin/everarcade-runtime" stop "$RUNTIME_ROOT" "$WORLD_ID" "$PACKAGE_PATH" >/dev/null
operations_status="PASS"

overall="FAIL"
if [[ "$vendor_status" == "PASS" && "$release_status" == "PASS" && "$protocol_status" == "PASS" && "$operations_status" == "PASS" ]]; then
  overall="PASS"
fi
cat > "$REPORT_PATH" <<REPORT
Runtime Appliance Validation Report
Version: everarcade-runtime-v0.1
Vendor Artifact Certification: $vendor_status
Release Candidate Validation: $release_status
Protocol Sovereignty Certification: $protocol_status
Runtime Operations Validation: $operations_status
Runtime Appliance Validation: $overall
REPORT

if [[ "$overall" != "PASS" ]]; then
  cat "$REPORT_PATH" >&2
  exit 1
fi
cat "$REPORT_PATH"
echo "Validation: PASS"
