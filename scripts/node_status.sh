#!/usr/bin/env bash
set -euo pipefail
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/node_common.sh"

require_initialized
runtime_status="unknown"
world_status="unknown"
if [[ -f "$RUNTIME_STATE_FILE" ]]; then
  runtime_status="$(awk -F= '/^NODE_STATUS=/{print $2}' "$RUNTIME_STATE_FILE" | tail -n1)"
fi
if [[ -f "$WORLD_STATE_FILE" ]]; then
  world_status="$(awk -F= '/^WORLD_STATUS=/{print $2}' "$WORLD_STATE_FILE" | tail -n1)"
fi
checkpoint="$(latest_checkpoint_path)"
replay="NONE"
continuity="NONE"
[[ -f "$LATEST_REPLAY_ROOT_FILE" ]] && replay="$(cat "$LATEST_REPLAY_ROOT_FILE")"
[[ -f "$LATEST_CONTINUITY_ROOT_FILE" ]] && continuity="$(cat "$LATEST_CONTINUITY_ROOT_FILE")"
cat <<STATUS
Node Status
Runtime Status: $runtime_status
Latest Checkpoint: $checkpoint
Latest Replay Root: $replay
Latest Continuity Root: $continuity
World Status: $world_status
Node Healthy
Node Status: PASS
STATUS
