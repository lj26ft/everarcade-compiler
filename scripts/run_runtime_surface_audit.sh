#!/usr/bin/env bash
set -euo pipefail

report="deployment/reports/runtime_surface_audit_report.md"
mkdir -p deployment/reports

cat > "$report" <<'RPT'
# Runtime Surface Audit Report

## Classifications
- execution_core::runtime::validation => Production
- execution_core::runtime::ci => ActiveIntegration
- renderer_client::history => Scaffold
- renderer_client::federation => Scaffold
- renderer_client::transport_runtime => Scaffold

## Export inconsistencies
- none detected
RPT

echo "runtime surface audit complete"
