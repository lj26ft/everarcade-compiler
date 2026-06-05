#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/game_discovery_validation_report.txt"

cd "$ROOT_DIR"
# shellcheck source=../game-discovery/discovery_model.sh
source "$ROOT_DIR/game-discovery/discovery_model.sh"
set +o pipefail
mkdir -p "$REPORT_DIR"
game_discovery_write_artifacts

catalog_status="FAIL"
index_status="FAIL"
categories_status="FAIL"
search_status="FAIL"
featured_status="FAIL"
recommendations_status="FAIL"
installs_status="FAIL"
creators_status="FAIL"
civilizations_status="FAIL"
analytics_status="FAIL"
replay_status="FAIL"
launcher_status="FAIL"
metrics_status="FAIL"
overall_status="FAIL"

validate_catalog && catalog_status="PASS"
validate_index && index_status="PASS"
validate_categories && categories_status="PASS"
validate_search && search_status="PASS"
validate_featured && featured_status="PASS"
validate_recommendations && recommendations_status="PASS"
validate_installs && installs_status="PASS"
validate_creators && creators_status="PASS"
validate_civilizations && civilizations_status="PASS"
validate_analytics && analytics_status="PASS"
validate_replay && replay_status="PASS"
validate_launcher && launcher_status="PASS"
validate_metrics && metrics_status="PASS"

if [[ "$catalog_status" == "PASS" \
  && "$index_status" == "PASS" \
  && "$categories_status" == "PASS" \
  && "$search_status" == "PASS" \
  && "$featured_status" == "PASS" \
  && "$recommendations_status" == "PASS" \
  && "$installs_status" == "PASS" \
  && "$creators_status" == "PASS" \
  && "$civilizations_status" == "PASS" \
  && "$analytics_status" == "PASS" \
  && "$replay_status" == "PASS" \
  && "$launcher_status" == "PASS" \
  && "$metrics_status" == "PASS" ]]; then
  overall_status="PASS"
fi

cat > "$REPORT_PATH" <<REPORT
EverArcade Game Discovery Validation Report
Version: $GAME_DISCOVERY_VERSION
Discovery ID: $GAME_DISCOVERY_ID
Scope: $GAME_DISCOVERY_SCOPE
Authority: $GAME_DISCOVERY_AUTHORITY
Ordering: $GAME_DISCOVERY_ORDERING

Catalog: $catalog_status
Index: $index_status
Categories: $categories_status
Search: $search_status
Featured: $featured_status
Recommendations: $recommendations_status
Installs: $installs_status
Creators: $creators_status
Civilizations: $civilizations_status
Analytics: $analytics_status
Replay: $replay_status
Launcher: $launcher_status
Metrics: $metrics_status

Catalog Root: $(catalog_root)
Index Root: $(index_root)
Category Root: $(categories_root)
Search Root: $(search_root)
Featured Root: $(featured_root)
Recommendation Root: $(recommendation_root)
Install Root: $(install_root)
Creator Discovery Root: $(creator_discovery_root)
Civilization Discovery Root: $(civilization_discovery_root)
Analytics Root: $(analytics_root)
Replay Root: $(replay_root)
Launcher Integration Root: $(launcher_root)
Discovery Metrics Root: $(metrics_root)
Discovery Root: $(discovery_root)
Replay Equals Discovery: $([[ "$(replay_root)" == "$(discovery_root)" ]] && printf true || printf false)

Game Discovery Validation: $overall_status
REPORT

cat "$REPORT_PATH"
[[ "$overall_status" == "PASS" ]]
