#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT/reports"; CERT="$REPORT_DIR/evernode_live_gameplay_certification_report.txt"; mkdir -p "$REPORT_DIR"
run_section(){ local name="$1"; shift; if "$@" >/tmp/everarcade-live-proof-section.log 2>&1; then echo "$name: PASS"; else cat /tmp/everarcade-live-proof-section.log >&2; echo "$name: FAIL"; return 1; fi; }
status=0
ENV_LINE=$(run_section "Environment Probe" bash "$ROOT/scripts/probe_evernode_environment.sh") || status=1
PKG_LINE=$(run_section "Package Layout" bash "$ROOT/scripts/build_live_gameplay_proof_package.sh") || status=1
HP_LINE=$(run_section "HotPocket Boundary" bash "$ROOT/scripts/validate_hotpocket_runtime_boundary.sh") || status=1
ARENA_LINE=$(run_section "Arena Session" bash "$ROOT/scripts/start_live_arena_session.sh") || status=1
FRONT_LINE=$(run_section "Frontend Access" bash "$ROOT/scripts/validate_frontend_access.sh") || status=1
RES_LINE=$(run_section "Resource Monitoring" bash "$ROOT/scripts/monitor_live_session_resources.sh") || status=1
REC_LINE=$(run_section "Recovery" bash "$ROOT/scripts/validate_live_session_recovery.sh") || status=1
NET_LINE=$(run_section "Network Failure Probe" bash "$ROOT/scripts/probe_network_failure_behavior.sh") || status=1
replay=FAIL; grep -q '^Replay Verification: PASS$' "$REPORT_DIR/evernode_live_gameplay_validation_report.txt" && replay=PASS
runtime=FAIL; grep -q '^Runtime Start: PASS$' "$REPORT_DIR/evernode_live_gameplay_validation_report.txt" && runtime=PASS
overall=FAIL
if [[ $status -eq 0 && "$replay" == PASS && "$runtime" == PASS ]]; then overall=PASS; fi
cat > "$CERT" <<REPORT_BODY
Evernode Live Gameplay Deployment Proof Certification Report v0.1
$ENV_LINE
$PKG_LINE
$HP_LINE
Runtime Start: $runtime
$ARENA_LINE
$FRONT_LINE
$RES_LINE
$REC_LINE
$NET_LINE
Replay Verification: $replay
Evernode Live Gameplay Deployment Proof v0.1: $overall
REPORT_BODY
cat "$CERT"
[[ "$overall" == PASS ]]
