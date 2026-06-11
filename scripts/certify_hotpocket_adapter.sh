#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
mkdir -p reports
status=FAIL
if bash scripts/validate_hotpocket_adapter.sh >/tmp/everarcade-hotpocket-adapter-validation.log; then status=PASS; fi
cat > reports/hotpocket_adapter_certification_report.txt <<RPT
HotPocket Contract Adapter Proof v0.1 Certification
Execution: $(awk -F': ' '/Execution Proof/ {print $2}' reports/hotpocket_adapter_validation_report.txt 2>/dev/null || echo FAIL)
State Mutation: $(awk -F': ' '/State Mutation Proof/ {print $2}' reports/hotpocket_adapter_validation_report.txt 2>/dev/null || echo FAIL)
Replay: $(awk -F': ' '/Replay Proof/ {print $2}' reports/hotpocket_adapter_validation_report.txt 2>/dev/null || echo FAIL)
Package Deployment: $(awk -F': ' '/Package Deployment Proof/ {print $2}' reports/hotpocket_adapter_validation_report.txt 2>/dev/null || echo FAIL)
Consensus: $(awk -F': ' '/Consensus Proof/ {print $2}' reports/hotpocket_adapter_validation_report.txt 2>/dev/null || echo FAIL)
Explicit Non-Claims: Evernode lease deployment, WAN federation, XRPL settlement, Xahau settlement, civilization hosting, multiplayer scale, production economics
HotPocket Contract Adapter Proof v0.1: $status
RPT
cat reports/hotpocket_adapter_certification_report.txt
[[ "$status" == PASS ]]
