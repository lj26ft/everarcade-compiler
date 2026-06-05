#!/usr/bin/env bash
# EverArcade Game Discovery Network v0.1 deterministic model.
# Non-authoritative: records index, rank, install, launch, and observe games
# without modifying runtime state, bypassing settlement, or owning games.

set -euo pipefail

GAME_DISCOVERY_VERSION="0.1"
GAME_DISCOVERY_ID="everarcade-game-discovery-network-v0.1"
GAME_DISCOVERY_SCOPE="player-discovery"
GAME_DISCOVERY_AUTHORITY="non-authoritative"
GAME_DISCOVERY_ORDERING="game-id-version-event-lexicographic"

sha256_text() { sha256sum | awk '{print $1}'; }
validate_hash() { [[ "$1" =~ ^[0-9a-f]{64}$ ]]; }
stable_root() { printf '%s|%s|%s\n' "$GAME_DISCOVERY_ID" "$GAME_DISCOVERY_VERSION" "$1" | sha256_text; }

discovery_common_transcript() {
  printf 'discovery_id=%s\n' "$GAME_DISCOVERY_ID"
  printf 'version=%s\n' "$GAME_DISCOVERY_VERSION"
  printf 'scope=%s\n' "$GAME_DISCOVERY_SCOPE"
  printf 'authority=%s\n' "$GAME_DISCOVERY_AUTHORITY"
  printf 'ordering=%s\n' "$GAME_DISCOVERY_ORDERING"
  printf 'runtime_state_mutation=false\n'
  printf 'settlement_bypass=false\n'
  printf 'game_modification=false\n'
}

GAME_IDS=(arena-vanguard rpg-questline trading-post civilization-seed sandbox-builder strategy-frontier action-runner adventure-isles)
CREATOR_IDS=(creator-alpha creator-beta creator-alpha creator-gamma creator-delta creator-gamma creator-beta creator-delta)
GAME_VERSIONS=(1.0.0 1.0.0 0.9.0 1.0.0 0.8.0 1.0.0 0.7.0 1.0.0)
GAME_STATUS=(published published updated deprecated archived published published published)
GAME_CATEGORIES=(Arena RPG Trading Civilization Sandbox Strategy Action Adventure)
GAME_TAGS=(combat,runtime,pvp quests,narrative,party economy,market,trade world,governance,regions builder,creative,modding tactics,turns,planning reflex,arcade,runner exploration,story,islands)
GAME_TEMPLATES=(arena-template rpg-template trading-template civilization-template sandbox-template strategy-template action-template adventure-template)
GAME_CIVS=(civ-arena civ-quest civ-market civ-seed civ-builder civ-frontier civ-runner civ-isles)

catalog_transcript() {
  discovery_common_transcript
  local i
  for i in "${!GAME_IDS[@]}"; do
    printf 'game=%s|creator=%s|version=%s|status=%s|category=%s|tags=%s|sovereign=true\n' \
      "${GAME_IDS[$i]}" "${CREATOR_IDS[$i]}" "${GAME_VERSIONS[$i]}" "${GAME_STATUS[$i]}" "${GAME_CATEGORIES[$i]}" "${GAME_TAGS[$i]}"
  done
  for action in Publish Update Deprecate Archive; do
    printf 'catalog_action=%s|record_type=Publication Record|replayable=true\n' "$action"
  done | LC_ALL=C sort
}
catalog_root() { stable_root catalog; }

index_transcript() {
  discovery_common_transcript
  printf 'catalog_root=%s\n' "$(catalog_root)"
  local i
  for i in "${!GAME_IDS[@]}"; do
    printf 'index_game=%s|creator=%s|template=%s|civilization=%s|category=%s|deterministic=true\n' \
      "${GAME_IDS[$i]}" "${CREATOR_IDS[$i]}" "${GAME_TEMPLATES[$i]}" "${GAME_CIVS[$i]}" "${GAME_CATEGORIES[$i]}"
  done | LC_ALL=C sort
  printf 'index_entity=Games|count=%s\n' "${#GAME_IDS[@]}"
  printf 'index_entity=Creators|count=4\n'
  printf 'index_entity=Templates|count=%s\n' "${#GAME_TEMPLATES[@]}"
  printf 'index_entity=Civilizations|count=%s\n' "${#GAME_CIVS[@]}"
}
index_root() { stable_root index; }

categories_transcript() {
  discovery_common_transcript
  printf 'index_root=%s\n' "$(index_root)"
  for category in Arena RPG Trading Civilization Sandbox Strategy Action Adventure; do
    printf 'category=%s|category_search=true|player_visible=true\n' "$category"
  done
}
categories_root() { stable_root categories; }

search_transcript() {
  discovery_common_transcript
  printf 'catalog_root=%s\n' "$(catalog_root)"
  printf 'index_root=%s\n' "$(index_root)"
  printf 'category_root=%s\n' "$(categories_root)"
  printf 'search_query=arena combat|mode=keyword|normalized=arena-combat\n'
  printf 'search_query=RPG|mode=category|normalized=rpg\n'
  printf 'search_query=trade|mode=tag|normalized=trade\n'
  printf 'search_result=arena-vanguard|query=arena-combat|rank=1|reason=keyword-category-tag\n'
  printf 'search_result=rpg-questline|query=rpg|rank=1|reason=category\n'
  printf 'search_result=trading-post|query=trade|rank=1|reason=tag\n'
  printf 'search_ranking=deterministic|rule=status-category-tag-game-id|ml=false\n'
}
search_root() { stable_root search; }

featured_transcript() {
  discovery_common_transcript
  printf 'search_root=%s\n' "$(search_root)"
  printf 'featured=arena-vanguard|bucket=Featured|order=001|rule=curated-static\n'
  printf 'featured=civilization-seed|bucket=Trending|order=002|rule=views-installs-launches\n'
  printf 'featured=rpg-questline|bucket=Popular|order=003|rule=launches-desc-game-id\n'
  printf 'featured=adventure-isles|bucket=Recent|order=004|rule=published-epoch-desc-game-id\n'
  printf 'featured=action-runner|bucket=New Releases|order=005|rule=version-epoch-desc-game-id\n'
  printf 'featured_ordering=deterministic|ties=game-id-lexicographic\n'
}
featured_root() { stable_root featured; }

recommendation_transcript() {
  discovery_common_transcript
  printf 'catalog_root=%s\n' "$(catalog_root)"
  printf 'related_games=arena-vanguard,action-runner,strategy-frontier|rule=shared-tags-category-neighbor\n'
  printf 'suggested_games=rpg-questline,adventure-isles,civilization-seed|rule=published-status-category-diversity\n'
  printf 'creator_collection=creator-alpha|games=arena-vanguard,trading-post|rule=same-creator\n'
  printf 'creator_collection=creator-gamma|games=civilization-seed,strategy-frontier|rule=same-creator\n'
  printf 'recommendation_ranking=deterministic|ml=false|sponsorship=false\n'
}
recommendation_root() { stable_root recommendation; }

install_transcript() {
  discovery_common_transcript
  printf 'launcher_source=player-gateway/launcher/launcher_integration.records\n'
  printf 'install_action=Install|game=arena-vanguard|player=player-alpha|record_type=Install Record\n'
  printf 'install_action=Update|game=rpg-questline|player=player-beta|record_type=Install Record\n'
  printf 'install_action=Remove|game=sandbox-builder|player=player-gamma|record_type=Install Record\n'
  printf 'install_action=Launch|game=arena-vanguard|player=player-alpha|record_type=Launch Record\n'
  printf 'gateway_integration=Player Gateway Launcher|non_authoritative=true\n'
}
install_root() { stable_root install; }

creator_discovery_transcript() {
  discovery_common_transcript
  printf 'marketplace_source=creator-marketplace/records/CREATOR_MARKETPLACE_ROOT\n'
  printf 'creator_profile=creator-alpha|published_games=arena-vanguard,trading-post|published_assets=sprite-heroes,relic-items|published_templates=arena-template,trading-template\n'
  printf 'creator_profile=creator-beta|published_games=rpg-questline,action-runner|published_assets=stone-textures,ambient-audio|published_templates=rpg-template,action-template\n'
  printf 'creator_profile=creator-gamma|published_games=civilization-seed,strategy-frontier|published_assets=settlement-structures|published_templates=civilization-template,strategy-template\n'
  printf 'creator_profile=creator-delta|published_games=sandbox-builder,adventure-isles|published_assets=villager-npcs|published_templates=sandbox-template,adventure-template\n'
}
creator_discovery_root() { stable_root creator_discovery; }

civilization_discovery_transcript() {
  discovery_common_transcript
  printf 'observation_only=true\n'
  printf 'world=world-arena|region=arena-prime|civilization=civ-arena|event=arena-season-opened\n'
  printf 'world=world-quest|region=old-road|civilization=civ-quest|event=party-arrived\n'
  printf 'world=world-market|region=trade-hub|civilization=civ-market|event=market-festival\n'
  printf 'world=world-seed|region=riverlands|civilization=civ-seed|event=council-election\n'
}
civilization_discovery_root() { stable_root civilization_discovery; }

analytics_transcript() {
  discovery_common_transcript
  printf 'views=arena-vanguard|count=1200\n'
  printf 'installs=arena-vanguard|count=420\n'
  printf 'launches=arena-vanguard|count=900\n'
  printf 'retention=arena-vanguard|d7=0.42\n'
  printf 'popularity=arena-vanguard|score=1720|rule=views+installs+launches-weighted\n'
  printf 'analytics_non_authoritative=true\n'
}
analytics_root() { stable_root analytics; }

launcher_transcript() {
  discovery_common_transcript
  printf 'player_gateway_root=player-gateway/records/GATEWAY_ROOT\n'
  for action in Browse Install Launch Update Remove; do
    printf 'launcher_action=%s|surface=Player Gateway|player_focused=true\n' "$action"
  done
}
launcher_root() { stable_root launcher; }

metrics_transcript() {
  discovery_common_transcript
  printf 'published_games=8\n'
  printf 'views=3400\n'
  printf 'installs=980\n'
  printf 'launches=2200\n'
  printf 'retention=d7:0.39\n'
  printf 'creator_reach=4\n'
}
metrics_root() { stable_root metrics; }

discovery_root_transcript() {
  discovery_common_transcript
  printf 'catalog_root=%s\n' "$(catalog_root)"
  printf 'index_root=%s\n' "$(index_root)"
  printf 'category_root=%s\n' "$(categories_root)"
  printf 'search_root=%s\n' "$(search_root)"
  printf 'featured_root=%s\n' "$(featured_root)"
  printf 'recommendation_root=%s\n' "$(recommendation_root)"
  printf 'install_root=%s\n' "$(install_root)"
  printf 'creator_discovery_root=%s\n' "$(creator_discovery_root)"
  printf 'civilization_discovery_root=%s\n' "$(civilization_discovery_root)"
  printf 'analytics_root=%s\n' "$(analytics_root)"
  printf 'launcher_root=%s\n' "$(launcher_root)"
  printf 'metrics_root=%s\n' "$(metrics_root)"
}
discovery_root() { discovery_root_transcript | sha256_text; }

replay_transcript() {
  discovery_common_transcript
  printf 'replay=Catalog|root=%s\n' "$(catalog_root)"
  printf 'replay=Search|root=%s\n' "$(search_root)"
  printf 'replay=Install|root=%s\n' "$(install_root)"
  printf 'replay=Launch|root=%s\n' "$(launcher_root)"
  printf 'replay=Analytics|root=%s\n' "$(analytics_root)"
  printf 'replay_equivalence=REPLAY_ROOT==DISCOVERY_ROOT\n'
  discovery_root_transcript
}
replay_root() { discovery_root; }

game_discovery_write_artifacts() {
  local root="${GAME_DISCOVERY_ROOT:-game-discovery}"
  mkdir -p "$root"/{catalog,index,categories,search,featured,recommendations,installs,creators,civilizations,analytics,replay,launcher,metrics,records}
  catalog_transcript > "$root/catalog/catalog.records"; catalog_root > "$root/catalog/CATALOG_ROOT"
  index_transcript > "$root/index/index.records"; index_root > "$root/index/INDEX_ROOT"
  categories_transcript > "$root/categories/categories.records"; categories_root > "$root/categories/CATEGORY_ROOT"
  search_transcript > "$root/search/search.records"; search_root > "$root/search/SEARCH_ROOT"
  featured_transcript > "$root/featured/featured.records"; featured_root > "$root/featured/FEATURED_ROOT"
  recommendation_transcript > "$root/recommendations/recommendations.records"; recommendation_root > "$root/recommendations/RECOMMENDATION_ROOT"
  install_transcript > "$root/installs/install_registry.records"; install_root > "$root/installs/INSTALL_ROOT"
  creator_discovery_transcript > "$root/creators/creator_discovery.records"; creator_discovery_root > "$root/creators/CREATOR_DISCOVERY_ROOT"
  civilization_discovery_transcript > "$root/civilizations/civilization_discovery.records"; civilization_discovery_root > "$root/civilizations/CIVILIZATION_DISCOVERY_ROOT"
  analytics_transcript > "$root/analytics/analytics.records"; analytics_root > "$root/analytics/ANALYTICS_ROOT"
  replay_transcript > "$root/replay/replay.records"; replay_root > "$root/replay/REPLAY_ROOT"
  launcher_transcript > "$root/launcher/launcher_integration.records"; launcher_root > "$root/launcher/LAUNCHER_INTEGRATION_ROOT"
  metrics_transcript > "$root/metrics/discovery_metrics.records"; metrics_root > "$root/metrics/DISCOVERY_METRICS_ROOT"
  discovery_root_transcript > "$root/records/discovery_root.records"; discovery_root > "$root/records/DISCOVERY_ROOT"
}

validate_catalog() { for action in Publish Update Deprecate Archive; do catalog_transcript | grep -q "catalog_action=$action|"; done && catalog_transcript | grep -q 'game=.*|creator=.*|version=.*|status=.*|category=.*|tags=' && validate_hash "$(catalog_root)"; }
validate_index() { for entity in Games Creators Templates Civilizations; do index_transcript | grep -q "index_entity=$entity|"; done && index_transcript | grep -q 'deterministic=true' && validate_hash "$(index_root)"; }
validate_categories() { for category in Arena RPG Trading Civilization Sandbox Strategy Action Adventure; do categories_transcript | grep -q "category=$category|"; done && validate_hash "$(categories_root)"; }
validate_search() { for mode in keyword category tag; do search_transcript | grep -q "mode=$mode|"; done && search_transcript | grep -q 'search_ranking=deterministic' && validate_hash "$(search_root)"; }
validate_featured() { for bucket in Featured Trending Popular Recent 'New Releases'; do featured_transcript | grep -q "bucket=$bucket|"; done && featured_transcript | grep -q 'featured_ordering=deterministic' && validate_hash "$(featured_root)"; }
validate_recommendations() { recommendation_transcript | grep -q 'related_games=' && recommendation_transcript | grep -q 'suggested_games=' && recommendation_transcript | grep -q 'creator_collection=' && recommendation_transcript | grep -q 'ml=false' && validate_hash "$(recommendation_root)"; }
validate_installs() { for action in Install Update Remove Launch; do install_transcript | grep -q "install_action=$action|"; done && install_transcript | grep -q 'Player Gateway Launcher' && validate_hash "$(install_root)"; }
validate_creators() { creator_discovery_transcript | grep -q 'creator_profile=' && creator_discovery_transcript | grep -q 'published_games=' && creator_discovery_transcript | grep -q 'published_assets=' && creator_discovery_transcript | grep -q 'published_templates=' && validate_hash "$(creator_discovery_root)"; }
validate_civilizations() { civilization_discovery_transcript | grep -q 'world=' && civilization_discovery_transcript | grep -q 'region=' && civilization_discovery_transcript | grep -q 'civilization=' && civilization_discovery_transcript | grep -q 'event=' && civilization_discovery_transcript | grep -q 'observation_only=true' && validate_hash "$(civilization_discovery_root)"; }
validate_analytics() { for metric in views installs launches retention popularity; do analytics_transcript | grep -q "^$metric="; done && validate_hash "$(analytics_root)"; }
validate_replay() { replay_transcript | grep -q 'replay=Catalog|' && replay_transcript | grep -q 'replay=Search|' && replay_transcript | grep -q 'replay=Install|' && replay_transcript | grep -q 'replay=Launch|' && replay_transcript | grep -q 'replay=Analytics|' && [[ "$(replay_root)" == "$(discovery_root)" ]] && validate_hash "$(replay_root)"; }
validate_launcher() { for action in Browse Install Launch Update Remove; do launcher_transcript | grep -q "launcher_action=$action|"; done && launcher_transcript | grep -q 'Player Gateway' && validate_hash "$(launcher_root)"; }
validate_metrics() { for metric in published_games views installs launches retention creator_reach; do metrics_transcript | grep -q "^$metric="; done && validate_hash "$(metrics_root)"; }
