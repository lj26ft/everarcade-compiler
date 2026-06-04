#!/usr/bin/env bash
# Renderer Runtime v0.1 deterministic projection model.
# This file is sourced by validation and certification scripts. It must remain
# non-authoritative: it only reads fixed projection artifacts and derives roots.

set -euo pipefail

RENDERER_RUNTIME_VERSION="0.1"
RENDERER_RUNTIME_ID="everarcade-renderer-runtime-v0.1"
PROJECTION_EPOCH="civilization-renderable-epoch-001"
CANONICAL_ORDERING="lexicographic-domain-order"
NON_AUTHORITATIVE="true"

WORLD_ZONES=(zone-capital zone-frontier)
WORLD_REGIONS=(region-citadel region-market region-wilds)
WORLD_CIVILIZATIONS=(civ-aurora)
WORLD_SETTLEMENTS=(settlement-genesis settlement-rivergate)
WORLD_OBJECTS=(object-beacon object-market-board object-vault-door)

ENTITY_PLAYERS=(player-founder player-trader)
ENTITY_NPCS=(npc-archivist npc-guard)
ENTITY_ASSETS=(asset-relic asset-founders-pass)
ENTITY_STRUCTURES=(structure-hall structure-vault)
ENTITY_CREATURES=(creature-sentinel)

PHYSICS_BODIES=(body-player-founder body-vault-door body-sentinel)
PHYSICS_TRANSFORMS=(transform-player-founder@0,0 transform-vault-door@4,2 transform-sentinel@8,3)
PHYSICS_COLLISIONS=(collision-none)
PHYSICS_MOVEMENT=(movement-readonly-tick-42)
PHYSICS_INTERACTIONS=(interaction-proximity-market-board)

INVENTORY_OWNERSHIP=(owner-player-founder:asset-relic owner-player-trader:asset-founders-pass)
INVENTORY_CONTAINERS=(container-player-founder-pack container-settlement-vault)
INVENTORY_EQUIPMENT=(equipment-player-founder:relic-slot equipment-player-trader:pass-slot)
INVENTORY_VAULT_ASSETS=(vault-asset-relic vault-asset-founders-pass)

EVENT_COMBAT=(combat-none)
EVENT_TRADES=(trade-market-offer-001-settled)
EVENT_MARKETPLACE=(marketplace-listing-001-visible)
EVENT_GOVERNANCE=(governance-proposal-001-visible)
EVENT_CIVILIZATION=(civilization-founded civilization-settlement-expanded)

REPLAY_CHECKPOINT_ID="checkpoint-renderer-0001"
REPLAY_CHECKPOINT_TICK="42"
REPLAY_STREAM=(frame-000040 frame-000041 frame-000042)

sha256_text() {
  sha256sum | awk '{print $1}'
}

emit_kv_list() {
  local key="$1"
  shift
  local value
  for value in "$@"; do
    printf '%s=%s\n' "$key" "$value"
  done | LC_ALL=C sort
}

renderer_common_transcript() {
  printf 'renderer_runtime_id=%s\n' "$RENDERER_RUNTIME_ID"
  printf 'version=%s\n' "$RENDERER_RUNTIME_VERSION"
  printf 'projection_epoch=%s\n' "$PROJECTION_EPOCH"
  printf 'non_authoritative=%s\n' "$NON_AUTHORITATIVE"
  printf 'canonical_ordering=%s\n' "$CANONICAL_ORDERING"
}

world_projection_transcript() {
  renderer_common_transcript
  emit_kv_list zone "${WORLD_ZONES[@]}"
  emit_kv_list region "${WORLD_REGIONS[@]}"
  emit_kv_list civilization "${WORLD_CIVILIZATIONS[@]}"
  emit_kv_list settlement "${WORLD_SETTLEMENTS[@]}"
  emit_kv_list world_object "${WORLD_OBJECTS[@]}"
}

entity_projection_transcript() {
  renderer_common_transcript
  emit_kv_list player "${ENTITY_PLAYERS[@]}"
  emit_kv_list npc "${ENTITY_NPCS[@]}"
  emit_kv_list asset "${ENTITY_ASSETS[@]}"
  emit_kv_list structure "${ENTITY_STRUCTURES[@]}"
  emit_kv_list creature "${ENTITY_CREATURES[@]}"
}

physics_projection_transcript() {
  renderer_common_transcript
  emit_kv_list body "${PHYSICS_BODIES[@]}"
  emit_kv_list transform "${PHYSICS_TRANSFORMS[@]}"
  emit_kv_list collision "${PHYSICS_COLLISIONS[@]}"
  emit_kv_list movement "${PHYSICS_MOVEMENT[@]}"
  emit_kv_list interaction "${PHYSICS_INTERACTIONS[@]}"
  printf 'simulation_authority=false\n'
}

inventory_projection_transcript() {
  renderer_common_transcript
  emit_kv_list ownership "${INVENTORY_OWNERSHIP[@]}"
  emit_kv_list container "${INVENTORY_CONTAINERS[@]}"
  emit_kv_list equipment "${INVENTORY_EQUIPMENT[@]}"
  emit_kv_list vault_asset "${INVENTORY_VAULT_ASSETS[@]}"
}

event_projection_transcript() {
  renderer_common_transcript
  emit_kv_list combat "${EVENT_COMBAT[@]}"
  emit_kv_list trade "${EVENT_TRADES[@]}"
  emit_kv_list marketplace "${EVENT_MARKETPLACE[@]}"
  emit_kv_list governance "${EVENT_GOVERNANCE[@]}"
  emit_kv_list civilization_event "${EVENT_CIVILIZATION[@]}"
}

projection_root_transcript() {
  renderer_common_transcript
  printf 'world_root=%s\n' "$(world_projection_root)"
  printf 'entity_root=%s\n' "$(entity_projection_root)"
  printf 'physics_root=%s\n' "$(physics_projection_root)"
  printf 'inventory_root=%s\n' "$(inventory_projection_root)"
  printf 'event_root=%s\n' "$(event_projection_root)"
}

replay_projection_transcript() {
  projection_root_transcript
  printf 'checkpoint_id=%s\n' "$REPLAY_CHECKPOINT_ID"
  printf 'checkpoint_tick=%s\n' "$REPLAY_CHECKPOINT_TICK"
  emit_kv_list replay_frame "${REPLAY_STREAM[@]}"
}

world_projection_root() { world_projection_transcript | sha256_text; }
entity_projection_root() { entity_projection_transcript | sha256_text; }
physics_projection_root() { physics_projection_transcript | sha256_text; }
inventory_projection_root() { inventory_projection_transcript | sha256_text; }
event_projection_root() { event_projection_transcript | sha256_text; }
projection_root() { projection_root_transcript | sha256_text; }
replay_projection_root() { projection_root_transcript | sha256_text; }
expected_replay_projection_root() { projection_root; }

validate_non_authoritative() {
  [[ "$NON_AUTHORITATIVE" == "true" ]]
}

validate_projection() {
  validate_non_authoritative
  [[ -n "$(projection_root)" ]]
}

validate_world() {
  [[ ${#WORLD_ZONES[@]} -gt 0 && ${#WORLD_REGIONS[@]} -gt 0 && ${#WORLD_CIVILIZATIONS[@]} -gt 0 && ${#WORLD_SETTLEMENTS[@]} -gt 0 && ${#WORLD_OBJECTS[@]} -gt 0 ]]
}

validate_entities() {
  [[ ${#ENTITY_PLAYERS[@]} -gt 0 && ${#ENTITY_NPCS[@]} -gt 0 && ${#ENTITY_ASSETS[@]} -gt 0 && ${#ENTITY_STRUCTURES[@]} -gt 0 && ${#ENTITY_CREATURES[@]} -gt 0 ]]
}

validate_physics() {
  [[ ${#PHYSICS_BODIES[@]} -gt 0 && ${#PHYSICS_TRANSFORMS[@]} -gt 0 && ${#PHYSICS_COLLISIONS[@]} -gt 0 && ${#PHYSICS_MOVEMENT[@]} -gt 0 && ${#PHYSICS_INTERACTIONS[@]} -gt 0 ]] \
    && physics_projection_transcript | grep -q '^simulation_authority=false$'
}

validate_inventory() {
  [[ ${#INVENTORY_OWNERSHIP[@]} -gt 0 && ${#INVENTORY_CONTAINERS[@]} -gt 0 && ${#INVENTORY_EQUIPMENT[@]} -gt 0 && ${#INVENTORY_VAULT_ASSETS[@]} -gt 0 ]]
}

validate_events() {
  [[ ${#EVENT_COMBAT[@]} -gt 0 && ${#EVENT_TRADES[@]} -gt 0 && ${#EVENT_MARKETPLACE[@]} -gt 0 && ${#EVENT_GOVERNANCE[@]} -gt 0 && ${#EVENT_CIVILIZATION[@]} -gt 0 ]]
}

validate_replay() {
  [[ ${#REPLAY_STREAM[@]} -gt 0 && -n "$REPLAY_CHECKPOINT_ID" ]]
  [[ "$(replay_projection_root)" == "$(projection_root)" ]]
}

renderer_write_artifacts() {
  local artifact_dir="renderer/projection/artifacts"
  mkdir -p "$artifact_dir"
  world_projection_transcript > "$artifact_dir/world_projection.txt"
  entity_projection_transcript > "$artifact_dir/entity_projection.txt"
  physics_projection_transcript > "$artifact_dir/physics_projection.txt"
  inventory_projection_transcript > "$artifact_dir/inventory_projection.txt"
  event_projection_transcript > "$artifact_dir/event_projection.txt"
  projection_root_transcript > "$artifact_dir/projection_root_transcript.txt"
  replay_projection_transcript > "$artifact_dir/replay_projection_transcript.txt"
}
