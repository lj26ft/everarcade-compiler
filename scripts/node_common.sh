#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
NODE_DIR="${EVERARCADE_NODE_DIR:-$ROOT_DIR/node}"
REPORT_DIR="$ROOT_DIR/reports"
WORLD_ID="${EVERARCADE_WORLD_ID:-civilization-alpha}"
CONFIG_DIR="$NODE_DIR/config"
STATE_DIR="$NODE_DIR/state"
WORLDS_DIR="$NODE_DIR/worlds"
JOURNALS_DIR="$NODE_DIR/journals"
CHECKPOINTS_DIR="$NODE_DIR/checkpoints"
BACKUPS_DIR="$NODE_DIR/backups"
NODE_REPORT_DIR="$NODE_DIR/reports"
LOGS_DIR="$NODE_DIR/logs"
CONFIG_FILE="$CONFIG_DIR/node.env"
MANIFEST_FILE="$NODE_DIR/node_manifest.json"
RUNTIME_STATE_FILE="$STATE_DIR/runtime_state.env"
WORLD_DIR="$WORLDS_DIR/$WORLD_ID"
WORLD_STATE_FILE="$WORLD_DIR/world_state.env"
CIVILIZATION_LOG="$WORLD_DIR/civilization_activity.log"
JOURNAL_FILE="$JOURNALS_DIR/$WORLD_ID.journal"
LATEST_CHECKPOINT_FILE="$CHECKPOINTS_DIR/latest_checkpoint"
LATEST_REPLAY_ROOT_FILE="$STATE_DIR/latest_replay_root"
LATEST_CONTINUITY_ROOT_FILE="$STATE_DIR/latest_continuity_root"

ensure_report_dir() {
  mkdir -p "$REPORT_DIR" "$NODE_REPORT_DIR"
}

node_dirs() {
  printf '%s\n' \
    "$CONFIG_DIR" \
    "$STATE_DIR" \
    "$WORLDS_DIR" \
    "$JOURNALS_DIR" \
    "$CHECKPOINTS_DIR" \
    "$BACKUPS_DIR" \
    "$NODE_REPORT_DIR" \
    "$LOGS_DIR" \
    "$WORLD_DIR"
}

require_file() {
  local path="$1"
  local label="$2"
  if [[ ! -f "$path" ]]; then
    printf 'Missing %s: %s\n' "$label" "$path" >&2
    return 1
  fi
}

write_runtime_state() {
  local status="$1"
  local timestamp
  timestamp="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
  cat > "$RUNTIME_STATE_FILE" <<STATE
NODE_STATUS=$status
WORLD_ID=$WORLD_ID
UPDATED_AT=$timestamp
STATE_ROOT=$(root_for "$WORLD_STATE_FILE" "$JOURNAL_FILE")
STATE
}

root_for() {
  local existing=()
  local path
  for path in "$@"; do
    [[ -f "$path" ]] && existing+=("$path")
  done
  if [[ ${#existing[@]} -eq 0 ]]; then
    printf 'GENESIS\n' | sha256sum | awk '{print $1}'
  else
    cat "${existing[@]}" | sha256sum | awk '{print $1}'
  fi
}

latest_checkpoint_path() {
  if [[ -f "$LATEST_CHECKPOINT_FILE" ]]; then
    cat "$LATEST_CHECKPOINT_FILE"
  else
    printf 'NONE'
  fi
}

latest_checkpoint_root() {
  local checkpoint
  checkpoint="$(latest_checkpoint_path)"
  if [[ "$checkpoint" != "NONE" && -f "$checkpoint" ]]; then
    root_for "$checkpoint"
  else
    printf 'GENESIS'
  fi
}

replay_root() {
  printf '%s:%s:%s\n' "$(latest_checkpoint_root)" "$(root_for "$JOURNAL_FILE")" "$(root_for "$CIVILIZATION_LOG")" | sha256sum | awk '{print $1}'
}

continuity_root() {
  printf '%s:%s:%s\n' "$(latest_checkpoint_root)" "$(replay_root)" "$(root_for "$WORLD_STATE_FILE")" | sha256sum | awk '{print $1}'
}

copy_report_to_node() {
  local report_path="$1"
  cp "$report_path" "$NODE_REPORT_DIR/$(basename "$report_path")"
}

require_initialized() {
  require_file "$CONFIG_FILE" "node config"
  require_file "$MANIFEST_FILE" "node manifest"
  require_file "$RUNTIME_STATE_FILE" "runtime state"
  require_file "$WORLD_STATE_FILE" "world state"
  require_file "$JOURNAL_FILE" "node journal"
}

runtime_appliance_status() {
  if [[ -x "$ROOT_DIR/appliance/installed/bin/everarcade-runtime" ]]; then
    printf 'INSTALLED'
  elif [[ -x "$ROOT_DIR/bin/everarcade-runtime" ]]; then
    printf 'LOCAL_BIN'
  elif [[ -f "$ROOT_DIR/scripts/runtime_doctor.sh" && -f "$ROOT_DIR/scripts/runtime_start.sh" && -f "$ROOT_DIR/scripts/runtime_stop.sh" && -f "$ROOT_DIR/scripts/runtime_status.sh" ]]; then
    printf 'REPOSITORY_RUNTIME_WRAPPERS_PRESENT'
  else
    printf 'MISSING'
  fi
}
