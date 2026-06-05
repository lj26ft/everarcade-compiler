#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
REPORT="reports/creator_sdk_validation_report.txt"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT
mkdir -p reports
: > "$REPORT"

pass() { echo "$1: PASS" | tee -a "$REPORT"; }
require_file() { [[ -f "$1" ]] || { echo "Missing $1" | tee -a "$REPORT"; exit 1; }; }
require_dir() { [[ -d "$1" ]] || { echo "Missing $1" | tee -a "$REPORT"; exit 1; }; }

require_file creator-sdk/cli/everarcade.mjs
node creator-sdk/cli/everarcade.mjs >/dev/null
node creator-sdk/cli/everarcade.mjs new --template blank-game --name first-game --dir "$TMP/first-game" >/dev/null
node creator-sdk/cli/everarcade.mjs build --project "$TMP/first-game" >/dev/null
node creator-sdk/cli/everarcade.mjs deploy --project "$TMP/first-game" >/dev/null
node creator-sdk/cli/everarcade.mjs publish --project "$TMP/first-game" >/dev/null
pass "CLI"

for template in blank-game rpg arena civilization marketplace-demo; do
  require_file "creator-sdk/templates/$template/everarcade.game.json"
  cp -R "creator-sdk/templates/$template" "$TMP/$template"
  node creator-sdk/cli/everarcade.mjs build --project "$TMP/$template" >/dev/null
  node creator-sdk/cli/everarcade.mjs test --project "$TMP/$template" >/dev/null
done
pass "Templates"

require_file creator-sdk/assets/ASSET_SDK_ROOT
require_file creator-sdk/assets/index.mjs
require_dir creator-sdk/assets/examples/sprites
require_dir creator-sdk/assets/examples/textures
require_dir creator-sdk/assets/examples/audio
require_dir creator-sdk/assets/examples/items
require_dir creator-sdk/assets/examples/npcs
require_dir creator-sdk/assets/examples/structures
pass "Assets"

require_file creator-sdk/inventory/INVENTORY_SDK_ROOT
require_file creator-sdk/inventory/index.mjs
pass "Inventory"

require_file creator-sdk/economy/ECONOMY_SDK_ROOT
require_file creator-sdk/economy/index.mjs
pass "Economy"

require_file creator-sdk/civilization/CIVILIZATION_SDK_ROOT
require_file creator-sdk/civilization/index.mjs
pass "Civilization"

require_file creator-sdk/deployment/DEPLOYMENT_SDK_ROOT
require_file creator-sdk/deployment/index.mjs
pass "Deployment"

require_file creator-sdk/monetization/MONETIZATION_SDK_ROOT
require_file creator-sdk/monetization/index.mjs
require_file creator-sdk/marketplace/MARKETPLACE_SDK_ROOT
pass "Monetization"

for example in arena-example rpg-example trading-example; do
  require_file "creator-sdk/examples/$example/everarcade.game.json"
  cp -R "creator-sdk/examples/$example" "$TMP/$example"
  node creator-sdk/cli/everarcade.mjs build --project "$TMP/$example" >/dev/null
  node creator-sdk/cli/everarcade.mjs deploy --project "$TMP/$example" >/dev/null
  node "$TMP/$example/src/game.js" >/dev/null
done
pass "Examples"

require_file creator-sdk/onboarding/ONBOARDING_ROOT
require_file docs/creator-sdk/README.md
require_file docs/creator-sdk/quick-start.md
require_file docs/creator-sdk/create-first-game.md
require_file docs/creator-sdk/build-first-game.md
require_file docs/creator-sdk/deploy-first-game.md
require_file docs/creator-sdk/monetize-first-game.md
require_file docs/creator-sdk/publish-first-game.md
pass "Onboarding"

require_file creator-sdk/success/CREATOR_SUCCESS_ROOT
require_file creator-sdk/success/metrics.json

echo "Creator SDK Validation: PASS" | tee -a "$REPORT"
