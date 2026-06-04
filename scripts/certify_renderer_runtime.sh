#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPORT_DIR="$ROOT_DIR/reports"
REPORT_PATH="$REPORT_DIR/renderer_runtime_certification_report.txt"

cd "$ROOT_DIR"
# shellcheck source=../renderer/projection/projection_model.sh
source "$ROOT_DIR/renderer/projection/projection_model.sh"
mkdir -p "$REPORT_DIR"

renderer_write_artifacts

projection_status="FAIL"
world_status="FAIL"
entities_status="FAIL"
physics_status="FAIL"
inventory_status="FAIL"
events_status="FAIL"
replay_status="FAIL"
overall_status="FAIL"

validate_projection && projection_status="PASS"
validate_world && world_status="PASS"
validate_entities && entities_status="PASS"
validate_physics && physics_status="PASS"
validate_inventory && inventory_status="PASS"
validate_events && events_status="PASS"
validate_replay && replay_status="PASS"

if [[ "$projection_status" == "PASS" \
  && "$world_status" == "PASS" \
  && "$entities_status" == "PASS" \
  && "$physics_status" == "PASS" \
  && "$inventory_status" == "PASS" \
  && "$events_status" == "PASS" \
  && "$replay_status" == "PASS" ]]; then
  overall_status="PASS"
fi

cat > "$REPORT_PATH" <<REPORT
Renderer Runtime Certification Report
Version: $RENDERER_RUNTIME_VERSION
Runtime ID: $RENDERER_RUNTIME_ID
Projection Epoch: $PROJECTION_EPOCH
Non-Authoritative: $NON_AUTHORITATIVE
Canonical Ordering: $CANONICAL_ORDERING

Projection: $projection_status
World: $world_status
Entities: $entities_status
Physics: $physics_status
Inventory: $inventory_status
Events: $events_status
Replay: $replay_status

Projection Root: $(projection_root)
World Projection Root: $(world_projection_root)
Entity Projection Root: $(entity_projection_root)
Physics Projection Root: $(physics_projection_root)
Inventory Projection Root: $(inventory_projection_root)
Event Projection Root: $(event_projection_root)
Replay Projection Root: $(replay_projection_root)
Replay Equals Projection: $(if [[ "$(replay_projection_root)" == "$(projection_root)" ]]; then printf 'true'; else printf 'false'; fi)

Renderer Runtime: $overall_status
REPORT

cat "$REPORT_PATH"
[[ "$overall_status" == "PASS" ]]
