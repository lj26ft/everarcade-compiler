#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
REPORT="reports/game_templates_validation_report.txt"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT
mkdir -p reports
: > "$REPORT"

pass() { echo "$1: PASS" | tee -a "$REPORT"; }
fail() { echo "$1: FAIL" | tee -a "$REPORT"; exit 1; }
require_file() { [[ -f "$1" ]] || { echo "Missing $1" | tee -a "$REPORT"; exit 1; }; }
require_dir() { [[ -d "$1" ]] || { echo "Missing $1" | tee -a "$REPORT"; exit 1; }; }

TEMPLATES=(arena rpg trading civilization sandbox)

require_file templates/TEMPLATE_REGISTRY_ROOT
require_file templates/registry.json
for template in "${TEMPLATES[@]}"; do
  node -e "const r=require('./templates/registry.json'); if (!r.templates.find(t => t.id === '$template' && t.version && t.type && t.status)) process.exit(1);"
done
pass "Registry"

validate_template() {
  local template="$1"
  local label="$2"
  require_dir "templates/$template"
  require_file "templates/$template/TEMPLATE_ROOT"
  require_file "templates/$template/template.json"
  require_file "templates/$template/systems.json"
  require_file "templates/$template/src/game.js"
  require_file "templates/$template/tests/template.test.json"
  require_file "templates/$template/deployments/local.deployment.json"
  node "templates/$template/src/game.js" >/dev/null
  node -e "const m=require('./templates/$template/template.json'); if (m.id !== '$template' || m.status !== 'playable-scaffold' || !m.features.length || !m.demonstrates.length) process.exit(1);"
  require_file "creator-sdk/templates/$template/everarcade.game.json"
  node creator-sdk/cli/everarcade.mjs new "$template-game" --template "$template" --dir "$TMP/$template-game" >/dev/null
  node creator-sdk/cli/everarcade.mjs build --project "$TMP/$template-game" >/dev/null
  node creator-sdk/cli/everarcade.mjs test --project "$TMP/$template-game" >/dev/null
  node creator-sdk/cli/everarcade.mjs deploy --project "$TMP/$template-game" >/dev/null
  node "$TMP/$template-game/src/game.js" >/dev/null
  pass "$label"
}

validate_template arena "Arena"
validate_template rpg "RPG"
validate_template trading "Trading"
validate_template civilization "Civilization"
validate_template sandbox "Sandbox"

require_file templates/asset-packs/ASSET_PACK_ROOT
for pack in fantasy sci-fi marketplace ui; do
  require_file "templates/asset-packs/$pack/manifest.json"
done
pass "Asset Packs"

require_file creator-sdk/generators/GAME_TEMPLATE_GENERATOR_ROOT
for template in "${TEMPLATES[@]}"; do
  node creator-sdk/cli/everarcade.mjs new "generated-$template" --template "$template" --dir "$TMP/generated-$template" >/dev/null
  require_file "$TMP/generated-$template/everarcade.game.json"
done
pass "Generator"

require_file templates/validation/TEMPLATE_VALIDATION_ROOT
for template in "${TEMPLATES[@]}"; do
  require_file "templates/$template/tests/template.test.json"
done
pass "Validation"

require_file examples/deployments/DEPLOYMENT_EXAMPLE_ROOT
for template in arena rpg trading civilization; do
  require_file "examples/deployments/$template/deployment.json"
  node -e "const d=require('./examples/deployments/$template/deployment.json'); if (d.status !== 'pass' || !d.flow.includes('deploy')) process.exit(1);"
done
pass "Deployments"

require_file templates/success/TEMPLATE_SUCCESS_ROOT
require_file templates/success/metrics.json
node -e "const m=require('./templates/success/metrics.json'); for (const key of ['templateUsage','buildSuccess','deploymentSuccess','timeToFirstPlayable','customizationEvents']) if (!(key in m)) process.exit(1);"
pass "Metrics"

echo "Game Templates Validation: PASS" | tee -a "$REPORT"
