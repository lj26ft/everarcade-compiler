#!/usr/bin/env bash
set -euo pipefail
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/lib_hotpocket_deployment_proof.sh"
run_hotpocket_deployment_proof discovery
