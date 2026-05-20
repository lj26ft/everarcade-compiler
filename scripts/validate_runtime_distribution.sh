#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DIST_DIR="${DIST_DIR:-$ROOT/dist}"
WORK="$(mktemp -d /tmp/everarcade-runtime-validate.XXXXXX)"
trap 'rm -rf "$WORK"' EXIT

bash "$ROOT/scripts/build_runtime_layout.sh" "$WORK/runtime"

ARCHIVE="$DIST_DIR/everarcade-runtime-v0.1.0.tar.gz"
mkdir -p "$DIST_DIR"

tar --sort=name --mtime='UTC 2020-01-01' --owner=0 --group=0 --numeric-owner -C "$WORK" -czf "$ARCHIVE" runtime

tar -xzf "$ARCHIVE" -C "$WORK"
mkdir -p "$WORK/runtime/world/federation/runtime" "$WORK/runtime/world/federation/supervisor" "$WORK/runtime/world/federation/recovery" "$WORK/runtime/world/federation/history" "$WORK/runtime/world/federation/sessions" "$WORK/runtime/world/federation/advancement" "$WORK/runtime/world/topology" "$WORK/runtime/world/leases"

bash "$WORK/runtime/scripts/start-everarcade.sh"
"$WORK/runtime/bin/everarcade-host" verify --state "$WORK/runtime/world/state"
"$WORK/runtime/bin/everarcade-host" verify-world --world-root "$WORK/runtime/world"
"$WORK/runtime/bin/everarcade-host" replay-world --world-root "$WORK/runtime/world"
"$WORK/runtime/bin/everarcade-host" federation-status --world-root "$WORK/runtime/world"
"$WORK/runtime/bin/everarcade-host" federation-inspect-topology --world-root "$WORK/runtime/world"
"$WORK/runtime/bin/everarcade-host" federation-sync --world-root "$WORK/runtime/world" --peer 10.0.0.2:9222
"$WORK/runtime/bin/everarcade-host" federation-reconcile --world-root "$WORK/runtime/world"
"$WORK/runtime/bin/everarcade-host" federation-verify-peer --peer 10.0.0.2:9222
"$WORK/runtime/bin/everarcade-host" federation-runtime-status --world-root "$WORK/runtime/world"
"$WORK/runtime/bin/everarcade-host" federation-monitor --world-root "$WORK/runtime/world"
"$WORK/runtime/bin/everarcade-host" federation-recover-runtime --world-root "$WORK/runtime/world"
"$WORK/runtime/bin/everarcade-host" federation-verify-convergence --world-root "$WORK/runtime/world"

echo "validate_runtime_distribution=ok archive=$ARCHIVE"

"$WORK/runtime/bin/everarcade-host" federation-runtime-health --world-root "$WORK/runtime/world"
"$WORK/runtime/bin/everarcade-host" federation-supervisor-status --world-root "$WORK/runtime/world"
"$WORK/runtime/bin/everarcade-host" federation-recovery-history --world-root "$WORK/runtime/world"
"$WORK/runtime/bin/everarcade-host" federation-verify-integrity --world-root "$WORK/runtime/world"
