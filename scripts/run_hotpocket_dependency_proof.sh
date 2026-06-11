#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$ROOT/scripts/lib_hotpocket_deployment_proof.sh"
run_hotpocket_deployment_proof dependencies
