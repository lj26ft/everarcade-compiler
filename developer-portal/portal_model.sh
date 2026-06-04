#!/usr/bin/env bash
# EverArcade Developer Portal v0.1 deterministic model.
# The portal is non-authoritative: it creates, configures, deploys, validates,
# and monitors through existing protocol surfaces while emitting replayable records.

set -euo pipefail

DEVELOPER_PORTAL_VERSION="0.1"
DEVELOPER_PORTAL_ID="everarcade-developer-portal-v0.1"
DEVELOPER_PORTAL_AUTHORITY="non-authoritative-builder-interface"
DEVELOPER_PORTAL_ORDERING="developer-id-project-id-version-action-lexicographic"
DEVELOPER_PORTAL_SETTLEMENT_MODE="intent-and-observation-only"
DEVELOPER_PORTAL_PRIVATE_KEY_CUSTODY="false"

sha256_text() {
  sha256sum | awk '{print $1}'
}

portal_common_transcript() {
  printf 'portal_id=%s\n' "$DEVELOPER_PORTAL_ID"
  printf 'version=%s\n' "$DEVELOPER_PORTAL_VERSION"
  printf 'authority=%s\n' "$DEVELOPER_PORTAL_AUTHORITY"
  printf 'ordering=%s\n' "$DEVELOPER_PORTAL_ORDERING"
  printf 'settlement_mode=%s\n' "$DEVELOPER_PORTAL_SETTLEMENT_MODE"
  printf 'private_key_custody=%s\n' "$DEVELOPER_PORTAL_PRIVATE_KEY_CUSTODY"
}

PROJECT_IDS=(project-arena project-civ project-render)
PROJECT_DEVELOPERS=(developer-alpha developer-beta developer-gamma)
PROJECT_NAMES=(ArenaVanguard CivilizationWorld ReplayRenderer)
PROJECT_TYPES=(game civilization-tool renderer-tool)
PROJECT_VERSIONS=(0.1.0 0.1.0 0.1.0)
PROJECT_STATUS=(published active archived)
PROJECT_ACTIONS=(create update publish archive)
PROJECT_ACTION_PROJECTS=(project-arena project-civ project-arena project-render)
PROJECT_ACTION_DEVELOPERS=(developer-alpha developer-beta developer-alpha developer-gamma)
PROJECT_ACTION_VERSIONS=(0.1.0 0.1.0 0.1.0 0.1.0)

project_registry_transcript() {
  portal_common_transcript
  local i
  for i in "${!PROJECT_IDS[@]}"; do
    printf 'project=%s|developer=%s|name=%s|type=%s|version=%s|status=%s\n' \
      "${PROJECT_IDS[$i]}" "${PROJECT_DEVELOPERS[$i]}" "${PROJECT_NAMES[$i]}" \
      "${PROJECT_TYPES[$i]}" "${PROJECT_VERSIONS[$i]}" "${PROJECT_STATUS[$i]}"
  done
  for i in "${!PROJECT_ACTIONS[@]}"; do
    printf 'project_action=%s|project=%s|developer=%s|version=%s\n' \
      "${PROJECT_ACTIONS[$i]}" "${PROJECT_ACTION_PROJECTS[$i]}" \
      "${PROJECT_ACTION_DEVELOPERS[$i]}" "${PROJECT_ACTION_VERSIONS[$i]}"
  done | LC_ALL=C sort
}
project_registry_root() { project_registry_transcript | sha256_text; }

GAME_STEPS=(create-project create-game runtime-manifest deployment-manifest)
GAME_IDS=(project-arena game-arena runtime-manifest-arena deployment-manifest-arena)
GAME_SURFACES=(project-registry runtime-api runtime-api deployment-api)

game_creation_transcript() {
  portal_common_transcript
  printf 'project_registry_root=%s\n' "$(project_registry_root)"
  local i
  for i in "${!GAME_STEPS[@]}"; do
    printf 'game_step=%s|artifact=%s|surface=%s|replayable=true\n' \
      "${GAME_STEPS[$i]}" "${GAME_IDS[$i]}" "${GAME_SURFACES[$i]}"
  done
}
game_creation_root() { game_creation_transcript | sha256_text; }

DEPLOYMENT_TYPES=(local lease federation)
DEPLOYMENT_IDS=(deploy-local-0001 deploy-lease-0001 deploy-federation-0001)
DEPLOYMENT_STATUS=(running scheduled healthy)
DEPLOYMENT_HEALTH=(pass pass pass)
DEPLOYMENT_CHECKPOINTS=(checkpoint-local-42 checkpoint-lease-77 checkpoint-federation-108)
DEPLOYMENT_REPLAYS=(replay-local-42 replay-lease-77 replay-federation-108)

deployment_transcript() {
  portal_common_transcript
  printf 'game_creation_root=%s\n' "$(game_creation_root)"
  local i
  for i in "${!DEPLOYMENT_TYPES[@]}"; do
    printf 'deployment=%s|type=%s|status=%s|health=%s|checkpoint=%s|replay=%s\n' \
      "${DEPLOYMENT_IDS[$i]}" "${DEPLOYMENT_TYPES[$i]}" "${DEPLOYMENT_STATUS[$i]}" \
      "${DEPLOYMENT_HEALTH[$i]}" "${DEPLOYMENT_CHECKPOINTS[$i]}" "${DEPLOYMENT_REPLAYS[$i]}"
  done
}
deployment_root() { deployment_transcript | sha256_text; }

ASSET_CLASSES=(assets inventories vault-assets marketplace-assets)
ASSET_IDS=(asset-sprite-001 inventory-arena-001 vault-relic-001 listing-skin-001)
ASSET_SURFACES=(renderer-api runtime-api settlement-api marketplace-api)

asset_registry_transcript() {
  portal_common_transcript
  local i
  for i in "${!ASSET_CLASSES[@]}"; do
    printf 'asset=%s|class=%s|surface=%s|custody=none|ownership_observed=true\n' \
      "${ASSET_IDS[$i]}" "${ASSET_CLASSES[$i]}" "${ASSET_SURFACES[$i]}"
  done
}
asset_registry_root() { asset_registry_transcript | sha256_text; }

WALLET_CLASSES=(authorities xrpl-wallets xaman-connections settlement-accounts)
WALLET_IDS=(authority-dev-alpha xrpl-r-dev-alpha xaman-link-alpha settlement-account-alpha)
WALLET_CUSTODY=(delegated non-custodial non-custodial observed)

wallet_transcript() {
  portal_common_transcript
  local i
  for i in "${!WALLET_CLASSES[@]}"; do
    printf 'wallet=%s|class=%s|custody=%s|private_key_custody=false\n' \
      "${WALLET_IDS[$i]}" "${WALLET_CLASSES[$i]}" "${WALLET_CUSTODY[$i]}"
  done
}
wallet_root() { wallet_transcript | sha256_text; }

CIV_CLASSES=(civilizations worlds regions governance economies)
CIV_IDS=(civ-genesis world-genesis region-north governance-council economy-credit)

civilization_transcript() {
  portal_common_transcript
  local i
  for i in "${!CIV_CLASSES[@]}"; do
    printf 'civilization_item=%s|class=%s|surface=civilization-runtime|authoritative=false\n' \
      "${CIV_IDS[$i]}" "${CIV_CLASSES[$i]}"
  done
}
civilization_root() { civilization_transcript | sha256_text; }

MARKET_CLASSES=(listings sales royalties creator-revenue settlement-events)
MARKET_IDS=(listing-0001 sale-0001 royalty-0001 revenue-creator-alpha settlement-event-0001)

marketplace_transcript() {
  portal_common_transcript
  local i
  for i in "${!MARKET_CLASSES[@]}"; do
    printf 'marketplace_item=%s|class=%s|surface=marketplace-settlement|mutates_settlement=false\n' \
      "${MARKET_IDS[$i]}" "${MARKET_CLASSES[$i]}"
  done
}
marketplace_root() { marketplace_transcript | sha256_text; }

GPU_CLASSES=(gpu-providers capacity assignments artifacts settlement-intents)
GPU_IDS=(provider-alpha capacity-alpha assignment-0001 artifact-0001 settlement-intent-0001)

gpu_marketplace_transcript() {
  portal_common_transcript
  local i
  for i in "${!GPU_CLASSES[@]}"; do
    printf 'gpu_item=%s|class=%s|surface=gpu-marketplace|settlement=intent-only\n' \
      "${GPU_IDS[$i]}" "${GPU_CLASSES[$i]}"
  done
}
gpu_marketplace_root() { gpu_marketplace_transcript | sha256_text; }

ANALYTICS_CLASSES=(deployments players settlements marketplace-activity gpu-activity civilization-activity)
ANALYTICS_IDS=(metric-deployments metric-players metric-settlements metric-marketplace metric-gpu metric-civilization)

analytics_transcript() {
  portal_common_transcript
  printf 'deployment_root=%s\n' "$(deployment_root)"
  printf 'marketplace_root=%s\n' "$(marketplace_root)"
  printf 'gpu_marketplace_root=%s\n' "$(gpu_marketplace_root)"
  printf 'civilization_root=%s\n' "$(civilization_root)"
  local i
  for i in "${!ANALYTICS_CLASSES[@]}"; do
    printf 'analytics=%s|class=%s|source=derived-observation|authoritative=false\n' \
      "${ANALYTICS_IDS[$i]}" "${ANALYTICS_CLASSES[$i]}"
  done
}
analytics_root() { analytics_transcript | sha256_text; }

ONBOARDING_STEPS=(create-project build validate deploy monitor)
ONBOARDING_SURFACES=(project-registry everarcade-cli validation deployment-api monitoring-api)

onboarding_transcript() {
  portal_common_transcript
  printf 'project_registry_root=%s\n' "$(project_registry_root)"
  printf 'deployment_root=%s\n' "$(deployment_root)"
  local i
  for i in "${!ONBOARDING_STEPS[@]}"; do
    printf 'onboarding_step=%02d|name=%s|surface=%s\n' "$((i + 1))" "${ONBOARDING_STEPS[$i]}" "${ONBOARDING_SURFACES[$i]}"
  done
}
onboarding_root() { onboarding_transcript | sha256_text; }

portal_dashboard_transcript() {
  portal_common_transcript
  printf 'project_registry_root=%s\n' "$(project_registry_root)"
  printf 'game_creation_root=%s\n' "$(game_creation_root)"
  printf 'deployment_root=%s\n' "$(deployment_root)"
  printf 'asset_registry_root=%s\n' "$(asset_registry_root)"
  printf 'wallet_root=%s\n' "$(wallet_root)"
  printf 'civilization_root=%s\n' "$(civilization_root)"
  printf 'marketplace_root=%s\n' "$(marketplace_root)"
  printf 'gpu_marketplace_root=%s\n' "$(gpu_marketplace_root)"
  printf 'analytics_root=%s\n' "$(analytics_root)"
  printf 'onboarding_root=%s\n' "$(onboarding_root)"
}

portal_root() {
  {
    portal_common_transcript
    printf 'project_registry_root=%s\n' "$(project_registry_root)"
    printf 'game_creation_root=%s\n' "$(game_creation_root)"
    printf 'deployment_root=%s\n' "$(deployment_root)"
    printf 'asset_registry_root=%s\n' "$(asset_registry_root)"
    printf 'wallet_root=%s\n' "$(wallet_root)"
    printf 'civilization_root=%s\n' "$(civilization_root)"
    printf 'marketplace_root=%s\n' "$(marketplace_root)"
    printf 'gpu_marketplace_root=%s\n' "$(gpu_marketplace_root)"
    printf 'analytics_root=%s\n' "$(analytics_root)"
    printf 'onboarding_root=%s\n' "$(onboarding_root)"
  } | sha256_text
}

portal_write_artifacts() {
  local root="${DEVELOPER_PORTAL_ROOT:-developer-portal}"
  mkdir -p "$root"/{dashboard,projects,deployments,marketplace,civilizations,wallets,assets,analytics,gpu,onboarding,games,records}

  portal_dashboard_transcript > "$root/dashboard/portal_dashboard.records"
  project_registry_transcript > "$root/projects/project_registry.records"
  game_creation_transcript > "$root/games/game_creation.records"
  deployment_transcript > "$root/deployments/deployment_dashboard.records"
  asset_registry_transcript > "$root/assets/asset_registry.records"
  wallet_transcript > "$root/wallets/wallet_dashboard.records"
  civilization_transcript > "$root/civilizations/civilization_dashboard.records"
  marketplace_transcript > "$root/marketplace/marketplace_dashboard.records"
  gpu_marketplace_transcript > "$root/gpu/gpu_marketplace_dashboard.records"
  gpu_marketplace_root > "$root/gpu/gpu_marketplace_root.txt"
  analytics_transcript > "$root/analytics/analytics_layer.records"
  onboarding_transcript > "$root/onboarding/developer_onboarding.records"

  cat > "$root/records/roots.env" <<ROOTS
PROJECT_REGISTRY_ROOT=$(project_registry_root)
GAME_CREATION_ROOT=$(game_creation_root)
DEPLOYMENT_ROOT=$(deployment_root)
ASSET_REGISTRY_ROOT=$(asset_registry_root)
WALLET_ROOT=$(wallet_root)
CIVILIZATION_ROOT=$(civilization_root)
MARKETPLACE_ROOT=$(marketplace_root)
GPU_MARKETPLACE_ROOT=$(gpu_marketplace_root)
ANALYTICS_ROOT=$(analytics_root)
ONBOARDING_ROOT=$(onboarding_root)
DEVELOPER_PORTAL_ROOT_HASH=$(portal_root)
ROOTS
}

validate_hash() {
  [[ "$1" =~ ^[0-9a-f]{64}$ ]]
}

validate_projects() {
  [[ "$(project_registry_transcript | grep -c '^project=')" -ge 3 ]] &&
  [[ "$(project_registry_transcript | grep -c '^project_action=')" -eq 4 ]] &&
  validate_hash "$(project_registry_root)"
}
validate_game_creation() {
  for step in create-project create-game runtime-manifest deployment-manifest; do
    game_creation_transcript | grep -q "game_step=$step|"
  done
  validate_hash "$(game_creation_root)"
}
validate_deployments() {
  for type in local lease federation; do
    deployment_transcript | grep -q "type=$type|status="
  done
  deployment_transcript | grep -q 'checkpoint=' && deployment_transcript | grep -q 'replay=' && validate_hash "$(deployment_root)"
}
validate_assets() {
  for class in assets inventories vault-assets marketplace-assets; do
    asset_registry_transcript | grep -q "class=$class|"
  done
  validate_hash "$(asset_registry_root)"
}
validate_wallets() {
  for class in authorities xrpl-wallets xaman-connections settlement-accounts; do
    wallet_transcript | grep -q "class=$class|"
  done
  wallet_transcript | grep -q 'private_key_custody=false' && validate_hash "$(wallet_root)"
}
validate_civilizations() {
  for class in civilizations worlds regions governance economies; do
    civilization_transcript | grep -q "class=$class|"
  done
  validate_hash "$(civilization_root)"
}
validate_marketplace() {
  for class in listings sales royalties creator-revenue settlement-events; do
    marketplace_transcript | grep -q "class=$class|"
  done
  validate_hash "$(marketplace_root)"
}
validate_gpu() {
  for class in gpu-providers capacity assignments artifacts settlement-intents; do
    gpu_marketplace_transcript | grep -q "class=$class|"
  done
  validate_hash "$(gpu_marketplace_root)"
}
validate_analytics() {
  for class in deployments players settlements marketplace-activity gpu-activity civilization-activity; do
    analytics_transcript | grep -q "class=$class|"
  done
  validate_hash "$(analytics_root)"
}
validate_onboarding() {
  for step in create-project build validate deploy monitor; do
    onboarding_transcript | grep -q "name=$step|"
  done
  validate_hash "$(onboarding_root)"
}
