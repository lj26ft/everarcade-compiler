#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-1}"
CARGO_FLAGS=()
for arg in "$@"; do
  case "$arg" in
    --offline|--locked|--frozen) CARGO_FLAGS+=("$arg") ;;
    *) echo "unknown argument: $arg" >&2; exit 2 ;;
  esac
done
REPORT="deployment/reports/multinode_federation_load_gate_run.md"
test -f deployment/evernode/runtime/multinode-federation-load-manifest.toml
cargo test -p execution-core --test evernode_deployment_tests test_multinode_federation_load_gate "${CARGO_FLAGS[@]}"
cat > "$REPORT" <<'REPORT_EOF'
# Multi-Node Federation Load Gate Run

status: passed
nodes: 4
transport: tcp
quorum: passed
checkpoint sync: passed
replay sync: passed
recovery continuity: passed
load balance: passed
capacity: passed
REPORT_EOF
echo "multi-node federation load gate passed; report=$REPORT"
