#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
VALIDATION_REPORT="$REPORT_DIR/game_discovery_validation_report.txt"
CERTIFICATION_REPORT="$REPORT_DIR/game_discovery_certification_report.txt"

cd "$ROOT_DIR"
mkdir -p "$REPORT_DIR"
bash "$ROOT_DIR/scripts/validate_game_discovery.sh" >/tmp/everarcade_game_discovery_validation.out

status="FAIL"
if grep -q '^Game Discovery Validation: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Catalog: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Index: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Categories: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Search: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Featured: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Recommendations: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Installs: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Creators: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Civilizations: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Analytics: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Replay: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Launcher: PASS$' "$VALIDATION_REPORT" \
  && grep -q '^Replay Equals Discovery: true$' "$VALIDATION_REPORT"; then
  status="PASS"
fi

cat > "$CERTIFICATION_REPORT" <<REPORT
EverArcade Game Discovery Certification Report
Validation Report: reports/game_discovery_validation_report.txt
Catalog Evidence: game-discovery/catalog/catalog.records
Index Evidence: game-discovery/index/index.records
Category Evidence: game-discovery/categories/categories.records
Search Evidence: game-discovery/search/search.records
Featured Evidence: game-discovery/featured/featured.records
Recommendation Evidence: game-discovery/recommendations/recommendations.records
Install Evidence: game-discovery/installs/install_registry.records
Creator Discovery Evidence: game-discovery/creators/creator_discovery.records
Civilization Discovery Evidence: game-discovery/civilizations/civilization_discovery.records
Analytics Evidence: game-discovery/analytics/analytics.records
Replay Evidence: game-discovery/replay/replay.records
Launcher Evidence: game-discovery/launcher/launcher_integration.records
Metrics Evidence: game-discovery/metrics/discovery_metrics.records

Catalog: $(grep '^Catalog:' "$VALIDATION_REPORT" | awk '{print $2}')
Index: $(grep '^Index:' "$VALIDATION_REPORT" | awk '{print $2}')
Categories: $(grep '^Categories:' "$VALIDATION_REPORT" | awk '{print $2}')
Search: $(grep '^Search:' "$VALIDATION_REPORT" | awk '{print $2}')
Featured: $(grep '^Featured:' "$VALIDATION_REPORT" | awk '{print $2}')
Recommendations: $(grep '^Recommendations:' "$VALIDATION_REPORT" | awk '{print $2}')
Installs: $(grep '^Installs:' "$VALIDATION_REPORT" | awk '{print $2}')
Creators: $(grep '^Creators:' "$VALIDATION_REPORT" | awk '{print $2}')
Civilizations: $(grep '^Civilizations:' "$VALIDATION_REPORT" | awk '{print $2}')
Analytics: $(grep '^Analytics:' "$VALIDATION_REPORT" | awk '{print $2}')
Replay: $(grep '^Replay:' "$VALIDATION_REPORT" | awk '{print $2}')
Launcher: $(grep '^Launcher:' "$VALIDATION_REPORT" | awk '{print $2}')
Replay Equals Discovery: $(grep '^Replay Equals Discovery:' "$VALIDATION_REPORT" | awk '{print $4}')

Game Discovery Network v0.1: $status
REPORT

cat "$CERTIFICATION_REPORT"
[[ "$status" == "PASS" ]]
