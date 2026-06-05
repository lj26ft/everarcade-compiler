#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/creator_marketplace_validation_report.txt"

cd "$ROOT_DIR"
# shellcheck source=../creator-marketplace/marketplace_model.sh
source "$ROOT_DIR/creator-marketplace/marketplace_model.sh"
set +o pipefail
mkdir -p "$REPORT_DIR"
creator_marketplace_write_artifacts

creators_status="FAIL"
publications_status="FAIL"
templates_status="FAIL"
assets_status="FAIL"
modules_status="FAIL"
licenses_status="FAIL"
royalties_status="FAIL"
usage_status="FAIL"
discovery_status="FAIL"
analytics_status="FAIL"
replay_status="FAIL"
portal_status="FAIL"
metrics_status="FAIL"
overall_status="FAIL"

validate_creators && creators_status="PASS"
validate_publications && publications_status="PASS"
validate_templates && templates_status="PASS"
validate_assets && assets_status="PASS"
validate_modules && modules_status="PASS"
validate_licenses && licenses_status="PASS"
validate_royalties && royalties_status="PASS"
validate_usage && usage_status="PASS"
validate_discovery && discovery_status="PASS"
validate_analytics && analytics_status="PASS"
validate_replay && replay_status="PASS"
validate_portal_integration && portal_status="PASS"
validate_metrics && metrics_status="PASS"

if [[ "$creators_status" == "PASS" \
  && "$publications_status" == "PASS" \
  && "$templates_status" == "PASS" \
  && "$assets_status" == "PASS" \
  && "$modules_status" == "PASS" \
  && "$licenses_status" == "PASS" \
  && "$royalties_status" == "PASS" \
  && "$usage_status" == "PASS" \
  && "$discovery_status" == "PASS" \
  && "$analytics_status" == "PASS" \
  && "$replay_status" == "PASS" \
  && "$portal_status" == "PASS" \
  && "$metrics_status" == "PASS" ]]; then
  overall_status="PASS"
fi

cat > "$REPORT_PATH" <<REPORT
Creator Marketplace Validation Report
Version: $CREATOR_MARKETPLACE_VERSION
Marketplace ID: $CREATOR_MARKETPLACE_ID
Scope: $CREATOR_MARKETPLACE_SCOPE
Custody: $CREATOR_MARKETPLACE_CUSTODY
Payments: $CREATOR_MARKETPLACE_PAYMENTS
Ordering: $CREATOR_MARKETPLACE_ORDERING

Creators: $creators_status
Publications: $publications_status
Templates: $templates_status
Assets: $assets_status
Modules: $modules_status
Licenses: $licenses_status
Royalties: $royalties_status
Usage: $usage_status
Discovery: $discovery_status
Analytics: $analytics_status
Replay: $replay_status
Portal Integration: $portal_status
Metrics: $metrics_status

Creator Registry Root: $(creator_registry_root)
Publication Registry Root: $(publication_registry_root)
Template Marketplace Root: $(template_marketplace_root)
Asset Marketplace Root: $(asset_marketplace_root)
Module Marketplace Root: $(module_marketplace_root)
License Root: $(license_root)
Royalty Root: $(royalty_root)
Usage Root: $(usage_root)
Discovery Root: $(discovery_root)
Analytics Root: $(analytics_root)
Marketplace Replay Root: $(marketplace_replay_root)
Portal Integration Root: $(portal_integration_root)
Creator Marketplace Metrics Root: $(metrics_root)
Marketplace Root: $(marketplace_root)
Replay Equals Marketplace: $([[ "$(marketplace_replay_root)" == "$(marketplace_root)" ]] && printf true || printf false)

Creator Marketplace Validation: $overall_status
REPORT

cat "$REPORT_PATH"
[[ "$overall_status" == "PASS" ]]
