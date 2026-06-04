#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"; cd "$ROOT"
REPORT_DIR="$ROOT/reports"; REPORT="$REPORT_DIR/evernode_recovery_report.txt"; mkdir -p "$REPORT_DIR" evernode/reports
checkpoint_exists="FAIL"; checkpoint_valid="FAIL"; replay_status="FAIL"; restore_status="FAIL"; continuity_status="FAIL"; overall="FAIL"
checkpoint_path="$(cat node/checkpoints/latest_checkpoint 2>/dev/null || printf NONE)"
[[ "$checkpoint_path" != NONE && -s "$checkpoint_path" ]] && checkpoint_exists="PASS"
if [[ "$checkpoint_exists" == PASS ]] && grep -q '^CHECKPOINT_ID=' "$checkpoint_path" && grep -q '^STATE_ROOT=' "$checkpoint_path"; then checkpoint_valid="PASS"; fi
if [[ "$checkpoint_valid" == PASS ]]; then
  if bash scripts/node_replay.sh >/dev/null; then replay_status="PASS"; fi
  if bash scripts/node_restore.sh >/dev/null; then restore_status="PASS"; fi
fi
[[ -s node/state/latest_continuity_root ]] && continuity_status="PASS"
if [[ "$checkpoint_exists" == PASS && "$checkpoint_valid" == PASS && "$replay_status" == PASS && "$restore_status" == PASS && "$continuity_status" == PASS ]]; then overall="PASS"; fi
cat > "$REPORT" <<REPORT_BODY
Evernode Recovery Report
Checkpoint Exists: $checkpoint_exists
Checkpoint Valid: $checkpoint_valid
Replay Verified: $replay_status
Restore Verified: $restore_status
Continuity Root Preserved: $continuity_status
Recovery: $overall
REPORT_BODY
cp "$REPORT" evernode/reports/
echo "Recovery: $overall"
[[ "$overall" == PASS ]]
