#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PROOF_DIR="$ROOT/runtime/hotpocket-contract-proof"
ensure_hotpocket_execution_deps() {
  if [[ ! -d "$PROOF_DIR/node_modules/hotpocket-js-client" || ! -d "$PROOF_DIR/node_modules/hotpocket-nodejs-contract" ]]; then
    npm install --prefix "$PROOF_DIR" --no-audit --no-fund
  fi
}
run_hotpocket_execution_validator() {
  local command="$1"
  ensure_hotpocket_execution_deps
  (cd "$PROOF_DIR" && node validation/hotpocket-proof.js "$command")
}
