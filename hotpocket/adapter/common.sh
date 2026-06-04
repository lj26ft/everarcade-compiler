#!/usr/bin/env bash
set -euo pipefail

ADAPTER_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$ADAPTER_DIR/../.." && pwd)"
HOTPOCKET_DIR="${EVERARCADE_HOTPOCKET_DIR:-$ROOT_DIR/hotpocket}"
NODE_DIR="${EVERARCADE_NODE_DIR:-$ROOT_DIR/node}"
REPORT_DIR="$ROOT_DIR/reports"
INPUT_DIR="$HOTPOCKET_DIR/input"
OUTPUT_DIR="$HOTPOCKET_DIR/output"
CHECKPOINT_DIR="$HOTPOCKET_DIR/checkpoint"
REPLAY_DIR="$HOTPOCKET_DIR/replay"
SETTLEMENT_DIR="$HOTPOCKET_DIR/settlement"
STATUS_DIR="$HOTPOCKET_DIR/status"
WORLD_ID="${EVERARCADE_WORLD_ID:-civilization-alpha}"

sha256_text() {
  sha256sum | awk '{print $1}'
}

sha256_file() {
  local path="$1"
  if [[ -f "$path" ]]; then
    sha256sum "$path" | awk '{print $1}'
  else
    printf 'GENESIS\n' | sha256_text
  fi
}

ensure_hotpocket_layout() {
  mkdir -p \
    "$HOTPOCKET_DIR/adapter" \
    "$INPUT_DIR" \
    "$OUTPUT_DIR" \
    "$CHECKPOINT_DIR" \
    "$REPLAY_DIR" \
    "$SETTLEMENT_DIR" \
    "$STATUS_DIR" \
    "$REPORT_DIR"
}

write_hash_envelope() {
  local output_path="$1"
  shift
  local body
  body="$(mktemp)"
  trap 'rm -f "$body"' RETURN
  printf '%s\n' "$@" > "$body"
  local hash
  hash="$(cat "$body" | sha256_text)"
  cat "$body" > "$output_path"
  printf 'Hash: %s\n' "$hash" >> "$output_path"
  rm -f "$body"
  trap - RETURN
}

root_from_hashes() {
  local dir="$1"
  local pattern="$2"
  local label="$3"
  {
    printf 'Root Type: %s\n' "$label"
    find "$dir" -maxdepth 1 -type f -name "$pattern" -print | sort | while IFS= read -r file; do
      local hash
      hash="$(awk -F': ' '/^Hash: /{print $2}' "$file")"
      printf '%s:%s\n' "$(basename "$file")" "$hash"
    done
  } | sha256_text
}

relative_to_root() {
  local path="$1"
  case "$path" in
    "$ROOT_DIR"/*) printf '%s' "${path#"$ROOT_DIR"/}" ;;
    *) printf '%s' "$path" ;;
  esac
}

safe_value() {
  local value="$1"
  printf '%s' "$value" | tr '\n\r' '  '
}

node_file_value() {
  local path="$1"
  local default="$2"
  if [[ -f "$path" ]]; then
    cat "$path"
  else
    printf '%s' "$default"
  fi
}

node_env_value() {
  local path="$1"
  local key="$2"
  local default="$3"
  if [[ -f "$path" ]]; then
    awk -F= -v key="$key" '$1 == key { value = $2 } END { if (value != "") print value; else print "'"$default"'" }' "$path"
  else
    printf '%s\n' "$default"
  fi
}
