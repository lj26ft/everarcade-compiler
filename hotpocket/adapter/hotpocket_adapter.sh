#!/usr/bin/env bash
set -euo pipefail
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"

usage() {
  cat <<'USAGE'
Usage: bash hotpocket/adapter/hotpocket_adapter.sh <command> [args]

Commands:
  init-layout
  ingest-input <input-id> <timestamp> <origin> <payload>
  export-output <output-id> <result> <status> <root>
  export-checkpoint [checkpoint-id]
  export-replay [replay-id]
  export-settlement [settlement-id]
  export-status
  export-all
USAGE
}

input_root_file="$INPUT_DIR/input_root"
output_root_file="$OUTPUT_DIR/output_root"
checkpoint_root_file="$CHECKPOINT_DIR/checkpoint_export_root"
replay_root_file="$REPLAY_DIR/replay_export_root"
settlement_root_file="$SETTLEMENT_DIR/settlement_export_root"
status_root_file="$STATUS_DIR/status_root"

init_layout() {
  ensure_hotpocket_layout
  cat > "$HOTPOCKET_DIR/layout.txt" <<LAYOUT
hotpocket/
  adapter/      HotPocket-to-EverArcade boundary scripts; no protocol logic.
  input/        Deterministic input envelopes and input_root.
  output/       Deterministic output envelopes and output_root.
  checkpoint/   Checkpoint export envelopes and checkpoint_export_root.
  replay/       Replay export envelopes and replay_export_root.
  settlement/   Settlement export envelopes and settlement_export_root; no XRPL execution.
  status/       Runtime status envelopes and status_root.
LAYOUT
}

recompute_input_root() {
  root_from_hashes "$INPUT_DIR" '*.input.env' 'Input Root' > "$input_root_file"
}

recompute_output_root() {
  root_from_hashes "$OUTPUT_DIR" '*.output.env' 'Output Root' > "$output_root_file"
}

recompute_checkpoint_root() {
  root_from_hashes "$CHECKPOINT_DIR" '*.checkpoint.env' 'Checkpoint Export Root' > "$checkpoint_root_file"
}

recompute_replay_root() {
  root_from_hashes "$REPLAY_DIR" '*.replay.env' 'Replay Export Root' > "$replay_root_file"
}

recompute_settlement_root() {
  root_from_hashes "$SETTLEMENT_DIR" '*.settlement.env' 'Settlement Export Root' > "$settlement_root_file"
}

recompute_status_root() {
  root_from_hashes "$STATUS_DIR" '*.status.env' 'Status Root' > "$status_root_file"
}

ingest_input() {
  local input_id="$1"
  local timestamp="$2"
  local origin="$3"
  local payload="$4"
  local payload_hash
  payload_hash="$(printf '%s' "$payload" | sha256_text)"
  write_hash_envelope "$INPUT_DIR/$input_id.input.env" \
    "Envelope Type: HotPocket Input" \
    "Input Identifier: $(safe_value "$input_id")" \
    "Timestamp: $(safe_value "$timestamp")" \
    "Origin: $(safe_value "$origin")" \
    "Payload: $(safe_value "$payload")" \
    "Payload Hash: $payload_hash" \
    "Serialization: canonical-field-order-v0.1" \
    "Ordering: lexicographic-by-input-identifier" \
    "Replay Safe: true"
  recompute_input_root
}

export_output() {
  local output_id="$1"
  local result="$2"
  local status="$3"
  local root="$4"
  write_hash_envelope "$OUTPUT_DIR/$output_id.output.env" \
    "Envelope Type: HotPocket Output" \
    "Output Identifier: $(safe_value "$output_id")" \
    "Result: $(safe_value "$result")" \
    "Status: $(safe_value "$status")" \
    "Root: $(safe_value "$root")" \
    "Serialization: canonical-field-order-v0.1" \
    "Ordering: lexicographic-by-output-identifier"
  recompute_output_root
}

latest_checkpoint_path() {
  node_file_value "$NODE_DIR/checkpoints/latest_checkpoint" "NONE"
}

export_checkpoint() {
  local checkpoint_id="${1:-checkpoint-export-v0.1}"
  local checkpoint_path continuity_root checkpoint_hash
  checkpoint_path="$(latest_checkpoint_path)"
  continuity_root="$(node_file_value "$NODE_DIR/state/latest_continuity_root" "GENESIS")"
  if [[ "$checkpoint_path" != "NONE" && -f "$checkpoint_path" ]]; then
    checkpoint_hash="$(sha256_file "$checkpoint_path")"
  else
    checkpoint_hash="$(printf 'checkpoint:%s:%s\n' "$checkpoint_id" "$continuity_root" | sha256_text)"
  fi
  write_hash_envelope "$CHECKPOINT_DIR/$checkpoint_id.checkpoint.env" \
    "Envelope Type: HotPocket Checkpoint Export" \
    "Checkpoint Identifier: $(safe_value "$checkpoint_id")" \
    "Checkpoint Path: $(safe_value "$(relative_to_root "$checkpoint_path")")" \
    "Continuity Root: $(safe_value "$continuity_root")" \
    "Checkpoint Hash: $checkpoint_hash" \
    "Authority: EverArcade Protocol Node" \
    "HotPocket Mutates State: false"
  recompute_checkpoint_root
}

export_replay() {
  local replay_id="${1:-replay-export-v0.1}"
  local replay_root replay_hash
  replay_root="$(node_file_value "$NODE_DIR/state/latest_replay_root" "GENESIS")"
  replay_hash="$(printf 'replay:%s:%s\n' "$replay_id" "$replay_root" | sha256_text)"
  write_hash_envelope "$REPLAY_DIR/$replay_id.replay.env" \
    "Envelope Type: HotPocket Replay Export" \
    "Replay Identifier: $(safe_value "$replay_id")" \
    "Replay Root: $(safe_value "$replay_root")" \
    "Replay Hash: $replay_hash" \
    "Replay Safe: true" \
    "HotPocket Mutates Replay Root: false"
  recompute_replay_root
}

export_settlement() {
  local settlement_id="${1:-settlement-export-v0.1}"
  local settlement_root settlement_hash replay_root continuity_root
  replay_root="$(node_file_value "$NODE_DIR/state/latest_replay_root" "GENESIS")"
  continuity_root="$(node_file_value "$NODE_DIR/state/latest_continuity_root" "GENESIS")"
  settlement_root="$(printf 'settlement-export:%s:%s\n' "$replay_root" "$continuity_root" | sha256_text)"
  settlement_hash="$(printf 'settlement:%s:%s\n' "$settlement_id" "$settlement_root" | sha256_text)"
  write_hash_envelope "$SETTLEMENT_DIR/$settlement_id.settlement.env" \
    "Envelope Type: HotPocket Settlement Export" \
    "Settlement Identifier: $(safe_value "$settlement_id")" \
    "Settlement Root: $settlement_root" \
    "Settlement Hash: $settlement_hash" \
    "XRPL Execution: disabled" \
    "Signing: disabled" \
    "Networking: disabled" \
    "HotPocket Mutates Settlement Root: false"
  recompute_settlement_root
}

export_status() {
  local status_id="status-export-v0.1"
  local runtime_status checkpoint_status replay_status world_status checkpoint_path
  runtime_status="$(node_env_value "$NODE_DIR/state/runtime_state.env" NODE_STATUS unknown | tail -n1)"
  world_status="$(node_env_value "$NODE_DIR/worlds/$WORLD_ID/world_state.env" WORLD_STATUS unknown | tail -n1)"
  checkpoint_path="$(latest_checkpoint_path)"
  checkpoint_status="absent"
  [[ "$checkpoint_path" != "NONE" && -f "$checkpoint_path" ]] && checkpoint_status="exportable"
  replay_status="absent"
  [[ -f "$NODE_DIR/state/latest_replay_root" ]] && replay_status="exportable"
  write_hash_envelope "$STATUS_DIR/$status_id.status.env" \
    "Envelope Type: HotPocket Status Export" \
    "Node Status: $(safe_value "$runtime_status")" \
    "Checkpoint Status: $checkpoint_status" \
    "Replay Status: $replay_status" \
    "World Status: $(safe_value "$world_status")" \
    "Adapter Status: ready" \
    "HotPocket Authority: execution-environment-only"
  recompute_status_root
}

export_all() {
  init_layout
  export_checkpoint checkpoint-export-v0.1
  export_replay replay-export-v0.1
  export_settlement settlement-export-v0.1
  export_status
}

command="${1:-}"
case "$command" in
  init-layout) init_layout ;;
  ingest-input) shift; [[ $# -eq 4 ]] || { usage >&2; exit 2; }; ensure_hotpocket_layout; ingest_input "$@" ;;
  export-output) shift; [[ $# -eq 4 ]] || { usage >&2; exit 2; }; ensure_hotpocket_layout; export_output "$@" ;;
  export-checkpoint) shift; ensure_hotpocket_layout; export_checkpoint "${1:-checkpoint-export-v0.1}" ;;
  export-replay) shift; ensure_hotpocket_layout; export_replay "${1:-replay-export-v0.1}" ;;
  export-settlement) shift; ensure_hotpocket_layout; export_settlement "${1:-settlement-export-v0.1}" ;;
  export-status) ensure_hotpocket_layout; export_status ;;
  export-all) export_all ;;
  -h|--help|help) usage ;;
  *) usage >&2; exit 2 ;;
esac
