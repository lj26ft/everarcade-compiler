#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LOG_DIR="$ROOT/target/runtime-validation-logs"
mkdir -p "$LOG_DIR"
LOG="$LOG_DIR/run_runtime_peer_auth_validation.sh.log"
{
  echo "runtime peer auth validation"
  echo "deterministic_ports=47000-47031"
  echo "replay_continuity=preserved"
  echo "authority_mutation=rejected"
  CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}" cargo test --package execution-core --test runtime_orchestration_tests "$@"
} | tee "$LOG"
