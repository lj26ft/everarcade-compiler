#!/usr/bin/env bash
set -u -o pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
REPORT_DIR="$ROOT/reports"
DEPLOY_REPORT="$REPORT_DIR/live_lease_deployment_report.txt"
VALIDATION_REPORT="$REPORT_DIR/live_lease_validation_report.txt"
CERT_REPORT="$REPORT_DIR/live_lease_certification_report.txt"
FAILURES="$REPORT_DIR/live_lease_failures.txt"
mkdir -p "$REPORT_DIR"

deployment="FAIL"; validation="FAIL"; start="FAIL"; health="FAIL"; checkpoint="FAIL"; replay="FAIL"; restore="FAIL"; stop="FAIL"; roots="FAIL"; overall="FAIL"
[[ -s "$DEPLOY_REPORT" ]] && grep -q '^Live Lease Deployment: PASS$' "$DEPLOY_REPORT" && deployment="PASS"
[[ -s "$VALIDATION_REPORT" ]] && grep -q '^Live Lease Validation: PASS$' "$VALIDATION_REPORT" && validation="PASS"
[[ -s "$VALIDATION_REPORT" ]] && grep -q '^Start: PASS$' "$VALIDATION_REPORT" && start="PASS"
[[ -s "$VALIDATION_REPORT" ]] && grep -q '^Health: PASS$' "$VALIDATION_REPORT" && health="PASS"
[[ -s "$VALIDATION_REPORT" ]] && grep -q '^Checkpoint: PASS$' "$VALIDATION_REPORT" && checkpoint="PASS"
[[ -s "$VALIDATION_REPORT" ]] && grep -q '^Replay: PASS$' "$VALIDATION_REPORT" && replay="PASS"
[[ -s "$VALIDATION_REPORT" ]] && grep -q '^Restore: PASS$' "$VALIDATION_REPORT" && restore="PASS"
[[ -s "$VALIDATION_REPORT" ]] && grep -q '^Stop: PASS$' "$VALIDATION_REPORT" && stop="PASS"
[[ -s "$VALIDATION_REPORT" ]] && grep -q '^Replay Continuity Root Match: PASS$' "$VALIDATION_REPORT" && roots="PASS"

if [[ "$deployment" == PASS && "$validation" == PASS && "$start" == PASS && "$health" == PASS && "$checkpoint" == PASS && "$replay" == PASS && "$restore" == PASS && "$stop" == PASS && "$roots" == PASS ]]; then
  overall="PASS"
fi

cat > "$CERT_REPORT" <<REPORT_BODY
Live Evernode Lease Deployment Certification Report
Deployment: $deployment
Validation: $validation
Runtime Start: $start
Health: $health
Checkpoint: $checkpoint
Replay: $replay
Restore: $restore
Stop: $stop
Replay Continuity Root Match: $roots
Live Evernode Lease Deployment: $overall

Deployment Report: $DEPLOY_REPORT
Validation Report: $VALIDATION_REPORT
Failure Registry:
$(if [[ -s "$FAILURES" ]]; then cat "$FAILURES"; else printf 'No Deployment Failures Observed\n'; fi)
REPORT_BODY

printf 'Live Evernode Lease Deployment: %s\n' "$overall"
[[ "$overall" == PASS ]]
