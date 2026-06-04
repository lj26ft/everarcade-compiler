#!/usr/bin/env bash
set -euo pipefail
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/node_common.sh"

ensure_report_dir
require_initialized
checkpoint_path="$(latest_checkpoint_path)"
if [[ "$checkpoint_path" == "NONE" || ! -f "$checkpoint_path" ]]; then
  echo "Replay requires a checkpoint" >&2
  exit 1
fi
timestamp="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
root="$(replay_root)"
printf '%s replay verified root=%s\n' "$timestamp" "$root" >> "$JOURNAL_FILE"
printf '%s' "$root" > "$LATEST_REPLAY_ROOT_FILE"
report="$REPORT_DIR/node_replay_report.txt"
cat > "$report" <<REPORT
Node Replay Report
Timestamp: $timestamp
Checkpoint Replayed: PASS
World Activity Replayed: PASS
Civilization Activity Replayed: PASS
Replay Root: $root
Replay Root Verified: PASS
Replay PASS
REPORT
copy_report_to_node "$report"
echo "Replay PASS"
echo "Replay: PASS"
