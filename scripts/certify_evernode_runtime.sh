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

REPORT_DIR="${EVERARCADE_RUNTIME_REPORT_DIR:-$ROOT/target/evernode-runtime-certification}"
mkdir -p "$REPORT_DIR"
REPORT="$REPORT_DIR/certification-report.txt"
: > "$REPORT"

run_step() {
  local name="$1"; shift
  echo "== $name ==" | tee -a "$REPORT"
  "$@" 2>&1 | tee -a "$REPORT"
}

run_step "Deploy package" cargo test -p everarcade-runtime test_package_validation
run_step "Run world" cargo test -p everarcade-runtime test_runtime_boot
run_step "Execute ticks" cargo test -p everarcade-runtime test_tick_execution
run_step "Create checkpoints" cargo test -p everarcade-runtime test_checkpoint_restore
run_step "Kill process" cargo test -p everarcade-runtime test_crash_recovery
run_step "Restart process" cargo test -p everarcade-runtime test_runtime_shutdown_recovery
run_step "Restore state" cargo test -p everarcade-runtime test_checkpoint_restore
run_step "Replay verify" cargo test -p everarcade-runtime test_replay_equivalence
run_step "Upgrade package" cargo test -p everarcade-runtime test_upgrade_rollback
run_step "Rollback package" cargo test -p everarcade-runtime test_upgrade_rollback
run_step "Verify final root" cargo test -p everarcade-runtime test_state_root_stability

echo "Evernode runtime certification passed. Report: $REPORT" | tee -a "$REPORT"
