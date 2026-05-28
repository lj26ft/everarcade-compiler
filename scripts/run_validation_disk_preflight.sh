#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
mkdir -p runtime/logs deployment/reports
MIN_MB="${VALIDATION_MIN_FREE_MB:-1024}"
avail_mb="$(df -Pm "$ROOT" | awk 'NR==2 {print $4}')"
target_mb="$(du -sm target 2>/dev/null | awk '{print $1}' || echo 0)"
vendor_mb="$(du -sm vendor 2>/dev/null | awk '{print $1}' || echo 0)"
log="runtime/logs/validation_disk_preflight.log"
{
  echo "validation_disk_preflight=started"
  echo "root=$ROOT"
  echo "available_mb=$avail_mb"
  echo "minimum_required_mb=$MIN_MB"
  echo "target_mb=$target_mb"
  echo "vendor_mb=$vendor_mb"
} | tee "$log"
cat > deployment/reports/runtime_storage_pressure_report.md <<RPT
# Runtime Storage Pressure Report
- disk_preflight: enabled
- minimum_required_mb: ${MIN_MB}
- available_mb: ${avail_mb}
- target_mb: ${target_mb}
- vendor_mb: ${vendor_mb}
- cleanup_script: scripts/run_validation_cleanup.sh
- low_resource_validation: scripts/run_low_resource_workspace_validation.sh
RPT
if (( avail_mb < MIN_MB )); then
  echo "validation_disk_preflight=failed low_disk available_mb=${avail_mb} required_mb=${MIN_MB}" | tee -a "$log" >&2
  exit 1
fi
echo "validation_disk_preflight=ok" | tee -a "$log"
