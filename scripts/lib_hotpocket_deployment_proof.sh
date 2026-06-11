#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PROOF_DIR="$ROOT/runtime/hotpocket-deployment-proof"
run_hotpocket_deployment_proof() {
  local command="$1"
  (cd "$PROOF_DIR" && node validation/hotpocket-deployment-proof.js "$command")
}
