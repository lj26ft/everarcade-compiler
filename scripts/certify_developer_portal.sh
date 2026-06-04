#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/developer_portal_certification_report.txt"

cd "$ROOT_DIR"
export DEVELOPER_PORTAL_ROOT="$ROOT_DIR/developer-portal"
# shellcheck source=../developer-portal/portal_model.sh
source "$ROOT_DIR/developer-portal/portal_model.sh"
mkdir -p "$REPORT_DIR"

portal_write_artifacts

projects_status="FAIL"
games_status="FAIL"
deployments_status="FAIL"
assets_status="FAIL"
wallets_status="FAIL"
civilizations_status="FAIL"
marketplace_status="FAIL"
gpu_status="FAIL"
analytics_status="FAIL"
onboarding_status="FAIL"
overall_status="FAIL"

validate_projects && projects_status="PASS"
validate_game_creation && games_status="PASS"
validate_deployments && deployments_status="PASS"
validate_assets && assets_status="PASS"
validate_wallets && wallets_status="PASS"
validate_civilizations && civilizations_status="PASS"
validate_marketplace && marketplace_status="PASS"
validate_gpu && gpu_status="PASS"
validate_analytics && analytics_status="PASS"
validate_onboarding && onboarding_status="PASS"

if [[ "$projects_status" == "PASS" \
  && "$games_status" == "PASS" \
  && "$deployments_status" == "PASS" \
  && "$assets_status" == "PASS" \
  && "$wallets_status" == "PASS" \
  && "$civilizations_status" == "PASS" \
  && "$marketplace_status" == "PASS" \
  && "$gpu_status" == "PASS" \
  && "$analytics_status" == "PASS" \
  && "$onboarding_status" == "PASS" ]]; then
  overall_status="PASS"
fi

cat > "$REPORT_PATH" <<REPORT
Developer Portal Certification Report
Version: $DEVELOPER_PORTAL_VERSION
Portal ID: $DEVELOPER_PORTAL_ID
Non-Authoritative: true
Replayable Records: true
No Private Key Custody: $([[ "$DEVELOPER_PORTAL_PRIVATE_KEY_CUSTODY" == "false" ]] && printf 'true' || printf 'false')
Settlement Bypass: false
Authority Bypass: false

Projects: $projects_status
Games: $games_status
Deployments: $deployments_status
Assets: $assets_status
Wallets: $wallets_status
Civilizations: $civilizations_status
Marketplace: $marketplace_status
GPU: $gpu_status
Analytics: $analytics_status
Onboarding: $onboarding_status

Project Registry Root: $(project_registry_root)
Game Creation Root: $(game_creation_root)
Deployment Root: $(deployment_root)
Asset Registry Root: $(asset_registry_root)
Wallet Root: $(wallet_root)
Civilization Root: $(civilization_root)
Marketplace Root: $(marketplace_root)
GPU Marketplace Root: $(gpu_marketplace_root)
Analytics Root: $(analytics_root)
Onboarding Root: $(onboarding_root)
Developer Portal Root: $(portal_root)

Developer Portal: $overall_status
REPORT

cat "$REPORT_PATH"
[[ "$overall_status" == "PASS" ]]
