#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/gpu_marketplace_validation_report.txt"

cd "$ROOT_DIR"
export GPU_MODEL_ROOT="$ROOT_DIR"
export GPU_MARKETPLACE_ROOT="$ROOT_DIR"
# shellcheck source=../gpu/marketplace/marketplace_model.sh
source "$ROOT_DIR/gpu/marketplace/marketplace_model.sh"

export GPU_PROJECTION_ROOT="$(projection_root)"
export GPU_JOB_ROOT="$(gpu_job_root)"
export MARKETPLACE_CAPABILITY_ROOT="$(capability_root)"
export MARKETPLACE_IDENTITY_ROOT="$(provider_identity_root)"
export MARKETPLACE_REGISTRATION_ROOT="$(registration_root)"
export MARKETPLACE_CAPACITY_ROOT="$(capacity_root)"
export MARKETPLACE_ASSIGNMENT_ROOT="$(assignment_root)"
export MARKETPLACE_ARTIFACT_ROOT="$(artifact_submission_root)"
export MARKETPLACE_VERIFICATION_ROOT="$(verification_root)"
export MARKETPLACE_SETTLEMENT_ROOT="$(settlement_intent_root)"
export MARKETPLACE_REPUTATION_ROOT="$(reputation_root)"
export MARKETPLACE_REPLAY_ROOT="$(marketplace_replay_root)"
export MARKETPLACE_ROOT_HASH="$(marketplace_root)"
export MARKETPLACE_LEASE_INTEGRATION_ROOT="$(lease_integration_root)"
mkdir -p "$REPORT_DIR"

gpu_marketplace_write_artifacts

identity_status="FAIL"
registration_status="FAIL"
capability_status="FAIL"
capacity_status="FAIL"
assignment_status="FAIL"
artifacts_status="FAIL"
verification_status="FAIL"
settlement_status="FAIL"
reputation_status="FAIL"
replay_status="FAIL"
lease_integration_status="FAIL"
overall_status="FAIL"

validate_identity && identity_status="PASS"
validate_registration && registration_status="PASS"
validate_capability && capability_status="PASS"
validate_capacity && capacity_status="PASS"
validate_assignment && assignment_status="PASS"
validate_artifacts && artifacts_status="PASS"
validate_verification && verification_status="PASS"
validate_settlement_intent && settlement_status="PASS"
validate_reputation && reputation_status="PASS"
validate_marketplace_replay && replay_status="PASS"
validate_lease_integration && lease_integration_status="PASS"

if [[ "$identity_status" == "PASS" \
  && "$registration_status" == "PASS" \
  && "$capability_status" == "PASS" \
  && "$capacity_status" == "PASS" \
  && "$assignment_status" == "PASS" \
  && "$artifacts_status" == "PASS" \
  && "$verification_status" == "PASS" \
  && "$settlement_status" == "PASS" \
  && "$reputation_status" == "PASS" \
  && "$replay_status" == "PASS" \
  && "$lease_integration_status" == "PASS" ]]; then
  overall_status="PASS"
fi

cat > "$REPORT_PATH" <<REPORT
GPU Marketplace Validation Report
Version: $GPU_MARKETPLACE_VERSION
Marketplace ID: $GPU_MARKETPLACE_ID
Authority: $GPU_MARKETPLACE_AUTHORITY
Canonical Ordering: $GPU_MARKETPLACE_CANONICAL_ORDERING
Replay Safe: $GPU_MARKETPLACE_REPLAY_SAFE
Live Discovery: $GPU_MARKETPLACE_LIVE_DISCOVERY
Payments: $GPU_MARKETPLACE_PAYMENTS

Identity: $identity_status
Registration: $registration_status
Capability: $capability_status
Capacity: $capacity_status
Assignment: $assignment_status
Artifacts: $artifacts_status
Verification: $verification_status
Settlement Intent: $settlement_status
Reputation: $reputation_status
Replay: $replay_status
Lease Integration: $lease_integration_status

Provider Identity Root: $(provider_identity_root)
Registration Root: $(registration_root)
Capability Root: $(capability_root)
Capacity Root: $(capacity_root)
Assignment Root: $(assignment_root)
Artifact Submission Root: $(artifact_submission_root)
Verification Root: $(verification_root)
Settlement Intent Root: $(settlement_intent_root)
Reputation Root: $(reputation_root)
Marketplace Root: $(marketplace_root)
Marketplace Replay Root: $(marketplace_replay_root)
Replay Equals Marketplace: $(if [[ "$(marketplace_replay_root)" == "$(marketplace_root)" ]]; then printf 'true'; else printf 'false'; fi)
Lease Integration Root: $(lease_integration_root)

GPU Marketplace Validation: $overall_status
REPORT

cat "$REPORT_PATH"
[[ "$overall_status" == "PASS" ]]
