#!/usr/bin/env bash
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

REPORT_PATH="reports/deterministic_execution_validation_report.txt"
WORK_DIR="${EVERARCADE_DETERMINISTIC_EXECUTION_WORK_DIR:-/tmp/everarcade-deterministic-execution-validation}"
PROJECT_DIR="$WORK_DIR/arena-project"
RUNTIME_ROOT="$WORK_DIR/runtime-root"
CLI="creator-sdk/cli/everarcade.mjs"

mkdir -p reports
rm -rf "$WORK_DIR"
mkdir -p "$WORK_DIR"

input_submitted_status="FAIL"
tick_executed_status="FAIL"
state_mutation_status="FAIL"
receipt_generated_status="FAIL"
journal_generated_status="FAIL"
checkpoint_generated_status="FAIL"
replay_root_generated_status="FAIL"
replay_verified_status="FAIL"
overall_status="FAIL"

node "$CLI" new --name arena-deterministic-proof --template sandbox --dir "$PROJECT_DIR" >/dev/null
node "$CLI" build --project "$PROJECT_DIR" >/dev/null
node "$CLI" test --project "$PROJECT_DIR" >/dev/null
node "$CLI" package --project "$PROJECT_DIR" >/dev/null
node "$CLI" launch-local --project "$PROJECT_DIR" --runtime-root "$RUNTIME_ROOT" >/dev/null
execute_output="$(node "$CLI" execute-local --project "$PROJECT_DIR" --runtime-root "$RUNTIME_ROOT")"

proof_path="$RUNTIME_ROOT/replay/replay-proof.json"
receipt_path="$RUNTIME_ROOT/receipts/receipt-00000000000000000001.json"
journal_path="$RUNTIME_ROOT/journals/journal.jsonl"
checkpoint_path="$(find "$RUNTIME_ROOT/checkpoints" -maxdepth 1 -type f -name 'checkpoint-*.json' | sort | tail -n 1 || true)"

if [[ -f "$proof_path" ]]; then
  eval "$(python3 - "$proof_path" "$receipt_path" "$journal_path" "$checkpoint_path" <<'PY'
import json, pathlib, sys
proof_path, receipt_path, journal_path, checkpoint_path = [pathlib.Path(p) for p in sys.argv[1:]]
proof = json.loads(proof_path.read_text()) if proof_path.exists() else {}
receipt_envelope = json.loads(receipt_path.read_text()) if receipt_path.exists() else {}
receipt = receipt_envelope.get('payload', receipt_envelope)
journal_lines = [l for l in journal_path.read_text().splitlines() if l.strip()] if journal_path.exists() else []
checkpoint = json.loads(checkpoint_path.read_text()) if checkpoint_path.exists() else {}
def status(name, ok):
    print(f'{name}={"PASS" if ok else "FAIL"}')
status('input_submitted_status', bool(proof.get('input_hash')) and proof.get('input', {}).get('player_id') == 'audit-player')
status('tick_executed_status', proof.get('ticks_executed') == 1 and proof.get('tick') == 1 and proof.get('tick_count_increased') is True)
status('state_mutation_status', proof.get('state_root_changed') is True and bool(proof.get('execution_root')))
status('receipt_generated_status', receipt.get('receipt_id') == 'receipt-00000000000000000001' and receipt.get('input_hash') == proof.get('input_hash') and receipt.get('state_root') == proof.get('execution_root'))
status('journal_generated_status', len(journal_lines) >= 1 and proof.get('journal_length', 0) >= 1)
status('checkpoint_generated_status', bool(checkpoint.get('payload', {}).get('manifest', {}).get('checkpoint_hash')) and proof.get('checkpoint_hash') == checkpoint.get('payload', {}).get('manifest', {}).get('checkpoint_hash'))
status('replay_root_generated_status', bool(proof.get('replay_root')))
status('replay_verified_status', proof.get('replay_verification') == 'PASS' and proof.get('replay_root') == proof.get('execution_root'))
PY
)"
fi

if [[ "$input_submitted_status" == "PASS" \
  && "$tick_executed_status" == "PASS" \
  && "$state_mutation_status" == "PASS" \
  && "$receipt_generated_status" == "PASS" \
  && "$journal_generated_status" == "PASS" \
  && "$checkpoint_generated_status" == "PASS" \
  && "$replay_root_generated_status" == "PASS" \
  && "$replay_verified_status" == "PASS" ]]; then
  overall_status="PASS"
fi

cat > "$REPORT_PATH" <<REPORT
Deterministic Execution Validation Report
Work Dir: $WORK_DIR
Runtime Root: $RUNTIME_ROOT
Proof Path: $proof_path
Receipt Path: $receipt_path
Journal Path: $journal_path
Checkpoint Path: ${checkpoint_path:-NONE}
Creator Execute Output: $execute_output

Input Submitted: $input_submitted_status
Tick Executed: $tick_executed_status
State Mutation: $state_mutation_status
Receipt Generated: $receipt_generated_status
Journal Generated: $journal_generated_status
Checkpoint Generated: $checkpoint_generated_status
Replay Root Generated: $replay_root_generated_status
Replay Verified: $replay_verified_status
Overall Result: $overall_status
REPORT

cat "$REPORT_PATH"
[[ "$overall_status" == "PASS" ]]
