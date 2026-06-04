#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/public_testnet_validation_report.txt"

cd "$ROOT_DIR"
export PUBLIC_TESTNET_ROOT="$ROOT_DIR/public-testnet"
# shellcheck source=../public-testnet/testnet_model.sh
source "$ROOT_DIR/public-testnet/testnet_model.sh"
mkdir -p "$REPORT_DIR"

public_testnet_write_artifacts

developers_status="FAIL"
operators_status="FAIL"
gpu_providers_status="FAIL"
deployments_status="FAIL"
civilizations_status="FAIL"
settlements_status="FAIL"
gpu_marketplace_status="FAIL"
governance_status="FAIL"
analytics_status="FAIL"
replay_status="FAIL"
overall_status="FAIL"

validate_developers && developers_status="PASS"
validate_operators && operators_status="PASS"
validate_gpu_providers && gpu_providers_status="PASS"
validate_deployments && deployments_status="PASS"
validate_civilizations && civilizations_status="PASS"
validate_settlements && settlements_status="PASS"
validate_gpu_marketplace && gpu_marketplace_status="PASS"
validate_governance && governance_status="PASS"
validate_analytics_public_testnet && analytics_status="PASS"
validate_replay && replay_status="PASS"

if [[ "$developers_status" == "PASS" \
  && "$operators_status" == "PASS" \
  && "$gpu_providers_status" == "PASS" \
  && "$deployments_status" == "PASS" \
  && "$civilizations_status" == "PASS" \
  && "$settlements_status" == "PASS" \
  && "$gpu_marketplace_status" == "PASS" \
  && "$governance_status" == "PASS" \
  && "$analytics_status" == "PASS" \
  && "$replay_status" == "PASS" ]]; then
  overall_status="PASS"
fi

cat > "$REPORT_PATH" <<REPORT
Public Testnet Validation Report
Version: $PUBLIC_TESTNET_VERSION
Testnet ID: $PUBLIC_TESTNET_ID
Network: $PUBLIC_TESTNET_NETWORK
Funds: $PUBLIC_TESTNET_FUNDS
Ordering: $PUBLIC_TESTNET_ORDERING
Authority: $PUBLIC_TESTNET_AUTHORITY

Developers: $developers_status
Operators: $operators_status
GPU Providers: $gpu_providers_status
Deployments: $deployments_status
Civilizations: $civilizations_status
Settlements: $settlements_status
GPU Marketplace: $gpu_marketplace_status
Governance: $governance_status
Analytics: $analytics_status
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

Public Testnet Validation: $overall_status
REPORT

cat "$REPORT_PATH"
[[ "$overall_status" == "PASS" ]]
