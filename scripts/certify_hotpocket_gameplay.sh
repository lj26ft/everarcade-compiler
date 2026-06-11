#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
mkdir -p reports
validation_status=FAIL
if node runtime/hotpocket-gameplay-proof/validation/hotpocket-gameplay-proof.js validate >/tmp/everarcade-hotpocket-gameplay-validation.log; then
  validation_status=PASS
fi
read_status() {
  local file="$1"
  local key="$2"
  awk -F': ' -v key="$key" '$1 == key { print $2; found=1; exit } END { if (!found) print "FAIL" }' "$file" 2>/dev/null || echo FAIL
}
validation_report="reports/hotpocket_gameplay_validation_report.txt"
deployment_status="$(read_status "$validation_report" 'Contract deployment validation')"
cluster_status="$(read_status "$validation_report" 'Cluster discovery')"
gameplay_status="$(read_status "$validation_report" 'Client gameplay execution')"
agreement_status="$(read_status "$validation_report" 'Validator agreement validation')"
replay_status="$(read_status "$validation_report" 'Replay validation')"
roundtrip_status="$(awk -F': ' '/HotPocket Gameplay Round-Trip Proof/ { print $2; found=1; exit } END { if (!found) print "FAIL" }' reports/gameplay_roundtrip_report.txt 2>/dev/null || echo FAIL)"
overall=FAIL
if [[ "$validation_status" == PASS && "$deployment_status" == PASS && "$cluster_status" == PASS && "$gameplay_status" == PASS && "$agreement_status" == PASS && "$replay_status" == PASS && "$roundtrip_status" == PASS ]]; then
  overall=PASS
fi
cat > reports/hotpocket_gameplay_certification_report.txt <<REPORT
HotPocket Consensus Gameplay Proof v0.1 Certification
Deployment proof: $deployment_status
Cluster discovery proof: $cluster_status
Gameplay execution proof: $gameplay_status
Validator agreement proof: $agreement_status
Replay proof: $replay_status
Round-trip proof: $roundtrip_status
Validation command: $validation_status
HotPocket Consensus Gameplay Proof v0.1: $overall
REPORT
cat reports/hotpocket_gameplay_certification_report.txt
[[ "$overall" == PASS ]]
