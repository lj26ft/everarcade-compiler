#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DIST="$ROOT/dist"; BUILD="$ROOT/.everarcade-live-proof/package-build"; PKG="$DIST/everarcade-live-gameplay-proof.tar.gz"
rm -rf "$BUILD"; mkdir -p "$BUILD/everarcade-live-gameplay-proof" "$DIST"
BASE="$BUILD/everarcade-live-gameplay-proof"
mkdir -p "$BASE/everarcade-runtime" "$BASE/creator-sdk" "$BASE/arena-runtime-package" "$BASE/operator-scripts" "$BASE/health-scripts" "$BASE/recovery-scripts" "$BASE/frontend/arena-live-client"
cp -a "$ROOT/runtime/everarcade-runtime"/. "$BASE/everarcade-runtime/" 2>/dev/null || true
cp -a "$ROOT/creator-sdk"/. "$BASE/creator-sdk/" 2>/dev/null || true
cp "$ROOT/scripts/lib/arena_live_runtime.mjs" "$BASE/arena-runtime-package/"
cp "$ROOT/scripts/start_live_arena_session.sh" "$ROOT/scripts/probe_evernode_environment.sh" "$BASE/operator-scripts/"
cp "$ROOT/scripts/validate_hotpocket_runtime_boundary.sh" "$ROOT/scripts/validate_frontend_access.sh" "$BASE/health-scripts/"
cp "$ROOT/scripts/validate_live_session_recovery.sh" "$ROOT/scripts/probe_network_failure_behavior.sh" "$BASE/recovery-scripts/"
cp -a "$ROOT/frontend/arena-live-client"/. "$BASE/frontend/arena-live-client/"
cat > "$BASE/README.txt" <<README
EverArcade Live Gameplay Proof v0.1 deployable lease package.
Includes a minimal Arena runtime, creator SDK assets, operator/health/recovery scripts, and browser client.
README
( cd "$BUILD" && tar -czf "$PKG" everarcade-live-gameplay-proof )
echo "Package: $PKG"
echo "Package Layout: PASS"
