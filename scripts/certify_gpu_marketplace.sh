#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
VALIDATION_REPORT="$REPORT_DIR/gpu_marketplace_validation_report.txt"
CERTIFICATION_REPORT="$REPORT_DIR/gpu_marketplace_certification_report.txt"

cd "$ROOT_DIR"
mkdir -p "$REPORT_DIR"

bash "$ROOT_DIR/scripts/validate_gpu_marketplace.sh" >/tmp/everarcade_gpu_marketplace_validation.out

status="FAIL"
if grep -q '^GPU Marketplace Validation: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Identity: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Registration: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Capability: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Capacity: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Assignment: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Artifacts: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Verification: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Settlement Intent: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Reputation: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Replay: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Lease Integration: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Replay Equals Marketplace: true$' "$VALIDATION_REPORT"; then
  status="PASS"
fi

cat > "$CERTIFICATION_REPORT" <<REPORT
GPU Marketplace Certification Report
Validation Report: reports/gpu_marketplace_validation_report.txt
Identity Evidence: gpu/marketplace/artifacts/identity_transcript.txt
Registration Evidence: gpu/marketplace/artifacts/registration_transcript.txt
Capability Evidence: gpu/marketplace/artifacts/capability_transcript.txt
Capacity Evidence: gpu/marketplace/artifacts/capacity_transcript.txt
Assignment Evidence: gpu/marketplace/artifacts/assignment_transcript.txt
Artifact Evidence: gpu/marketplace/artifacts/artifact_submission_transcript.txt
Verification Evidence: gpu/marketplace/artifacts/verification_transcript.txt
Settlement Intent Evidence: gpu/marketplace/artifacts/settlement_intent_transcript.txt
Reputation Evidence: gpu/marketplace/artifacts/reputation_transcript.txt
Replay Evidence: gpu/marketplace/artifacts/marketplace_replay_transcript.txt
Lease Integration Evidence: runtime/gpu-marketplace/artifacts/lease_integration_transcript.txt

GPU Marketplace: $status
REPORT

cat "$CERTIFICATION_REPORT"
[[ "$status" == "PASS" ]]
