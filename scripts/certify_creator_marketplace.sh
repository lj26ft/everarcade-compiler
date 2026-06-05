#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
VALIDATION_REPORT="$REPORT_DIR/creator_marketplace_validation_report.txt"
CERTIFICATION_REPORT="$REPORT_DIR/creator_marketplace_certification_report.txt"

cd "$ROOT_DIR"
mkdir -p "$REPORT_DIR"
bash "$ROOT_DIR/scripts/validate_creator_marketplace.sh" >/tmp/everarcade_creator_marketplace_validation.out

status="FAIL"
if grep -q '^Creator Marketplace Validation: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Creators: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Publications: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Templates: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Assets: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Modules: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Licenses: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Royalties: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Usage: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Discovery: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Analytics: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Replay: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Portal Integration: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Replay Equals Marketplace: true$' "$VALIDATION_REPORT"; then
  status="PASS"
fi

cat > "$CERTIFICATION_REPORT" <<REPORT
Creator Marketplace Certification Report
Validation Report: reports/creator_marketplace_validation_report.txt
Creator Registry Evidence: creator-marketplace/creators/creator_registry.records
Publication Registry Evidence: creator-marketplace/publications/publication_registry.records
Template Marketplace Evidence: creator-marketplace/templates/template_marketplace.records
Asset Marketplace Evidence: creator-marketplace/assets/asset_marketplace.records
Module Marketplace Evidence: creator-marketplace/modules/module_marketplace.records
License Evidence: creator-marketplace/licenses/license_layer.records
Royalty Evidence: creator-marketplace/royalties/royalty_layer.records
Usage Evidence: creator-marketplace/usage/usage_tracking.records
Discovery Evidence: creator-marketplace/discovery/discovery_layer.records
Analytics Evidence: creator-marketplace/analytics/creator_analytics.records
Replay Evidence: creator-marketplace/replay/marketplace_replay.records
Portal Integration Evidence: creator-marketplace/portal/portal_integration.records
Metrics Evidence: creator-marketplace/metrics/creator_success_metrics.records

Creators: $(grep '^Creators:' "$VALIDATION_REPORT" | awk '{print $2}')
Publications: $(grep '^Publications:' "$VALIDATION_REPORT" | awk '{print $2}')
Templates: $(grep '^Templates:' "$VALIDATION_REPORT" | awk '{print $2}')
Assets: $(grep '^Assets:' "$VALIDATION_REPORT" | awk '{print $2}')
Modules: $(grep '^Modules:' "$VALIDATION_REPORT" | awk '{print $2}')
Licenses: $(grep '^Licenses:' "$VALIDATION_REPORT" | awk '{print $2}')
Royalties: $(grep '^Royalties:' "$VALIDATION_REPORT" | awk '{print $2}')
Usage: $(grep '^Usage:' "$VALIDATION_REPORT" | awk '{print $2}')
Discovery: $(grep '^Discovery:' "$VALIDATION_REPORT" | awk '{print $2}')
Analytics: $(grep '^Analytics:' "$VALIDATION_REPORT" | awk '{print $2}')
Replay: $(grep '^Replay:' "$VALIDATION_REPORT" | awk '{print $2}')
Portal Integration: $(grep '^Portal Integration:' "$VALIDATION_REPORT" | awk '{print $3}')
Replay Equals Marketplace: $(grep '^Replay Equals Marketplace:' "$VALIDATION_REPORT" | awk '{print $4}')

Creator Marketplace v0.1: $status
REPORT

cat "$CERTIFICATION_REPORT"
[[ "$status" == "PASS" ]]
