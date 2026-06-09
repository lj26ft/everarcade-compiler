#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PROJECT="$ROOT/contracts/arena-proof-contract"
RUNTIME_ROOT="$ROOT/target/wasm-guest-runtime-root"
REPORT="$ROOT/reports/wasm_guest_execution_validation_report.txt"
LOG_DIR="$ROOT/target/wasm-guest-validation"
LOG="$LOG_DIR/execute-guest.log"
mkdir -p "$ROOT/reports" "$LOG_DIR"
rm -rf "$RUNTIME_ROOT" "$PROJECT/dist"

status() { printf '%s: %s\n' "$1" "$2"; }
fail_report() {
  {
    status "Guest Build" "${guest_build:-FAIL}"
    status "Guest Package" "${guest_package:-FAIL}"
    status "Guest Load" "${guest_load:-FAIL}"
    status "Guest Execute" "${guest_execute:-FAIL}"
    status "Guest State Mutation" "${guest_state_mutation:-FAIL}"
    status "Guest Receipt Generation" "${guest_receipt:-FAIL}"
    status "Guest Journal Generation" "${guest_journal:-FAIL}"
    status "Guest Replay Generation" "${guest_replay_generation:-FAIL}"
    status "Guest Replay Verification" "${guest_replay_verification:-FAIL}"
  } > "$REPORT"
}
trap 'fail_report' ERR

guest_build=FAIL
guest_package=FAIL
guest_load=FAIL
guest_execute=FAIL
guest_state_mutation=FAIL
guest_receipt=FAIL
guest_journal=FAIL
guest_replay_generation=FAIL
guest_replay_verification=FAIL

CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" node "$ROOT/creator-sdk/cli/everarcade.mjs" execute-guest \
  --project "$PROJECT" \
  --runtime-root "$RUNTIME_ROOT" | tee "$LOG"

guest_build=PASS
[[ -s "$PROJECT/dist/runtime-package/world.wasm" ]] && [[ "$(od -An -tx1 -N4 "$PROJECT/dist/runtime-package/world.wasm" | tr -d " \n")" == "0061736d" ]]
guest_package=PASS
jq -e '.package_classification == "wasm-guest-runtime-package"' "$PROJECT/dist/runtime-package/world.json" >/dev/null
guest_load=PASS
jq -e '.status == "WASM Guest Execution: PASS"' "$RUNTIME_ROOT/replay/guest-replay-proof.json" >/dev/null
guest_execute=PASS
jq -e '.state_root_changed == true and .state_mutation_origin == "guest_output"' "$RUNTIME_ROOT/replay/guest-replay-proof.json" >/dev/null
guest_state_mutation=PASS
jq -e '.payload.guest_id and .payload.guest_hash and .payload.guest_output_hash and .payload.state_root and .payload.receipt_hash' "$RUNTIME_ROOT/receipts/receipt-00000000000000000001.json" >/dev/null
guest_receipt=PASS
jq -e 'select(.guest_id and .guest_output_hash and .state_root and .receipt_hash)' "$RUNTIME_ROOT/journals/journal.jsonl" >/dev/null
guest_journal=PASS
jq -e '.guest_hash and .guest_output_hash and .state_root and .replay_root' "$RUNTIME_ROOT/replay/guest-replay-proof.json" >/dev/null
guest_replay_generation=PASS
jq -e '.replay_verification == "PASS" and .replay_root == .state_root' "$RUNTIME_ROOT/replay/guest-replay-proof.json" >/dev/null
guest_replay_verification=PASS
trap - ERR
fail_report
cat "$REPORT"
