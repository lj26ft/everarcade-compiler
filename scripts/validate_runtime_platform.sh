#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

LOCAL_CARGO_CONFIG="$ROOT/.cargo/config.toml"
DISABLED_CARGO_CONFIG=""
if [[ -f "$LOCAL_CARGO_CONFIG" ]] && grep -q 'replace-with = "vendored-sources"' "$LOCAL_CARGO_CONFIG"; then
  DISABLED_CARGO_CONFIG="$LOCAL_CARGO_CONFIG.runtime-validation-disabled"
  mv "$LOCAL_CARGO_CONFIG" "$DISABLED_CARGO_CONFIG"
  trap '[[ -n "$DISABLED_CARGO_CONFIG" && -f "$DISABLED_CARGO_CONFIG" ]] && mv "$DISABLED_CARGO_CONFIG" "$LOCAL_CARGO_CONFIG"' EXIT
fi

REPORT_DIR="${EVERARCADE_RUNTIME_REPORT_DIR:-$ROOT/target/runtime-platform-validation}"
mkdir -p "$REPORT_DIR"
REPORT="$REPORT_DIR/validation-report.txt"
: > "$REPORT"

run_gate() {
  local name="$1"; shift
  echo "== $name ==" | tee -a "$REPORT"
  "$@" 2>&1 | tee -a "$REPORT"
}

run_gate "Fresh Install" cargo check -p everarcade-runtime --lib
run_gate "Package Verification" cargo test -p everarcade-runtime test_package_validation
run_gate "Runtime Startup" cargo test -p everarcade-runtime test_runtime_boot
run_gate "Tick Execution" cargo test -p everarcade-runtime test_tick_execution
run_gate "Journal Validation" cargo test -p everarcade-runtime test_journal_integrity
run_gate "Checkpoint Validation" cargo test -p everarcade-runtime test_checkpoint_restore
run_gate "Replay Validation" cargo test -p everarcade-runtime test_replay_equivalence
run_gate "Crash Recovery" cargo test -p everarcade-runtime test_crash_recovery
run_gate "Backup Validation" cargo test -p everarcade-runtime test_backup_restore
run_gate "Restore Validation" cargo test -p everarcade-runtime test_runtime_shutdown_recovery
run_gate "Upgrade Validation" cargo test -p everarcade-runtime test_upgrade_rollback
run_gate "Runtime Shutdown" cargo test -p everarcade-runtime test_runtime_shutdown_recovery

echo "Runtime platform validation passed. Report: $REPORT" | tee -a "$REPORT"
