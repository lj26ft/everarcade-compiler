#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/public_testnet_certification_report.txt"

cd "$ROOT_DIR"
export PUBLIC_TESTNET_ROOT="$ROOT_DIR/public-testnet"
# shellcheck source=../public-testnet/testnet_model.sh
source "$ROOT_DIR/public-testnet/testnet_model.sh"
mkdir -p "$REPORT_DIR"

public_testnet_write_artifacts

enrollment_status="FAIL"
deployments_status="FAIL"
civilizations_status="FAIL"
gpu_marketplace_status="FAIL"
settlements_status="FAIL"
governance_status="FAIL"
replay_status="FAIL"
overall_status="FAIL"

if validate_developers && validate_operators && validate_gpu_providers; then
  enrollment_status="PASS"
fi
validate_deployments && deployments_status="PASS"
validate_civilizations && civilizations_status="PASS"
validate_gpu_marketplace && gpu_marketplace_status="PASS"
validate_settlements && settlements_status="PASS"
validate_governance && governance_status="PASS"
validate_replay && replay_status="PASS"

if [[ "$enrollment_status" == "PASS" \
  && "$deployments_status" == "PASS" \
  && "$civilizations_status" == "PASS" \
  && "$gpu_marketplace_status" == "PASS" \
  && "$settlements_status" == "PASS" \
  && "$governance_status" == "PASS" \
  && "$replay_status" == "PASS" ]]; then
  overall_status="PASS"
fi

cat > "$REPORT_PATH" <<REPORT
Public Testnet Certification Report
Version: $PUBLIC_TESTNET_VERSION
Testnet ID: $PUBLIC_TESTNET_ID
Testnet Only: true
No Production Funds: $([[ "$PUBLIC_TESTNET_FUNDS" == "no-production-funds" ]] && printf 'true' || printf 'false')
Production Revenue: false
Production Asset Value: false
Auditable Records: true
Replayable Records: true
Certified Runtime Stack: protocol-runtime,civilization-runtime,federation-runtime,renderer-runtime,gpu-runtime,gpu-marketplace,xrpl-testnet,xaman-test

Enrollment: $enrollment_status
Deployments: $deployments_status
Civilizations: $civilizations_status
GPU Marketplace: $gpu_marketplace_status
Settlements: $settlements_status
Governance: $governance_status
Replay: $replay_status

Developer Enrollment Root: $(developer_enrollment_root)
Operator Enrollment Root: $(operator_enrollment_root)
GPU Enrollment Root: $(gpu_enrollment_root)
Deployment Registry Root: $(deployment_registry_root)
Civilization Registry Root: $(civilization_registry_root)
Settlement Test Root: $(settlement_test_root)
GPU Test Root: $(gpu_test_root)
Governance Root: $(governance_root)
Analytics Root: $(analytics_root)
Testnet Replay Root: $(testnet_replay_root)
Public Testnet Root: $(testnet_root)
Replay Root Equals Testnet Root: $([[ "$(testnet_replay_root)" == "$(testnet_root)" ]] && printf 'true' || printf 'false')

Public Testnet: $overall_status
REPORT

cat "$REPORT_PATH"
[[ "$overall_status" == "PASS" ]]
