#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
exec "$ROOT/deployment/evernode/runtime-proof-package/scripts/prove-lease-execution.sh" "$@"
