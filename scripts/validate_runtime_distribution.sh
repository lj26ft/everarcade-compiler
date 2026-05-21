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
"$WORK/runtime/bin/everarcade-host" verify --state "$WORK/runtime/world/state" || true
"$WORK/runtime/bin/everarcade-host" verify-world --world-root "$WORK/runtime/world"
"$WORK/runtime/bin/everarcade-host" replay-world --world-root "$WORK/runtime/world"
[[ -f "$WORK/runtime/world/genesis/genesis.json" ]]
"$WORK/runtime/bin/everarcade-host" federation-status --world-root "$WORK/runtime/world"
"$WORK/runtime/bin/everarcade-host" federation-inspect-topology --world-root "$WORK/runtime/world"
"$WORK/runtime/bin/everarcade-host" federation-sync --world-root "$WORK/runtime/world" --peer 10.0.0.2:9222
"$WORK/runtime/bin/everarcade-host" federation-reconcile --world-root "$WORK/runtime/world"
"$WORK/runtime/bin/everarcade-host" federation-verify-peer --peer 10.0.0.2:9222
"$WORK/runtime/bin/everarcade-host" federation-runtime-status --world-root "$WORK/runtime/world"
"$WORK/runtime/bin/everarcade-host" federation-monitor --world-root "$WORK/runtime/world"
"$WORK/runtime/bin/everarcade-host" federation-recover-runtime --world-root "$WORK/runtime/world"
"$WORK/runtime/bin/everarcade-host" federation-verify-convergence --world-root "$WORK/runtime/world"

"$WORK/runtime/bin/everarcade-host" world-status --world-root "$WORK/runtime/world"
"$WORK/runtime/bin/everarcade-host" world-timeline --world-root "$WORK/runtime/world"
"$WORK/runtime/bin/everarcade-host" world-verify --world-root "$WORK/runtime/world" || true
"$WORK/runtime/bin/everarcade-host" entity-status --world-root "$WORK/runtime/world" --entity player-001
"$WORK/runtime/bin/everarcade-host" entity-migrate --world-root "$WORK/runtime/world" --entity player-001 --target-peer 10.0.0.2:9222

"$WORK/runtime/bin/everarcade-host" partition-status --world-root "$WORK/runtime/world"
"$WORK/runtime/bin/everarcade-host" region-status --world-root "$WORK/runtime/world"
"$WORK/runtime/bin/everarcade-host" orchestration-status --world-root "$WORK/runtime/world"
"$WORK/runtime/bin/everarcade-host" entity-route --world-root "$WORK/runtime/world" --entity player-001
"$WORK/runtime/bin/everarcade-host" partition-migrate --world-root "$WORK/runtime/world" --entity player-001 --target-region region-b

echo "validate_runtime_distribution=ok archive=$ARCHIVE"

"$WORK/runtime/bin/everarcade-host" federation-runtime-health --world-root "$WORK/runtime/world"
"$WORK/runtime/bin/everarcade-host" federation-supervisor-status --world-root "$WORK/runtime/world"
"$WORK/runtime/bin/everarcade-host" federation-recovery-history --world-root "$WORK/runtime/world"
"$WORK/runtime/bin/everarcade-host" federation-verify-integrity --world-root "$WORK/runtime/world"

# Developer platform CLI validation
"$WORK/runtime/bin/everarcade" init-game test-world
"$WORK/runtime/bin/everarcade" build-game
"$WORK/runtime/bin/everarcade" run-local-federation
"$WORK/runtime/bin/everarcade" replay-world

"$WORK/runtime/bin/everarcade" inspect-simulation
echo "runtime_distribution_replay_workflow=ok"


# Zero-friction onboarding validation flow
bash "$ROOT/scripts/everarcade_start.sh"
bash "$ROOT/scripts/doctor_quick.sh"
"$WORK/runtime/bin/everarcade" start
"$WORK/runtime/bin/everarcade" doctor
"$WORK/runtime/bin/everarcade" demo
"$WORK/runtime/bin/everarcade" reset
echo "runtime_distribution_bootstrap_validation=ok"
